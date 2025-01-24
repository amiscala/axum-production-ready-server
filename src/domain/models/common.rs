use sha2::{Sha256,Digest};
use uuid::{ContextV7, Timestamp, Uuid};

pub fn create_uuid_v7() -> Uuid{
    let ctx = ContextV7::new();
    Uuid::new_v7(Timestamp::now(ctx))
}

pub fn generate_client_secret() -> String {
    let mut hasher = Sha256::new();
    let random_uuid = Uuid::new_v4();
    hasher.update(random_uuid.as_bytes());
    let hash: [u8;32] = hasher.finalize().into();
    bytes_to_hex(&hash)
}
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}