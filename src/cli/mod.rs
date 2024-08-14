pub mod command;
mod error;
mod fmt;
pub mod generator;
#[cfg(feature = "js")]
pub mod javascript;
mod llm;
pub mod metrics;
pub mod runtime;
pub mod server;
mod tc;
pub mod telemetry;
pub(crate) mod update_checker;
pub use error::CLIError;
pub use tc::run::run;

pub async fn start(command: &Command) -> Result<()> {
    match command {
        Command::Start { file_paths, watch } => {
            let config_module = ConfigModule::from_paths(file_paths)?;
            let server = Server::new(config_module, *watch);

            if *watch {
                server.start_with_watch().await?;
            } else {
                server.start().await?;
            }
        }
    }
    Ok(())
}