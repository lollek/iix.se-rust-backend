use actix_web::{error, HttpRequest, HttpResponse, Error, Result};

use postgres::{Connection, rows::Row, rows::Rows};
use helpers::json;
use chrono::NaiveDate;

#[derive(Serialize, Deserialize)]
pub struct Note {
    id: i32,
    title: String,
    text: Option<String>,
    date: NaiveDate,
}

impl Note {
    fn marshall_shallow(row: Row) -> Note {
        Note {
            id: row.get("id"),
            title: row.get("title"),
            date: row.get("date"),
            text: None,
        }
    }

    fn marshall_deep(row: Row) -> Note {
        Note {
            id: row.get("id"),
            title: row.get("title"),
            date: row.get("date"),
            text: Some(row.get("text")),
        }
    }
}

pub fn list(conn: Connection) -> Result<HttpResponse, Error> {
    let data: Vec<Note> =
        conn.query("SELECT id, title, date FROM notes", &[])
        .map_err(error::ErrorInternalServerError)?
        .iter()
        .map(|row| Note::marshall_shallow(row))
        .collect();

    json(&data)
}

pub fn post(_req: HttpRequest) -> Result<HttpResponse, Error> {
    let data = Note {
        id: 0,
        title: "Hello world!".to_string(),
        date: NaiveDate::from_num_days_from_ce(735671),
        text: None,
    };
    json(&data)
}


pub fn get(id: i32, conn: Connection) -> Result<HttpResponse, Error> {
    let result: Rows =
        conn.query("SELECT id, title, date, text FROM notes WHERE id=$1", &[&id])
        .map_err(error::ErrorInternalServerError)?;

    match result.iter().next() {
        Some(row) => json(&Note::marshall_deep(row)),
        None => Ok(HttpResponse::NotFound().finish())
    }
}

pub fn put(_req: HttpRequest, id: u32) -> Result<HttpResponse, Error> {
    let data = Note {
        id: 0,
        title: "Hello world!".to_string(),
        date: NaiveDate::from_num_days_from_ce(735671),
        text: None,
    };
    json(&data)
}

pub fn delete(_req: HttpRequest, _id: u32) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NoContent().finish())
}
