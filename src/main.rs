mod indygreg;
mod install;

use clap::{Parser, Subcommand};

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
        Some(Commands::Venv { version }) => {}
        Some(Commands::Install { version }) => {}
        None => {
            println!("Aucune sous-commande fournie, exécution sans options...");
            let target = indygreg::target::get_target()?;
            let assets = indygreg::cpython::get_release(target).await?;

            for asset in &assets {
                println!("Version {} available at {}", asset.0, asset.1.name);
            }

            let version = "3.10";

            indygreg::download::download(assets, version, ".zz/temp").await?;
            install::targz::decompress_and_extract(
                ".zz/temp/python.tar.gz",
                format!(".zz/python-{}", version).as_str(),
            )
            .await?;
        }
    }

    Ok(())
}
