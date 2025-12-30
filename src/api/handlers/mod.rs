use actix_multipart::Multipart;
use actix_web::{HttpResponse, Error, web};
use futures_util::TryStreamExt;
use crate::domain::services::minting::MintingService;
use std::path::Path;

pub async fn mint_media(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut file_bytes: Option<Vec<u8>> = None;
    let mut public_key: Option<String> = None;
    let mut algo: Option<String> = None;
    let mut file_id: Option<String> = None;

    // Iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition(); // This returns &ContentDisposition in recent versions, but error says Option? Let's assume Option if compiler insists.
        // Actually, let's look at the error again: "method not found in std::option::Option<&ContentDisposition>".
        // This means `content_disposition` variable is Option<&ContentDisposition>.
        // So `field.content_disposition()` returns `Option`.
        // We will unwrap it safely.
        if content_disposition.is_none() {
            continue;
        }
        let cd = content_disposition.unwrap();
        let name = cd.get_name().unwrap_or("");

        if name == "file" {
            // *** VALIDATION CHECK ***
            let filename = cd.get_filename().unwrap_or("");
            let extension = Path::new(filename)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_lowercase();

            use crate::domain::types::media::SupportedMediaTypes;
            
            match SupportedMediaTypes::from_extension(&extension) {
                Some(_) => {
                    // Valid Image
                    let mut data = Vec::new();
                    while let Some(chunk) = field.try_next().await? {
                        data.extend_from_slice(&chunk);
                    }
                    file_bytes = Some(data);
                },
                None => {
                    return Ok(HttpResponse::BadRequest().body(format!("Invalid file type: {}. Only images (jpg, png, webp, etc.) are allowed.", extension)));
                }
            }
        } else if name == "public_key" {
            let mut data = Vec::new();
            while let Some(chunk) = field.try_next().await? {
                data.extend_from_slice(&chunk);
            }
            public_key = Some(String::from_utf8_lossy(&data).to_string());
        } else if name == "algo" {
            let mut data = Vec::new();
            while let Some(chunk) = field.try_next().await? {
                data.extend_from_slice(&chunk);
            }
            algo = Some(String::from_utf8_lossy(&data).to_string());
        } else if name == "file_id" {
            let mut data = Vec::new();
            while let Some(chunk) = field.try_next().await? {
                data.extend_from_slice(&chunk);
            }
            file_id = Some(String::from_utf8_lossy(&data).to_string());
        }
    }

    // Check optional fields
    if file_bytes.is_none() {
        return Ok(HttpResponse::BadRequest().body("Missing 'file' field"));
    }
    if public_key.is_none() {
        return Ok(HttpResponse::BadRequest().body("Missing 'public_key' field"));
    }
    if file_id.is_none() {
        return Ok(HttpResponse::BadRequest().body("Missing 'file_id' field"));
    }
    // Default algo if missing?
    let algo_str = algo.unwrap_or("x25519".to_string());
    use crate::domain::types::algorithms::SupportedAsymmetricAlgos;
    
    let asym_algo_enum = SupportedAsymmetricAlgos::from_str(&algo_str)
        .ok_or_else(|| actix_web::error::ErrorBadRequest(format!("Unsupported algorithm: {}", algo_str)))?;

    // Call Service
    match MintingService::mint(file_bytes.unwrap(), public_key.unwrap(), file_id.unwrap(), asym_algo_enum).await {
        Ok(kc_bytes) => {
            Ok(HttpResponse::Ok()
                .content_type("application/octet-stream")
                .body(kc_bytes))
        },
        Err(e) => {
            Ok(HttpResponse::InternalServerError().body(format!("Minting failed: {}", e)))
        }
    }
}


pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}
