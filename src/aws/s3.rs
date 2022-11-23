mod client;
mod storage;
mod upload;

#[derive(Debug)]
pub enum Error {
    SdkError(String),
}
