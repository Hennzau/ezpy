use indygreg::{ensure_python_version, metadata::VersionString};

use crate::install_home;

pub async fn create_local_env(version: Option<VersionString>) -> eyre::Result<()> {
    if let Some(version) = version.clone() {
        ensure_python_version(version)?
    }

    let version = match version {
        Some(version) => version,
        None => crate::pin::get_pinned_version().await?,
    };

    let bin = indygreg::python_path(version.clone())?.join(crate::bin_path());
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
        "Created virtual environment with Python version {}",
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

    let bin = indygreg::python_path(version.clone())?.join(crate::bin_path());
    if !bin.exists() {
        eyre::bail!(
            "Python version {} is not installed, please install it with `ezpy install python {}`",
            version,
            version
        );
    }

    let dir = install_home()?.join("env").join(&name);
    if dir.exists() {
        eyre::bail!("Virtual environment already exists at {}", dir.display());
    }

    let cmd = tokio::process::Command::new(bin)
        .arg("-m")
        .arg("venv")
        .arg(dir)
        .spawn()?;

    let out = cmd.wait_with_output().await?;

    if !out.status.success() {
        eyre::bail!(
            "Failed to create virtual environment: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    println!(
        "Created global virtual environment {} with Python version {}",
        name, version
    );

    Ok(())
}
