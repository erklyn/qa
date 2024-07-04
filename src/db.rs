use libsql::{Builder, Connection};

pub async fn init_db(turso_url: Option<String>) -> Result<Connection, String> {
    let db = match turso_url {
        Some(url) => Builder::new_local(url)
            .build()
            .await
            .map_err(|err| err.to_string())?,
        None => Builder::new_local("local.db")
            .build()
            .await
            .map_err(|err| err.to_string())?,
    };

    let conn = db.connect().map_err(|err| err.to_string())?;

    if let Err(err) = conn
        .execute("CREATE TABLE questions (name TEXT, question TEXT)", ())
        .await
    {
        return Err(err.to_string());
    }

    Ok(conn)
}
