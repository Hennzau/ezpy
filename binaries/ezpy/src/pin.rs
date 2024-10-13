use std::path::PathBuf;

use eyre::OptionExt;
use indygreg::{install::install_home, metadata::VersionString};

pub async fn pin_version(version: VersionString) -> eyre::Result<()> {
    indygreg::ensure_python_version(version.clone())?;

    let path = install_home()?.join("python.txt");

    tokio::fs::create_dir_all(path.parent().unwrap()).await?;
    tokio::fs::write(&path, version.to_string()).await?;

    println!("Pinned version {} for future commands", version);

    Ok(())
}

pub async fn get_pinned_version() -> eyre::Result<VersionString> {
    let path = install_home()?.join("python.txt");

    if !path.exists() {
        eyre::bail!("No pinned version found, you may need a pinned version to run this command without specifying a version");
    }

    let contents = tokio::fs::read_to_string(&path).await?;

    Ok(contents)
}
