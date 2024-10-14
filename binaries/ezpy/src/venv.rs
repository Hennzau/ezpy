use indygreg::{ensure_python_version, metadata::VersionString};

use crate::install_home_ezpy;

pub async fn create_local_env(version: Option<VersionString>) -> eyre::Result<()> {
    if let Some(version) = version.clone() {
        ensure_python_version(version)?
    }

    let version = match version {
        Some(version) => version,
        None => crate::pin::get_pinned_version().await?,
    };

    let bin = indygreg::python_path(version.clone())?.join(crate::python_bin_path());
    if !bin.exists() {
        eyre::bail!(
            "Python version {} is not installed, please install it with `ezpy install python {}`",
            version,
            version
        );
    }

    let cmd = tokio::process::Command::new(bin)
        .arg("-m")
        .arg("venv")
        .arg(".venv")
        .spawn()?;

    let out = cmd.wait_with_output().await?;

    if !out.status.success() {
        eyre::bail!(
            "Failed to create virtual environment: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    println!(
        "Created virtual environment with Python version {} at .venv",
        version
    );

    Ok(())
}

pub async fn create_global_env(version: Option<VersionString>, name: String) -> eyre::Result<()> {
    if let Some(version) = version.clone() {
        ensure_python_version(version)?
    }

    let version = match version {
        Some(version) => version,
        None => crate::pin::get_pinned_version().await?,
    };

    let bin = indygreg::python_path(version.clone())?.join(crate::python_bin_path());
    if !bin.exists() {
        eyre::bail!(
            "Python version {} is not installed, please install it with `ezpy install python {}`",
            version,
            version
        );
    }

    let dir = install_home_ezpy()?.join("env").join(&name);
    if dir.exists() {
        eyre::bail!("Virtual environment already exists at {}", dir.display());
    }

    let cmd = tokio::process::Command::new(bin)
        .arg("-m")
        .arg("venv")
        .arg(dir.clone())
        .spawn()?;

    let out = cmd.wait_with_output().await?;

    if !out.status.success() {
        eyre::bail!(
            "Failed to create virtual environment: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    println!(
        "Created global virtual environment {} with Python version {} at {}",
        name,
        version,
        dir.display()
    );

    Ok(())
}

pub async fn delete_global_env(name: String) -> eyre::Result<()> {
    let dir = install_home_ezpy()?.join("env").join(&name);
    if !dir.exists() {
        eyre::bail!("Virtual environment does not exist at {}", dir.display());
    }

    tokio::fs::remove_dir_all(dir).await?;

    println!("Deleted global virtual environment {}", name);

    Ok(())
}

pub async fn list_global_envs() -> eyre::Result<()> {
    let env_dir = install_home_ezpy()?.join("env");

    if !env_dir.exists() {
        println!("No global virtual environments found");
        return Ok(());
    }

    let mut entries = tokio::fs::read_dir(env_dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        println!("{:?}", entry.file_name());
    }

    Ok(())
}

pub async fn get_nearest_env() -> eyre::Result<std::path::PathBuf> {
    let mut current = std::env::current_dir()?;

    let mut count = 0;
    loop {
        if count > 3 {
            break;
        }

        count += 1;

        if current.join(".venv").exists() {
            return Ok(current.join(".venv"));
        }

        if !current.pop() {
            break;
        }
    }

    return Err(eyre::eyre!("No virtual environment found"));
}
