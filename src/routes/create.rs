use crate::routes::ExtractLocalizer;
use crate::routes::ExtractUserPath;
use crate::{AppError, AppResponse};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use fluent::{FluentArgs, FluentValue};
use std::io::ErrorKind;
use tracing::log::debug;

#[tracing::instrument(name = "Creating a directory", skip(localizer))]
pub async fn create_directory(
    localizer: ExtractLocalizer,
    ExtractUserPath(path): ExtractUserPath,
) -> Result<Response, AppError> {
    let directory = path.directory.display();

    debug!("Creating a directory: '{}'", directory);

    let mut args = FluentArgs::new();
    args.set("directory", FluentValue::from(directory.to_string()));

    if let Err(err) = tokio::fs::create_dir(path.directory).await {
        debug!("Error creating directory: {:?}", err);

        let error_message = match err.kind() {
            ErrorKind::AlreadyExists => {
                localizer.get_message("create-directory-already-exists", Some(&args))
            }
            _ => localizer.get_message("create-directory-error", Some(&args)),
        };

        let response = AppResponse {
            success: false,
            message: error_message,
        };

        Ok((StatusCode::BAD_REQUEST, Json(response)).into_response())
    } else {
        let response = AppResponse {
            success: true,
            message: localizer.get_message("create-directory-success", Some(&args)),
        };

        Ok(Json(response).into_response())
    }
}

#[tracing::instrument(name = "Creating a file", skip(localizer))]
pub async fn create_file(
    localizer: ExtractLocalizer,
    ExtractUserPath(path): ExtractUserPath,
) -> Result<Response, AppError> {
    let file = path.file.unwrap_or_default();
    let file = file.display();

    debug!("Creating a file: '{}'", file);

    let mut args = FluentArgs::new();
    args.set("file", FluentValue::from(file.to_string()));

    return if let Err(err) = tokio::fs::File::create_new(path.path).await {
        debug!("Error creating file: {:?}", err);

        let error_message = match err.kind() {
            ErrorKind::AlreadyExists => {
                localizer.get_message("create-file-already-exists", Some(&args))
            }
            _ => localizer.get_message("create-file-error", Some(&args)),
        };

        let response = AppResponse {
            success: false,
            message: error_message,
        };

        Ok((StatusCode::BAD_REQUEST, Json(response)).into_response())
    } else {
        let response = AppResponse {
            success: true,
            message: localizer.get_message("create-file-success", Some(&args)),
        };

        Ok(Json(response).into_response())
    };
}
