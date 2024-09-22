use std::path::PathBuf;

use eyre::ContextCompat;
use python::PythonVersion;
use simple_home_dir::home_dir;

pub struct UV {
    pub installed: bool,
    pub bin: Option<PathBuf>,
}

pub mod python;

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

    async fn bin_execute(&self, cmd: &[String]) -> eyre::Result<()> {
        if let Some(bin) = &self.bin {
            let mut uv_cmd = tokio::process::Command::new(bin);
            uv_cmd.stdout(std::process::Stdio::piped());
            uv_cmd.stderr(std::process::Stdio::piped());

            for arg in cmd {
                uv_cmd.arg(arg);
            }

            let status = uv_cmd.status().await?;

            if !status.success() {
                eyre::bail!("Failed to execute uv command");
            }
        }

        Ok(())
    }

    async fn bin_execute_output(&self, cmd: &[String]) -> eyre::Result<String> {
        if let Some(bin) = &self.bin {
            let mut uv_cmd = tokio::process::Command::new(bin);
            uv_cmd.stdout(std::process::Stdio::piped());
            uv_cmd.stderr(std::process::Stdio::piped());

            for arg in cmd {
                uv_cmd.arg(arg);
            }

            let output = uv_cmd.output().await?;

            if !output.status.success() {
                eyre::bail!("Failed to execute uv command");
            }

            let output_text = String::from_utf8(output.stdout)?;

            Ok(output_text)
        } else {
            eyre::bail!("UV is not installed");
        }
    }

    pub async fn install_python(&self, python_version: PythonVersion) -> eyre::Result<()> {
        self.bin_execute(&[
            "python".to_string(),
            "install".to_string(),
            python_version.to_string(),
        ])
        .await
    }

    pub async fn uninstall_python(&self, python_version: PythonVersion) -> eyre::Result<()> {
        self.bin_execute(&[
            "python".to_string(),
            "uninstall".to_string(),
            python_version.to_string(),
        ])
        .await
    }

    pub async fn python_dir(&self, python_version: Option<PythonVersion>) -> eyre::Result<PathBuf> {
        let home = self
            .bin_execute_output(&["python".to_string(), "dir".to_string()])
            .await?;

        let home = home.trim();

        if let Some(python_version) = python_version {
            let mut entries = tokio::fs::read_dir(&home).await?;

            while let Some(entry) = entries.next_entry().await? {
                let name = entry.file_name().to_string_lossy().to_string();
                if entry.path().is_dir()
                    && name.starts_with(format!("cpython-{}", python_version.to_string()).as_str())
                {
                    #[cfg(target_os = "windows")]
                    return Ok(entry.path().join("python.exe"));

                    #[cfg(not(target_os = "windows"))]
                    return Ok(entry.path().join("bin").join("python"));
                }
            }
        }

        Ok(PathBuf::from(home.trim()))
    }

    pub async fn list_python(&self) -> eyre::Result<Vec<PathBuf>> {
        let home = self.python_dir(None).await?;

        let mut entries = tokio::fs::read_dir(&home).await?;

        let mut python_dirs = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            let name = entry.file_name().to_string_lossy().to_string();
            if entry.path().is_dir() && name.starts_with("cpython-") {
                python_dirs.push(entry.path());
            }
        }

        Ok(python_dirs)
    }

    pub async fn pip_install(&self, args: &[String]) -> eyre::Result<()> {
        self.bin_execute(&["pip".to_string(), "install".to_string(), args.join(" ")])
            .await
    }

    pub async fn create_venv(&self, python_version: PythonVersion) -> eyre::Result<()> {
        self.bin_execute(&[
            "venv".to_string(),
            "--python".to_string(),
            python_version.to_string(),
        ])
        .await
    }
}
