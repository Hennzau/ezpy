use std::path::Path;

use flate2::bufread::GzDecoder;
use tar::Archive;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::BufReader;

pub async fn decompress_and_extract(archive_path: &str, output_dir: &str) -> eyre::Result<()> {
    fs::remove_dir_all(output_dir).await?;

    let file = File::open(archive_path).await?;
    let mut buf_reader = BufReader::new(file);

    let mut buffer = Vec::new();
    buf_reader.read_to_end(&mut buffer).await?;

    let gz_decoder = GzDecoder::new(&buffer[..]);

    let mut archive = Archive::new(gz_decoder);

    archive.unpack(output_dir)?;

    let source_dir = Path::new(output_dir).join("python");
    move_contents_up_and_delete(&source_dir.to_string_lossy()).await?;

    Ok(())
}

pub async fn move_contents_up_and_delete(source_dir: &str) -> eyre::Result<()> {
    let source_path = Path::new(source_dir);

    let parent_path = source_path
        .parent()
        .ok_or_else(|| eyre::eyre!("No parent directory"))?;

    let mut entries = fs::read_dir(source_path).await?;

    while let Some(entry) = entries.next_entry().await? {
        let entry_path = entry.path();
        let file_name = entry.file_name();
        let destination_path = parent_path.join(file_name);

        fs::rename(&entry_path, &destination_path).await?;
    }

    fs::remove_dir_all(source_path).await?;

    Ok(())
}
