use lazy_static::lazy_static;
use url::Url;

lazy_static! {
    pub static ref CONFIG: Configuration = Configuration {
        container_image: String::new(),
        redis_url: String::from("redis://localhost"),
        public_url: Url::parse("https://rustic.maxjoehnk.me").unwrap()
    };
}

#[derive(Debug)]
pub struct Configuration {
    pub container_image: String,
    pub public_url: Url,
    pub redis_url: String
}
