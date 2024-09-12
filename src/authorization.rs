use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use std::convert::Infallible;

pub async fn auth_middleware(request: Request, next: Next) -> Result<Response, Infallible> {
    let has_permission = true;

    if has_permission {
        Ok(next.run(request).await)
    } else {
        Ok((StatusCode::FORBIDDEN, "Forbidden").into_response())
    }
}

#[derive(Debug, PartialEq)]
pub enum Permission {
    List,
    Read,
    Write,
    Create,
    SearchInFiles,
    URLDownload,
    Extract,
    Download,
    Rename,
    Copy,
    Move,
    Delete,
    ReadTrack,
    WriteTrack,
    None,
}
