use async_trait::async_trait;
use crate::remote_access::node_creators::AccessNodeCreator;
use crate::remote_access::access_node::AccessNode;

pub struct RedisCreator {
    client: redis::Client
}

#[async_trait(?Send)]
impl AccessNodeCreator for RedisCreator {
    async fn create(&self, node: &AccessNode) -> Result<(), failure::Error> {
        unimplemented!()
    }
}
