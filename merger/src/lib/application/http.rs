use std::sync::Arc;

use crate::domain::{
    car::ports::CarService, ferri::ports::FerriService, passenger::ports::PassengerService,
};
use anyhow::{Context, Result};
use axum::routing::get;
use axum::Extension;
use handlers::get_ferries::get_ferries;
use tokio::net;
use tracing::{info, info_span};

pub mod handlers;
pub mod responses;

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
struct AppState<F, C, P>
where
    F: FerriService,
    C: CarService,
    P: PassengerService,
{
    ferri_service: Arc<F>,
    car_service: Arc<C>,
    passenger_service: Arc<P>,
}

impl HttpServer {
    pub async fn new<F, C, P>(
        config: HttpServerConfig,
        ferri_service: Arc<F>,
        car_service: Arc<C>,
        passenger_service: Arc<P>,
    ) -> Result<Self>
    where
        F: FerriService,
        C: CarService,
        P: PassengerService,
    {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request| {
                let uri: String = request.uri().to_string();
                info_span!("http_request", method = ?request.method(), uri)
            },
        );
        let state = AppState {
            ferri_service: Arc::clone(&ferri_service),
            car_service: Arc::clone(&car_service),
            passenger_service: Arc::clone(&passenger_service),
        };

        let router = axum::Router::new()
            //.nest("", api_routes())
            .merge(api_routes())
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

fn api_routes<F, C, P>() -> axum::Router<AppState<F, C, P>>
where
    F: FerriService,
    C: CarService,
    P: PassengerService,
{
    axum::Router::new().route("/ferries", get(get_ferries::<F>))
}
