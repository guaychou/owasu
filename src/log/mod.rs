use {
    env,
    figlet_rs::FIGfont,
    time::macros::format_description,
    tracing::subscriber::set_global_default,
    tracing_log::LogTracer,
    tracing_subscriber::{
        filter::EnvFilter,
        fmt::{format, time::LocalTime},
        layer::SubscriberExt,
        Registry,
    },
};

pub fn log_init() {
    let time_format = LocalTime::new(format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second]"
    ));
    let tracing_format = format()
        .with_timer(time_format)
        .with_thread_names(true)
        .with_thread_ids(true);
    let fmt_layer = tracing_subscriber::fmt::Layer::default().event_format(tracing_format);
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", env!("CARGO_PKG_NAME").to_owned() + "=info")
    }
    let collector = Registry::default()
        .with(EnvFilter::from_default_env())
        .with(fmt_layer);
    LogTracer::init().expect("Failed to set logger");
    set_global_default(collector).expect("Failed to set subscriber");
    print_banner();
}

fn print_banner() {
    let standard_font = FIGfont::standand().unwrap();
    let figure = standard_font.convert(env!("CARGO_PKG_NAME"));
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    tracing::info!(
        "Starting {} version: {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
}
