use actix_web::{http, error, AsyncResponder, HttpMessage, HttpRequest, HttpResponse, FutureResponse, Error, Result};
use chrono::NaiveDate;
use futures::{future, Future};
use helpers::{futurize, json, get_id};
use postgres::{rows::Row, rows::Rows};
use state::AppState;

#[derive(Serialize, Deserialize, Debug)]
pub struct NoteRef {
    id: i32,
    title: String,
    date: NaiveDate,
}

impl NoteRef {
    fn marshall(row: Row) -> NoteRef {
        NoteRef {
            id: row.get("id"),
            title: row.get("title"),
            date: row.get("date"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    id: i32,
    title: String,
    text: String,
    date: NaiveDate,
}

impl Note {
    fn marshall(row: Row) -> Note {
        Note {
            id: row.get("id"),
            title: row.get("title"),
            date: row.get("date"),
            text: row.get("text"),
        }
    }
}

pub fn notes(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    match *req.method() {
        http::Method::GET => list(req),
        http::Method::POST => post(req),
        _ => Box::new(future::ok(HttpResponse::NotFound().into()))
    }
}

pub fn note(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    match *req.method() {
        http::Method::GET => get(req),
        //http::Method::PUT => post(req),
        //http::Method::DELETE => delete(req),
        _ => Box::new(future::ok(HttpResponse::NotFound().into()))
    }
}

fn list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    futurize(list_inner(req))
}

fn list_inner(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let data: Vec<NoteRef> =
        req.state().db_config.connect()?
        .query("SELECT id, title, date FROM notes", &[])
        .map_err(error::ErrorInternalServerError)?
        .iter()
        .map(NoteRef::marshall)
        .collect();
    json(&data)
}

fn post(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let db_config = req.state().db_config.to_owned();
    req.json()
        .from_err()
        .and_then(move |note: Note| {
            db_config.connect()?.
                execute("INSERT INTO notes
                (title, text, date) VALUES ($1, $2, $3)",
                &[&note.title, &note.text, &note.date])
                .map_err(error::ErrorInternalServerError)?;
            Ok(HttpResponse::Ok().into())
        }).responder()
}

fn get(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    futurize(get_inner(req))
}

fn get_inner(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let id = get_id(&req)?;
    let result: Rows =
        req.state().db_config.connect()?
        .query("SELECT id, title, date, text FROM notes WHERE id=$1", &[&id])
        .map_err(error::ErrorInternalServerError)?;

    match result.iter().next() {
        Some(row) => json(&Note::marshall(row)),
        None => Ok(HttpResponse::NotFound().finish())
    }
}

pub fn put(_req: HttpRequest, id: u32) -> Result<HttpResponse, Error> {
    let data = Note {
        id: 0,
        title: "Hello world!".to_owned(),
        date: NaiveDate::from_num_days_from_ce(735671),
        text: "Hello".to_owned(),
    };
    json(&data)
}

pub fn delete(_req: HttpRequest, _id: u32) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NoContent().finish())
}
