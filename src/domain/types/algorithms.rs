use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportedAsymmetricAlgos {
    #[serde(rename = "x25519")]
    X25519,
}

impl SupportedAsymmetricAlgos {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::X25519 => "x25519",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "x25519" => Some(Self::X25519),
            _ => None,
        }
    }
}

impl fmt::Display for SupportedAsymmetricAlgos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportedSymmetricAlgos {
    #[serde(rename = "xchacha20-poly1305")]
    XChaCha20Poly1305,
}

impl SupportedSymmetricAlgos {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::XChaCha20Poly1305 => "xchacha20-poly1305",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "xchacha20-poly1305" => Some(Self::XChaCha20Poly1305),
            _ => None,
        }
    }
}

impl fmt::Display for SupportedSymmetricAlgos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

