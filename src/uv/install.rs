use serde::Deserialize;
use tokio::fs;
use tokio::io;
use tokio::io::AsyncReadExt;

use std::path::PathBuf;

use flate2::bufread::GzDecoder;
use tar::Archive;

pub async fn download_and_install(destination: &PathBuf) -> eyre::Result<()> {
    #[derive(Deserialize, Debug)]
    struct Asset {
        name: String,
        browser_download_url: String,
    }

    #[derive(Deserialize, Debug)]
    struct Release {
        #[allow(dead_code)]
        tag_name: String,
        #[allow(dead_code)]
        name: String,
        assets: Vec<Asset>,
    }

    let target = crate::target::get_target()?;

    async fn download_targz(destination: &PathBuf, target: &String) -> eyre::Result<()> {
        let owner = "astral-sh";
        let repo = "uv";

        let url = format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            owner, repo
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("User-Agent", "rust-client")
            .send()
            .await?
            .json::<Release>()
            .await?;

        let assets: Vec<Asset> = response
            .assets
            .into_iter()
            .filter(|asset| asset.name.contains(target) && asset.name.ends_with(".tar.gz"))
            .collect();

        let asset = assets
            .first()
            .ok_or_else(|| eyre::eyre!("Asset not found"))?;

        let mut response = client
            .get(asset.browser_download_url.as_str())
            .header("User-Agent", "rust-client")
            .send()
            .await?;

        let _ = fs::create_dir_all(destination).await;

        let mut file = fs::File::create(destination.join("uv.tar.gz"))
            .await
            .unwrap();

        while let Some(chunk) = response.chunk().await? {
            use tokio::io::AsyncWriteExt;

            file.write_all(&chunk).await?;
        }

        Ok(())
    }

    if !destination.join("uv.tar.gz").exists() {
        download_targz(destination, &target).await?;
    }

    let output_dir = destination.join("bin");

    if output_dir.exists() {
        fs::remove_dir_all(&output_dir).await?;
    }

    let file = fs::File::open(destination.join("uv.tar.gz")).await?;
    let mut buf_reader = io::BufReader::new(file);

    let mut buffer = Vec::new();
    buf_reader.read_to_end(&mut buffer).await?;

    let gz_decoder = GzDecoder::new(&buffer[..]);

    let mut archive = Archive::new(gz_decoder);
    archive.unpack(&output_dir)?;

    let source_path = destination.join("bin").join(format!("uv-{}", target));

    let parent_path = source_path
        .parent()
        .ok_or_else(|| eyre::eyre!("No parent directory"))?;

    let mut entries = fs::read_dir(&source_path).await?;

    while let Some(entry) = entries.next_entry().await? {
        let entry_path = entry.path();
        let file_name = entry.file_name();
        let destination_path = parent_path.join(file_name);

        fs::rename(&entry_path, &destination_path).await?;
    }

    fs::remove_dir_all(source_path).await?;

    Ok(())
}
