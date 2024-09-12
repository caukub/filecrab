pub mod create;
pub mod list;

use crate::{AppResponse, Locales, UserPathRequest};
use axum::extract::{FromRef, FromRequest, FromRequestParts, Request};
use axum::http::header::ACCEPT_LANGUAGE;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::{async_trait, Json};
use fluent::FluentArgs;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::log::{debug, error};
use unic_langid::LanguageIdentifier;

pub struct ExtractLocalizer {
    pub language: LanguageIdentifier,
    state: Arc<LocalizerState>,
}

impl ExtractLocalizer {
    pub fn get_message(&self, key: &'static str, fluent_args: Option<&FluentArgs>) -> String {
        let bundle = self.state.locales.get(&self.language).unwrap();

        let msg = bundle
            .get_message(key)
            .unwrap_or_else(|| panic!("Message {key} not found"));

        let mut errors = vec![];

        let pattern = msg.value().expect("Message has no value");

        let value = bundle.format_pattern(pattern, fluent_args, &mut errors);

        value.to_string()
    }
}

pub struct LocalizerState {
    pub locales: Locales,
    pub fallback_language: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for ExtractLocalizer
where
    Arc<LocalizerState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<AppResponse>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let language = parts
            .headers
            .get(ACCEPT_LANGUAGE)
            .and_then(|langs| langs.to_str().ok());

        let language: LanguageIdentifier = language.unwrap_or("en").parse().unwrap();

        let state = Arc::from_ref(state);

        Ok(ExtractLocalizer { language, state })
    }
}

pub struct ExtractUserPath(pub Path);

#[derive(Debug)]
pub struct Path {
    pub path: PathBuf,
    pub directory: PathBuf,
    pub file: Option<PathBuf>,
}

#[async_trait]
impl<S> FromRequest<S> for ExtractUserPath
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<AppResponse>);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let base_path = std::env::var("FILECRAB_BASE_PATH").unwrap_or_else(|_| {
            error!("'FILECRAB_BASE_PATH' is not set, using default value");
            ".".to_string()
        });

        let base_path = PathBuf::from(base_path);

        match Json::<UserPathRequest>::from_request(req, state).await {
            Ok(req) => {
                let directory = PathBuf::from(req.0.directory);
                let file = req.0.file.map(PathBuf::from);

                let path = if file.is_some() {
                    base_path.join(&directory).join(file.clone().unwrap())
                } else {
                    base_path.join(&directory)
                };

                Ok(ExtractUserPath(Path {
                    path,
                    directory,
                    file,
                }))
            }
            Err(err) => {
                debug!("JSON request was rejected: {}", err.to_string());

                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(AppResponse {
                        success: false,
                        message: err.to_string(),
                    }),
                ));
            }
        }
    }
}
