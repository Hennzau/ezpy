use eyre::Result;

pub struct PythonVersion {
    major: u8,
    minor: u8,
    patch: Option<u8>,
}

impl PythonVersion {
    /// Create a new PythonVersion with mandatory major and minor, optional patch
    pub fn new(major: u8, minor: u8, patch: Option<u8>) -> Self {
        PythonVersion {
            major,
            minor,
            patch,
        }
    }

    /// Create a PythonVersion from a version string like "3.9.7" or "3.9"
    pub fn from_str(version_str: &str) -> Result<Self> {
        let parts: Vec<&str> = version_str.split('.').collect();

        let major = parts
            .get(0)
            .ok_or(eyre::Report::msg("Missing major version"))?
            .parse::<u8>()
            .map_err(|_| eyre::Report::msg("Invalid major version"))?;

        let minor = parts
            .get(1)
            .ok_or(eyre::Report::msg("Missing minor version"))?
            .parse::<u8>()
            .map_err(|_| eyre::Report::msg("Invalid minor version"))?;

        let patch = if let Some(patch_str) = parts.get(2) {
            Some(
                patch_str
                    .parse::<u8>()
                    .map_err(|_| eyre::Report::msg("Invalid patch version"))?,
            )
        } else {
            None
        };

        Ok(PythonVersion {
            major,
            minor,
            patch,
        })
    }

    /// Display the Python version as a string
    pub fn to_string(&self) -> String {
        match self.patch {
            Some(patch) => format!("{}.{}.{}", self.major, self.minor, patch),
            None => format!("{}.{}", self.major, self.minor),
        }
    }
}
