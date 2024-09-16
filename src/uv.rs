use std::path::PathBuf;

pub mod install;

pub struct Uv {
    pub installed: bool,
    pub bin: std::path::PathBuf,
    pub home: std::path::PathBuf,
}

impl Uv {
    pub fn new(home: &str) -> Self {
        #[cfg(target_os = "windows")]
        let bin = std::path::Path::new(home).join("bin/uv.exe");
        #[cfg(not(target_os = "windows"))]
        let bin = std::path::Path::new(home).join("bin/uv");

        let installed = bin.exists();

        let home = std::path::Path::new(home).to_path_buf();

        Self {
            installed,
            bin,
            home,
        }
    }

    async fn search_venv(&self) -> eyre::Result<PathBuf> {
        let mut path = std::env::current_dir()?;
        loop {
            let venv = path.join(".venv");
            if venv.exists() {
                return Ok(venv);
            }

            if !path.pop() {
                break;
            }
        }

        Err(eyre::eyre!("No venv found."))
    }

    pub async fn install(&mut self) -> eyre::Result<()> {
        if !self.installed {
            install::download_and_install(&self.home).await?;
            self.installed = true;
        }

        Ok(())
    }

    async fn execute(&self, mut cmd: tokio::process::Command) -> eyre::Result<()> {
        if !self.installed {
            return Err(eyre::eyre!("uv is not installed."));
        }

        let status = cmd.status().await?;
        if !status.success() {
            return Err(eyre::eyre!("Failed to execute command."));
        }

        Ok(())
    }

    pub async fn install_python(&self, version: Option<String>) -> eyre::Result<()> {
        let mut cmd = tokio::process::Command::new(&self.bin);
        cmd.arg("python").arg("install");
        if let Some(version) = version {
            cmd.arg(version);
        }

        self.execute(cmd).await?;

        Ok(())
    }

    pub async fn uninstall_python(&self, version: String) -> eyre::Result<()> {
        let mut cmd = tokio::process::Command::new(&self.bin);
        cmd.arg("python").arg("uninstall").arg(version);

        self.execute(cmd).await?;

        Ok(())
    }

    pub async fn venv(&self, version: String) -> eyre::Result<()> {
        let venv = self.search_venv().await;

        if let Err(_) = venv {
            let mut cmd = tokio::process::Command::new(&self.bin);
            cmd.arg("venv").arg("--python").arg(version);

            self.execute(cmd).await?;
        }

        Ok(())
    }

    pub async fn pip_install(&self, package: String) -> eyre::Result<()> {
        let mut cmd = tokio::process::Command::new(&self.bin);
        cmd.arg("pip").arg("install").arg(package);

        self.execute(cmd).await?;

        Ok(())
    }

    pub async fn path_bin(&self, version: &str) -> eyre::Result<PathBuf> {
        if !self.installed {
            return Err(eyre::eyre!("uv is not installed."));
        }

        let venv = self.search_venv().await;

        if let Ok(venv) = venv {
            #[cfg(target_os = "windows")]
            return Ok(venv.join("Scripts").join("python.exe"));

            #[cfg(not(target_os = "windows"))]
            return Ok(venv.join("bin").join("python3"));
        }

        let mut cmd = tokio::process::Command::new(&self.bin);
        cmd.arg("python").arg("dir");

        let dir = cmd.output().await?;
        let dir = PathBuf::from(std::str::from_utf8(&dir.stdout)?.trim());

        let mut entries = tokio::fs::read_dir(&dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let name = entry.file_name().to_string_lossy().to_string();
            if entry.path().is_dir() && name.starts_with(format!("cpython-{}", version).as_str()) {
                #[cfg(target_os = "windows")]
                return Ok(entry.path().join("python.exe"));

                #[cfg(not(target_os = "windows"))]
                return Ok(entry.path().join("bin").join("python3"));
            }
        }

        Err(eyre::eyre!("Python version not found."))
    }
}
