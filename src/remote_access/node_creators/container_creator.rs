use async_trait::async_trait;
use crate::remote_access::node_creators::AccessNodeCreator;
use bollard::Docker;
use crate::remote_access::access_node::AccessNode;
use bollard::container::{Config, CreateContainerOptions};
use crate::config::CONFIG;

pub struct ContainerCreator {
    api: Docker
}

#[async_trait(?Send)]
impl AccessNodeCreator for ContainerCreator {
    async fn create(&self, node: &AccessNode) -> Result<(), failure::Error> {
        let container_config = CreateContainerOptions {
            name: format!("access_node_{}", node.prefix)
        };
        let config = Config {
            image: Some(CONFIG.container_image.clone()),
            // TODO: traefik labels
            ..Config::default()
        };
        let result = self.api.create_container(Some(container_config), config).await?;

        unimplemented!()
    }
}
