use actix_web::{error, Error};
use postgres::{TlsMode, Connection};

use std::borrow::ToOwned;

pub struct Config {
    query_string: String,
}

impl ToOwned for Config {
    type Owned = Config;

    fn to_owned(&self) -> Config {
        Config { query_string: self.query_string.to_owned() }
    }
}

impl Config {
    pub fn connect(&self) -> Result<Connection, Error> {
        Ok(Connection::connect(self.query_string.to_owned(), TlsMode::None)
            .map_err(error::ErrorInternalServerError)?)
    }
}

pub fn init(host: &str, port: &str, username: &str, password: &str,
            database: &str) -> Config {
    let query_string = format!("postgresql://{}:{}@{}:{}/{}",
                               username, password, host, port, database);
    Config { query_string: query_string }
}
