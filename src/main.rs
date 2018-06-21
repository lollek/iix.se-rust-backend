extern crate actix_web;
extern crate env_logger;

use actix_web::{server, middleware, App, HttpRequest};
use std::env;

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    server::new(|| App::new()
        .middleware(middleware::Logger::default())
        .resource("/", |r| r.f(index)))
        .bind("0.0.0.0:8080")
        .unwrap()
        .run();
}
