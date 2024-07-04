use crate::{db::DB_CONNECTION, lectures};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use libsql::params;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateQuestion {
    name: String,
    question: String,
}

#[derive(Serialize, Debug)]
pub struct Question {
    name: String,
    question: String,
    lecture: String,
}

pub async fn create_question(
    Path(path): Path<String>,
    Json(payload): Json<CreateQuestion>,
) -> Result<impl IntoResponse, String> {
    let lecture = lectures::get_lecture(path.clone()).await?;
    if !lecture.active {
        return Ok(StatusCode::LOCKED);
    };

    let conn = DB_CONNECTION.get().unwrap().lock().await;
    let question = Question {
        name: payload.name,
        question: payload.question,
        lecture: path,
    };

    let _ = conn
        .query(
            "INSERT into questions (name, question, lecture) values (?1, ?2, ?3)",
            params![
                question.name.clone(),
                question.question.clone(),
                question.lecture.clone()
            ],
        )
        .await
        .map_err(|err| err.to_string())?;

    return Ok(StatusCode::CREATED);
}

pub async fn get_questions(Path(lecture): Path<String>) -> Result<impl IntoResponse, String> {
    let conn = DB_CONNECTION.get().unwrap().lock().await;

    let mut results = conn
        .query("SELECT * FROM questions", ())
        .await
        .map_err(|err| err.to_string())?;

    let mut questions: Vec<Question> = Vec::new();

    while let Some(row) = results.next().await.map_err(|err| err.to_string())? {
        let question = Question {
            name: row.get(0).map_err(|err| err.to_string())?,
            question: row.get(1).map_err(|err| err.to_string())?,
            lecture: row.get(2).map_err(|err| err.to_string())?,
        };

        questions.push(question);
    }

    let questions: Vec<Question> = questions
        .into_iter()
        .filter(|question| question.lecture == lecture)
        .collect();

    Ok((StatusCode::OK, Json(questions)))
}
