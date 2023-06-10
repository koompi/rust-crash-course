use std::str::FromStr;

use actix_web::{get, post, web, HttpResponse, Responder};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    models::user::User,
    utils::db::{self, user_coll},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignUp {
    pub email: String,
    pub password: String,
}

#[post("/users/register")]
pub async fn register(body: web::Json<SignUp>, db: web::Data<Database>) -> impl Responder {
    let USER = user_coll(db);
    let data = body.into_inner();

    let new_user = User::new(data.email, data.password);
    USER.insert_one(&new_user, None).await.unwrap();

    HttpResponse::Ok().json(new_user)
}

#[get("/users/{id}")]
pub async fn get_user(id: web::Path<String>, db: web::Data<Database>) -> impl Responder {
    let USER = user_coll(db);
    let user = USER
        .find_one(
            doc! {
                "_id": ObjectId::from_str(&id).unwrap()
            },
            None,
        )
        .await
        .unwrap();

    if user.is_none() {
        return HttpResponse::NotFound().json(json! ({
            "status": 404,
            "message": "User not found"
        }));
    }

    HttpResponse::Ok().json(user.unwrap())
}
