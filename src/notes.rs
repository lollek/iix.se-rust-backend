extern crate serde;
extern crate serde_json;
extern crate chrono;

use actix_web::{Json, HttpRequest, HttpResponse, Error, Responder, Path, Result};

#[derive(Serialize, Deserialize)]
pub struct Note {
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

pub fn list(_req: HttpRequest) -> Result<HttpResponse, Error> {
    let data = vec!(
        Note { id: 1, title: "Hello world!".to_string(), date: chrono::Utc::now() },
        Note { id: 1, title: "Hello world!".to_string(), date: chrono::Utc::now() }
        );
    let body = serde_json::to_string(&data)?;
    Ok(HttpResponse::Ok()
       .content_type("application/json")
       .body(body))
}

pub fn post(_req: HttpRequest) -> Result<HttpResponse, Error> {
    let data = Note { id: 0, title: "Hello world!".to_string(), date: chrono::Utc::now() };
    let body = serde_json::to_string(&data)?;
    Ok(HttpResponse::Ok()
       .content_type("application/json")
       .body(body))
}


pub fn get(req: HttpRequest) -> Result<HttpResponse, Error> {
    warn!("uri-path: {}", req.uri().path().to_string());
    warn!("path: {}", req.uri().path().to_string());
    warn!("info0: {}", req.match_info().get("id").unwrap_or(""));
    //let id: u32 = info.into_inner();
    let data = Note { id: 1, title: "Hello world!".to_string(), date: chrono::Utc::now() };
    let body = serde_json::to_string(&data)?;
    Ok(HttpResponse::Ok()
       .content_type("application/json")
       .body(body))
}

pub fn put(info: Path<u32>) -> Result<HttpResponse, Error> {
    let id: u32 = info.into_inner();
    let data = Note { id: id, title: "Hello world!".to_string(), date: chrono::Utc::now() };
    let body = serde_json::to_string(&data)?;
    Ok(HttpResponse::Ok()
       .content_type("application/json")
       .body(body))
}

pub fn delete(info: Path<u32>) -> Result<HttpResponse, Error> {
    let id: u32 = info.into_inner();
    Ok(HttpResponse::NoContent().finish())
}
