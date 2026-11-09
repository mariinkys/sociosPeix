use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::{
    app_error::{AppError, AppResult},
    use_cases::auth::AccessTokenIssuer,
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: u64,
    exp: u64,
}

/// HMAC-signed JWT access tokens.
///
/// The algorithm is picked from the secret length exactly like jjwt's
/// `Keys.hmacShaKeyFor` did, so tokens issued by the previous backend stay
/// valid across the upgrade.
pub struct JwtTokenIssuer {
    algorithm: Algorithm,
    encoding: EncodingKey,
    decoding: DecodingKey,
    validation: Validation,
    ttl_secs: u64,
}

impl JwtTokenIssuer {
    pub const MIN_SECRET_LEN: usize = 32;

    pub fn new(secret: &[u8], ttl_secs: u64) -> anyhow::Result<Self> {
        let algorithm = match secret.len() {
            n if n < Self::MIN_SECRET_LEN => anyhow::bail!(
                "JWT_SECRET must be at least {} bytes long (got {n})",
                Self::MIN_SECRET_LEN
            ),
            32..=47 => Algorithm::HS256,
            48..=63 => Algorithm::HS384,
            _ => Algorithm::HS512,
        };
        let mut validation = Validation::new(algorithm);
        validation.leeway = 0;
        validation.set_required_spec_claims(&["exp", "sub"]);
        Ok(Self {
            algorithm,
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
            validation,
            ttl_secs,
        })
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default()
}

impl AccessTokenIssuer for JwtTokenIssuer {
    fn issue(&self, email: &str) -> AppResult<String> {
        let iat = now_secs();
        let claims = Claims {
            sub: email.to_owned(),
            iat,
            exp: iat + self.ttl_secs,
        };
        encode(&Header::new(self.algorithm), &claims, &self.encoding).map_err(AppError::internal)
    }

    fn verify(&self, token: &str) -> Option<String> {
        decode::<Claims>(token, &self.decoding, &self.validation)
            .ok()
            .map(|data| data.claims.sub)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_and_tamper() {
        let jwt = JwtTokenIssuer::new(b"0123456789abcdef0123456789abcdef", 60).unwrap();
        let token = jwt.issue("a@b.c").unwrap();
        assert_eq!(jwt.verify(&token).as_deref(), Some("a@b.c"));

        let other = JwtTokenIssuer::new(b"ffffffffffffffffffffffffffffffff", 60).unwrap();
        assert!(other.verify(&token).is_none());
        assert!(jwt.verify("not.a.jwt").is_none());
    }

    #[test]
    fn rejects_short_secrets_and_picks_alg_by_length() {
        assert!(JwtTokenIssuer::new(b"short", 60).is_err());
        assert_eq!(
            JwtTokenIssuer::new(&[1; 48], 60).unwrap().algorithm,
            Algorithm::HS384
        );
        assert_eq!(
            JwtTokenIssuer::new(&[1; 64], 60).unwrap().algorithm,
            Algorithm::HS512
        );
    }

    #[test]
    fn expired_tokens_are_rejected() {
        let jwt = JwtTokenIssuer::new(&[7; 32], 0).unwrap();
        let claims = Claims {
            sub: "a@b.c".into(),
            iat: 1,
            exp: 2,
        };
        let token = encode(&Header::new(Algorithm::HS256), &claims, &jwt.encoding).unwrap();
        assert!(jwt.verify(&token).is_none());
    }
}
