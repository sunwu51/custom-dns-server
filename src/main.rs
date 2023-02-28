use std::net::{ SocketAddr};
use std::str::FromStr;
use trust_dns_client::client::{AsyncClient, ClientHandle};
use trust_dns_client::proto::iocompat::AsyncIoTokioAsStd;
use trust_dns_client::rr::{DNSClass, Name,  RecordType};
use trust_dns_client::tcp::TcpClientStream;

use tokio::net::TcpStream as TokioTcpStream;

#[tokio::main]
async fn main() {
    let name = Name::from_str("www.google.com").expect("domain name error");
    let addr: SocketAddr = "8.8.8.8:53".parse().unwrap();

    let (stream, sender) = TcpClientStream::<AsyncIoTokioAsStd<TokioTcpStream>>::new(addr);

    let (mut client, bg) = AsyncClient::new(Box::new(stream), sender, None)
        .await
        .unwrap();

    tokio::spawn(bg);

    let query = client.query(name, DNSClass::IN, RecordType::A);
    let resp = query.await.unwrap();

    let ans = resp.answers();

    for ele in ans {
        println!("{:#?}", ele);
    }
}
