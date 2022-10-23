pub mod s3;
use aws_sdk_s3::Client;

pub struct S3Client {
    client: Client,
}
