#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;

use anyhow::{anyhow, Result};
use cargo::{
    core::{PackageId, Workspace},
    ops::{CompileFilter, CompileOptions, FilterRule, LibRule, Packages},
    util::command_prelude::CompileMode,
    Config,
};
use serde::Serialize;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_packages, run_example])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn list_packages(manifest_path: String) -> Result<Vec<Package>, String> {
    list_packages_internal(manifest_path).map_err(|err| format!("{}", err))
}

#[tauri::command]
fn run_example(manifest_path: String, example_name: String) -> Result<(), String> {
    run_example_internal(manifest_path, example_name).map_err(|err| format!("{}", err))
}

#[derive(Debug, Serialize)]
struct Target {
    name: String,
    src_path: String,
    kind: &'static str,
}

#[derive(Debug, Serialize)]
struct Package {
    id: PackageId,
    name: String,
    version: String,
    manifest_path: String,
    targets: Vec<Target>,
}

fn run_example_internal<P: Into<PathBuf>>(manifest_path: P, example_name: String) -> Result<()> {
    let config = Config::default()?;
    let ws = Workspace::new(&manifest_path.into(), &config)?;
    let package = ws
        .members()
        .find(|pkg| {
            pkg.targets()
                .iter()
                .any(|t| t.is_example() && t.name() == example_name)
        })
        .ok_or_else(|| anyhow!("Example could not be located."))?;

    let mut options = CompileOptions::new(ws.config(), CompileMode::Build)?;
    options.spec = Packages::Packages(vec![package.name().to_string()]);
    options.filter = CompileFilter::Only {
        all_targets: false,
        lib: LibRule::False,
        bins: FilterRule::Just(vec![]),
        examples: FilterRule::Just(vec![example_name.to_string()]),
        tests: FilterRule::Just(vec![]),
        benches: FilterRule::Just(vec![]),
    };

    cargo::ops::run(&ws, &options, &[])?;

    Ok(())
}

fn list_packages_internal<P: Into<PathBuf>>(manifest_path: P) -> Result<Vec<Package>> {
    let config = Config::default()?;
    let ws = Workspace::new(&manifest_path.into(), &config)?;

    let packages = ws.members().map(|pkg| {
        let id = pkg.package_id();
        let name = pkg.name().to_string();
        let version = pkg.version().to_string();
        let manifest_path = pkg
            .manifest_path()
            .to_str()
            .map(ToString::to_string)
            .unwrap_or_default();
        let targets = pkg
            .targets()
            .iter()
            .map(|t| Target {
                name: t.name().to_string(),
                src_path: t
                    .src_path()
                    .path()
                    .and_then(|p| p.to_str().map(ToString::to_string))
                    .unwrap_or_default(),
                kind: t.kind().description(),
            })
            .collect();

        Package {
            id,
            name,
            version,
            manifest_path,
            targets,
        }
    });

    Ok(packages.collect())
}
