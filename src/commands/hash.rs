use anyhow::{Ok, Result};
use sha2::{Sha256, Digest};
use std::{fs::File, io::{Read}};

pub async fn run(file: String) -> Result<()> {
    let mut file = File::open(file)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 4096];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let result = hasher.finalize();
    println!("SHA-256: {:x}", result);

    Ok(())
}