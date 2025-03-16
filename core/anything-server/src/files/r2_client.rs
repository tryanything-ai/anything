use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::Client as S3Client;

// Initialize R2 client
pub async fn get_r2_client() -> S3Client {
    println!("[FILES] Initializing R2 client");
    let r2_account_id = std::env::var("R2_ACCOUNT_ID").expect("R2_ACCOUNT_ID must be set");
    let r2_access_key_id = std::env::var("R2_ACCESS_KEY_ID").expect("R2_ACCESS_KEY_ID must be set");
    let r2_secret_access_key =
        std::env::var("R2_SECRET_ACCESS_KEY").expect("R2_SECRET_ACCESS_KEY must be set");

    let credentials = Credentials::new(
        r2_access_key_id,
        r2_secret_access_key,
        None,
        None,
        "anything-r2",
    );

    let config = aws_sdk_s3::Config::builder()
        .behavior_version_latest()
        .region(Region::new("auto"))
        .endpoint_url(format!(
            "https://{}.r2.cloudflarestorage.com",
            r2_account_id
        ))
        .credentials_provider(credentials)
        .build();

    println!("[FILES] R2 client initialized successfully");
    S3Client::from_conf(config)
}
