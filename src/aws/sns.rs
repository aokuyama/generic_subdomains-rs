pub mod client;
pub mod fifo;

#[derive(Debug)]
pub enum Error {
    SdkError(String),
}
