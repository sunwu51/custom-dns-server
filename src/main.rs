use std::{net::{IpAddr, Ipv4Addr}, str::FromStr, collections::HashMap};

use server::{prepare_records, DnsHandler};
use tokio::net::UdpSocket;
use anyhow::Result as AnyResult;
use trust_dns_server::ServerFuture;

mod client;
mod server;


#[tokio::main]
async fn main() -> AnyResult<()>{
    prepare_records();
    
    // apt-get install dnsutils
    // dig @127.0.0.1 -p 5353 www.baidu.com
    let udp_server = UdpSocket::bind("0.0.0.0:5353").await?;

    let mut server = ServerFuture::new(DnsHandler{});

    server.register_socket(udp_server);

    server.block_until_done().await?;

    Ok(())

}
