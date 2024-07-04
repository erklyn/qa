use libsql::{Builder, Connection};
use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell};

#[derive(Clone, Debug)]
pub struct Db {}

pub static DB_CONNECTION: OnceCell<Arc<Mutex<Connection>>> = OnceCell::const_new();

impl Db {
    pub async fn connect(url: Option<String>) -> Result<bool, String> {
        let db = match url {
            Some(url) => Builder::new_local(url.clone())
                .build()
                .await
                .map_err(|err| err.to_string())?,
            None => Builder::new_local("local.db")
                .build()
                .await
                .map_err(|err| err.to_string())?,
        };
        let conn = db.connect().unwrap();
        let _ = conn
            .execute(
                "CREATE TABLE IF NOT EXISTS questions(name varchar, question varchar, lecture varchar)",
                (),
            )
            .await;
        let _ = conn
            .execute(
                "CREATE TABLE IF NOT EXISTS lectures(name varchar, active boolean)",
                (),
            )
            .await;

        let connection = db.connect().map_err(|err| err.to_string())?;
        DB_CONNECTION
            .set(Arc::new(Mutex::new(connection)))
            .map_err(|err| err.to_string())?;

        Ok(true)
    }
}
