use std::time::{SystemTime, UNIX_EPOCH};

use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256, Sha512};

use crate::Result;

pub(super) fn nonce() -> Result<u128> {
    let now = SystemTime::now();
    Ok(now.duration_since(UNIX_EPOCH)?.as_millis())
}

fn sha512(input: Vec<u8>, secret: &[u8]) -> Result<Vec<u8>> {
    let mut mac = Hmac::<Sha512>::new_from_slice(secret).expect("HMAC should not fail.");
    mac.update(&input);
    Ok(mac.finalize().into_bytes().to_vec())
}

// HMAC-SHA512 of (URI path + SHA256(nonce + POST data)) and base64 decoded secret API key
pub(super) fn get_signature(
    url_path: &str,
    data: &str,
    nonce: u128,
    secret: &str,
) -> Result<String> {
    let mut sha256_input = nonce.to_string();
    sha256_input.push_str(data);

    let mut hasher = Sha256::new();
    hasher.update(sha256_input.as_bytes());
    let mut sha256 = hasher.finalize().to_vec();

    let mut sha512_input = url_path.as_bytes().to_owned();

    sha512_input.append(&mut sha256);

    let secret = general_purpose::STANDARD.decode(secret)?;

    let sha512 = sha512(sha512_input, &secret)?;

    Ok(general_purpose::STANDARD.encode(&sha512))
}
