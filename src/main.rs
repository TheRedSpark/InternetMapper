use std::time::Duration;

use chrono::{DateTime, Local};
use ipnet::{Ipv4AddrRange, Ipv4Subnets};
use mysql::{params, Pool, PooledConn};
use mysql::prelude::Queryable;
use tokio::task::JoinHandle;

mod variables;

const MULIT_THREADING: bool = true;

#[tokio::main]
async fn main() {
    let hosts = Ipv4AddrRange::new(
        "141.30.0.1".parse().unwrap(),
        //"141.30.255.255".parse().unwrap(),
        "141.30.2.255".parse().unwrap(),
    );
    let subnets = Ipv4Subnets::new(
        "141.30.224.1".parse().unwrap(),
        "141.30.224.254".parse().unwrap(),
        24,
    );
    let pool: Pool = Pool::new(&*string_builder()).expect("Pool bildung fehlgeschlagen");
    let db_pool: Pool = pool.clone();
    let mut tasks = Vec::with_capacity(2);
    for host in hosts {
        if (MULIT_THREADING) {
            tasks.push(tokio::spawn(pre_ping(host.to_string(), db_pool.clone())));

        } else {
            pre_ping(host.to_string(), db_pool.clone()).await
        }
    }
    println!("Menge der Tasks:{}", tasks.len());
    let mut outputs = Vec::with_capacity(tasks.len());
    for task in tasks {
        outputs.push(task.await.unwrap());
    }
    //println!("{:?}", outputs);
}

async fn tast_stop(tasks:Vec<JoinHandle<()>>){
    let mut outputs = Vec::with_capacity(tasks.len());
    for task in tasks {
        outputs.push(task.await.unwrap());
    }
}


async fn ping(ipaddr: String) -> Result<Duration, Box<dyn std::error::Error>> {
    let payload = [0; 8];

    let (_packet, duration) = surge_ping::ping(ipaddr.parse().unwrap(), &payload).await?;

    Ok(duration)
}

async fn pre_ping(host: String, db_pool: Pool) {
    //println!("Spawning Tread");
    let duration_zeit: Duration = match ping(host.to_string()).await {
        Ok(file) => file,
        Err(_) => {
            Duration::ZERO
        }
    };
    println!("Der Host mit der Adresse:{} hat die Antwortzeit:{:?}", host.to_string(), duration_zeit);
    uploader(host, duration_zeit, db_pool).unwrap()
}


fn uploader(host: String, duration: Duration, db_pool: Pool) -> Result<(), Box<dyn std::error::Error>> {
    let stamp: DateTime<Local> = Local::now();
    let stamp: String = format!("{}", stamp.format("%Y-%m-%d %H:%M:%S"));
    let mut conn: PooledConn = db_pool.get_conn()?;
    conn.exec_drop(
        "insert into InternetMapper.hosts (host, duration) values (:host, :duration)",
        params! {
            "host" => host.to_string().clone(),
            "duration" => duration.as_secs_f64().to_string(),

        },
    ).expect("Fehler beim Upload von Host");

    Ok(())
}

fn string_builder() -> String {
    let mysql_ipaddr: String = variables::mysql_ip();
    let mysql_user: String = variables::mysql_user();
    let mysql_database: String = variables::mysql_database();
    let mysql_passwort: String = variables::mysql_passwort();
    let url: String = format!("mysql://{mysql_user}:{mysql_passwort}@{mysql_ipaddr}:3306/{mysql_database}");
    return url.to_string();
}