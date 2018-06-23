extern crate actix_web;
extern crate env_logger;

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

use actix_web::{http, server, middleware, App, HttpResponse};
use std::env;

mod notes;
mod helpers;

fn main() {
    env::set_var("RUST_LOG", "info, actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    server::new(|| App::new()
        .middleware(middleware::Logger::default())

        // Notes
        .resource("/notes", |r| r.f(|req| {
            match *req.method() {
                http::Method::GET => notes::list(req),
                http::Method::POST => notes::post(req),
                _ => Ok(HttpResponse::NotFound().finish())
            }
        }))
        .resource("/notes/{id}", |r| r.f(|req| {
            let id: u32 = helpers::get_id(&req)?;
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
