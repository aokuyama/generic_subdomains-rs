mod client;
mod storage;
mod upload;

#[derive(Debug)]
pub enum Error {
    SdkError(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
impl std::error::Error for Error {}
