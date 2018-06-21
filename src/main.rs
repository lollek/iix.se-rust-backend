#[macro_use] extern crate serde_derive;
extern crate actix_web;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
extern crate chrono;

use actix_web::{http, server, middleware, Json,
App, HttpRequest, HttpResponse, Error, Responder, Path, Result};
use std::env;

#[derive(Serialize, Deserialize)]
struct Note {
    id: u32,
    title: String,
    date: chrono::DateTime<chrono::Utc>
}

impl Responder for Note {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

fn notes_list(_req: HttpRequest) -> Result<Json<Vec<Note>>> {
    Ok(Json(vec!(Note { id: 1, title: "Hello world!".to_string(), date: chrono::Utc::now() },
    Note { id: 1, title: "Hello world!".to_string(), date: chrono::Utc::now() })))

}

fn notes_get(info: Path<u32>) -> Result<Note> {
    let id: u32 = info.into_inner();
    Ok(Note { id: id, title: "Hello world!".to_string(), date: chrono::Utc::now() })
}

fn notes_put(info: Path<u32>) -> Result<Note> {
    let id: u32 = info.into_inner();
    Ok(Note { id: id, title: "Hello world!".to_string(), date: chrono::Utc::now() })
}

fn notes_post(info: Path<u32>) -> Result<Note> {
    let id: u32 = info.into_inner();
    Ok(Note { id: id, title: "Hello world!".to_string(), date: chrono::Utc::now() })
}

fn notes_delete(info: Path<u32>) -> HttpResponse {
    let id: u32 = info.into_inner();
    HttpResponse::NoContent().body("")
}


fn main() {
    env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    server::new(|| App::new()
        .middleware(middleware::Logger::default())
        .resource("/", |r| r.f(index))
        .resource("/notes", |r| r.method(http::Method::GET).f(notes_list))
        .resource("/notes/{id}", |r| r.method(http::Method::GET).with(notes_get))
        .resource("/notes/{id}", |r| r.method(http::Method::PUT).with(notes_put))
        .resource("/notes/{id}", |r| r.method(http::Method::POST).with(notes_post))
        .resource("/notes/{id}", |r| r.method(http::Method::DELETE).with(notes_delete)))
        .bind("0.0.0.0:8080")
        .unwrap()
        .run();
}
