use std::error::Error;

use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

const JWT_SECRET: &[u8; 11] = b"Test secret";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    username: String,
    exp: u64,
}

pub fn encode_jwt(username: String) -> String {
    let my_claims = Claims {
        username,
        aud: "test".to_owned(),
        sub: "test@test.com".to_owned(),
        exp: 10000000000,
    };

    let token = match encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(JWT_SECRET),
    ) {
        Ok(t) => t,
        Err(err) => {
            // Add error log and return custom log
            println!("{:?}", err);
            "error".to_owned()
        }
    };
    token
}

pub fn verify_auth_token(token: String) -> bool {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.sub = Some("test@test.com".to_string());
    validation.set_audience(&["test"]);
    validation.set_required_spec_claims(&["exp", "sub", "aud"]);

    match decode::<Claims>(&token, &DecodingKey::from_secret(JWT_SECRET), &validation) {
        Ok(_) => true,
        Err(err) => {
            match *err.kind() {
                ErrorKind::InvalidToken => println!("Token is invalid"), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => println!("Issuer is invalid"), // Example on how to handle a specific error
                _ => println!("Some other errors"),
            }
            false
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ClaimsTime {
    username: String,
    sub: String,
    #[serde(with = "jwt_numeric_date")]
    iat: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    exp: OffsetDateTime,
}

impl ClaimsTime {
    /// If a token should always be equal to its representation after serializing and deserializing
    /// again, this function must be used for construction. `OffsetDateTime` contains a microsecond
    /// field but JWT timestamps are defined as UNIX timestamps (seconds). This function normalizes
    /// the timestamps.
    pub fn new(sub: String, iat: OffsetDateTime, exp: OffsetDateTime, username: String) -> Self {
        // normalize the timestamps by stripping of microseconds
        let iat = iat
            .date()
            .with_hms_milli(iat.hour(), iat.minute(), iat.second(), 0)
            .unwrap()
            .assume_utc();
        let exp = exp
            .date()
            .with_hms_milli(exp.hour(), exp.minute(), exp.second(), 0)
            .unwrap()
            .assume_utc();

        Self {
            sub,
            iat,
            exp,
            username,
        }
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

fn jwt_time() -> Result<(), Box<dyn Error>> {
    let sub = "Custom OffsetDateTime ser/de".to_string();
    let iat = OffsetDateTime::now_utc();
    let exp = iat + Duration::days(1);

    let claims = ClaimsTime::new(sub, iat, exp, "test".to_string());

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )?;

    println!("serialized token: {}", &token);

    let token_data = jsonwebtoken::decode::<ClaimsTime>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;

    println!("token data:\n{:#?}", &token_data);
    Ok(())
}
fn main() {
    // let token = encode_jwt("test_user".to_owned());

    // println!("token {:?}", token);

    // let is_valid = verify_auth_token(token);

    // println!("is valid {:?}", is_valid)
    jwt_time().unwrap();
}
