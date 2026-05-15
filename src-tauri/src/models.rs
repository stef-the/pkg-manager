use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub manager: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutdatedPackage {
    pub name: String,
    pub current_version: String,
    pub latest_version: String,
    pub description: String,
    pub manager: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagerInfo {
    pub id: String,
    pub name: String,
    pub available: bool,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageDetail {
    pub name: String,
    pub version: String,
    pub description: String,
    pub manager: String,
    pub homepage: String,
    pub license: String,
    pub repository: String,
    pub dependencies: Vec<String>,
    pub install_size: String,
    pub installed_on: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub os: String,
    pub arch: String,
    pub hostname: String,
}
