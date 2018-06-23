extern crate serde;
extern crate serde_json;

use actix_web::{error, HttpRequest, HttpResponse, Error};

pub fn json(data: &impl serde::ser::Serialize) -> Result<HttpResponse, Error> {
    let body = serde_json::to_string(data)?;
    Ok(HttpResponse::Ok()
       .content_type("application/json")
       .body(body))
}

pub fn get_id(req: &HttpRequest) -> Result<u32, error::UriSegmentError> {
    let text = req.match_info().get("id").unwrap_or("");
    return match text.parse::<u32>() {
        Ok(id) => Ok(id),
        Err(_) => Err(error::UriSegmentError::BadEnd(text.chars().next().unwrap()))
    }
}

