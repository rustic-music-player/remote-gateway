use async_trait::async_trait;
use crate::remote_access::node_creators::AccessNodeCreator;
use crate::remote_access::access_node::AccessNode;
use crate::remote_access::DnsClient;

pub struct DnsCreator {
    client: Box<dyn DnsClient>
}

#[async_trait(?Send)]
impl AccessNodeCreator for DnsCreator {
    async fn create(&self, node: &AccessNode) -> Result<(), failure::Error> {
        self.client.add_subdomain(&node.prefix).await
    }
}
