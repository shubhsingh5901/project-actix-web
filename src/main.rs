use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

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

#[get("/login")]
async fn login_fn() -> impl Responder {
    HttpResponse::Ok().body("Login Endpoint!")
}

#[get("/signup")]
async fn signup_fn() -> impl Responder {
    HttpResponse::Ok().body("signup Endpoint!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(login_fn)
            .service(signup_fn)
            .service(test_service_fn)
            .service(get_usage_fn)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
