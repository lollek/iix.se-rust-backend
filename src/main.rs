extern crate chrono;
extern crate actix_web;
extern crate env_logger;
#[macro_use] extern crate log;
extern crate postgres;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate futures;

use actix_web::{server, middleware, App};
use std::env;

mod state;
mod helpers;
mod notes;
mod database;

fn main() {
    env::set_var("RUST_LOG", "info, actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    server::new(move || App::with_state(state::AppState{
        db_config: database::init("localhost", "5432", "www-data", "www-data", "iix-notes"),
    })
        .middleware(middleware::Logger::default())

        // Notes
        .resource("/notes", |r| r.h(notes::notes))
                //|req| {
        //    match *req.method() {
                //http::Method::GET => notes::list(req),
                //http::Method::POST => notes::post(req),
        //        _ => Ok(HttpResponse::NotFound().finish())
        //    }
        //}))
        //.resource("/notes/{id}", |r| r.with(|req| {
        //    //let id: i32 = helpers::get_id(&req)?;
        //    match *req.method() {
                //http::Method::GET => notes::get(id, db),
                //http::Method::PUT => notes::put(req, id),
                //http::Method::DELETE => notes::delete(req, id),
        //        _ => Ok(HttpResponse::NotFound().finish())
        //    }
        //})))
        )
        .bind("0.0.0.0:8080")
        .unwrap()
        .run();
}
