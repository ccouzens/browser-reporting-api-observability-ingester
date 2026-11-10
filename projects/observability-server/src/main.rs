use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
mod application_reports_json;

#[derive(thiserror::Error, Debug)]
pub enum TopLevelError {
    #[error("bind failed")]
    Bind(#[source] std::io::Error),
    #[error("serve failed")]
    Serve(#[source] std::io::Error),
}

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let app = Router::new()
        .route("/status", get(status))
        .route("/reports", post(create_report));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4174")
        .await
        .map_err(TopLevelError::Bind)?;
    axum::serve(listener, app)
        .await
        .map_err(TopLevelError::Serve)?;
    Ok(())
}

async fn status() -> &'static str {
    "ok"
}

async fn create_report(
    Json(payload): Json<application_reports_json::Reports>,
) -> (StatusCode, String) {
    println!("{:?}", payload);
    (StatusCode::CREATED, "".to_string())
}
