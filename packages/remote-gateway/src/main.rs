use bollard::Docker;
use actix_web::{Scope, web, HttpServer, middleware, App};
use dotenv::dotenv;

use crate::remote_access::node_creators::*;
use url::Url;
use crate::remote_access::node_store::NodeStore;

pub(crate) mod config;
mod remote_access;

pub struct ApiState {
    pub node_creators: Vec<Box<dyn AccessNodeCreator>>,
    pub public_url: Url,
    pub node_store: NodeStore
}

fn build_api(creators: Vec<Box<dyn AccessNodeCreator>>, public_url: Url, node_store: NodeStore) -> Scope {
    web::scope("/api")
        .data(ApiState {
            node_creators: creators,
            public_url,
            node_store
        })
        .service(crate::remote_access::api::register_node)
        .service(crate::remote_access::api::get_node)
        .service(crate::remote_access::api::remove_node)
}

#[actix_rt::main]
async fn main() -> Result<(), failure::Error> {
    dotenv().ok();
    env_logger::init();
    let config = config::load_config()?;
    let docker = Docker::connect_with_local_defaults()?;
    let redis = redis::Client::open(config.redis_url.clone())?;
    let dns = crate::remote_access::cloudflare_dns::CloudflareDns::new(config.cloudflare_credentials.clone(), config.cloudflare_zone.clone(), config.cloudflare_zone_id.clone())?;
    HttpServer::new(move|| {
        let container_creator = ContainerCreator::new(docker.clone(), config.container_image.clone());
        let redis_creator = RedisCreator::new(redis.clone());
        let dns_creator = DnsCreator::new(dns.clone());
        let node_creators: Vec<Box<dyn AccessNodeCreator>> = vec![
            Box::new(container_creator),
            Box::new(redis_creator),
            Box::new(dns_creator)
        ];
        let node_store = NodeStore::new(redis.clone());
        App::new()
            .wrap(middleware::Logger::default())
            .service(build_api(node_creators, config.public_url.clone(), node_store))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    Ok(())
}
