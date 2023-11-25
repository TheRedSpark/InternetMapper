use std::net::IpAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ping("127.0.0.1".parse().unwrap()).await
}

async fn ping(ipaddr: IpAddr) -> Result<(), Box<dyn std::error::Error>> {
    let payload = [0; 8];

    let (_packet, duration) = surge_ping::ping(ipaddr, &payload).await?;
    println!("Ping took {:.3?}", duration);

    Ok(())
}