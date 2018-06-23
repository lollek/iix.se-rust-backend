extern crate actix_web;
extern crate env_logger;
#[macro_use] extern crate log;
extern crate postgres;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use actix_web::{http, server, middleware, App, HttpResponse};
use std::env;
use std::cell::RefCell;

mod helpers;
mod notes;
mod database;


fn main() {
    env::set_var("RUST_LOG", "info, actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();
    thread_local!(
        static DB_CONFIG: RefCell<database::Config> = RefCell::new(
            database::init("localhost", "5432", "www-data", "www-data", "iix-notes")));

    server::new(|| App::new()
        .middleware(middleware::Logger::default())

        // Notes
        .resource("/notes", |r| r.f(|req| {
            DB_CONFIG.with(|dbc| {
                let db = dbc.borrow().connect()?;
                match *req.method() {
                    http::Method::GET => notes::list(req, db),
                    http::Method::POST => notes::post(req),
                    _ => Ok(HttpResponse::NotFound().finish())
                }
            })
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
