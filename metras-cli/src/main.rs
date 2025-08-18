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
use secrecy::{ExposeSecret, SecretString};

use std::time::Duration;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use clap::{Parser, command, arg};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "127.0.0.1:62021")]
    bind: rama::net::address::SocketAddress,
    #[arg(long)]
    user: String,
    #[arg(long)]
    pass: SecretString,

    #[arg(long, default_value = "30")]
    graceful_shutdown_s: u64,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .init();

    let graceful = rama::graceful::Shutdown::default();

    let tcp_service = TcpListener::bind(args.bind)
        .await
        .expect("bind proxy to port");

    let authorizer = Basic::new(args.user, args.pass.expose_secret()).into_authorizer();
    let socks5_acceptor = Socks5Acceptor::default().with_authorizer(authorizer);

    graceful.spawn_task_fn(|guard| tcp_service.serve_graceful(guard, socks5_acceptor));

    graceful
        .shutdown_with_limit(Duration::from_secs(args.graceful_shutdown_s))
        .await
        .expect("graceful shutdown");
}