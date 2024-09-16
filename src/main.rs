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

    #[arg()]
    args: Option<Vec<String>>,
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
    Dir, // "copy"
    Clean, // "clean"
}

#[tokio::main] // Using tokio for the main function
async fn main() -> Result<()> {
    // Returns a Result for error handling with eyre
    let cli = Cli::parse();

    let python_version = "3.12.6".to_string();
    let zz_home = simple_home_dir::home_dir()
        .unwrap()
        .join(".zz")
        .to_str()
        .unwrap()
        .to_string();

    if cli.command.is_none() {
        // If a file is passed without a subcommand
        if let Some(args) = cli.args {
            let file = args.first().unwrap();
            let args = args.iter().skip(1).collect::<Vec<_>>();

            let uv = uv::Uv::new(&zz_home);
            let path = uv.path_bin(&python_version).await?;

            let mut cmd = tokio::process::Command::new(path);
            cmd.arg(file);

            for arg in args {
                cmd.arg(arg);
            }

            let status = cmd.status().await?;

            if !status.success() {
                println!("Failed to run the python file, see the error above.");
            }
        } else {
            println!(
                "No file provided. If you want to run a Python file, please provide a file name."
            );
        }

        return Ok(());
    }

    // If a subcommand is specified
    if let Some(command) = cli.command {
        match command {
            Commands::Init => {
                let mut uv = uv::Uv::new(&zz_home);
                uv.install().await?;

                uv.install_python(Some(python_version)).await?;
            }
            Commands::Clean => {
                let uv = uv::Uv::new(&zz_home);

                uv.uninstall_python(python_version).await?;
            }
            Commands::Install { package } => {
                let uv = uv::Uv::new(&zz_home);

                uv.venv(python_version).await?;
                uv.pip_install(package).await?;
            }
            Commands::Venv => {
                let uv = uv::Uv::new(&zz_home);

                uv.venv(python_version).await?;
            }
            Commands::Dir => {
                let uv = uv::Uv::new(&zz_home);

                let path = uv.path_bin(&python_version).await?;
                let path_string = path.to_str().unwrap().to_string();
                println!("{}", path_string);
            }
        }
    }

    Ok(())
}
