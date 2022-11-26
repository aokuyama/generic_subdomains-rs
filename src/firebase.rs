mod validate;
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub enum Error {
    FatalError(String),
    HttpError(String),
    JWKSFetchError(String),
    JwtVaridationError(String),
    JsonDecordeError(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
impl std::error::Error for Error {}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct User {
    user_id: String,
    name: String,
    email: String,
}

pub async fn get_user<T: DeserializeOwned>(token: &str) -> Result<T, Error> {
    let jwk_url = std::env::var("JWK_URL").expect("env JWK_URL must be set");
    let jwk_issuer = std::env::var("JWK_ISSUER").expect("env JWK_ISSUER must be set");

    let value = validate::get_claim(&jwk_url, jwk_issuer, token).await?;
    let user = serde_json::from_value::<T>(value);
    match user {
        Ok(x) => Ok(x),
        Err(err) => Err(Error::JsonDecordeError(err.to_string())),
    }
}
