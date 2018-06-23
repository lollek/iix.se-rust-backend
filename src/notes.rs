extern crate serde;
extern crate serde_json;
extern crate chrono;

use actix_web::{HttpRequest, HttpResponse, Error, Result};

use database::Connection;
use helpers::json;

#[derive(Serialize, Deserialize)]
pub struct Note {
    id: u32,
    title: String,
    date: chrono::DateTime<chrono::Utc>
}

pub fn list(_req: HttpRequest, db: Connection) -> Result<HttpResponse, Error> {
    let data = vec!(
        Note { id: 1, title: "Hello world!".to_string(), date: chrono::Utc::now() },
        Note { id: 1, title: "Hello world!".to_string(), date: chrono::Utc::now() }
        );
    json(&data)
}

pub fn post(_req: HttpRequest) -> Result<HttpResponse, Error> {
    let data = Note { id: 0, title: "Hello world!".to_string(), date: chrono::Utc::now() };
    json(&data)
}


pub fn get(_req: HttpRequest, id: u32) -> Result<HttpResponse, Error> {
    let data = Note { id: id, title: "Hello world!".to_string(), date: chrono::Utc::now() };
    json(&data)
}

pub fn put(_req: HttpRequest, id: u32) -> Result<HttpResponse, Error> {
    let data = Note { id: id, title: "Hello world!".to_string(), date: chrono::Utc::now() };
    json(&data)
}

pub fn delete(_req: HttpRequest, _id: u32) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NoContent().finish())
}
