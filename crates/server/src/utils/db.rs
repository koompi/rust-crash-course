use actix_web::web;
use mongodb::{Collection, Database};

use crate::models::user::User;

fn get_db<T>(db: web::Data<Database>, name: &str) -> Collection<T> {
    db.into_inner().as_ref().to_owned().collection::<T>(name)
}

pub fn user_coll(db: web::Data<Database>) -> Collection<User> {
    get_db(db, "users")
}

pub fn film_coll(db: web::Data<Database>) -> Collection<User> {
    get_db(db, "film")
}
