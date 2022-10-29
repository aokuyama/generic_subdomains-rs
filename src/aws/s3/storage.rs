use super::Error;
use crate::aws::{Config, S3Client, S3Storage};

impl S3Storage {
    pub fn new(config: &Config, bucket: &str) -> Self {
        let client = S3Client::new(config);
        S3Storage {
            client,
            bucket: bucket.to_string(),
        }
    }
    pub async fn upload(&self, local_path: &str, upload_path: &str) -> Result<(), Error> {
        self.client
            .upload(&self.bucket, local_path, upload_path)
            .await
    }
}
