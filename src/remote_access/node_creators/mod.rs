use async_trait::async_trait;
use crate::remote_access::access_node::AccessNode;

mod container_creator;
mod dns_creator;
mod redis_creator;

#[async_trait(?Send)]
pub trait AccessNodeCreator {
    async fn create(&self, node: &AccessNode) -> Result<(), failure::Error>;
}
