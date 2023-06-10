use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::{
    serialize_hex_string_as_object_id, serialize_object_id_as_hex_string,
};
use mongodb::bson::{doc, Document};
use serde::Serializer;
use std::str::FromStr;

pub fn serialize_id<S>(id: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match id {
        Some(id) => serialize_hex_string_as_object_id(&id.to_string(), serializer),
        _ => unreachable!(),
    }
}

pub fn id_filter(id: &str) -> Document {
    doc! { "_id": ObjectId::from_str(id).unwrap() }
}
