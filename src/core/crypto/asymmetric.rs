use x25519_dalek::{PublicKey, StaticSecret};
use rand::rngs::OsRng;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305, XNonce
};
use anyhow::{Context, Result};
use rand::RngCore;

/// Encrypts the symmetric key for the recipient.
/// Uses an Ephemeral Key Exchange (Sealed Box equivalent logic).
/// 1. Generate Ephemeral Private Key (E_priv).
/// 2. Derive Shared Secret (S = E_priv * Recipient_Pub).
/// 3. Derive Wrapping Key (K_wrap = HKDF(S) or simpler Hash(S)).
///    For MVP we will just use the Shared Secret directly as the Key for XChaCha20 (assuming S is 32 bytes).
///    X25519 Shared Secret is 32 bytes.
/// 4. Encrypt `sym_key` with `K_wrap`.
/// 5. Return `E_pub` + `Encrypted_Sym_Key` + `Nonce`.
/// 
/// Wait, standard SealedBox usually handles nonces.
/// Let's implement a manual "Ephem-Static DH" wrap.
/// Output format: `[EphemeralPubKey (32)][Nonce (24)][Ciphertext]`
pub fn encrypt_key(sym_key: &[u8], recipient_pub_key_bytes: &[u8; 32]) -> Result<Vec<u8>> {
    let recipient_pub = PublicKey::from(*recipient_pub_key_bytes);
    let ephemeral_priv = StaticSecret::random_from_rng(OsRng);
    let ephemeral_pub = PublicKey::from(&ephemeral_priv);
    
    let shared_secret = ephemeral_priv.diffie_hellman(&recipient_pub);
    
    // Use Shared Secret as Key for XChaCha20
    let wrapper_cipher = XChaCha20Poly1305::new(shared_secret.as_bytes().into());
    
    let mut nonce_bytes = [0u8; 24];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = XNonce::from_slice(&nonce_bytes);
    
    let encrypted_key = wrapper_cipher.encrypt(nonce, sym_key)
        .map_err(|e| anyhow::anyhow!("Key Wrap failure: {}", e))?;

    // Pack it up: EphemeralPub + Nonce + Ciphertext
    let mut output = Vec::new();
    output.extend_from_slice(ephemeral_pub.as_bytes());
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&encrypted_key);
    
    Ok(output)
}
