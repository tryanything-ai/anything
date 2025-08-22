use aws_sdk_s3::primitives::ByteStream;
use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, Set, QueryOrder, Order};

use std::sync::Arc;
use uuid::Uuid;

use crate::{custom_auth::User, AppState, entities::files};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileAccessType {
    Private, // Requires signed URL
    Public,  // Always accessible via CDN
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    pub file_id: String,
    pub file_name: String,
    pub file_size: i64,
    pub content_type: String,
    // created_at: chrono::DateTime<chrono::Utc>,
    pub account_id: String,
    pub path: Option<String>,        // For future folder support
    pub public_url: Option<String>,  // For public files
    pub access_type: FileAccessType, // Controls how the file can be accessed
}

#[derive(Debug, Serialize)]
pub struct FileResponse {
    file_id: String,
    file_name: String,
    content_type: String,
    size: i64,
    url: String,
    base64: Option<String>,
}

// Add this helper function at the top with other imports
fn make_filename_url_safe(filename: &str) -> String {
    // Replace spaces and problematic characters with underscores or dashes
    // Remove or encode special characters that could cause URL issues
    filename
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
}

// Get files for an account
pub async fn get_files(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(_user): Extension<User>,
) -> impl IntoResponse {
    println!("[FILES] Getting files for account: {} (SeaORM)", account_id);

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    let files_result = match files::Entity::find()
        .filter(files::Column::AccountId.eq(account_uuid))
        .order_by(files::Column::CreatedAt, Order::Desc)
        .all(&*state.db)
        .await
    {
        Ok(files) => files,
        Err(err) => {
            println!("[FILES] Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch files").into_response();
        }
    };

    // Convert to JSON format expected by frontend
    let files_json: Vec<Value> = files_result
        .into_iter()
        .map(|file| {
            json!({
                "file_id": file.file_id,
                "file_name": file.file_name,
                "file_size": file.file_size,
                "content_type": file.file_type,
                "account_id": file.account_id,
                "path": file.file_key,
                "public_url": file.file_url,
                "access_level": if file.file_url.is_some() { "public" } else { "private" },
                "created_at": file.created_at,
                "updated_at": file.updated_at
            })
        })
        .collect();

    println!(
        "[FILES] Successfully retrieved {} files (SeaORM)",
        files_json.len()
    );
    Json(files_json).into_response()
}

// Upload a file
pub async fn upload_file(
    Path((account_id, access)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(_user): Extension<User>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    println!("[FILES] Starting file upload for account: {} (SeaORM)", account_id);
    let r2_client = &state.r2_client;
    let bucket = std::env::var("R2_BUCKET").expect("R2_BUCKET must be set");
    let cdn_domain = std::env::var("R2_PUBLIC_DOMAIN").expect("R2_PUBLIC_DOMAIN must be set");

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    // Check access type from path parameter
    let is_private = access == "private";
    println!(
        "[FILES] File access type: {}",
        if is_private { "private" } else { "public" }
    );

    while let Some(field) = multipart.next_field().await.unwrap() {
        let original_filename = field.file_name().unwrap_or("unnamed").to_string();
        let safe_filename = make_filename_url_safe(&original_filename);
        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();
        let data = field.bytes().await.unwrap();

        println!(
            "[FILES] Processing file: {} ({} bytes) - sanitized name: {}",
            original_filename,
            data.len(),
            safe_filename
        );

        let file_id = Uuid::new_v4();
        // Use the safe filename for the storage key
        let r2_key = format!("{}/{}", account_id, safe_filename);

        let mut put_object = r2_client
            .put_object()
            .bucket(&bucket)
            .key(&r2_key)
            .body(ByteStream::from(data.clone()))
            .content_type(content_type.clone());

        // Set ACL based on access type
        if !is_private {
            put_object = put_object.acl(aws_sdk_s3::types::ObjectCannedAcl::PublicRead);
        }

        match put_object.send().await {
            Ok(_) => {
                println!("[FILES] Successfully uploaded file to R2: {}", r2_key);
                
                let new_file = files::ActiveModel {
                    file_id: Set(file_id),
                    account_id: Set(account_uuid),
                    file_name: Set(safe_filename.clone()),
                    file_size: Set(Some(data.len() as i64)),
                    file_type: Set(Some(content_type)),
                    file_key: Set(Some(r2_key.clone())),
                    file_url: Set(if !is_private {
                        Some(format!("{}/{}", cdn_domain, r2_key))
                    } else {
                        None
                    }),
                    archived: Set(false),
                    ..Default::default()
                };

                match new_file.insert(&*state.db).await {
                    Ok(_) => {
                        println!("[FILES] Successfully stored file metadata for: {} (SeaORM)", file_id);
                        return Json(json!({
                            "status": "success",
                            "file_id": file_id.to_string()
                        }))
                        .into_response();
                    }
                    Err(err) => {
                        println!("[FILES] Failed to store file metadata: {:?}", err);
                        // Cleanup R2 if database insert fails
                        let _ = r2_client
                            .delete_object()
                            .bucket(&bucket)
                            .key(&r2_key)
                            .send()
                            .await;
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to store file metadata",
                        )
                            .into_response();
                    }
                }
            }
            Err(e) => {
                println!("[FILES] Failed to upload file to R2: {:?}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to upload file to storage",
                )
                    .into_response();
            }
        }
    }

    println!("[FILES] No file provided in request");
    (StatusCode::BAD_REQUEST, "No file provided").into_response()
}

// Delete a file
pub async fn delete_file(
    Path((account_id, file_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(_user): Extension<User>,
) -> impl IntoResponse {
    println!(
        "[FILES] Deleting file {} for account {} (SeaORM)",
        file_id, account_id
    );
    let r2_client = &state.r2_client;
    let bucket = std::env::var("R2_BUCKET").expect("R2_BUCKET must be set");

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    let file_uuid = match Uuid::parse_str(&file_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid file ID").into_response(),
    };

    // First, get the file metadata
    let file = match files::Entity::find()
        .filter(files::Column::FileId.eq(file_uuid))
        .filter(files::Column::AccountId.eq(account_uuid))
        .one(&*state.db)
        .await
    {
        Ok(Some(file)) => file,
        Ok(None) => {
            println!("[FILES] File not found");
            return (StatusCode::NOT_FOUND, "File not found").into_response();
        }
        Err(err) => {
            println!("[FILES] Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    // Delete from R2
    let r2_key = file.file_key.clone().unwrap_or_else(|| format!("{}_{}", file_id, file.file_name));
    println!("[FILES] Deleting file from R2: {}", r2_key);

    match r2_client
        .delete_object()
        .bucket(&bucket)
        .key(&r2_key)
        .send()
        .await
    {
        Ok(_) => {
            println!("[FILES] Successfully deleted file from R2");
            // Delete metadata from database
            match files::Entity::delete_by_id(file_uuid).exec(&*state.db).await {
                Ok(_) => {
                    println!("[FILES] Successfully deleted file metadata (SeaORM)");
                    Json(json!({"status": "success"})).into_response()
                }
                Err(err) => {
                    println!("[FILES] Failed to delete file metadata: {:?}", err);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to delete file metadata",
                    )
                        .into_response()
                }
            }
        }
        Err(e) => {
            println!("[FILES] Failed to delete file from R2: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete file from storage",
            )
                .into_response()
        }
    }
}

// Get a pre-signed download URL for a file
pub async fn get_file_download_url(
    Path((account_id, file_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(_user): Extension<User>,
) -> impl IntoResponse {
    println!(
        "[FILES] Getting download URL for file {} in account {} (SeaORM)",
        file_id, account_id
    );
    let r2_client = &state.r2_client;
    let bucket = std::env::var("R2_BUCKET").expect("R2_BUCKET must be set");

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    let file_uuid = match Uuid::parse_str(&file_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid file ID").into_response(),
    };

    // Get file metadata
    let file = match files::Entity::find()
        .filter(files::Column::FileId.eq(file_uuid))
        .filter(files::Column::AccountId.eq(account_uuid))
        .one(&*state.db)
        .await
    {
        Ok(Some(file)) => file,
        Ok(None) => {
            println!("[FILES] File not found");
            return (StatusCode::NOT_FOUND, "File not found").into_response();
        }
        Err(err) => {
            println!("[FILES] Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    // If public, return CDN URL
    if let Some(public_url) = file.file_url {
        println!("[FILES] Returning public CDN URL for file");
        return Json(json!({
            "download_url": public_url
        }))
        .into_response();
    }

    // If private, generate presigned URL
    let r2_key = file.file_key.clone().unwrap_or_else(|| format!("{}_{}", file_id, file.file_name));
    
    println!("[FILES] Generating presigned URL for private file");
    let presigned_request = match r2_client
        .get_object()
        .bucket(&bucket)
        .key(&r2_key)
        .presigned(
            aws_sdk_s3::presigning::PresigningConfig::expires_in(std::time::Duration::from_secs(
                3600,
            ))
            .unwrap(),
        )
        .await
    {
        Ok(url) => url,
        Err(e) => {
            println!("[FILES] Failed to generate presigned URL: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to generate download URL",
            )
                .into_response();
        }
    };

    println!("[FILES] Successfully generated download URL (SeaORM)");
    Json(json!({
        "download_url": presigned_request.uri().to_string()
    }))
    .into_response()
}
