use eyre::bail;
use std::env;

pub fn get_target() -> eyre::Result<String> {
    let target = match env::consts::OS {
        "macos" => match env::consts::ARCH {
            "aarch64" => "aarch64-apple-darwin",
            "x86_64" => "x86_64-apple-darwin",
            _ => bail!("Unsupported architecture"),
        },
        "windows" => "x86_64-pc-windows-msvc",
        "linux" => "x86_64-unknown-linux-gnu",
        _ => bail!("Unsupported OS"),
    };

    Ok(target.to_string())
}
