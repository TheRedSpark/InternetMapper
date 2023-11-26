use std::time::Duration;

use ipnet::{Ipv4AddrRange, Ipv4Subnets};

const MULIT_THREADING: bool = false;

#[tokio::main]
async fn main() {
    let hosts = Ipv4AddrRange::new(
        "141.30.224.1".parse().unwrap(),
        "141.30.224.6".parse().unwrap(),
    );
    let subnets = Ipv4Subnets::new(
        "141.30.224.1".parse().unwrap(),
        "141.30.224.254".parse().unwrap(),
        24,
    );
    for host in hosts {
        if (MULIT_THREADING) {
            println!("Spawning Tread");
            tokio::spawn(async move {
                // Process each socket concurrently.
                pre_ping(host.to_string()).await
            });
            //thread::spawn(pre_ping(host.to_string()).await)
        } else {
            pre_ping(host.to_string()).await
        }
    }
}

async fn ping(ipaddr: String) -> Result<Duration, Box<dyn std::error::Error>> {
    let payload = [0; 8];

    let (_packet, duration) = surge_ping::ping(ipaddr.parse().unwrap(), &payload).await?;

    Ok(duration)
}

async fn pre_ping(host: String) {
    let duration_zeit: Duration = match ping(host.to_string()).await {
        Ok(file) => file,
        Err(_) => {
            Duration::ZERO
        }
    };
    println!("Der Host mit der Adresse:{} hat die Antwortzeit:{:?}", host.to_string(), duration_zeit);
}
