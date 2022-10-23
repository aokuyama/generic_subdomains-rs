use crate::aws::S3Client;
use aws_sdk_s3::types::ByteStream;
use std::path::Path;

impl S3Client {
    pub async fn upload(&self, bucket_name: &str, local_path: &str, upload_path: &str) {
        let body = ByteStream::from_path(Path::new(local_path)).await.unwrap();
        let key = upload_path;

        let result = self
            .client
            .put_object()
            .bucket(bucket_name)
            .body(body)
            .key(key)
            .send()
            .await;

        match result {
            Ok(_) => (),
            Err(err) => panic!("{}", err),
        }
    }
}
