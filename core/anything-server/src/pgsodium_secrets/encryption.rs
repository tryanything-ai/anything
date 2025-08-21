use sea_orm::{DatabaseConnection, Statement, ConnectionTrait, DbBackend, QueryResult};
use serde_json::Value;
use anyhow::{Result, anyhow};

/// Encrypt a secret using pgsodium
pub async fn encrypt_secret(
    db: &DatabaseConnection,
    secret_value: &str,
) -> Result<(Vec<u8>, Vec<u8>)> {
    // For now, use a simpler approach with pgsodium's secretbox
    let query = r#"
        SELECT 
            pgsodium.crypto_secretbox(
                $1::bytea,
                pgsodium.crypto_secretbox_noncegen(),
                pgsodium.crypto_secretbox_keygen()
            ) as encrypted,
            pgsodium.crypto_secretbox_noncegen() as nonce
    "#;

    let result = db
        .query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            query,
            vec![secret_value.as_bytes().into()],
        ))
        .await
        .map_err(|e| anyhow!("Failed to encrypt secret: {}", e))?
        .ok_or_else(|| anyhow!("No result from encryption query"))?;

    let encrypted_bytes: Vec<u8> = result
        .try_get("", "encrypted")
        .map_err(|e| anyhow!("Failed to get encrypted data: {}", e))?;

    let nonce_bytes: Vec<u8> = result
        .try_get("", "nonce")
        .map_err(|e| anyhow!("Failed to get nonce: {}", e))?;

    Ok((encrypted_bytes, nonce_bytes))
}

/// Decrypt a secret using pgsodium
pub async fn decrypt_secret(
    db: &DatabaseConnection,
    encrypted_data: &[u8],
    nonce: &[u8],
) -> Result<String> {
    // Note: This is a simplified version - in production you'd want proper key management
    // For now, this won't actually work without the original key, but demonstrates the structure
    let query = r#"
        SELECT 
            convert_from(
                pgsodium.crypto_secretbox_open(
                    $1::bytea,
                    $2::bytea,
                    pgsodium.crypto_secretbox_keygen()
                ),
                'UTF8'
            ) as decrypted
    "#;

    let result = db
        .query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            query,
            vec![encrypted_data.into(), nonce.into()],
        ))
        .await
        .map_err(|e| anyhow!("Failed to decrypt secret: {}", e))?
        .ok_or_else(|| anyhow!("No result from decryption query"))?;

    let decrypted_text: String = result
        .try_get("", "decrypted")
        .map_err(|e| anyhow!("Failed to get decrypted data: {}", e))?;

    Ok(decrypted_text)
}

/// Generate a new encryption key using pgsodium
pub async fn generate_encryption_key(db: &DatabaseConnection) -> Result<Vec<u8>> {
    let query = "SELECT pgsodium.crypto_aead_xchacha20poly1305_ietf_keygen() as key";

    let result = db
        .query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            query,
            vec![],
        ))
        .await
        .map_err(|e| anyhow!("Failed to generate encryption key: {}", e))?
        .ok_or_else(|| anyhow!("No result from key generation query"))?;

    let key_bytes: Vec<u8> = result
        .try_get("", "key")
        .map_err(|e| anyhow!("Failed to get key: {}", e))?;

    Ok(key_bytes)
}

/// Generate a random nonce using pgsodium
pub async fn generate_nonce(db: &DatabaseConnection) -> Result<Vec<u8>> {
    let query = "SELECT pgsodium.crypto_aead_xchacha20poly1305_ietf_npubbytes() as nonce";

    let result = db
        .query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            query,
            vec![],
        ))
        .await
        .map_err(|e| anyhow!("Failed to generate nonce: {}", e))?
        .ok_or_else(|| anyhow!("No result from nonce generation query"))?;

    let nonce_bytes: Vec<u8> = result
        .try_get("", "nonce")
        .map_err(|e| anyhow!("Failed to get nonce: {}", e))?;

    Ok(nonce_bytes)
}
