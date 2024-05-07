use redis::Client;

#[tokio::main]
pub async fn main() -> anyhow::Result<()>
{
    let client = Client::open("redis://redis:6379/").unwrap();
    let mut tokio_conn = client.get_connection()?;
    let mut pubsub_conn = tokio_conn.as_pubsub();

    pubsub_conn.subscribe("render")?;

    loop {
        let msg = pubsub_conn.get_message()?;
        let payload : String = msg.get_payload()?;
        println!("channel '{}': {}", msg.get_channel_name(), payload);
    }
}