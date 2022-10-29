pub mod fifo;
pub mod sns;

#[derive(Debug)]
pub enum Error {
    SdkError(String),
}
