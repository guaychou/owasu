/*
    MODULE IMPORT

*/

use crate::configuration::ServerConfig;
use crate::domain::seatalk::Seatalk;
use crate::error::handle_error;
use crate::handler::alert::alert;
use crate::handler::uptime_kuma::kuma_alert;
use axum::error_handling::HandleErrorLayer;
use axum::{
    body::{Body, Bytes},
    extract::{ConnectInfo, Extension},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};

use axum_server::Handle;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::sleep;
use tower::{
    buffer::BufferLayer,
    limit::{ConcurrencyLimitLayer, RateLimitLayer},
    timeout::TimeoutLayer,
    ServiceBuilder,
};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer, ServiceBuilderExt};
use tracing::Span;

/*
    APPLICATION BUILDING SEGMENT
*/

pub struct Application {
    pub router: Router,
    pub handle: Handle,
}

pub fn build(config: ServerConfig, seatalk: Seatalk) -> Application {
    tracing::info!("Building {} application", env!("CARGO_PKG_NAME"));
    tracing::info!("Initialize HTTP tracing");
    let http_trace = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<Body>| {
            tracing::info_span!(
                "Request",
                status_code = tracing::field::Empty,
                ms = tracing::field::Empty,
                path = tracing::field::display(request.uri().path()),
                ip = tracing::field::debug(
                    request
                        .extensions()
                        .get::<ConnectInfo<SocketAddr>>()
                        .unwrap()
                )
            )
        })
        .on_response(|response: &Response<_>, latency: Duration, span: &Span| {
            span.record(
                "status_code",
                &tracing::field::display(response.status().as_u16()),
            );
            span.record("ms", &tracing::field::display(latency.as_millis()));
            if response.status().as_u16() < 500 {
                tracing::info!("response processed")
            }
        })
        .on_failure(
            |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                tracing::error!("{}", error)
            },
        );
    tracing::info!("Initialize middleware stack | {}", config);
    let middleware_stack = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .load_shed()
        .layer(BufferLayer::new(*config.get_buffer()))
        .layer(ConcurrencyLimitLayer::new(*config.get_concurrency_limit()))
        .layer(TimeoutLayer::new(*config.get_timeout()))
        .layer(RateLimitLayer::new(
            *config.get_rate_limit(),
            *config.get_limiter_timeout(),
        ))
        .layer(http_trace)
        .layer(Extension(seatalk))
        .compression();
    tracing::info!("Setting up router...");
    let router = Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .route("/alert", post(alert))
                .route("/kuma", post(kuma_alert)),
        )
        .layer(middleware::from_fn(print_request_response))
        .layer(middleware_stack);

    let apps = Application {
        router,
        handle: Handle::new(),
    };
    tracing::info!("Setting up graceful shutdown handler...");
    tokio::spawn(graceful_shutdown(apps.handle.clone()));
    apps
}

async fn graceful_shutdown(handle: Handle) {
    use std::io;
    use tokio::signal::unix::SignalKind;
    async fn terminate() -> io::Result<()> {
        tokio::signal::unix::signal(SignalKind::terminate())?
            .recv()
            .await;
        Ok(())
    }

    tokio::select! {
        _ = terminate() => {},
        _ = tokio::signal::ctrl_c() => {},
    }
    tracing::info!("signal received, starting graceful shutdown");
    // Signal the server to shutdown using Handle.
    handle.graceful_shutdown(Some(Duration::from_secs(30)));

    // Print alive connection count every second.
    loop {
        sleep(Duration::from_secs(1)).await;
        tracing::info!("alive connections: {}", handle.connection_count());
    }
}

async fn print_request_response(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {} body: {}", direction, err),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("{} body = {:?}", direction, body);
    }

    Ok(bytes)
}
