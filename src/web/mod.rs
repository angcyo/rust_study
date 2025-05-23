use crate::utils::now_date_time;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/23
///
/// 启动一个服务
#[allow(dead_code)]
pub async fn start_serve() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /json` goes to `create_user`
        .route("/json", post(post_json));

    // run our app with hyper, listening globally on port 9292
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9292").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
#[allow(dead_code)]
async fn root() -> String {
    format!("Hello, Axum! {}", now_date_time())
}
#[allow(dead_code)]
async fn post_json(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<String>,
) -> (StatusCode, Json<String>) {
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (
        StatusCode::CREATED,
        Json(format!("{}\n<-{}", payload, now_date_time())),
    )
}
