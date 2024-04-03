use clap::{Parser};
use std::arch::x86_64::_rdrand64_step;
use tch::Tensor;
use enso_darknet::{StableDiffusionTask, StableDiffusionVersion};

#[derive(Parser)]
#[command(author, version, about = "Stable Diffusion 2.1 CLI", long_about = None)]
pub struct Args {
    /// The prompt to be used for image generation.
    #[arg(
    long,
    default_value = "A very realistic photo of a rusty robot walking on a sandy beach"
    )]
    pub prompt: String,

    /// When set, use the CPU for the listed devices, can be 'all', 'unet', 'clip', etc.
    /// Multiple values can be set.
    #[arg(long)]
    cpu: Vec<String>,

    /// The height in pixels of the generated image.
    #[arg(long)]
    height: Option<i64>,

    /// The width in pixels of the generated image.
    #[arg(long)]
    width: Option<i64>,

    /// The UNet weight file, in .ot or .safetensors format.
    #[arg(long, value_name = "FILE")]
    unet_weights: Option<String>,

    /// The CLIP weight file, in .ot or .safetensors format.
    #[arg(long, value_name = "FILE")]
    clip_weights: Option<String>,

    /// The VAE weight file, in .ot or .safetensors format.
    #[arg(long, value_name = "FILE")]
    vae_weights: Option<String>,

    #[arg(long, value_name = "FILE", default_value = "data/vocab_16e6.txt")]
    /// The file specifying the vocabulary to used for tokenization.
    vocab_file: String,

    /// The size of the sliced attention or 0 for automatic slicing (disabled by default)
    #[arg(long)]
    sliced_attention_size: Option<i64>,

    /// The number of steps to run the diffusion for.
    #[arg(long, default_value_t = 30)]
    n_steps: usize,

    /// The random seed to be used for the generation. Default is 0, means generate random value.
    #[arg(long, default_value_t = 0)]
    pub seed: i64,

    /// The number of samples to generate.
    #[arg(long, default_value_t = 1)]
    num_samples: i64,

    /// The name of the final image to generate.
    #[arg(long, value_name = "FILE", default_value = "output.png")]
    pub final_image: String,

    /// Use autocast (disabled by default as it may use more memory in some cases).
    #[arg(long, action)]
    pub autocast: bool,

    #[arg(long, value_enum, default_value = "v2-1")]
    sd_version: StableDiffusionVersion,

    /// Generate intermediary images at each step.
    #[arg(long, action)]
    intermediary_images: bool,
}

fn run(args: Args, seed: i64) -> anyhow::Result<()>
{
    let task = StableDiffusionTask {
        prompt: args.prompt,
        cpu: args.cpu,
        height: args.height,
        width: args.width,
        unet_weights: args.unet_weights,
        clip_weights: args.clip_weights,
        vae_weights: args.vae_weights,
        vocab_file: args.vocab_file,
        sliced_attention_size: args.sliced_attention_size,
        n_steps: args.n_steps,
        seed: seed.clone(),
        num_samples: args.num_samples,
        final_image: args.final_image,
        autocast: args.autocast,
        sd_version: args.sd_version,
        intermediary_images: args.intermediary_images
    };
    let final_image = task.final_image.clone();
    let image: Tensor = StableDiffusionTask::run(task, seed)?;

    let final_image = StableDiffusionTask::output_filename(final_image.as_str(), 1, 0, None);

    tch::vision::image::save(&image, final_image)?;

    Ok(())
}

fn main() -> anyhow::Result<()>
{
    let args = Args::parse();
    let mut final_seed: u64 = args.seed as u64;

    if args.seed == 0 {
        unsafe { _rdrand64_step(&mut final_seed); }
    }

    if !args.autocast {
        run(args, final_seed as i64)
    } else {
        tch::autocast(true, || run(args, final_seed as i64))
    }
}
