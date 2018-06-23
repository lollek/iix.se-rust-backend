extern crate actix_web;
extern crate env_logger;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

use actix_web::{http, server, middleware, App, HttpRequest, HttpResponse};
use std::env;

mod notes;

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
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
            match *req.method() {
                http::Method::GET => notes::get(req),
//              http::Method::PUT => notes::put(req),
//              http::Method::DELETE => notes::delete(req),
                _ => Ok(HttpResponse::NotFound().finish())
            }
        })))
        .bind("0.0.0.0:8080")
        .unwrap()
        .run();
}
