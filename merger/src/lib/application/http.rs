use std::sync::Arc;

use anyhow::{Context, Result};
use axum::Extension;
use tokio::net;
use tracing::{info, info_span};

use crate::domain::ferri::ports::FerriService;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig {
    pub port: String,
}

impl HttpServerConfig {
    pub fn new(port: String) -> Self {
        Self { port }
    }
}

pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

#[derive(Clone)]
struct AppState<F>
where
    F: FerriService,
{
    ferri_service: Arc<F>,
}

impl HttpServer {
    pub async fn new<F>(config: HttpServerConfig, ferri_service: Arc<F>) -> Result<Self>
    where
        F: FerriService,
    {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request| {
                let uri: String = request.uri().to_string();
                info_span!("http_request", method = ?request.method(), uri)
            },
        );
        let state = AppState {
            ferri_service: Arc::clone(&ferri_service),
        };

        let router = axum::Router::new()
            //.nest("/", api_routes())
            .layer(trace_layer)
            .layer(Extension(Arc::clone(&state.ferri_service)))
            .with_state(state);

        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("Failed to bind to port {}", config.port))?;

        Ok(Self { router, listener })
    }

    pub async fn run(self) -> Result<()> {
        info!(
            "Server is running on http://{}",
            self.listener.local_addr()?
        );
        axum::serve(self.listener, self.router)
            .await
            .context("received error while running server")?;

        Ok(())
    }
}

fn api_routes<F>() -> axum::Router<AppState<F>>
where
    F: FerriService,
{
    axum::Router::new()
}
