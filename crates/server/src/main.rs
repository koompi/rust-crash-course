#![allow(non_snake_case)]

pub mod models;
pub mod routes;
pub mod utils;

use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use mongodb::Client;
use routes::user::{get_user, register};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = 0;
    let client = Client::with_uri_str("mongodb://127.0.0.1:27017")
        .await
        .unwrap();
    let db = client.database("jvp");

    HttpServer::new(move || {
        // App instance
        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(counter))
            .service(index)
            .service(register)
            .service(get_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignUp {
    pub email: String,
    pub password: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]

pub struct P(String, u32);

#[get("/stars/{email}/{password}")]
async fn params(para: web::Path<P>) -> impl Responder {
    println!("{:?}", para);
    HttpResponse::Ok()
}

#[get("/state")]
async fn state(counter: web::Data<i32>) -> impl Responder {
    counter.to_string()
}
#[get("/query")]
async fn query(q: web::Query<SignUp>) -> impl Responder {
    let data = q.into_inner();

    HttpResponse::Ok().json(data)
}

#[post("/body")]
async fn body(body: web::Json<SignUp>) -> impl Responder {
    let data = body.into_inner();

    HttpResponse::Ok().json(data)
}

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    let header = req.headers().get("authorization");

    // if header.is_none() {
    //     return HttpResponse::Unauthorized().json(json!({
    //         "status": 401,
    //         "message": "Login required"
    //     }));
    // }

    match header {
        Some(h) => {
            // let token = h.to_owned();
            let token = h.to_str().unwrap();
            // println!("{}", token);
            if !token.contains("Bearer ") {
                return HttpResponse::Unauthorized().json(json!({
                    "status": 401,
                    "message": "Invalid token"
                }));
            }

            let jwt = token.replace("Bearer ", "");

            HttpResponse::Ok().json(json!({
                "token": jwt
            }))
        }
        None => HttpResponse::Unauthorized().json(json!({
            "status": 401,
            "message": "Login required"
        })),
    }
}
