use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use time::{Duration, OffsetDateTime};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenPayload {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub role: String,
    pub sub: String,
    #[serde(with = "jwt_numeric_date")]
    pub iat: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    pub exp: OffsetDateTime,
}

pub fn sign_token(id: String, name: String, phone: String, role: String) -> String {
    let secret = env::var("SECRET").unwrap();
    let sub = "Custom OffsetDateTime ser/de".to_string();
    let iat = OffsetDateTime::now_utc();
    let exp = iat + Duration::days(365);

    let option = TokenPayload {
        id,
        name,
        phone,
        role,
        sub,
        iat,
        exp,
    };

    let token = encode(
        &Header::default(),
        &option,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();
    token
}

pub fn verify_token(token: String) -> Option<TokenPayload> {
    let secret = env::var("SECRET").unwrap_or("123".to_string());

    let validation = Validation::new(Algorithm::HS256);

    let decoded = decode::<TokenPayload>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    );

    if let Ok(tk) = decoded {
        Some(tk.claims)
    } else {
        None
    }
}

mod jwt_numeric_date {
    //! Custom serialization of OffsetDateTime to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    /// Serializes an OffsetDateTime to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.unix_timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        OffsetDateTime::from_unix_timestamp(i64::deserialize(deserializer)?)
            .map_err(|_| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}
