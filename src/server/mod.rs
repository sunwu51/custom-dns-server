pub mod conf;
use std::{collections::HashMap, net::Ipv4Addr, str::FromStr, fs};

use anyhow::Result as AnyResult;
use async_recursion::async_recursion;
use lazy_static::lazy_static;
use trust_dns_client::{
    op::Header,
    rr::{RData, Record, LowerName, RecordType, RrKey, rdata::name},
};
use trust_dns_resolver::Name;
use trust_dns_server::{
    authority::{MessageResponseBuilder, ZoneType, Authority, LookupOptions},
    server::{Request, RequestHandler, ResponseHandler, ResponseInfo}, store::file::{FileConfig, FileAuthority},
};

use crate::client::forward::query_a_record;

use self::conf::RawConf;

lazy_static! {
    static ref DNS_RECORDS: HashMap<String, RawConf> = prepare_records();
}

pub struct DnsHandler {}

#[async_trait::async_trait]
impl RequestHandler for DnsHandler {
    async fn handle_request<R: ResponseHandler>(
        &self,
        request: &Request,
        mut response_handle: R,
    ) -> ResponseInfo {
        let builder = MessageResponseBuilder::from_message_request(request);
        let header = Header::response_from_request(request.header());
        let name_str = request.query().name().to_string();
        let records = look_up_record(&name_str).await;
        let response = builder.build(header, &records, &[], &[], &[]);
        response_handle.send_response(response).await.unwrap()
    }
}

#[async_recursion]
pub async fn look_up_record(name_str: &str) -> Vec<Record>{
    let mut opt = DNS_RECORDS.get(name_str.trim_end_matches("."));
    let mut records: Vec<Record> = vec![];

    // 1 if not present in the config, use google dns server to lookup
    if opt.is_none() {
        let ip_list = query_a_record(&name_str).await.unwrap();
        for ip in ip_list {
            let name = Name::from_str(&name_str).unwrap();
            records.push(Record::from_rdata(name, 60, RData::A(ip.clone())));    
        }
    } else {
        let raw_conf = opt.unwrap();
        match raw_conf.t.as_str() {

            // if A record, use the value as the ipv4 result
            "A" => {
                let rdata = RData::A(
                    Ipv4Addr::from_str(&raw_conf.value).unwrap()
                );
                let name = Name::from_str(&name_str).unwrap();
                records.push(Record::from_rdata(name, 60, rdata));
            }
            // if CNAME record, change the name, then recursively lookup
            "CNAME" => {
                let name_str = raw_conf.value.as_str();
                records = look_up_record(name_str).await;
            },
            _ => panic!("Invalid type")
        };
    }

    records
}



pub fn prepare_records() -> HashMap<String, RawConf> {
    let mut map = HashMap::new();

    let content = String::from_utf8(fs::read("config.json").unwrap()).unwrap();

    let confs: Vec<RawConf> = serde_json::from_str(&content).unwrap();


    for ele in confs {
        match ele.t.as_str() {
            "A" => map.insert(ele.name.clone(), ele),
            "CNAME" => map.insert(ele.name.clone(), ele),
            _ => panic!("Invalid type in config file")
        };
    }
    map
}
