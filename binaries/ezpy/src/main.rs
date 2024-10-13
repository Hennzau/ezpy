use std::path::PathBuf;

use clap::{Parser, Subcommand};
use eyre::{OptionExt, Result};
use indygreg::metadata::VersionString;

pub mod install;
pub mod pin;
pub mod venv;

pub mod tui;

pub fn install_home() -> eyre::Result<PathBuf> {
    if cfg!(windows) {
        Ok(simple_home_dir::home_dir()
            .ok_or_eyre(eyre::eyre!(
                "Failed to get home directory, your home directory is not set"
            ))?
            .join("ezpy")
            .join("data"))
    } else {
        Ok(simple_home_dir::home_dir()
            .ok_or_eyre(eyre::eyre!(
                "Failed to get home directory, your home directory is not set"
            ))?
            .join(".local")
            .join("share")
            .join("ezpy"))
    }
}

pub fn bin_path() -> PathBuf {
    if cfg!(windows) {
        PathBuf::from("python.exe")
    } else {
        PathBuf::from("bin").join("python")
    }
}

#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
struct EzpyCLI {
    #[command(subcommand)]
    command: Option<EzpyCommands>,
}

#[derive(Subcommand)]
enum EzpyCommands {
    #[command(
        about = "Install a Python packages for the current environment. Or install a specific version of Python."
    )]
    Install(InstallArgs),

    #[command(about = "Manage Python environments (create, activate, deactivate, list, etc.)")]
    Env(EnvArgs),

    #[command(
        about = "Pin a Python binary to a specific version so it's used in all following commands."
    )]
    Pin(PinArgs),
}

#[derive(Parser)]
struct InstallArgs {
    #[arg(short = 'r', value_name = "FILE")]
    requirements: Option<String>,

    #[arg(value_name = "PACKAGES OR `python <VERSION>'")]
    packages: Vec<String>,
}

#[derive(Parser)]
struct EnvArgs {
    #[command(subcommand)]
    command: Option<EnvCommand>,
    #[arg(required = false)]
    version: Option<VersionString>,
}

#[derive(Parser)]
struct PinArgs {
    #[arg(value_name = "VERSION")]
    version: VersionString,
}

#[derive(Subcommand)]
enum EnvCommand {
    Global(GlobalArgs),
    Activate(ActivateArgs),
}

#[derive(Parser)]
struct GlobalArgs {
    #[arg(value_name = "NAME")]
    global: String,
}

#[derive(Parser)]
struct ActivateArgs {
    #[arg(value_name = "NAME")]
    name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = EzpyCLI::parse();

    if let Some(command) = cli.command {
        match command {
            EzpyCommands::Install(args) => handle_install(args).await?,
            EzpyCommands::Env(env_args) => handle_env(env_args).await?,
            EzpyCommands::Pin(args) => handle_pin(args).await?,
        }
    } else {
        handle_no_command().await?;
    }

    Ok(())
}

async fn handle_no_command() -> Result<()> {
    println!("No command provided. Use --help to see available commands.");
    Ok(())
}

async fn handle_install(args: InstallArgs) -> Result<()> {
    if let Some(requirements_file) = args.requirements {
        install_from_requirements(&requirements_file).await?;
    } else if !args.packages.is_empty() {
        if args.packages[0] == "python" {
            if args.packages.len() > 1 {
                let version = &args.packages[1];
                install_python_version(version.to_string()).await?;
            } else {
                return Err(eyre::eyre!(
                    "A value is required for 'python <VERSION>' but none was supplied"
                ));
            }
        } else {
            install_packages(args).await?;
        }
    }
    Ok(())
}

async fn handle_env(env_args: EnvArgs) -> Result<()> {
    if env_args.command.is_none() {
        create_local_env(env_args.version).await?;
    } else {
        match env_args.command.unwrap() {
            EnvCommand::Global(args) => create_global_env(env_args.version, args).await?,
            EnvCommand::Activate(args) => activate_env(args).await?,
        }
    }

    Ok(())
}

async fn create_local_env(version: Option<VersionString>) -> Result<()> {
    venv::create_local_env(version).await
}

async fn create_global_env(version: Option<VersionString>, args: GlobalArgs) -> Result<()> {
    venv::create_global_env(version, args.global).await
}

async fn activate_env(args: ActivateArgs) -> Result<()> {
    if let Some(env_name) = args.name {
        println!("Activate global environment named {}", env_name);
    } else {
        println!("Activate local environment if found");
    }

    Ok(())
}

async fn handle_pin(args: PinArgs) -> Result<()> {
    pin::pin_version(args.version).await
}

async fn install_python_version(version: VersionString) -> Result<()> {
    let packages = indygreg::metadata::download_packages().await?;
    let package = indygreg::package::Package::from_string(version.to_string(), packages)?;

    println!("Package found, installing Python {}...", version);

    indygreg::install::download_install(package).await?;

    println!("Python {} installed successfully", version);

    Ok(())
}

async fn install_packages(args: InstallArgs) -> Result<()> {
    println!("Installing packages: {:?}", args.packages);

    Ok(())
}

async fn install_from_requirements(requirements_file: &str) -> Result<()> {
    println!("Installing packages from {}", requirements_file);

    Ok(())
}
