use bollard::Docker;
use actix_web::{Scope, web, HttpServer, middleware, App};

use crate::config::CONFIG;

pub(crate) mod config;
mod remote_access;

pub struct ApiState {
    pub docker: Docker,
    pub redis: redis::Client
}

fn build_api(client: redis::Client) -> Result<Scope, failure::Error> {
    let docker = Docker::connect_with_local_defaults()?;
    Ok(web::scope("/api")
        .data(ApiState {
            docker,
            redis: client
        })
        .service(crate::remote_access::api::register_node))
}

#[actix_rt::main]
async fn main() -> Result<(), failure::Error> {
    env_logger::init();
    let redis = redis::Client::open(CONFIG.redis_url.clone())?;
    HttpServer::new(move|| App::new()
        .wrap(middleware::Logger::default())
        .service(build_api(redis.clone()).unwrap()))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    Ok(())
}
