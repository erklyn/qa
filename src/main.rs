use axum::{routing::get, Router};
use qa::db;

#[tokio::main]
async fn main() {
    let _db_conn = match db::init_db(None).await {
        Ok(conn) => conn,
        Err(err) => panic!("{}", err),
    };

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
