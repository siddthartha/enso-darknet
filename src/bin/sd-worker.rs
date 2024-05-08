use redis::{Client, Commands};
use std::string::String;
use enso_darknet::models::SDRequest;

#[tokio::main]
pub async fn main() -> anyhow::Result<()>
{
    let client = Client::open("redis://redis:6379/").unwrap();
    let mut connection = client.get_connection()?;
    let mut write_connection = client.get_connection()?;
    let mut pubsub_connection = connection.as_pubsub();

    pubsub_connection.subscribe("render")?;

    loop {
        let msg = pubsub_connection.get_message()?;
        let payload : String = msg.get_payload()?;
        println!("channel '{}': {}", msg.get_channel_name(), payload);

        let _request: SDRequest = serde_json::from_str(payload.as_str()).unwrap();

        let uuid = _request.clone().uuid;

        write_connection.set::<String, String, String>(format!("task:{uuid}").to_string(), payload).unwrap();
    }
}