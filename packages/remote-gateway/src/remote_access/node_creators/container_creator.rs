use async_trait::async_trait;
use crate::remote_access::node_creators::AccessNodeCreator;
use bollard::Docker;
use crate::remote_access::access_node::AccessNode;
use bollard::container::{Config, CreateContainerOptions, StartContainerOptions, RemoveContainerOptions, StopContainerOptions};
use log::debug;
use maplit::hashmap;

const PREFIX_LABEL: &str = "remotegateway.prefix";
const ENABLE_TRAEFIK_LABEL: &str = "traefik.enable";

#[derive(Clone)]
pub struct ContainerCreator {
    api: Docker,
    image: String
}

impl std::fmt::Debug for ContainerCreator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContainerCreator")
            .field("image", &self.image)
            .finish()
    }
}

impl ContainerCreator {
    pub fn new(api: Docker, image: String) -> ContainerCreator {
        ContainerCreator {
            api,
            image
        }
    }

    fn get_container_name(node: &AccessNode) -> String {
        format!("access_node_{}", node.prefix)
    }
}

#[async_trait]
impl AccessNodeCreator for ContainerCreator {
    async fn create(&self, node: &AccessNode) -> Result<(), failure::Error> {
        debug!("Creating Container for AccessNode {:?}", node);
        let container_config = CreateContainerOptions {
            name: ContainerCreator::get_container_name(node)
        };
        let config = Config {
            image: Some(self.image.clone()),
            env: Some(vec![
                format!("SSH_USER={}", &node.username),
                format!("PUBLIC_KEY={}", &node.public_key)
            ]),
            labels: Some(hashmap!{
                String::from(PREFIX_LABEL) => node.prefix.clone(),
                String::from(ENABLE_TRAEFIK_LABEL) => String::from("true")
            }),
            ..Config::default()
        };
        let result = self.api.create_container(Some(container_config), config).await?;
        self.api.start_container(&result.id, None::<StartContainerOptions<String>>).await?;

        Ok(())
    }

    async fn remove(&self, node: &AccessNode) -> Result<(), failure::Error> {
        debug!("Removing Container for AccessNode {:?}", node);
        let name = ContainerCreator::get_container_name(node);
        self.api.stop_container(&name, None::<StopContainerOptions>).await?;
        let options = RemoveContainerOptions {
            force: false,
            v: true,
            link: false
        };
        self.api.remove_container(&name, Some(options)).await?;
        Ok(())
    }
}
