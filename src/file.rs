use std::fs::File;
use std::io;

#[derive(Debug)]
pub enum Error {
    ReqwestError(String),
    StdError(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
impl std::error::Error for Error {}

pub async fn download(url: &str, local_path: &str) -> Result<(), Error> {
    let response = match reqwest::get(url).await {
        Ok(response) => response,
        Err(err) => return Err(Error::ReqwestError(err.to_string())),
    };
    let bytes = match response.bytes().await {
        Ok(bytes) => bytes,
        Err(err) => return Err(Error::ReqwestError(err.to_string())),
    };
    let mut out = match File::create(local_path) {
        Ok(bytes) => bytes,
        Err(err) => return Err(Error::StdError(err.to_string())),
    };
    match io::copy(&mut bytes.as_ref(), &mut out) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::StdError(err.to_string())),
    }
}
