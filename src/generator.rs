use rand::{thread_rng, Rng};
use uuid::Uuid;
use base64::{Engine as _, engine::general_purpose};

#[derive(Debug, Clone, Copy)]
pub enum TokenFormat {
    Hex,
    Base64,
    Uuid,
}

pub fn generate_password(length: usize, use_special: bool, use_numbers: bool) -> String {
    let mut chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    
    if use_numbers {
        chars.push_str("0123456789");
    }
    
    if use_special {
        chars.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
    }
    
    let mut rng = thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars.chars().nth(idx).unwrap()
        })
        .collect()
}

pub fn generate_token(format: TokenFormat, length: usize) -> String {
    let mut rng = thread_rng();
    
    match format {
        TokenFormat::Hex => {
            (0..length)
                .map(|_| format!("{:02x}", rng.gen::<u8>()))
                .collect::<String>()
                .chars()
                .take(length)
                .collect()
        }
        TokenFormat::Base64 => {
            let bytes: Vec<u8> = (0..(length * 3 / 4 + 1))
                .map(|_| rng.gen())
                .collect();
            general_purpose::STANDARD.encode(&bytes).chars().take(length).collect()
        }
        TokenFormat::Uuid => {
            let mut bytes = [0u8; 16];
            for byte in &mut bytes {
                *byte = rng.gen();
            }
            Uuid::from_bytes(bytes).to_string()
        }
    }
}