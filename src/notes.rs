use actix_web::{http, error, AsyncResponder, HttpMessage, HttpRequest, HttpResponse, FutureResponse, Error, Result};

use postgres::{Connection, rows::Row, rows::Rows};
use helpers::json;
use chrono::NaiveDate;
use futures::{future, Future};

use state::AppState;

#[derive(Serialize, Deserialize, Debug)]
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

pub fn notes(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    match *req.method() {
        //http::Method::GET => list(req),
        http::Method::POST => post(req),
        _ => Box::new(future::ok(HttpResponse::NotFound().into()))
    }
}

pub fn list(conn: Connection) -> Result<HttpResponse, Error> {
    let data: Vec<Note> =
        conn.query("SELECT id, title, date FROM notes", &[])
        .map_err(error::ErrorInternalServerError)?
        .iter()
        .map(Note::marshall_shallow)
        .collect();

    json(&data)
}

pub fn post(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    //let conn = req.state().db_config.connect()?;
    req.json()
        .from_err()
        .and_then(|val: Note| {
            println!("==== BODY ==== {:?}", val);
            Ok(HttpResponse::Ok().into())
        }).responder()
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
