use actix_web::http::header;
use actix_web::{guard::GuardContext, HttpResponse};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

// const BEARER: &str = "Bearer";
const JWT_SECRET: &[u8; 11] = b"Test secret";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    username: String,
    exp: u64,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub token: String,
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

pub fn verify_auth_token(ctx: &GuardContext) -> bool {
    let token = match ctx.head().headers.get(header::AUTHORIZATION) {
        Some(value) => value.to_str().unwrap(),
        None => "",
    };

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
            unautorized_response();
            false
        }
    }
}

pub fn unautorized_response() -> HttpResponse {
    HttpResponse::Unauthorized().body("Unautorized user.")
}
