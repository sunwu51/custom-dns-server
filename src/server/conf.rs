use trust_dns_client::rr::RecordType;
use trust_dns_resolver::Name;



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawConf {
    pub name: String,
    #[serde(rename="type")]
    pub t: String,
    pub value: String
}
