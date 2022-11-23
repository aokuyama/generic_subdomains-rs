use super::Error;
use alcoholic_jwt::{token_kid, Validation, JWK, JWKS};
use serde_json::Value;

pub async fn get_claim(jwk_url: &str, jwk_issuer: String, token: &str) -> Result<Value, Error> {
    let jwks = fetch_jwks(jwk_url).await?;
    let jwk = get_jwk(&jwks, token).await?;
    let validations = vec![Validation::Issuer(jwk_issuer), Validation::SubjectPresent];
    match alcoholic_jwt::validate(token, jwk, validations) {
        Ok(jwt) => Ok(jwt.claims),
        Err(err) => Err(Error::JwtVaridationError(err.to_string())),
    }
}

async fn get_jwk<'a>(jwks: &'a JWKS, token: &'a str) -> Result<&'a JWK, Error> {
    let kid = match token_kid(token) {
        Ok(res) => match res {
            Some(kid) => kid,
            None => return Err(Error::JWKSFetchError("failed to decode kid".to_owned())),
        },
        Err(err) => return Err(Error::JwtVaridationError(err.to_string())),
    };

    match jwks.find(&kid) {
        Some(jwk) => Ok(jwk),
        None => {
            return Err(Error::JWKSFetchError(
                "Specified key not found in set".to_owned(),
            ))
        }
    }
}

async fn fetch_jwks(uri: &str) -> Result<JWKS, Error> {
    let val: JWKS = match reqwest::get(uri).await {
        Ok(res) => match res.json().await {
            Ok(v) => v,
            Err(err) => return Err(Error::FatalError(err.to_string())),
        },
        Err(err) => {
            return Err(Error::HttpError(
                "fetch error:".to_string() + &err.to_string(),
            ))
        }
    };
    Ok(val)
}
