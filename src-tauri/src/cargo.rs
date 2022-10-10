use std::path::PathBuf;

use anyhow::Result;
use cargo_metadata::{Metadata, MetadataCommand};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct Target {
    pub(crate) name: String,
    pub(crate) src_path: String,
    pub(crate) kind: Vec<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct Package {
    pub(crate) name: String,
    pub(crate) targets: Vec<Target>,
}

#[derive(Debug)]
pub(crate) struct CargoReader {
    metadata: Metadata,
}

impl CargoReader {
    pub(crate) fn open<P: Into<PathBuf>>(manifest_path: P) -> Result<Self> {
        Ok(Self {
            metadata: MetadataCommand::new().manifest_path(manifest_path).exec()?,
        })
    }

    pub(crate) fn list_packages(&self) -> Result<Vec<Package>> {
        let packages = self
            .metadata
            .workspace_packages()
            .into_iter()
            .map(|package| {
                let targets = package
                    .targets
                    .iter()
                    .map(|target| Target {
                        name: target.name.clone(),
                        src_path: target.src_path.to_string(),
                        kind: target.kind.clone(),
                    })
                    .collect();

                let name = package.name.clone();

                Package { name, targets }
            })
            .collect();

        Ok(packages)
    }
}
