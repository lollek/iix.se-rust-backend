use actix_web::{error, Error};
use postgres::{TlsMode, Connection as pg_Connection};

pub struct Config {
    query_string: String,
}

pub struct Connection {
    con: pg_Connection,
}

impl Config {
    pub fn connect(&self) -> Result<Connection, Error> {
        let pg_con = pg_Connection::connect(self.query_string.to_owned(), TlsMode::None)
            .map_err(error::ErrorInternalServerError)?;
        Ok(Connection {
            con: pg_con,
        })
    }
}

pub fn init(host: &str, port: &str, username: &str, password: &str,
            database: &str) -> Config {
    let query_string = format!("postgresql://{}:{}@{}:{}/{}",
                               username, password, host, port, database);
    Config {
        query_string: query_string,
    }
}

