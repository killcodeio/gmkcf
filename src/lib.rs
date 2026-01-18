#[cfg(feature = "server")]
pub mod api;
pub mod core;
pub mod domain;

pub use domain::models::kc_header::KcHeader;
pub use core::format::builder::KcFileBuilder;
pub use core::format::constants::MAGIC_BYTES;
