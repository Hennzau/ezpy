mod target;
mod uv;

use clap::{Parser, Subcommand};
use eyre::Result; // Using eyre for error handling

#[derive(Parser)]
#[command(name = "zz")]
#[command(
    about = "Manage the latest Python: perfect for educative purpose",
    version = "0.0.1"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    // To capture the file or cases without a subcommand
    #[arg()]
    file: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes the application
    Init,
    /// Installs something
    Install {
        #[arg()]
        package: String, // [string] like a package name
    },
    /// Creates a virtual environment
    Venv, // "venv"
    /// Copies a file or an item
    Copy, // "copy"
    Clean, // "clean"
}

#[tokio::main] // Using tokio for the main function
async fn main() -> Result<()> {
    // Returns a Result for error handling with eyre
    let cli = Cli::parse();

    if cli.command.is_none() {
        // If a file is passed without a subcommand
        if let Some(file) = cli.file {
            println!("File provided: {}", file);
        } else {
            println!("No file provided.");
        }
    }

    let python_version = "3.12.6".to_string();

    // If a subcommand is specified
    if let Some(command) = cli.command {
        match command {
            Commands::Init => {
                let mut uv = uv::Uv::new(".zz");
                uv.install().await?;

                uv.install_python(Some(python_version)).await?;
            }
            Commands::Clean => {
                let uv = uv::Uv::new(".zz");

                uv.uninstall_python(python_version).await?;
            }
            Commands::Install { package } => {
                let uv = uv::Uv::new(".zz");

                uv.pip_install(package).await?;
            }
            Commands::Venv => {
                let uv = uv::Uv::new(".zz");

                uv.venv(python_version).await?;
            }
            Commands::Copy => {
                let uv = uv::Uv::new(".zz");

                println!("{:?}", uv.path_bin(python_version).await?);
            }
        }
    }

    Ok(())
}
