use aes_gcm::{aead::Aead, Aes128Gcm, Key, KeyInit, Nonce};
use anyhow::anyhow;
use base64::{engine::general_purpose, Engine};
use rand::Rng;

pub fn aes128encrypt(str: &str) -> anyhow::Result<String> {
    let keystr = super::config::get_aes128key();
    let key = Key::<Aes128Gcm>::from_slice(keystr.as_bytes());
    let cipher = Aes128Gcm::new(key);

    let nonce_bytes: [u8; 12] = rand::thread_rng().gen();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, str.as_bytes())
        .map_err(|err| anyhow!(err))?;
    let result = general_purpose::STANDARD.encode([&nonce_bytes[..], &ciphertext[..]].concat());
    Ok(result)
}

pub fn aes128decrypt(str: &str) -> anyhow::Result<String> {
    let keystr = super::config::get_aes128key();
    let key = Key::<Aes128Gcm>::from_slice(keystr.as_bytes());
    let cipher = Aes128Gcm::new(key);
    let decoded_bytes = general_purpose::STANDARD.decode(str)?;

    let (nonce_bytes, ciphertext) = decoded_bytes.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let decrypted_text = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|err| anyhow!(err))?;
    let result = String::from_utf8(decrypted_text)?;
    Ok(result)
}

#[tokio::test]
async fn enctest() {
    use crate::config::app_config::AppConfig;
    AppConfig::init().await;
    let plain = r#""{"key1":"ha", "key2":"haha"}""#;
    let enc = aes128encrypt(plain).unwrap();
    let dec = aes128decrypt(&enc).unwrap();
    assert_eq!(plain, dec);
}
