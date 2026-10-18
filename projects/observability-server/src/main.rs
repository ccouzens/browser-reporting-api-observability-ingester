use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::Deserialize;

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

async fn create_report(Json(payload): Json<Vec<CreateReport>>) -> (StatusCode, String) {
    println!("{:?}", payload);
    (StatusCode::CREATED, "".to_string())
}

#[derive(Debug, Deserialize)]
struct CreateReport {
    /**
    The number of milliseconds between report’s timestamp and the current time.

    https://www.w3.org/TR/reporting-1/#serialize-reports
    */
    age: u32,
    /**
    A report type is a non-empty string that specifies the set of data that is contained in the body of a report.

    https://www.w3.org/TR/reporting-1/#report-type
    */
    r#type: String,
    /**
    Each report has a url, which is typically the address of the Document or Worker from which the report was generated.

    https://www.w3.org/TR/reporting-1/#report-url */
    url: String,
    /**
    Each report has a user agent, which is the value of the User-Agent header of the request from which the report was generated.

    Note: The user agent of a report represents the User-Agent sent by the browser for the page which generated the report. This is potentially distinct from the User-Agent sent in the HTTP headers when uploading the report to a collector — for instance, where the browser has chosen to use a non-default User-Agent string such as the "request desktop site" feature.

    https://www.w3.org/TR/reporting-1/#report-user-agent
    */
    user_agent: String,
    /**
    Each report has a body, which is either null or an object which can be serialized into a JSON text. The fields contained in a report’s body are determined by the report’s type.

    https://www.w3.org/TR/reporting-1/#report-body
    */
    body: serde_json::Value,
}
