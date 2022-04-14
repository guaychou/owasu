use {getset::Getters, serde::Deserialize, std::fmt, std::time::Duration};

#[derive(Debug, Deserialize, Getters, Clone)]
pub struct SeatalkConfig {
    #[getset(get = "pub with_prefix")]
    chat_id: String,
}

#[derive(Deserialize, Getters)]
pub struct ServerConfig {
    #[getset(get = "pub with_prefix")]
    port: u16,
    #[getset(get = "pub with_prefix")]
    buffer: usize,
    #[getset(get = "pub with_prefix")]
    concurrency_limit: usize,
    #[getset(get = "pub with_prefix")]
    rate_limit: u64,
    #[getset(get = "pub with_prefix")]
    #[serde(with = "humantime_serde")]
    limiter_timeout: Duration,
    #[getset(get = "pub with_prefix")]
    #[serde(with = "humantime_serde")]
    timeout: Duration,
}

#[derive(Deserialize, Getters, Default)]
pub struct Config {
    pub seatalk: SeatalkConfig,
    pub server: ServerConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            buffer: 10,
            concurrency_limit: 5,
            rate_limit: 5,
            limiter_timeout: Duration::from_secs(10),
            timeout: Duration::from_secs(10),
        }
    }
}

impl Default for SeatalkConfig {
    fn default() -> Self {
        Self {
            chat_id: String::from(""),
        }
    }
}

impl fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "buffer: {} | concurrency limit: {} | rate limit: {} | limiter timeout: {}s | timeout: {}s", self.buffer, self.concurrency_limit, self.rate_limit, self.limiter_timeout.as_secs(), self.timeout.as_secs())
    }
}

pub fn read_config(config_path: &str) -> Config {
    let config: Config = match std::fs::File::open(config_path) {
        Ok(file) => serde_yaml::from_reader(file).unwrap_or_default(),
        Err(_) => {
            tracing::warn!("File config not found, Using default config");
            Config::default()
        }
    };
    config
}
