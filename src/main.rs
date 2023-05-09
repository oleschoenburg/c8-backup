use clap::{Parser, Subcommand};
use targets::remote::RemoteHelmInstallation;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};
use tracing_tree::HierarchicalLayer;

mod common;
mod components;
mod create;
mod elasticsearch;
mod list;
mod operate;
mod restore;
mod targets;
mod types;
mod zeebe;

#[derive(Subcommand)]
enum Commands {
    List,
    Create,
    Restore,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Registry::default()
        .with(
            EnvFilter::builder()
                .with_default_directive(Level::INFO.into())
                .from_env_lossy(),
        )
        .with(
            HierarchicalLayer::new(2)
                .with_targets(true)
                .with_bracketed_fields(true),
        )
        .init();
    let cli = Cli::parse();

    let target = RemoteHelmInstallation::find().await?;
    match cli.command {
        Commands::List => list::list(&target).await,
        Commands::Create => create::create().await,
        Commands::Restore => restore::restore().await,
    }
}
