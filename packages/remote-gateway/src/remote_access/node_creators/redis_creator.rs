use async_trait::async_trait;
use crate::remote_access::node_creators::AccessNodeCreator;
use crate::remote_access::access_node::AccessNode;
use redis::AsyncCommands;
use log::debug;

#[derive(Clone)]
pub struct RedisCreator {
    client: redis::Client
}

impl std::fmt::Debug for RedisCreator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedisCreator")
            .finish()
    }
}

impl RedisCreator {
    pub fn new(client: redis::Client) -> RedisCreator {
        RedisCreator {
            client
        }
    }
}

#[async_trait]
impl AccessNodeCreator for RedisCreator {
    async fn create(&self, node: &AccessNode) -> Result<(), failure::Error> {
        debug!("Storing AccessNode {:?} in redis", node);
        let mut con = self.client.get_async_connection().await?;
        let value = serde_json::to_string(node)?;
        con.set(node.id.as_bytes(), &value).await?;

        Ok(())
    }

    async fn remove(&self, node: &AccessNode) -> Result<(), failure::Error> {
        debug!("Removing AccessNode {:?} out of redis", node);
        let mut con = self.client.get_async_connection().await?;
        con.del(node.id.as_bytes()).await?;

        Ok(())
    }
}
