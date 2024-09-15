mod indygreg;
mod install;
mod venv;

use clap::{Parser, Subcommand};
use eyre::bail;

#[derive(Parser)]
#[command(
    name = "ZZ",
    version = "0.0.1",
    about = "ZZ is a tool to manage Python installations and environnements."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Venv {
        #[arg(short, long)]
        version: String,

        #[arg(short, long)]
        name: Option<String>,
    },
    Install {
        #[arg(short, long)]
        version: String,
    },
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Venv { version, name }) => {
            use std::env;

            let python = match env::consts::OS {
                "macos" => format!(".zz/python-{}/bin/python3", version),
                "windows" => format!(".zz/python-{}/python", version),
                "linux" => format!(".zz/python-{}/bin/python3", version),
                _ => bail!("Unsupported OS"),
            };

            let name = name.unwrap_or(".venv".to_string());

            venv::make(python.as_str(), name.as_str()).await?;
        }
        Some(Commands::Install { version }) => {
            let target = indygreg::target::get_target()?;
            let assets = indygreg::cpython::get_release(target).await?;

            indygreg::download::download(assets, version.as_str(), ".zz/temp").await?;
            install::decompress_and_extract(
                ".zz/temp/python.tar.gz",
                format!(".zz/python-{}", version).as_str(),
            )
            .await?;
        }
        None => {}
    };

    Ok(())
}
