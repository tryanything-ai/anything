use data_encoding::HEXLOWER;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io;
use std::io::Error;
use std::path::Path;

pub fn hash_matches(file_hash: &str, correct_hash: &str) -> bool {
    file_hash == correct_hash
}

pub fn hash_file_path(path: &Path) -> Result<String, Error> {
    let mut file = File::open(&path)?;
    hash_file_sha256(&mut file)
}

fn hash_file_sha256(file: &mut File) -> Result<String, Error> {
    let mut hasher = Sha256::new();
    io::copy(file, &mut hasher)?;
    let hash = hasher.finalize();
    Ok(HEXLOWER.encode(hash.as_slice()))
}

pub fn hash_string_sha256(str: &str) -> Result<String, Error> {
    let mut hasher = Sha256::new();
    hasher.update(str);
    let hash = hasher.finalize();
    Ok(HEXLOWER.encode(hash.as_slice()))
}
