use crate::configuration::Settings;
use axum::body::Bytes;
use axum::extract::MatchedPath;
use axum::http::{HeaderMap, Request};
use axum::response::Response;
use axum::Router;
use std::fs::OpenOptions;
use std::time::Duration;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::TraceLayer;
use tracing::{info_span, Span};
use tracing_subscriber::fmt::layer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing(configuration: &Settings) {
    let current_directory = std::env::current_dir().expect("Failed to get current directory.");
    let log_directory = current_directory.join("logs");

    if !std::path::Path::exists(&log_directory) {
        std::fs::create_dir(&log_directory).expect("Failed to create '/logs/' directory");
    }

    let local_time = chrono::Local::now()
        .format(&configuration.application.date_format)
        .to_string();

    let log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_directory.join(&local_time))
        .unwrap();

    let pretty_layer = layer().pretty();
    let json_layer = layer().json().with_writer(log_file);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(pretty_layer)
        .with(json_layer)
        .init();
}

/*
Due to all the generics `TraceLayer` uses, Router should be returned
instead of `TraceLayer` directly
*/
pub fn create_trace_layer(router: Router) -> Router {
    router.layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    uuid = ?uuid::Uuid::new_v4(),
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            })
            .on_request(|_request: &Request<_>, _span: &Span| {})
            .on_response(|_response: &Response, _latency: Duration, _span: &Span| {})
            .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {})
            .on_eos(|_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {})
            .on_failure(|_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {}),
    )
}
