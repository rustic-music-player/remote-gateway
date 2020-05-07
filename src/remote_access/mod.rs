use async_trait::async_trait;
mod access_node;
mod node_creators;
pub(crate) mod api;
mod cloudflare_dns;

#[async_trait]
pub trait DnsClient {
    async fn add_subdomain(&self, sub_domain: &str) -> Result<(), failure::Error>;
    async fn remove_subdomain(&self, sub_domain: &str) -> Result<(), failure::Error>;
}
