use async_trait::async_trait;
mod access_node;
pub(crate) mod node_creators;
pub(crate) mod api;
#[cfg(feature = "cloudflare")]
pub(crate) mod cloudflare_dns;
pub(crate) mod node_store;

#[async_trait]
pub trait DnsClient {
    async fn add_subdomain(&self, sub_domain: &str) -> Result<(), failure::Error>;
    async fn remove_subdomain(&self, sub_domain: &str) -> Result<(), failure::Error>;
}
