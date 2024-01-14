//! Automatic config files

use std::{fs, io::Write, path::PathBuf};

use jsonc_parser::ParseOptions;
use serde::{de::DeserializeOwned, Serialize};

pub use ccanvas_config_derive::CcanvasConfig;

/// A trait for loading ccanvas config files.
///
/// The crate name of your component should be in format:
/// - `ccanvas-COMPONENTNAME` for single component apps.
/// - `ccanvas-COMPONENTNAME-SUBCOMPONENTNAME-...` for multicomponet apps.
pub trait CcanvasConfig: Default + Serialize + DeserializeOwned {
    /// Name of the top level crate.
    const CNAME: &'static str;

    /// Load config file content.
    ///
    /// Will try to load a config file at
    /// `~/.config/ccanvas/COMPONENTNAME/.../SUBCOMPONENTNAME.jsonc`.
    /// - Create all required folders and create a default config if a config file is not found.
    /// - Backup a config file with error message, and generate a default config if parsing failed.
    /// - Will always return an instance of Self, never panics.
    fn load() -> Self {
        let name = Self::CNAME.split('-').collect::<Vec<_>>();

        let path = match dirs::config_dir() {
            Some(dir) if name.len() > 1 && name[0] == "ccanvas" => dir
                .join(PathBuf::from_iter(name.iter()))
                .with_extension("jsonc"),
            _ => return Self::default(),
        };

        let parent = path.parent().unwrap();
        if !parent.exists() && fs::create_dir_all(parent).is_err() {
            return Self::default();
        }

        if path.exists() {
            let content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(_) => return Self::default(),
            };

            let error = match jsonc_parser::parse_to_serde_value(&content, &ParseOptions::default())
            {
                Ok(Some(value)) => match serde_json::from_value(value) {
                    Ok(parsed) => return parsed,
                    Err(e) => e.to_string(),
                },
                Ok(None) => "unspecified error".to_string(),
                Err(e) => e.to_string(),
            };

            let mut id: u32 = 1;
            let backup = loop {
                let path = path.with_file_name(format!("{}_{id}.jsonc", name.last().unwrap()));
                if !path.exists() {
                    break path;
                }
                id += 1;
            };

            if let Ok(mut file) = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(backup)
            {
                let _ = file.write_all(format!("// {error}\n// -----\n{content}").as_bytes());
            }
        }

        let default = Self::default();

        if let Ok(mut file) = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
        {
            serde_json::to_vec(&default).unwrap();
            let to_write = match serde_json::to_vec_pretty(&default) {
                Ok(bytes) => bytes,
                Err(_) => return default,
            };
            let _ = file.write_all(&to_write);
        }

        default
    }
}
