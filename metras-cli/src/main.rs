//! An example to showcase how one can build an authenticated socks5 CONNECT proxy server.
//!
//! # Expected output
//!
//! The server will start and listen on `:62021`. You can use `curl` to interact with the service:
//!
//! ```sh
//! curl -v -x socks5://127.0.0.1:62021 --proxy-user 'john:secret' https://www.example.com/
//! curl -v -x socks5h://127.0.0.1:62021 --proxy-user 'john:secret' https://www.example.com/
//! ```

use rama::{
    net::user::Basic, proxy::socks5::Socks5Acceptor, tcp::server::TcpListener,
    telemetry::tracing::level_filters::LevelFilter,
};

use std::time::Duration;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

const PROXY_SOCKS_ADDR: &str = "127.0.0.1:62021";
const PROXY_SOCKS_USER: &str = "john";
const PROXY_SOCKS_PASS: &str = "secret";

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .init();

    let graceful = rama::graceful::Shutdown::default();

    let tcp_service = TcpListener::bind(PROXY_SOCKS_ADDR)
        .await
        .expect("bind proxy to port");

    let socks5_acceptor = Socks5Acceptor::default()
        .with_authorizer(Basic::new_static(PROXY_SOCKS_USER, PROXY_SOCKS_PASS).into_authorizer());
    graceful.spawn_task_fn(|guard| tcp_service.serve_graceful(guard, socks5_acceptor));

    graceful
        .shutdown_with_limit(Duration::from_secs(30))
        .await
        .expect("graceful shutdown");
}