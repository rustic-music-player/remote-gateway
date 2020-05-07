use actix_web::{web, post, Responder, Result};
use crate::ApiState;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::remote_access::access_node::AccessNode;
use std::convert::TryFrom;
use crate::config::CONFIG;
use url::Url;
use log::debug;

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterNodeRequest {
    // #[serde(deserialize_with = "from_base64")]
    pub public_key: String,
    pub preferred_prefix: Option<String>
}

#[derive(Debug, Clone, Serialize)]
pub struct AccessNodeModel {
    pub id: Uuid,
    pub access_url: String,
    pub tunnel_url: String
}

impl AccessNode {
    fn get_access_url(&self, public_url: &Url) -> Result<String, failure::Error> {
        debug!("access url based on url {:?}", &public_url);
        let mut access_url = public_url.clone();
        let host = access_url.host_str().unwrap();
        let host = format!("{}.{}", self.prefix, host);
        access_url.set_host(Some(&host))?;

        Ok(access_url.into_string())
    }

    fn get_tunnel_url(&self, public_url: &Url) -> Result<String, failure::Error> {
        debug!("tunnel url based on url {:?}", &public_url);
        let host = public_url.host_str().unwrap();
        let tunnel_url = Url::parse(&format!("ssh://{}@tunnel.{}", self.username, host))?;

        Ok(tunnel_url.into_string())
    }
}

impl TryFrom<AccessNode> for AccessNodeModel {
    type Error = failure::Error;

    fn try_from(node: AccessNode) -> Result<Self, Self::Error> {
        debug!("converting access node {:?}", &node);
        let access_url = node.get_access_url(&CONFIG.public_url)?;
        let tunnel_url = node.get_tunnel_url(&CONFIG.public_url)?;

        Ok(AccessNodeModel {
            id: node.id,
            access_url,
            tunnel_url
        })
    }
}

#[post("/gateway")]
pub async fn register_node(state: web::Data<ApiState>, config: web::Json<RegisterNodeRequest>) -> Result<impl Responder> {
    let node = AccessNode::create(&config);
    let model = AccessNodeModel::try_from(node)?;

    Ok(web::Json(model))
}
