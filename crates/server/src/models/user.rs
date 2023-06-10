use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

// use bson::serde_helpers::
use bson::serde_helpers::{
    deserialize_hex_string_from_object_id, serialize_hex_string_as_object_id,
    serialize_object_id_as_hex_string,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    // #[serde(with = "hex_string_as_object_id")]
    // #[serde(rename(deserialize = "id"))]
    // #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    // #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
    #[serde(skip_deserializing)]
    pub _id: ObjectId,
    #[serde(default = "_id::to_string")]
    pub id: String,
}

impl User {
    pub fn new(email: String, password: String) -> Self {
        Self {
            _id: ObjectId::new(),
            // created_at: DateTime::now().try_to_rfc3339_string().unwrap(),
        }
    }
}
