use crate::core::crypto::{symmetric, asymmetric};
use crate::core::format::builder::KcFileBuilder;
use crate::domain::models::kc_header::KcHeader;
use anyhow::{Context, Result};
use hex;

use crate::domain::types::algorithms::{SupportedAsymmetricAlgos, SupportedSymmetricAlgos};

pub struct MintingService;

impl MintingService {
    pub async fn mint(file_bytes: Vec<u8>, public_key_hex: String, file_id: String, asym_algo: SupportedAsymmetricAlgos) -> Result<Vec<u8>> {
        // MVP: We currently hardcode the Symmetric Algo to XChaCha20Poly1305 internally
        // or we could accept it as an argument if needed. 
        // For now, `asym_algo` drives the key exchange.
        let _sym_algo = SupportedSymmetricAlgos::XChaCha20Poly1305;

        // 1. Decode Public Key
        let public_key_bytes = hex::decode(&public_key_hex)
            .context("Invalid Hex Public Key")?;
        
        let public_key: [u8; 32] = public_key_bytes.try_into()
            .map_err(|_| anyhow::anyhow!("Public Key must be 32 bytes"))?;

        // 3. Generate Symmetric Key
        let sym_key = symmetric::generate_sym_key();

        // 4. Encrypt Body (Symmetric)
        let (encrypted_body, body_nonce) = symmetric::encrypt_body(&file_bytes, &sym_key)?;

        // 5. Encrypt Key (Asymmetric - Key Wrapping)
        let encrypted_key_blob = asymmetric::encrypt_key(&sym_key, &public_key)?;
        
        // 6. Construct Header
        let header = KcHeader {
            algo: asym_algo.as_str().to_string(), // In future we might combine "asym+sym" string here
            enc_sym_key: hex::encode(encrypted_key_blob),
            nonce: hex::encode(body_nonce),
            recipient_public_key: public_key_hex, // Store server's public key for lookup
            file_id, // Store file ID for key exchange
            genealogy: None, 
        };

        // 7. Assemble File
        let kc_file = KcFileBuilder::new()
            .set_header(header)
            .set_body(encrypted_body)
            .build()?;

        Ok(kc_file)
    }
}
