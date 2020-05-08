use async_trait::async_trait;
use crate::remote_access::access_node::AccessNode;
pub use self::container_creator::ContainerCreator;
pub use self::dns_creator::DnsCreator;
pub use self::redis_creator::RedisCreator;

mod container_creator;
mod dns_creator;
mod redis_creator;

#[async_trait]
pub trait AccessNodeCreator: Sync + std::fmt::Debug {
    async fn create(&self, node: &AccessNode) -> Result<(), failure::Error>;
    async fn remove(&self, node: &AccessNode) -> Result<(), failure::Error>;
}
