pub mod backend;

use env_logger::{Builder, Env, Target};
use log::{error, info};
use std::io::Write;
use std::process;

#[tokio::main]
async fn main() {
    let mut builder;
    log_panics::init();

    if cfg!(debug_assertions) {
        builder = Builder::from_env(Env::default().default_filter_or("debug"));
    } else {
        builder = Builder::from_env(Env::default().default_filter_or("info"));
    }

    builder.format(|f, record| match record.level() {
        log::Level::Error => {
            eprintln!("{}", record.args());
            Ok(())
        }
        _ => {
            writeln!(f, "{}", record.args())
        }
    });

    builder.target(Target::Stdout).init();

    info!("module version: {}", env!("CARGO_PKG_VERSION"));

    if let Err(e) = backend::run().await {
        error!("application error: {e:#}");

        process::exit(1);
    }

    info!("application shutdown")
}
