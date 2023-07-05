use std::process::Output;

use axum::body::Full;
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{body, BoxError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0} should be present after cargo build but is not")]
    BuildFileNotFound(&'static str),
    #[error("request must have a body but none was found")]
    NoBody,
    #[error("build failed with error {}\n{}", .0.status, String::from_utf8_lossy(&.0.stderr))]
    BuildFailed(Output),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Error::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::BuildFileNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::NoBody => StatusCode::BAD_REQUEST,
            Error::BuildFailed(_) => StatusCode::BAD_REQUEST,
            Error::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Response::builder()
            .status(status)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static("utf-8"),
            )
            .body(body::boxed(Full::from(self.to_string())))
            .unwrap()
    }
}

pub async fn timeout_or_500(err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", err),
        )
    }
}