extern crate actix_web;
extern crate env_logger;

#[macro_use] extern crate serde_derive;

use actix_web::{http, server, middleware, App, HttpRequest};
use std::env;

mod notes;

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    server::new(|| App::new()
        .middleware(middleware::Logger::default())
        .resource("/", |r| r.f(index))
        .resource("/notes", |r| r.method(http::Method::GET).f(notes::list))
        .resource("/notes", |r| r.method(http::Method::POST).with(notes::post))
        .resource("/notes/{id}", |r| r.method(http::Method::GET).with(notes::get))
        .resource("/notes/{id}", |r| r.method(http::Method::PUT).with(notes::put))
        .resource("/notes/{id}", |r| r.method(http::Method::DELETE).with(notes::delete)))
        .bind("0.0.0.0:8080")
        .unwrap()
        .run();
}
