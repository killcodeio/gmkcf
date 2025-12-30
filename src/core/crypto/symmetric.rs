use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    XChaCha20Poly1305, XNonce
};
use rand::RngCore;
use anyhow::{Context, Result};

pub fn generate_sym_key() -> [u8; 32] {
    XChaCha20Poly1305::generate_key(&mut OsRng).into()
}

pub fn encrypt_body(data: &[u8], key: &[u8; 32]) -> Result<(Vec<u8>, [u8; 24])> {
    let cipher = XChaCha20Poly1305::new(key.into());
    let mut nonce_bytes = [0u8; 24];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = XNonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, data)
        .map_err(|e| anyhow::anyhow!("Encryption failure: {}", e))?;

    Ok((ciphertext, nonce_bytes))
}
