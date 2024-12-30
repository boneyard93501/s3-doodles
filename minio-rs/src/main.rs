use minio_rsc::client::{BucketArgs, KeyArgs};
use minio_rsc::error::Result;
use minio_rsc::provider::StaticProvider;
use minio_rsc::Minio;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the MinIO client
    let provider = StaticProvider::new("adminUser", "securePassword123", None);

    // build the client
    let minio = Minio::builder()
        .endpoint("localhost:9001")
        .provider(provider)
        .secure(false)
        .build()
        .unwrap();

    // Create a new bucket
    minio.make_bucket(BucketArgs::new("test-bucket"), false).await?;
    println!("Bucket created successfully");

    // Upload an object to the bucket
    minio.put_object("test-bucket", "file.txt", "Hello, MinIO!".into()).await?;
    println!("File uploaded successfully");

    // Retrieve the object
    let data = minio.get_object("test-bucket", "file.txt").await?;
    println!("Retrieved object data: {:?}", data);

    // Delete the object
    minio.remove_object("test-bucket", "file.txt").await?;
    println!("File deleted successfully");

    // Remove the bucket
    minio.remove_bucket("test-bucket").await?;
    println!("Bucket removed successfully");

    Ok(())
}
