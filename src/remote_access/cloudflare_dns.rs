use async_trait::async_trait;
use crate::remote_access::DnsClient;

struct CloudflareDns {}

#[async_trait]
impl DnsClient for CloudflareDns {
    async fn add_subdomain(&self, sub_domain: &str) -> Result<(), failure::Error> {
        unimplemented!()
    }

    async fn remove_subdomain(&self, sub_domain: &str) -> Result<(), failure::Error> {
        unimplemented!()
    }
}
