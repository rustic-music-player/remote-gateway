use actix_web::{delete, get, post, Responder, Result, web};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::ApiState;
use crate::remote_access::access_node::AccessNode;

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterNodeRequest {
    pub public_key: String,
    pub preferred_prefix: Option<String>,
}

#[derive(Deserialize)]
pub struct TunnelParams {
    id: Uuid,
}

#[derive(Debug, Clone, Serialize)]
pub struct AccessNodeModel {
    pub id: Uuid,
    pub access_url: String,
    pub tunnel_url: String,
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

impl AccessNodeModel {
    fn from_node(node: &AccessNode, public_url: &Url) -> Result<Self, failure::Error> {
        debug!("converting access node {:?}", &node);
        let access_url = node.get_access_url(public_url)?;
        let tunnel_url = node.get_tunnel_url(public_url)?;

        Ok(AccessNodeModel {
            id: node.id,
            access_url,
            tunnel_url,
        })
    }
}

#[post("/gateway")]
pub async fn register_node(state: web::Data<ApiState>, config: web::Json<RegisterNodeRequest>) -> Result<impl Responder> {
    let node = AccessNode::create(&config);
    let mut successful_creators = Vec::new();
    for creator in &state.node_creators {
        match creator.create(&node).await {
            Ok(()) => successful_creators.push(creator),
            Err(e) => {
                for rollback in successful_creators {
                    if let Err(e) = rollback.remove(&node).await {
                        error!("Error rollbacking node creator {:?}: {:?}", rollback, e)
                    }
                }
                return Err(e.into());
            }
        }
    }
    let model = AccessNodeModel::from_node(&node, &state.public_url)?;

    Ok(web::Json(model))
}

#[delete("/gateway/{id}")]
pub async fn remove_node(state: web::Data<ApiState>, params: web::Path<TunnelParams>) -> Result<impl Responder> {
    let node = state.node_store.get(params.id).await?;
    if let Some(node) = node {
        for creator in &state.node_creators {
            if let Err(e) = creator.remove(&node).await {
                error!("Error removing node from creator {:?}: {:?}", creator, e)
            }
        }
        Ok(web::HttpResponse::NoContent())
    } else {
        Ok(web::HttpResponse::NotFound())
    }
}

#[get("/gateway/{id}")]
pub async fn get_node(state: web::Data<ApiState>, params: web::Path<TunnelParams>) -> Result<impl Responder> {
    let node = state.node_store.get(params.id).await?;
    if let Some(node) = node {
        let model = AccessNodeModel::from_node(&node, &state.public_url)?;

        Ok(web::HttpResponse::Ok().json(model))
    } else {
        Ok(web::HttpResponse::NotFound().finish())
    }
}
