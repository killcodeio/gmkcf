use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportedMediaTypes {
    Jpeg,
    Png,
    Gif,
    Webp,
    Bmp,
}

impl SupportedMediaTypes {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Jpeg => "jpeg", // Normalized to jpeg, or could handle jpg/jpeg in from_str
            Self::Png => "png",
            Self::Gif => "gif",
            Self::Webp => "webp",
            Self::Bmp => "bmp",
        }
    }

    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "png" => Some(Self::Png),
            "gif" => Some(Self::Gif),
            "webp" => Some(Self::Webp),
            "bmp" => Some(Self::Bmp),
            _ => None,
        }
    }
}

impl fmt::Display for SupportedMediaTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
