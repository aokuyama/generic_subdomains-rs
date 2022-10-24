pub mod config;
pub mod s3;
use aws_config::SdkConfig;
use aws_sdk_s3::Client;

pub struct Config {
    sdk_config: SdkConfig,
}
pub struct S3Client {
    client: Client,
}
pub struct S3Storage {
    client: S3Client,
    bucket: String,
}
