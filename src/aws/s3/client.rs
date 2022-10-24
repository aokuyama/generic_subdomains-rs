use crate::aws::{Config, S3Client};
use aws_sdk_s3::Client;

impl S3Client {
    pub fn new(config: &Config) -> Self {
        let client = Client::new(config.sdk());
        S3Client { client }
    }
}
