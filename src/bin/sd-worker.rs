use redis::{Client, Commands};
use std::string::String;
use enso_darknet::models::SDRequest;
use enso_darknet::{RENDER_QUEUE, TASK_PREFIX};

#[tokio::main]
pub async fn main() -> anyhow::Result<()>
{

    let client = Client::open("redis://redis:6379/").unwrap();
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

        write_connection.set::<String, String, String>(format!("{}:{uuid}", TASK_PREFIX.to_string()).to_string(), payload).unwrap();
    }
}