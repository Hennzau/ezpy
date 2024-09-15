pub async fn make(python: &str, name: &str) -> eyre::Result<()> {
    let cmd = tokio::process::Command::new(python)
        .arg("-m")
        .arg("venv")
        .arg(name)
        .output()
        .await?;

    match cmd.status.success() {
        true => Ok(()),
        false => Err(eyre::eyre!("Failed to create venv")),
    }
}
