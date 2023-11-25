



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let payload = [0; 8];

    let (_packet, duration) = surge_ping::ping("127.0.0.1".parse()?, &payload).await?;
    surge_ping::Pinger::
    println!("Ping took {:.3?}", duration);

    Ok(())
}