use diffusers::pipelines::stable_diffusion;
use diffusers::transformers::clip;
use tch::{nn::Module, Device, Kind, Tensor};

const GUIDANCE_SCALE: f64 = 7.5;

pub struct StableDiffusionTask {
    /// The prompt to be used for image generation.
    pub prompt: String,

    /// When set, use the CPU for the listed devices, can be 'all', 'unet', 'clip', etc.
    /// Multiple values can be set.
    pub cpu: Vec<String>,

    /// The height in pixels of the generated image.
    pub height: Option<i64>,

    /// The width in pixels of the generated image.
    pub width: Option<i64>,

    /// The UNet weight file, in .ot or .safetensors format.
    pub unet_weights: Option<String>,

    /// The CLIP weight file, in .ot or .safetensors format.
    pub clip_weights: Option<String>,

    /// The VAE weight file, in .ot or .safetensors format.
    pub vae_weights: Option<String>,

    /// The file specifying the vocabulary to used for tokenization.
    pub vocab_file: String,

    /// The size of the sliced attention or 0 for automatic slicing (disabled by default)
    pub sliced_attention_size: Option<i64>,

    /// The number of steps to run the diffusion for.
    pub n_steps: usize,

    /// The random seed to be used for the generation. Default is 0, means generate random value.
    pub seed: i64,

    /// The number of samples to generate.
    pub num_samples: i64,

    /// The name of the final image to generate.
    pub final_image: String,

    /// Use autocast (disabled by default as it may use more memory in some cases).
    pub autocast: bool,

    pub sd_version: StableDiffusionVersion,

    /// Generate intermediary images at each step.
    pub intermediary_images: bool,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum StableDiffusionVersion {
    V1_5,
    V2_1,
}

impl StableDiffusionTask {
    fn clip_weights(&self) -> String {
        match &self.clip_weights {
            Some(w) => w.clone(),
            None => match self.sd_version {
                StableDiffusionVersion::V1_5 => "data/clip-1.5.ot".to_string(),
                StableDiffusionVersion::V2_1 => "data/clip-2.1.ot".to_string(),
            },
        }
    }

    fn vae_weights(&self) -> String {
        match &self.vae_weights {
            Some(w) => w.clone(),
            None => match self.sd_version {
                StableDiffusionVersion::V1_5 => "data/vae-1.5.ot".to_string(),
                StableDiffusionVersion::V2_1 => "data/vae-2.1.ot".to_string(),
            },
        }
    }

    fn unet_weights(&self) -> String {
        match &self.unet_weights {
            Some(w) => w.clone(),
            None => match self.sd_version {
                StableDiffusionVersion::V1_5 => "data/unet-1.5.ot".to_string(),
                StableDiffusionVersion::V2_1 => "data/unet-2.1.ot".to_string(),
            },
        }
    }

    pub fn output_filename(
        basename: &str,
        sample_idx: i64,
        num_samples: i64,
        timestep_idx: Option<usize>,
    ) -> String {
        let filename = if num_samples > 1 {
            match basename.rsplit_once('.') {
                None => format!("{basename}.{sample_idx}.jpg"),
                Some((filename_no_extension, extension)) => {
                    format!("{filename_no_extension}.{sample_idx}.{extension}")
                }
            }
        } else {
            basename.to_string()
        };
        match timestep_idx {
            None => filename,
            Some(timestep_idx) => match filename.rsplit_once('.') {
                None => format!("{filename}-{timestep_idx}.jpg"),
                Some((filename_no_extension, extension)) => {
                    format!("{filename_no_extension}-{timestep_idx}.{extension}")
                }
            },
        }
    }

    pub fn run(args: StableDiffusionTask, seed: i64) -> Result<Tensor, anyhow::Error>
    {
        let clip_weights = args.clip_weights();
        let vae_weights = args.vae_weights();
        let unet_weights = args.unet_weights();
        let StableDiffusionTask {
            prompt,
            cpu,
            height,
            width,
            n_steps,
//        seed,
            vocab_file,
            final_image,
            sliced_attention_size,
            num_samples,
            sd_version,
            ..
        } = args;

        tch::maybe_init_cuda();

        println!("Cuda available: {}", tch::Cuda::is_available());
        println!("Cudnn available: {}", tch::Cuda::cudnn_is_available());
        println!("MPS available: {}", tch::utils::has_mps());

        let sd_config = match sd_version {
            StableDiffusionVersion::V1_5 => {
                stable_diffusion::StableDiffusionConfig::v1_5(sliced_attention_size, height, width)
            }
            StableDiffusionVersion::V2_1 => {
                stable_diffusion::StableDiffusionConfig::v2_1(sliced_attention_size, height, width)
            }
        };

        let device_setup = diffusers::utils::DeviceSetup::new(cpu);
        let clip_device = device_setup.get("clip");
        let vae_device = device_setup.get("vae");
        let unet_device = device_setup.get("unet");
        let scheduler = sd_config.build_scheduler(n_steps);

        let tokenizer = clip::Tokenizer::create(vocab_file, &sd_config.clip)?;
        println!("Running with prompt \"{prompt}\".");
        let tokens = tokenizer.encode(&prompt)?;
        let tokens: Vec<i64> = tokens.into_iter().map(|x| x as i64).collect();
        let tokens = Tensor::from_slice(&tokens).view((1, -1)).to(clip_device);
        let uncond_tokens = tokenizer.encode("")?;
        let uncond_tokens: Vec<i64> = uncond_tokens.into_iter().map(|x| x as i64).collect();
        let uncond_tokens = Tensor::from_slice(&uncond_tokens).view((1, -1)).to(clip_device);

        let no_grad_guard = tch::no_grad_guard();

        println!("Building the Clip transformer.");
        let text_model = sd_config.build_clip_transformer(&clip_weights, clip_device)?;
        let text_embeddings = text_model.forward(&tokens);
        let uncond_embeddings = text_model.forward(&uncond_tokens);
        let text_embeddings = Tensor::cat(&[uncond_embeddings, text_embeddings], 0).to(unet_device);

        println!("Building the autoencoder.");
        let vae = sd_config.build_vae(&vae_weights, vae_device)?;
        println!("Building the unet.");
        let unet = sd_config.build_unet(&unet_weights, unet_device, 4)?;

        let bsize = 1;
        let idx:i64 = 0;

        println!("Seed {}", seed + idx);

        tch::manual_seed(seed + idx);
        let mut latents = Tensor::randn(
            [bsize, 4, sd_config.height / 8, sd_config.width / 8],
            (Kind::Float, unet_device),
        );

        // scale the initial noise by the standard deviation required by the scheduler
        latents *= scheduler.init_noise_sigma();

        for (timestep_index, &timestep) in scheduler.timesteps().iter().enumerate() {
            println!("Timestep {timestep_index}/{n_steps}");
            let latent_model_input = Tensor::cat(&[&latents, &latents], 0);

            let latent_model_input = scheduler.scale_model_input(latent_model_input, timestep);
            let noise_pred = unet.forward(&latent_model_input, timestep as f64, &text_embeddings);
            let noise_pred = noise_pred.chunk(2, 0);
            let (noise_pred_uncond, noise_pred_text) = (&noise_pred[0], &noise_pred[1]);
            let noise_pred =
                noise_pred_uncond + (noise_pred_text - noise_pred_uncond) * GUIDANCE_SCALE;
            latents = scheduler.step(&noise_pred, timestep, &latents);

            if args.intermediary_images {
                let latents = latents.to(vae_device);
                let image = vae.decode(&(&latents / 0.18215));
                let image = (image / 2 + 0.5).clamp(0., 1.).to_device(Device::Cpu);
                let image = (image * 255.).to_kind(Kind::Uint8);
                let final_image =
                    Self::output_filename(&final_image, idx + 1, num_samples, Some(timestep_index + 1));
                tch::vision::image::save(&image, final_image)?;
            }
        }

        println!("Generating the final image for sample {}/{}.", idx + 1, num_samples);
        let latents = latents.to(vae_device);
        let image = vae.decode(&(&latents / 0.18215));
        let image = (image / 2 + 0.5).clamp(0., 1.).to_device(Device::Cpu);
        let image = (image * 255.).to_kind(Kind::Uint8);

        drop(no_grad_guard);

        return Ok(image);
    }

}

pub extern fn testlibrary()
{
    println!("test")
}