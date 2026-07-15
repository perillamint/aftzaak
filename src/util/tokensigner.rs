use crate::error::AppError;
use crate::types::api::auth::TokenPayload;
use chrono::Duration;
use jsonwebtoken::{DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub struct TokenSigner {
    /// JWT secret (HS256)
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    /// JWT ttl
    ttl: Duration,
    /// Issuer
    issuer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub iat: i64,
    pub exp: i64,
    #[serde(flatten)]
    pub custom_claim: TokenPayload,
}

impl TokenSigner {
    pub fn new(config: crate::config::Jwt) -> Self {
        let ttl = Duration::seconds(config.ttl);
        Self {
            encoding_key: EncodingKey::from_secret(config.secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(config.secret.as_bytes()),
            ttl,
            issuer: config.issuer,
        }
    }

    pub fn sign(
        &self,
        sub: String,
        additional_claims: TokenPayload,
    ) -> Result<(String, i64), AppError> {
        let now = chrono::Utc::now();
        let exp = now + self.ttl;
        let claims = Claims {
            iss: self.issuer.clone(),
            sub,
            iat: now.timestamp(),
            exp: exp.timestamp(),
            custom_claim: additional_claims,
        };

        let token = jsonwebtoken::encode(&Header::default(), &claims, &self.encoding_key)?;

        Ok((token, exp.timestamp()))
    }

    pub fn verify(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &self.decoding_key,
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
        )?;

        // Check token validity
        let now = chrono::Utc::now().timestamp();
        if token_data.claims.iat > now {
            return Err(AppError::InvalidCredentials);
        }
        if token_data.claims.exp < now {
            return Err(AppError::InvalidCredentials);
        }
        if token_data.claims.iss != self.issuer {
            return Err(AppError::InvalidCredentials);
        }

        Ok(token_data.claims)
    }
}
