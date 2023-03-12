use std::{net::SocketAddr, str::FromStr, time::Duration};

use anyhow::Result as AnyResult;
use arg::Args;
use clap::Parser;
use server::{prepare_records, DnsHandler};
use tokio::net::{TcpListener, UdpSocket};
use trust_dns_server::ServerFuture;

mod arg;
mod client;
mod server;

#[macro_use]
extern crate serde;
#[tokio::main]
async fn main() -> AnyResult<()> {
    let args = Args::parse();

    prepare_records();

    // apt-get install dnsutils
    // dig @127.0.0.1 -p 5353 www.baidu.com
    let socket_addr = SocketAddr::from_str(&format!("0.0.0.0:{}", args.port))?;
    let mut server = ServerFuture::new(DnsHandler {});

    if args.udp {
        let udp_server = UdpSocket::bind(&socket_addr).await?;
        server.register_socket(udp_server);
    }

    if args.tcp {
        let tcp_server = TcpListener::bind(&socket_addr).await?;
        server.register_listener(tcp_server, Duration::from_secs(10));
    }

    server.block_until_done().await?;

    Ok(())
}
