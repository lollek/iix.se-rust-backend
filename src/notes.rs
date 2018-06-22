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


pub fn list(_req: HttpRequest) -> Result<Json<Vec<Note>>> {
    Ok(Json(vec!(Note { id: 1, title: "Hello world!".to_string(), date: chrono::Utc::now() },
    Note { id: 1, title: "Hello world!".to_string(), date: chrono::Utc::now() })))

}

pub fn get(info: Path<u32>) -> Result<Note> {
    let id: u32 = info.into_inner();
    Ok(Note { id: id, title: "Hello world!".to_string(), date: chrono::Utc::now() })
}

pub fn put(info: Path<u32>) -> Result<Note> {
    let id: u32 = info.into_inner();
    Ok(Note { id: id, title: "Hello world!".to_string(), date: chrono::Utc::now() })
}

pub fn post(_req: HttpRequest) -> Result<Note> {
    Ok(Note { id: 0, title: "Hello world!".to_string(), date: chrono::Utc::now() })
}

pub fn delete(info: Path<u32>) -> HttpResponse {
    let id: u32 = info.into_inner();
    HttpResponse::NoContent().body("")
}
