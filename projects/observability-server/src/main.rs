use axum::{Router, routing::get};

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
    let app = Router::new().route("/status", get(status));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
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
