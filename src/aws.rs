pub mod config;
pub mod s3;
pub mod sns;
use aws_config::SdkConfig;
use aws_sdk_s3::Client as S3ClientBase;
use aws_sdk_sns::Client as SnsClientBase;

pub struct Config {
    sdk_config: SdkConfig,
}
pub struct S3Client {
    client: S3ClientBase,
}
pub struct S3Storage {
    client: S3Client,
    bucket: String,
}
pub struct SnsClient {
    client: SnsClientBase,
    topic_arn: String,
    default_channel: Option<String>,
}
pub struct SnsFifoClient {
    client: SnsClientBase,
    topic_arn: String,
}
