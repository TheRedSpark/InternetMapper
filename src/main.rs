use std::time::Duration;

use ipnet::{Ipv4AddrRange, Ipv4Subnets};

#[tokio::main]
async fn main() {
    let hosts = Ipv4AddrRange::new(
        "141.30.224.1".parse().unwrap(),
        "141.30.224.254".parse().unwrap(),
    );
    let subnets = Ipv4Subnets::new(
        "141.30.224.1".parse().unwrap(),
        "141.30.224.254".parse().unwrap(),
        24,
    );
    for host in hosts {
        let duration: Duration = match ping(host.to_string()).await {
            Ok(file) => file,
            Err(_) => {
                Duration::ZERO
            }
        };
        println!("Der Host mit der Adresse:{} hat die Antwortzeit:{:?}", host.to_string(), duration);
    }

    async fn ping(ipaddr: String) -> Result<Duration, Box<dyn std::error::Error>> {
        let payload = [0; 8];

        let (_packet, duration) = surge_ping::ping(ipaddr.parse().unwrap(), &payload).await?;
        //println!("Ping took {:.3?}", duration);

        Ok(duration)
    }
}