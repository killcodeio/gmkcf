use crate::domain::models::kc_header::KcHeader;
use crate::core::format::constants::MAGIC_BYTES;
use anyhow::{Context, Result};
use byteorder::{BigEndian, WriteBytesExt};
use serde_json;
use std::io::Write;

pub struct KcFileBuilder {
    header: Option<KcHeader>,
    body: Option<Vec<u8>>,
}

impl KcFileBuilder {
    pub fn new() -> Self {
        Self {
            header: None,
            body: None,
        }
    }

    pub fn set_header(mut self, header: KcHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn set_body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn build(self) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.write_to(&mut buffer)?;
        Ok(buffer)
    }

    pub fn write_to<W: Write>(self, writer: &mut W) -> Result<()> {
        let header = self.header.context("Header is required")?;
        
        // 1. Write Immutables: Magic
        writer.write_all(MAGIC_BYTES)?;

        // 2. Prepare Header JSON
        let json_bytes = serde_json::to_vec(&header).context("Failed to serialize header")?;
        let header_len = json_bytes.len() as u32;

        // 3. Write Header Length (u32 Big Endian)
        writer.write_u32::<BigEndian>(header_len)?;

        // 4. Write Header JSON
        writer.write_all(&json_bytes)?;

        // 5. Write Body (if present)
        if let Some(body) = self.body {
            writer.write_all(&body)?;
        }

        Ok(())
    }
}
