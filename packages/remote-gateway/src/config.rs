use url::Url;
use cloudflare::framework::auth::Credentials;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub container_image: String,
    pub public_url: Url,
    pub redis_url: String,
    pub ssh_port: u32,
    pub cloudflare_zone: String,
    pub cloudflare_zone_id: String,
    pub cloudflare_credentials: CloudflareCredentials
}

#[derive(Debug)]
pub struct CloudflareCredentials(Credentials);

impl Clone for CloudflareCredentials {
    fn clone(&self) -> Self {
        match &self.0 {
            Credentials::UserAuthToken { token } => CloudflareCredentials(Credentials::UserAuthToken { token: token.clone() }),
            Credentials::UserAuthKey { email, key } => CloudflareCredentials(Credentials::UserAuthKey { email: email.clone(), key: key.clone() }),
            Credentials::Service { key } => CloudflareCredentials(Credentials::Service { key: key.clone() }),
        }
    }
}

impl From<CloudflareCredentials> for Credentials {
    fn from(credentials: CloudflareCredentials) -> Self {
        credentials.0
    }
}

pub fn load_config() -> Result<Configuration, failure::Error> {
    Ok(Configuration {
        container_image: std::env::var("CONTAINER_IMAGE")?,
        redis_url: std::env::var("REDIS_URL")?,
        public_url: Url::parse(&std::env::var("PUBLIC_URL")?)?,
        ssh_port: u32::from_str(&std::env::var("SSH_PORT")?)?,
        cloudflare_zone: std::env::var("CLOUDFLARE_ZONE")?,
        cloudflare_zone_id: std::env::var("CLOUDFLARE_ZONE_ID")?,
        cloudflare_credentials: CloudflareCredentials(Credentials::UserAuthKey {
            email: std::env::var("CLOUDFLARE_EMAIL")?,
            key: std::env::var("CLOUDFLARE_API_TOKEN")?
        })
    })
}
