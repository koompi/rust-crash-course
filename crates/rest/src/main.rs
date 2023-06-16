use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use middleware::auth::TokenPayload;
use serde_json::json;

// pub mod api;
// pub mod models;
pub mod middleware;

#[get("/public")]
async fn public() -> impl Responder {
    HttpResponse::Ok().body("404!")
}

#[get("")]
async fn private() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn get_auth(req: HttpRequest) -> Option<TokenPayload> {
    let u = req.headers().get("Authorization");

    if u.is_none() {
        return None;
    }
    let bearer = u.unwrap().to_str().unwrap().to_string();
    let token = bearer.replace("Bearer ", "");
    let data = middleware::auth::verify_token(token);

    data
}

#[get("/user")]
async fn user(req: HttpRequest) -> impl Responder {
    let user = get_auth(req);
    if user.is_none() {
        return HttpResponse::Unauthorized().body("asdfasdfsad");
    }

    HttpResponse::Ok().json(user.unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::NormalizePath::trim())
            .service(
                web::scope("/api/private")
                    .service(private)
                    .service(user)
                    .wrap(middleware::mw::Authorization),
            )
            // .service(private)
            //
            .service(public)

        //
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
