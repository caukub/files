use std::time::Duration;

use axum::routing::delete;
use axum::{Router, routing::get};
use files::configuration::get_configuration;
use files::routes::delete::delete_file;
use files::routes::file_list::get_file_list;
use files::routes::foo::foo_handler;
use files::routes::index::get_index;
use files::routes::video::video_handler;
use files::tracing::init_tracing;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::services::ServeDir;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::log::debug;

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration");
    init_tracing();

    let router = Router::new()
        .route("/", get(get_index))
        .route("/files", get(get_file_list))
        .route("/foo", get(foo_handler))
        .route("/video", get(video_handler))
        .route("/delete", delete(delete_file))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
        ))
        .nest_service("/resources", ServeDir::new("resources"));

    let address = configuration.application.address();
    let listener = TcpListener::bind(&address).await.unwrap();

    debug!("Listening on {}", &address);

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
