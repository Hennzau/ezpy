use std::process::Stdio;
use tokio::io::AsyncWriteExt;

async fn fetch_script_curl(url: &str) -> eyre::Result<String> {
    let curl_output = tokio::process::Command::new("curl")
        .arg("-LsSf")
        .arg(url)
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()
        .await?;

    if !curl_output.status.success() {
        eyre::bail!("Failed to fetch install script");
    }

    let script_text = String::from_utf8(curl_output.stdout)?;

    Ok(script_text)
}

async fn fetch_script_powershell(url: &str) -> eyre::Result<String> {
    let curl_output = tokio::process::Command::new("powershell")
        .arg("-c")
        .arg(format!("irm {}", url))
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()
        .await?;

    if !curl_output.status.success() {
        eyre::bail!("Failed to fetch install script");
    }

    let script_text = String::from_utf8(curl_output.stdout)?;

    Ok(script_text)
}

pub async fn fetch_script(url: &str) -> eyre::Result<String> {
    #[cfg(target_os = "windows")]
    let script = fetch_script_powershell(url).await?;
    #[cfg(not(target_os = "windows"))]
    let script = fetch_script_curl(url).await?;

    Ok(script)
}

async fn execute_script_sh(script: String) -> eyre::Result<()> {
    let mut sh_process = tokio::process::Command::new("sh")
        .stdin(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = sh_process.stdin.take() {
        stdin.write_all(script.as_bytes()).await?;
    }

    let sh_status = sh_process.wait().await?;

    if !sh_status.success() {
        eyre::bail!("Failed to install uv");
    }

    Ok(())
}

async fn execute_script_powershell(script: String) -> eyre::Result<()> {
    let mut powershell_process = tokio::process::Command::new("powershell")
        .arg("-c")
        .stdin(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = powershell_process.stdin.take() {
        stdin.write_all(script.as_bytes()).await?;
    }

    let powershell_status = powershell_process.wait().await?;

    if !powershell_status.success() {
        eyre::bail!("Failed to install uv");
    }

    Ok(())
}

pub async fn execute_script(script: String) -> eyre::Result<()> {
    #[cfg(target_os = "windows")]
    execute_script_powershell(script).await?;
    #[cfg(not(target_os = "windows"))]
    execute_script_sh(script).await?;

    Ok(())
}
