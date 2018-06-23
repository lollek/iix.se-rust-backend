use actix_web::{error, HttpRequest, HttpResponse, Error};
use serde::Serialize;
use serde_json::to_string;

pub fn json(data: &impl Serialize) -> Result<HttpResponse, Error> {
    let body = to_string(data)?;
    Ok(HttpResponse::Ok()
       .content_type("application/json")
       .body(body))
}

pub fn get_id(req: &HttpRequest) -> Result<u32, Error> {
    req.match_info()
        .get("id").unwrap_or("")
        .parse::<u32>().map_err(error::ErrorBadRequest)
}
