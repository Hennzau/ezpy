use clap::{Parser, Subcommand};
use eyre::Result;

#[derive(Parser)]
#[command(
    name = "ezpy",
    version = env!("CARGO_PKG_VERSION"),
    about = "Python Management made ez"
)]
struct EzpyCLI {
    #[command(subcommand)]
    command: Option<EzpyCommands>,
}

#[derive(Subcommand)]
enum EzpyCommands {
    Install(InstallArgs),
    Env(EnvArgs),
    Pin(PinArgs),
}

#[derive(Parser)]
struct InstallArgs {
    #[arg(short = 'r', long = "requirements", value_name = "FILE")]
    requirements: Option<String>,
    packages: Vec<String>,
}

#[derive(Parser)]
struct EnvArgs {
    #[command(subcommand)]
    command: Option<EnvCommand>,
    #[arg(required = false)]
    version: Option<String>,
}

#[derive(Parser)]
struct PinArgs {
    version: String,
}

#[derive(Subcommand)]
enum EnvCommand {
    Global(GlobalArgs),
    Activate(ActivateArgs),
}

#[derive(Parser)]
struct GlobalArgs {
    global: String,
}

#[derive(Parser)]
struct ActivateArgs {
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
                install_python_version(version).await?;
            } else {
                return Err(eyre::eyre!("Please specify a Python version to install."));
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
            EnvCommand::Global(args) => create_global_env(args).await?,
            EnvCommand::Activate(args) => activate_env(args).await?,
        }
    }
    Ok(())
}

async fn create_local_env(version: Option<String>) -> Result<()> {
    println!("Create a local environment in the current directory");
    if let Some(version) = version {
        println!("Optional version provided: {}", version);
    }
    Ok(())
}

async fn create_global_env(args: GlobalArgs) -> Result<()> {
    println!("Creating a global environment named {}", args.global);
    Ok(())
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
    println!("Pin version: {}", args.version);
    Ok(())
}

async fn install_python_version(version: &str) -> Result<()> {
    println!("Installing Python version {}", version);
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
