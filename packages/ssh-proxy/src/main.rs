use actix_web::{HttpServer, middleware, Scope, web, Responder, post, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct PipeRequest {
    user: String,
    public_key: String
}

#[post("/pipes")]
async fn add_pipe(req: web::Json<PipeRequest>) -> Result<impl Responder> {

    unimplemented!("create configuration folder");
    Ok(web::HttpResponse::NoContent())
}

#[delete("/pipes/{user}")]
async fn remove_pipe() -> Result<impl Responder> {
    unimplemented!("remove folder")
}

fn build_api() -> Scope {
    web::scope("/api")
        .service(add_pipe)
        .service(remove_pipe)
}

#[actix_rt::main]
async fn main() -> Result<(), failure::Error> {
    dotenv().ok();
    env_logger::init();
    HttpServer::new(move|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(build_api())
    })
        .bind("0.0.0.0:8081")?
        .run()
        .await?;

    Ok(())
}
