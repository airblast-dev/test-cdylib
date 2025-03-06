use crate::dependencies::{Dependency, Patch, RegistryPatch};
use cargo_metadata::{CrateType, Edition};
use serde::Serialize;
use std::collections::BTreeMap as Map;
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(Serialize, Debug)]
pub struct Manifest {
    pub package: Package,
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub features: Map<String, Vec<String>>,
    pub dependencies: Map<String, Dependency>,
    pub lib: Lib,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<Workspace>,
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub patch: Map<String, RegistryPatch>,
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub replace: Map<String, Patch>,
}

#[derive(Serialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub edition: Edition,
    pub publish: bool,
}

#[derive(Serialize, Debug)]
pub struct Lib {
    pub path: PathBuf,
    crate_type: [CrateType; 1],
}
impl Lib {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            crate_type: [CrateType::CDyLib],
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Name(pub String);

#[derive(Serialize, Debug)]
pub struct Config {
    pub build: Build,
}

#[derive(Serialize, Debug)]
pub struct Build {
    pub rustflags: Vec<&'static str>,
}

#[derive(Serialize, Debug)]
pub struct Workspace {}

impl AsRef<OsStr> for Name {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}
