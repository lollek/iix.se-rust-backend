use actix_web::{http, error, AsyncResponder, HttpMessage, HttpRequest, HttpResponse, FutureResponse, Error, Result};
use chrono::NaiveDate;
use database::lastval;
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
        http::Method::PUT => put(req),
        http::Method::DELETE => delete(req),
        _ => Box::new(future::ok(HttpResponse::NotFound().into()))
    }
}

fn list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    futurize(inner_list(req))
}

fn inner_list(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
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
        .and_then(move |mut note: Note| {
            let conn = db_config.connect()?;

            conn.execute("INSERT INTO notes
                (title, text, date) VALUES ($1, $2, $3)",
                &[&note.title, &note.text, &note.date])
                .map_err(error::ErrorInternalServerError)?;

            note.id = lastval(conn)?.unwrap_or(0) as i32;
            Ok(json(&note).into())

        }).responder()
}

fn get(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    futurize(inner_get(req))
}

fn inner_get(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let result: Rows =
        req.state().db_config.connect()?
        .query("SELECT id, title, date, text FROM notes WHERE id=$1",
               &[&get_id(&req)?])
        .map_err(error::ErrorInternalServerError)?;

    match result.iter().next() {
        Some(row) => json(&Note::marshall(row)),
        None => Ok(HttpResponse::NotFound().finish())
    }
}

fn put(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let db_config = req.state().db_config.to_owned();
    let id = get_id(&req);

    req.json()
        .from_err()
        .and_then(move |mut note: Note| {
            note.id = id?;

            let affected_rows = db_config.connect()?
                .execute("UPDATE notes
                SET title = $1, text = $2, date = $3
                WHERE id = $4",
                &[&note.title, &note.text, &note.date, &note.id])
                .map_err(error::ErrorInternalServerError)?;

            match affected_rows {
                1 => Ok(json(&note).into()),
                0 => Ok(HttpResponse::NotFound().finish().into()),
                _ => Ok(HttpResponse::InternalServerError().finish().into())
            }

        }).responder()
}

fn delete(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    futurize(inner_delete(req))
}

fn inner_delete(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let affected_rows = req.state().db_config.connect()?
        .execute("DELETE FROM notes WHERE id=$1", &[&get_id(&req)?])
        .map_err(error::ErrorInternalServerError)?;

    match affected_rows {
        1 => Ok(HttpResponse::NoContent().finish()),
        0 => Ok(HttpResponse::NotFound().finish()),
        _ => Ok(HttpResponse::InternalServerError().finish())
    }
}
