use actix_web::{error, HttpRequest, HttpResponse, Error, Result};

use postgres::Connection;
use helpers::json;
use chrono::NaiveDate;

#[derive(Serialize, Deserialize)]
pub struct Note {
    id: i32,
    title: String,
    date: NaiveDate,
}

pub fn list(_req: HttpRequest, conn: Connection) -> Result<HttpResponse, Error> {
    let rows = conn.query("SELECT id, title, date FROM notes", &[])
        .map_err(error::ErrorInternalServerError)?;

    let data: Vec<Note> = rows.iter()
        .map(|row| Note {
            id: row.get("id"),
            title: row.get("title"),
            date: row.get("date"),
        }).collect();

    json(&data)
}

pub fn post(_req: HttpRequest) -> Result<HttpResponse, Error> {
    let data = Note { id: 0, title: "Hello world!".to_string(), date: NaiveDate::from_num_days_from_ce(735671) };
    json(&data)
}


pub fn get(_req: HttpRequest, id: u32) -> Result<HttpResponse, Error> {
    let data = Note { id: 0, title: "Hello world!".to_string(), date: NaiveDate::from_num_days_from_ce(735671) };
    json(&data)
}

pub fn put(_req: HttpRequest, id: u32) -> Result<HttpResponse, Error> {
    let data = Note { id: 0, title: "Hello world!".to_string(), date: NaiveDate::from_num_days_from_ce(735671) };
    json(&data)
}

pub fn delete(_req: HttpRequest, _id: u32) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NoContent().finish())
}
