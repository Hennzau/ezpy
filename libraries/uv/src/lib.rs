use std::path::PathBuf;

use eyre::ContextCompat;
use simple_home_dir::home_dir;

pub struct UV {
    pub installed: bool,
    pub bin: Option<PathBuf>,
}

impl UV {
    pub fn new() -> eyre::Result<Self> {
        let home = home_dir().wrap_err(eyre::Report::msg("Could not find home directory"))?;

        #[cfg(target_os = "windows")]
        let bin = home.join(".cargo").join("bin").join("uv.exe");
        #[cfg(not(target_os = "windows"))]
        let bin = home.join(".cargo").join("bin").join("uv");

        Ok(Self {
            installed: bin.exists(),
            bin: Some(bin),
        })
    }

    pub async fn install_bin(&mut self) -> eyre::Result<()> {
        if !self.installed {
            #[cfg(target_os = "windows")]
            let script = script::fetch_script("https://astral.sh/uv/install.ps1").await?;
            #[cfg(not(target_os = "windows"))]
            let script = script::fetch_script("https://astral.sh/uv/install.sh").await?;

            script::execute_script(script).await?;

            self.installed = true;
        }

        Ok(())
    }

    pub async fn uninstall_bin(&mut self) -> eyre::Result<()> {
        if self.installed {
            if let Some(bin) = &self.bin {
                tokio::fs::remove_file(bin).await?;
            }

            self.installed = false;
        }

        Ok(())
    }
}
