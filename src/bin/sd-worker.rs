use redis::{Client, Commands};
use std::string::String;
use tch::Tensor;
use enso_darknet::models::SDRequest;
use enso_darknet::{
    RENDER_QUEUE,
    TASK_PREFIX,
    StableDiffusionTask,
    StableDiffusionVersion,
};

#[tokio::main]
pub async fn main() -> anyhow::Result<()>
{

    let client = Client::open(enso_darknet::redis_host()).unwrap();
    let mut connection = client.get_connection()?;
    let mut write_connection = client.get_connection()?;
    let mut pubsub_connection = connection.as_pubsub();

    pubsub_connection.subscribe(RENDER_QUEUE)?;

    loop {
        let msg = pubsub_connection.get_message()?;
        let payload : String = msg.get_payload()?;
        println!("channel '{}': {}", msg.get_channel_name(), payload);

        let request: SDRequest = serde_json::from_str(payload.as_str()).unwrap();

        let uuid = request.clone().uuid;
        let seed= request.clone().seed;
        let prompt = request.clone().prompt;
        let steps = request.clone().steps;
        let width = request.clone().width;
        let height = request.clone().height;
        let intermediary_images = request.clone().intermediates;
        let version = request.clone().version;

        write_connection.set::<String, String, String>(
            format!("{}:{uuid}", TASK_PREFIX.to_string()).to_string(),
            payload
        ).unwrap();

        let task = StableDiffusionTask {
            prompt: prompt.clone(),
            cpu: vec![],
            height: Some(height as i64),
            width: Some(width as i64),
            unet_weights: None,
            clip_weights: None,
            vae_weights: None,
            vocab_file: "data/vocab_16e6.txt".to_string(),
            sliced_attention_size: None,
            n_steps: steps as usize,
            seed: seed.clone(),
            num_samples: 0,
            final_image: format!("./media/{}.jpg", uuid.clone().to_string()),
            autocast: false,
            sd_version: match version {
                1 => StableDiffusionVersion::V1_5,
                _ => StableDiffusionVersion::V2_1,
            },
            intermediary_images,
        };

        write_connection.set::<String, String, String>(
            format!("{}:{uuid}", TASK_PREFIX.to_string()).to_string(),
            serde_json::to_string(&task).unwrap()
        ).unwrap();

        let final_image = task.final_image.clone();

        let image: Tensor = task.run(seed)?;

        let final_image = StableDiffusionTask::output_filename(final_image.as_str(), 1, 0, None);

        tch::vision::image::save(&image, final_image)?;
    }
}