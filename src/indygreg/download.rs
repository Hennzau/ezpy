use super::cpython::Asset;
use tokio::{
    fs::{create_dir_all, File},
    io::AsyncWriteExt,
};

pub async fn download(
    assets: Vec<(String, Asset)>,
    version: &str,
    destination: &str,
) -> eyre::Result<()> {
    let asset = assets
        .iter()
        .find(|(v, _)| v == &version)
        .map(|(_, asset)| asset)
        .ok_or_else(|| eyre::eyre!("Version {} not found", version))?;

    let client = reqwest::Client::new();
    let mut response = client
        .get(asset.browser_download_url.as_str())
        .header("User-Agent", "rust-client")
        .send()
        .await?;

    let _ = create_dir_all(destination).await;

    let mut file = File::create(format!("{}/python.tar.gz", destination))
        .await
        .unwrap();

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await.unwrap();
    }

    Ok(())
}
