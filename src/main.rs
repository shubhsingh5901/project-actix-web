mod encrypt;
mod jwt;
mod jwt_guard;

use actix_web::{get, guard, post, web, App, HttpResponse, HttpServer, Responder};
use jwt_guard::verify_auth_token;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/getusage")]
async fn get_usage_fn() -> impl Responder {
    HttpResponse::Ok().body("get usage api endpoint!")
}

#[get("/testservice")]
async fn test_service_fn() -> impl Responder {
    HttpResponse::Ok().body("test service endpoint")
}

#[post("/testservicepost")]
async fn test_service_post_fn() -> impl Responder {
    HttpResponse::Ok().body("test service endpoint")
}

#[post("/login")]
async fn login_fn() -> impl Responder {
    HttpResponse::Ok().body("Login Endpoint!")
}

#[post("/signup")]
async fn signup_fn() -> impl Responder {
    HttpResponse::Ok().body("signup Endpoint!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/auth").service(login_fn).service(signup_fn))
            .service(
                web::scope("/service")
                    .guard(guard::fn_guard(verify_auth_token))
                    .service(test_service_fn)
                    .service(test_service_post_fn),
            )
            .service(home)
            .service(get_usage_fn)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
