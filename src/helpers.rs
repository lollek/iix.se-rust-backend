use actix_web::{error, Error, FutureResponse, HttpResponse, HttpRequest};
use futures::future;
use serde::Serialize;
use serde_json::to_string;
use state::AppState;

pub fn json(data: &impl Serialize) -> Result<HttpResponse, Error> {
    let body = to_string(data)?;
    Ok(HttpResponse::Ok()
       .content_type("application/json")
       .body(body))
}

pub fn get_id(req: &HttpRequest<AppState>) -> Result<i32, Error> {
    req.match_info()
        .get("id")
        .unwrap_or("")
        .parse::<i32>()
        .map_err(error::ErrorBadRequest)
}

pub fn futurize(data: Result<HttpResponse, Error>) -> FutureResponse<HttpResponse> {
    match data {
        Ok(data) => Box::new(future::ok(data)),
        Err(err) => Box::new(future::err(err))
    }
}
