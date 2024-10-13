use std::path::{Path, PathBuf};

use crate::package::Package;

use eyre::OptionExt;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub fn install_home() -> eyre::Result<PathBuf> {
    if cfg!(windows) {
        Ok(simple_home_dir::home_dir()
            .ok_or_eyre(eyre::eyre!(
                "Failed to get home directory, your home directory is not set"
            ))?
            .join("indygreg")
            .join("data")
            .join("python"))
    } else {
        Ok(simple_home_dir::home_dir()
            .ok_or_eyre(eyre::eyre!(
                "Failed to get home directory, your home directory is not set"
            ))?
            .join(".local")
            .join("share")
            .join("indygreg")
            .join("python"))
    }
}

pub async fn download_install(package: Package) -> eyre::Result<()> {
    let url = package.url;
    let destination = install_home()?.join("python.tar.gz");
    let unpacked = install_home()?.join("python");
    let final_destination = install_home()?.join(format!(
        "python-{}-{}.{}",
        package.major, package.minor, package.patch
    ));

    download_as_tar_gz(&url, &destination).await?;
    unpack_tar_gz(&destination, &unpacked).await?;

    // Move content from unpacked/python to final_destination

    let source = unpacked.join("python");
    tokio::fs::remove_dir_all(&final_destination).await.ok();
    tokio::fs::rename(&source, &final_destination).await?;

    tokio::fs::remove_dir_all(&unpacked).await?;
    tokio::fs::remove_file(&destination).await?;

    Ok(())
}

async fn download_as_tar_gz(url: &str, destination: &Path) -> eyre::Result<()> {
    let client = Client::new();
    let mut response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(eyre::eyre!(
            "Error downloading: {}, check your internet connection",
            response.status()
        ));
    }

    let path = Path::new(destination);
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    let mut file = File::create(destination).await?;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }

    file.flush().await?;

    Ok(())
}

async fn unpack_tar_gz(source: &Path, destination: &Path) -> eyre::Result<()> {
    let tar_gz = std::fs::File::open(source)?;

    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(destination)?;

    Ok(())
}
