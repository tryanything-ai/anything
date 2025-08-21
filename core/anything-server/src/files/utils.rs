use crate::files::routes_seaorm::FileMetadata;
use crate::templater::utils::FileRequirement;
use crate::AppState;
use crate::entities::files;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileData {
    pub file_name: String,
    pub file_extension: String,
    pub content: Value, // Will contain either file_base64 or file_url
}

pub async fn get_files(
    state: Arc<AppState>,
    account_id: &str,
    file_requirements: Vec<FileRequirement>,
) -> Result<Vec<FileData>, Box<dyn Error + Send + Sync>> {
    // Early return if no file requirements
    if file_requirements.is_empty() {
        println!("[FILES] No file requirements");
        return Ok(Vec::new());
    }

    let cdn_domain = std::env::var("R2_PUBLIC_DOMAIN").expect("R2_PUBLIC_DOMAIN must be set");
    let bucket = std::env::var("R2_BUCKET").expect("R2_BUCKET must be set");
    let mut files_data = Vec::new();

    // Get all files for this account using SeaORM
    let account_uuid = uuid::Uuid::parse_str(account_id)
        .map_err(|e| format!("Invalid account_id: {}", e))?;
    
    let file_models = files::Entity::find()
        .filter(files::Column::AccountId.eq(account_uuid))
        .all(&*state.db)
        .await?;

    // Convert SeaORM models to FileMetadata format
    let files: Vec<FileMetadata> = file_models.iter().map(|model| FileMetadata {
        file_id: model.file_id,
        account_id: model.account_id,
        file_name: model.file_name.clone(),
        file_size: model.file_size,
        file_type: model.file_type.clone(),
        path: model.path.clone(),
        public_url: model.public_url.clone(),
        access_level: model.access_level.clone(),
        created_at: model.created_at,
        updated_at: model.updated_at,
    }).collect();

    println!("[FILES] Files from database: {:?}", files);

    // Create a map of filename to metadata for quick lookup
    let file_metadata_map: HashMap<String, &FileMetadata> = files
        .iter()
        .map(|file| (file.file_name.clone(), file))
        .collect();

    for requirement in file_requirements {
        // Look up the file metadata
        if let Some(metadata) = file_metadata_map.get(&requirement.file_name_with_extension) {
            let content = match requirement.format.as_str() {
                "base64" => {
                    // For base64, we need to fetch from R2 and convert
                    let r2_key = metadata
                        .path
                        .clone()
                        .unwrap_or_else(|| format!("{}/{}", account_id, metadata.file_name));

                    let r2_client = state.r2_client.clone();

                    let object = r2_client
                        .get_object()
                        .bucket(&bucket)
                        .key(&r2_key)
                        .send()
                        .await?;

                    let data = object.body.collect().await?.into_bytes();
                    let base64_data = base64::encode(data);

                    json!({
                            "file_base64": base64_data
                    })
                }
                "url" => {
                    // For URLs, use the public URL from metadata or construct from CDN
                    let url = metadata.public_url.clone().unwrap_or_else(|| {
                        format!("{}/{}/{}", cdn_domain, account_id, metadata.file_name)
                    });

                    json!({
                            "file_url": url
                    })
                }
                _ => return Err("Unsupported file format".into()),
            };

            files_data.push(FileData {
                file_name: requirement.file_name.clone(),
                file_extension: requirement.file_extension.clone(),
                content,
            });
        } else {
            println!("[FILES] File not found: {}", requirement.file_name);
            // Skip files that don't exist
            continue;
        }
    }

    println!("[FILES] Files data: {:?}", files_data);

    Ok(files_data)
}
