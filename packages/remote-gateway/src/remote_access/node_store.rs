use crate::remote_access::access_node::AccessNode;
use redis::AsyncCommands;
use log::debug;
use uuid::Uuid;

#[derive(Clone)]
pub struct NodeStore {
    client: redis::Client
}

impl std::fmt::Debug for NodeStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeStore")
            .finish()
    }
}

impl NodeStore {
    pub fn new(client: redis::Client) -> Self {
        NodeStore {
            client
        }
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<AccessNode>, failure::Error> {
        debug!("Resolving AccessNode for id {}", id);
        let mut con = self.client.get_async_connection().await?;
        let res: Option<String> = con.get(id.as_bytes()).await?;
        if let Some(res) = res {
            let node = serde_json::from_str(&res)?;
            Ok(Some(node))
        }else {
            Ok(None)
        }
    }
}
