use std::path::PathBuf;

use metadata::VersionString;

pub mod install;
pub mod metadata;
pub mod package;

pub fn python_path(version: VersionString) -> eyre::Result<PathBuf> {
    Ok(install::install_home()?.join(format!("python-{}", version)))
}

pub fn ensure_python_version(version: VersionString) -> eyre::Result<()> {
    let path = python_path(version.clone())?;

    if path.exists() {
        Ok(())
    } else {
        Err(eyre::eyre!(
            "Python version {} is not installed, please install it with `ezpy install python {}`",
            version,
            version
        ))
    }
}
