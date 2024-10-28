use crate::jwt::{self};
use actix_web::http::header;
use actix_web::{guard::GuardContext, HttpResponse};

// const BEARER: &str = "Bearer";
const JWT_SECRET: &str = "Test secret";
// const STRING_PASSWORD: &str = "TestPass";

pub fn verify_auth_token(ctx: &GuardContext) -> bool {
    let token = match ctx.head().headers.get(header::AUTHORIZATION) {
        Some(value) => value.to_str().unwrap().split(" ").last().unwrap(),
        None => "",
    };
    println!("{:?}", token);
    match jwt::jwt_decode(token.to_string(), JWT_SECRET.to_string()) {
        Ok(claim) => {
            println!("{:?}", claim);
            true
        }
        Err(err) => {
            println!("Error in JWT decode {:?}", err);
            false
        }
    }
}

pub fn unautorized_response() -> HttpResponse {
    HttpResponse::Unauthorized().body("Unautorized user.")
}
