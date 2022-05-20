use owasu::application;
use owasu::cli;
use owasu::configuration;
use owasu::domain::seatalk::Seatalk;
use owasu::log;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let cli = cli::Options::new();
    log::log_init();
    let config = configuration::read_config(cli.get_config_path());
    let seatalk = Seatalk::new(config.seatalk);
    let addr: SocketAddr = SocketAddr::from((
        "0.0.0.0".parse::<std::net::Ipv4Addr>().unwrap(),
        *config.server.get_port(),
    ));
    let apps = application::build(config.server, seatalk);
    let server = axum_server::bind(addr).handle(apps.handle).serve(
        apps.router
            .into_make_service_with_connect_info::<SocketAddr>(),
    );
    tracing::info!("Listening on {:?}", addr);
    if let Err(err) = server.await {
        tracing::error!("server error: {:?}", err);
    }
}
