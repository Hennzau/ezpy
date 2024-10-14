use crate::venv::get_nearest_env;

pub async fn install_packages(packages: Vec<String>, global: Option<String>) -> eyre::Result<()> {
    let venv = match global {
        Some(global) => crate::venv::get_global_env(global)?,
        None => get_nearest_env().await?,
    };

    let bin = venv.join(crate::env_bin_path());

    let mut cmd = tokio::process::Command::new(bin);
    cmd.arg("-m")
        .arg("pip")
        .arg("install")
        .args(packages.clone());

    let out = cmd.spawn()?.wait_with_output().await?;

    if !out.status.success() {
        eyre::bail!(
            "Failed to install packages: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    Ok(())
}

pub async fn install_from_requirements(
    requirements_file: &str,
    global: Option<String>,
) -> eyre::Result<()> {
    let venv = match global {
        Some(global) => crate::venv::get_global_env(global)?,
        None => get_nearest_env().await?,
    };

    let bin = venv.join(crate::env_bin_path());

    let mut cmd = tokio::process::Command::new(bin);
    cmd.arg("-m")
        .arg("pip")
        .arg("install")
        .arg("-r")
        .arg(requirements_file);

    let out = cmd.spawn()?.wait_with_output().await?;

    if !out.status.success() {
        eyre::bail!(
            "Failed to install packages from requirements file: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    Ok(())
}
