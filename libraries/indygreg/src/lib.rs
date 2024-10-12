pub mod metadata;
pub mod query;

#[derive(Debug, Clone)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

/*
"cpython-3.13.0-darwin-aarch64-none": {
  "name": "cpython",
  "arch": "aarch64",
  "os": "darwin",
  "libc": "none",
  "major": 3,
  "minor": 13,
  "patch": 0,
  "prerelease": "",
  "url": "https://github.com/indygreg/python-build-standalone/releases/download/20241008/cpython-3.13.0%2B20241008-aarch64-apple-darwin-install_only_stripped.tar.gz",
  "sha256": "7bc4b23590a1e4b41b21b6aae6f92046c1d16d09bc0c1ab81272aa81b55221d1"
},
*/

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub arch: String,
    pub os: String,
    pub libc: String,
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub prerelease: Option<String>,
    pub url: String,
    pub sha256: Option<String>,
}
