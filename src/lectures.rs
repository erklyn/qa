use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use libsql::params;
use serde::Serialize;

use crate::db::DB_CONNECTION;

#[derive(Serialize)]
pub struct Lecture {
    name: String,
    pub active: bool,
}

pub async fn get_lecture(lec_name: String) -> Result<Lecture, String> {
    let conn = DB_CONNECTION.get().unwrap().lock().await;

    let mut stmt = conn
        .prepare("SELECT * FROM lectures WHERE name = ?1")
        .await
        .unwrap();

    let mut rows = stmt
        .query([lec_name])
        .await
        .map_err(|err| err.to_string())?;

    let row = rows
        .next()
        .await
        .map_err(|err| err.to_string())?
        .expect("There to be a value");

    let lecture = Lecture {
        name: row.get(0).map_err(|err| err.to_string())?,
        active: row.get(1).map_err(|err| err.to_string())?,
    };

    Ok(lecture)
}

pub async fn disable_lecture(Path(lec_name): Path<String>) -> Result<impl IntoResponse, String> {
    let conn = DB_CONNECTION.get().unwrap().lock().await;

    let mut stmt = conn
        .prepare("UPDATE lectures SET active = false WHERE name = ?1")
        .await
        .unwrap();

    let _ = match stmt.query([lec_name]).await {
        Err(err) => return Err(err.to_string()),
        _ => {}
    };

    Ok(StatusCode::OK)
}

pub async fn create_lecture(Path(lec_name): Path<String>) -> Result<impl IntoResponse, String> {
    let conn = DB_CONNECTION.get().unwrap().lock().await;

    let _ = conn
        .query(
            "INSERT into lectures (name, active) values (?1, true)",
            params![lec_name.clone(),],
        )
        .await;

    Ok((
        StatusCode::CREATED,
        Json(Lecture {
            name: lec_name,
            active: true,
        }),
    ))
}
