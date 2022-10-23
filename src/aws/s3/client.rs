use crate::aws::S3Client;
use aws_config::SdkConfig;
use aws_sdk_s3::Client;

impl S3Client {
    pub fn new(config: &SdkConfig) -> Self {
        let client = Client::new(config);
        S3Client { client }
    }
}
