use axum::{
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};
use qa::{db, lectures, questions};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let _ = match db::Db::connect(None).await {
        Err(err) => panic!("Cannot create db -> {}", err),
        _ => {}
    };

    // build our application with a single route
    let app = Router::new()
        .route("/lectures", get(|| lectures::get_lectures()))
        .route(
            "/lecture/:lecture",
            post(|path| lectures::create_lecture(path)),
        )
        .route(
            "/lecture/:lecture",
            get(|path| lectures::get_single_lecture(path)),
        )
        .route(
            "/lecture/disable/:lecture",
            post(|path| lectures::disable_lecture(path)),
        )
        .route(
            "/:lecture",
            get(|lecture| questions::get_questions(lecture)),
        )
        .route(
            "/:lecture",
            post(|(path, payload)| questions::create_question(path, payload)),
        )
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
