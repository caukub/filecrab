#![deny(clippy::disallowed_methods)]

use axum::handler::Handler;
use axum::routing::MethodRouter;
use axum::Router;
use filecrab::authorization::{auth_middleware, Permission};
use filecrab::configuration::{get_configuration, Settings};
use filecrab::localization::get_locales;
use filecrab::routes::create::{create_directory, create_file};
use filecrab::routes::LocalizerState;
use filecrab::tracing::{create_trace_layer, init_tracing};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tracing::log::debug;

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration");

    init_tracing(&configuration);

    let app = App::new(configuration.clone());

    let locales = get_locales();

    let state = LocalizerState {
        locales,
        fallback_language: configuration.localization.default,
    };

    let router = Router::new()
        .route(
            "/create/directory",
            post(create_directory, Permission::Create),
        )
        .route("/create/file", post(create_file, Permission::Create))
        .with_state(Arc::new(state));

    let router = create_trace_layer(router);

    debug!("Listening on {}", app.address());

    let listener = tokio::net::TcpListener::bind(app.address()).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

struct App {
    host: String,
    port: u16,
}

impl App {
    fn new(configuration: Settings) -> Self {
        let host = configuration.application.host;
        let port = configuration.application.port;

        Self { host, port }
    }

    fn address(&self) -> SocketAddr {
        let address = format!("{}:{}", self.host, self.port);

        SocketAddr::from_str(&address).unwrap_or_else(|err| {
            tracing::log::error!("Failed to create SocketAddr from '{address}': {err}");
            std::process::exit(1);
        })
    }
}

#[allow(clippy::disallowed_methods)]
pub fn post<H, T, S>(handler: H, _permission: Permission) -> MethodRouter<S, Infallible>
where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    let permission_check = axum::middleware::from_fn(auth_middleware);

    axum::routing::post(handler).route_layer(permission_check)
}

#[allow(clippy::disallowed_methods)]
pub fn get<H, T, S>(handler: H, _permission: Permission) -> MethodRouter<S, Infallible>
where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    let permission_check = axum::middleware::from_fn(auth_middleware);

    axum::routing::get(handler).route_layer(permission_check)
}
