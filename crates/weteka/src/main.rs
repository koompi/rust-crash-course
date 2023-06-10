pub mod models;

use std::fmt::Display;

use libweteka::add;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty, to_value};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Genders {
    Male,
    Female,
    Others(String),
}

impl Default for Genders {
    fn default() -> Self {
        Self::Male
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct User {
    pub name: String,
    pub gender: Genders,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Register {
    pub email: String,
    pub password: String,
}

impl Register {
    pub fn to_user(&self) -> User {
        User {
            email: self.email.to_owned(),
            password: self.password.to_owned(),
            ..User::default()
        }
    }

    pub fn sign_up(email: String, password: String) -> Register {
        Register { email, password }
    }
}

impl User {
    pub fn new(name: String, gender: Genders, email: String, password: String) -> Self {
        Self {
            name,
            gender,
            email,
            password,
        }
    }
    pub fn from_json(data: String) -> Self {
        serde_json::from_str(&data).unwrap()
    }

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }
}

fn main() {
    // let v = User::new("vuthy".to_string(), Genders::Male);
    // let json = to_string_pretty(&v).unwrap();
    // let u = User::from_json(json);
    // let v2 = from_str::<User>(&json);

    // println!("{:?}", u);
    // println!("{:?}", u.get_name());

    let req_body = r#"
    {
        "email": "lay@gmail.com",
        "password": "123"
    }
    "#;

    let data: Register = serde_json::from_str(req_body).unwrap();
    println!("{data:?}");

    let user = data.to_user();
    println!("{user:?}");
}
