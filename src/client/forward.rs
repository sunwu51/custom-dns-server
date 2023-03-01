use std::net::Ipv4Addr;
use std::str::FromStr;

use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use trust_dns_proto::rr::{RecordType};
use trust_dns_resolver::{ Name, config::NameServerConfigGroup};
use trust_dns_server::{
    authority::{Authority, LookupObject, ZoneType},
    store::forwarder::{ForwardAuthority, ForwardConfig},
};
use anyhow::Result as AnyResult;

lazy_static!{
    static ref FORWARDER: ForwardAuthority = ForwardAuthority::try_from_config(Name::root().into(), ZoneType::Forward, &ForwardConfig {
        name_servers: NameServerConfigGroup::google(),
        options: None,
    }).unwrap();
}

pub async fn query_a_record(name: &str) -> AnyResult<Vec<Ipv4Addr>> {
    let lookup = FORWARDER.lookup(
        &Name::from_str(name).unwrap().into(),
        RecordType::A,
        Default::default(),
    ).await?;

    let record_type = RecordType::A;

    let res: Vec<Ipv4Addr> = lookup.iter()
        .into_iter()
        .filter(|r| r.record_type() == record_type)
        .map(|r| { r.data().unwrap().as_a().unwrap().clone()})
    .collect();
    AnyResult::Ok(res)
    
}
#[test]
fn test_lookup() {
    let runtime = Runtime::new().expect("failed to create Tokio Runtime");
    let ipv4s = runtime
        .block_on(query_a_record("www.example.com."))
        .unwrap();

    assert_eq!(ipv4s.iter().next().unwrap(), &Ipv4Addr::new(93, 184, 216, 34));
}