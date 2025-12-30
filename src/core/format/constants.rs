// RFC 001: "KCF\x01"
pub const MAGIC_BYTES: &[u8; 4] = b"KCF\x01";

// Current version is implicit in the magic byte locally, 
// but we might want explicit constants if we change it.
pub const VERSION_V1: u8 = 0x01;
