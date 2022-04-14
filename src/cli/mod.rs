use clap::Parser;

///Owasu, Seatalk alert service adapter, written in Rust !
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Options {
    /// Owasu config path
    #[clap(long = "config", default_value = "config/owasu.yaml")]
    config: String,
}

impl Options {
    pub fn new() -> Self {
        Options::parse()
    }

    pub fn get_config_path(&self) -> &str {
        self.config.as_str()
    }
}

impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}
