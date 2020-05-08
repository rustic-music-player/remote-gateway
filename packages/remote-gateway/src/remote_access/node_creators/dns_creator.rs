use async_trait::async_trait;
use crate::remote_access::node_creators::AccessNodeCreator;
use crate::remote_access::access_node::AccessNode;
use crate::remote_access::DnsClient;
use crate::remote_access::cloudflare_dns::CloudflareDns;
use log::debug;

pub struct DnsCreator {
    client: CloudflareDns
}

impl std::fmt::Debug for DnsCreator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DnsCreator")
            .finish()
    }
}

impl DnsCreator {
    pub fn new(client: CloudflareDns) -> DnsCreator {
        DnsCreator {
            client
        }
    }
}

#[async_trait]
impl AccessNodeCreator for DnsCreator {
    async fn create(&self, node: &AccessNode) -> Result<(), failure::Error> {
        debug!("Creating DNS Entry for AccessNode {:?}", node);
        self.client.add_subdomain(&node.prefix).await
    }

    async fn remove(&self, node: &AccessNode) -> Result<(), failure::Error> {
        debug!("Removing DNS Entry for AccessNode {:?}", node);
        self.client.remove_subdomain(&node.prefix).await
    }
}
