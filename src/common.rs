// Package represents an archlinux package.
pub struct Package {
    pub name: String,
    pub maintainer: Vec<User>,
}

// User represents a package maintainer.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub github: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LilacYAML {
    pub maintainers: Vec<User>,
}
