use uuid::Uuid;
use crate::remote_access::api::RegisterNodeRequest;

use rand::seq::IteratorRandom;

const ASCII_CHARS: &str = "0123456789abcdefghijklmnopqrstuvwxyz";

#[derive(Default)]
pub struct AccessNode {
    pub id: Uuid,
    pub public_key: Vec<u8>,
    pub prefix: String,
    pub username: String
}

impl std::fmt::Debug for AccessNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccessNode")
            .field("id", &self.id)
            .field("prefix", &self.prefix)
            .field("username", &self.username)
            .finish()
    }
}

impl AccessNode {
    pub fn create(request: &RegisterNodeRequest) -> AccessNode {
        AccessNode {
            id: Uuid::new_v4(),
            public_key: request.public_key.clone().into_bytes(), //base64::decode(&request.public_key).unwrap(),
            prefix: request.preferred_prefix.clone()
                .unwrap_or_else(|| generate_prefix()),
            username: generate_username()
        }
    }
}

fn generate_prefix() -> String {
    generate_random_string(6)
}

fn generate_username() -> String {
    generate_random_string(8)
}

fn generate_random_string(count: usize) -> String {
    let mut rng = rand::thread_rng();
    ASCII_CHARS.chars()
        .choose_multiple(&mut rng, count)
        .iter()
        .collect::<String>()
}
