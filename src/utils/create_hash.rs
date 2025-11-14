use sha2::{Digest, Sha256};

pub fn create_hash(url: &str, length: usize) -> String {
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    let result = hasher.finalize();
    let hex = hex::encode(result);

    hex[..length.min(hex.len())].to_string()
}
