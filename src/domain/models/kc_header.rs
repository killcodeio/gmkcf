use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KcHeader {
    pub algo: String,
    pub enc_sym_key: String, // Hex encoded: [EphemeralPub][Nonce][Ciphertext]
    pub nonce: String,       // Hex encoded (body nonce)
    pub recipient_public_key: String, // Hex encoded - Server's genesis public key for lookup
    pub file_id: String, // UUID - Root file ID for key lookup
}
