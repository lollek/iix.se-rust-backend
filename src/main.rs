extern crate actix_web;
extern crate env_logger;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

use actix_web::{http, error, server, middleware, App, HttpRequest, HttpResponse};
use std::env;

mod notes;

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

fn get_id(req: &HttpRequest) -> Result<u32, error::UriSegmentError> {
    let text = req.match_info().get("id").unwrap_or("");
    return match text.parse::<u32>() {
        Ok(id) => Ok(id),
        Err(_) => Err(error::UriSegmentError::BadEnd(text.chars().next().unwrap()))
    }
}

fn main() {
    env::set_var("RUST_LOG", "info, actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    server::new(|| App::new()
        .middleware(middleware::Logger::default())
        .resource("/", |r| r.f(index))

        // Notes
        .resource("/notes", |r| r.f(|req| {
            match *req.method() {
                http::Method::GET => notes::list(req),
                http::Method::POST => notes::post(req),
                _ => Ok(HttpResponse::NotFound().finish())
            }
        }))
        .resource("/notes/{id}", |r| r.f(|req| {
            let id: u32 = get_id(&req)?;
            match *req.method() {
                http::Method::GET => notes::get(req, id),
                http::Method::PUT => notes::put(req, id),
                http::Method::DELETE => notes::delete(req, id),
                _ => Ok(HttpResponse::NotFound().finish())
            }
        })))
        .bind("0.0.0.0:8080")
        .unwrap()
        .run();
}
