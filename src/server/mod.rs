use std::{collections::HashMap, net::Ipv4Addr, str::FromStr};

use lazy_static::lazy_static;
use trust_dns_client::{op::Header, rr::{RData, Record}};
use trust_dns_resolver::Name;
use trust_dns_server::{server::{RequestHandler, ResponseInfo, ResponseHandler, Request}, authority::MessageResponseBuilder};
use anyhow::Result as AnyResult;

use crate::client::forward::query_a_record;

lazy_static!{
    static ref DNS_RECORDS: HashMap<String, Ipv4Addr>  = prepare_records();
}

pub struct DnsHandler{}

#[async_trait::async_trait]
impl RequestHandler for DnsHandler{
    async fn handle_request<R: ResponseHandler>(
        &self,
        request: &Request,
        mut response_handle: R,
    ) -> ResponseInfo {
        let builder = MessageResponseBuilder::from_message_request(request);
        let header = Header::response_from_request(request.header());
        let name_str = request.query().name().to_string();
        let mut ipv4 = DNS_RECORDS.get(&name_str);
        let mut records: Vec<Record> = vec![];

        match ipv4 {
            Some(ip) => {
                let rdata = RData::A(ip.clone());
                let name = Name::from_str(&name_str).unwrap();
                records.push(Record::from_rdata(name, 60, rdata));
            },
            None => {
                let ip_list = query_a_record(&name_str).await.unwrap();
                for ip in ip_list {
                    let name = Name::from_str(&name_str).unwrap();
                    records.push(Record::from_rdata(name, 60, RData::A(ip.clone())));    
                }
            }
        }

        let response = builder.build(header, &records, &[], &[], &[]);

        response_handle.send_response(response).await.unwrap()
    }
}

pub fn prepare_records() -> HashMap<String, Ipv4Addr> {
    let mut map = HashMap::new();
    map.insert("www.baidu.com.".to_owned(), Ipv4Addr::from_str("1.1.1.1").unwrap());
    map.insert("www.google.com.".to_owned(), Ipv4Addr::from_str("2.2.2.2").unwrap());
    map.insert("www.hulu.com.".to_owned(), Ipv4Addr::from_str("3.3.3.3").unwrap());
    map
}
