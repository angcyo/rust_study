use axum::http::StatusCode;
use axum::routing::{any, get, post};
use axum::{Json, Router};
use rc_basis::now_date_time;
use std::collections::HashMap;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/23
///
/// 启动一个服务
#[allow(dead_code)]
pub async fn start_serve() {
    // initialize tracing
    rc_log::tracing_subscriber::fmt::try_init().ok();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", any(root))
        .route("/get", get(root))
        .route("/post", post(root))
        // `POST /json` goes to `create_user`
        .route("/json", post(post_json));

    // run our app with hyper, listening globally on port 9292
    let addr = "0.0.0.0:9292";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    rc_log::log::info!("listening on {addr}");
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
    Json(payload): Json<HashMap<String, serde_json::Value>>,
) -> (StatusCode, Json<HashMap<String, serde_json::Value>>) {
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    let mut result: HashMap<String, serde_json::Value> = HashMap::new();
    result.extend(payload);
    result.insert(
        "reply".to_string(),
        serde_json::Value::String(now_date_time()),
    );
    (StatusCode::CREATED, Json(result))
}
