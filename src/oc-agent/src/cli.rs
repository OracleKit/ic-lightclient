use anyhow::{anyhow, Result};
use clap::Parser;
use std::sync::OnceLock;

static INNER: OnceLock<CliArgs> = OnceLock::new();

#[derive(Parser, Debug)]
#[command()]
struct CliArgs {
    #[arg(short, long, default_value = "oraclekit.toml")]
    config_file: String,
}

pub struct Cli {}

impl Cli {
    pub fn init() -> Result<()> {
        let args = CliArgs::parse();
        INNER
            .set(args)
            .map_err(|_| anyhow!("Attempting to initialize CLI Args twice."))?;

        Ok(())
    }

    pub fn config_file() -> String {
        INNER.get().unwrap().config_file.clone()
    }
}
