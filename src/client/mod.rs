pub mod forward;
use std::net::{ SocketAddr, Ipv4Addr};
use std::str::FromStr;
use lazy_static::lazy_static;
use tokio::sync::Mutex;
use trust_dns_client::client::{AsyncClient, ClientHandle};
use trust_dns_client::proto::iocompat::AsyncIoTokioAsStd;
use trust_dns_client::rr::{DNSClass, Name,  RecordType};
use trust_dns_client::tcp::TcpClientStream;
use tokio::net::TcpStream as TokioTcpStream;
use anyhow::Result as AnyResult;


lazy_static! {
    static ref DNS_SERVER: SocketAddr = "8.8.8.8:53".parse().unwrap();
    static ref DNS_CLIENT: Mutex<Vec<AsyncClient>> = Mutex::new(vec![]);
}

async fn get_dns_client() -> AnyResult<AsyncClient> {
    let (stream, sender) = TcpClientStream::<AsyncIoTokioAsStd<TokioTcpStream>>::new(*DNS_SERVER);
    let (client, bg) = AsyncClient::new(Box::new(stream), sender, None).await?;
    // need to run the connection in background
    tokio::spawn(bg);
    AnyResult::Ok(client)
}

pub async fn query_a_record(name: &str) -> AnyResult<Vec<Ipv4Addr>> {

    // init the dns_client
    let mut v = DNS_CLIENT.lock().await;
    if v.is_empty() {
        v.push(get_dns_client().await?);
    }
    let client = &mut v[0];


    // query the domain name get the ipv4 address
    // todo: not only for ipv4 (A record)
    let name = Name::from_str(name)?;
    let record_type = RecordType::A;
    let query = client.query(name, DNSClass::IN, record_type);
    let resp = query.await.unwrap();
    let ans = resp.answers();
    let res: Vec<Ipv4Addr> = ans.to_vec()
        .into_iter()
        .filter(|r| r.record_type() == record_type)
        .map(|r| { r.data().unwrap().as_a().unwrap().clone()})
    .collect();
    AnyResult::Ok(res)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_a_record_test() {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(
                async {
                    let res = query_a_record("www.example.com").await.unwrap();
                    println!("{:?}", res);

                    assert_eq!(1, res.len());
                    assert_eq!(Ipv4Addr::from_str("93.184.216.34").unwrap(), res[0]);
                }
            );
    }
}