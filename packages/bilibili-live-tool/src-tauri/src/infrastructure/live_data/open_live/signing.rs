#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignedHeaders {
    pub content_md5: String,
    pub timestamp: String,
    pub nonce: String,
    pub access_key_id: String,
    pub authorization: String,
}

pub fn canonical_signature(headers: &SignedHeaders) -> String {
    format!(
        "x-bili-accesskeyid:{}\nx-bili-content-md5:{}\nx-bili-signature-method:HMAC-SHA256\nx-bili-signature-nonce:{}\nx-bili-signature-version:1.0\nx-bili-timestamp:{}",
        headers.access_key_id, headers.content_md5, headers.nonce, headers.timestamp
    )
}

pub fn sign_request(
    body: &[u8],
    access_key_id: &str,
    access_key_secret: &str,
    nonce: &str,
    timestamp: i64,
) -> SignedHeaders {
    let mut digest = Md5::new();
    digest.update(body);
    let mut headers = SignedHeaders {
        content_md5: format!("{:x}", digest.finalize()),
        timestamp: timestamp.to_string(),
        nonce: nonce.to_owned(),
        access_key_id: access_key_id.to_owned(),
        authorization: String::new(),
    };
    headers.authorization = sign_request_canonical(&headers, access_key_secret);
    headers
}

pub fn sign_request_canonical(headers: &SignedHeaders, access_key_secret: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(access_key_secret.as_bytes())
        .expect("HMAC accepts keys of any size");
    mac.update(canonical_signature(headers).as_bytes());
    mac.finalize()
        .into_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
use hmac::{Hmac, Mac};
use md5::{Digest, Md5};
use sha2::Sha256;
