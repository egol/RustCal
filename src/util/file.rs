// STD Dependencies -----------------------------------------------------------
use std::collections::HashMap;
#[cfg(not(target_family = "wasm"))]
use std::env;
#[cfg(not(target_family = "wasm"))]
use std::fs;
#[cfg(not(target_family = "wasm"))]
use std::fs::File;
#[cfg(not(target_family = "wasm"))]
use std::io::prelude::*;
#[cfg(not(target_family = "wasm"))]
use std::path::PathBuf;

// External Dependencies ------------------------------------------------------
use chrono::prelude::*;
#[cfg(target_family = "wasm")]
use web_sys::window;

// Internal dependencies ------------------------------------------------------
use crate::util::textevent::*;

#[cfg(not(target_family = "wasm"))]
const DATA_DIR_ENV: &str = "RUSTCAL_DATA_DIR";
const DISABLE_SAVE_ENV: &str = "RUSTCAL_DISABLE_SAVE";
#[cfg(target_family = "wasm")]
const STORAGE_KEY: &str = "rustcal.events";

#[cfg(not(target_family = "wasm"))]
fn data_dir() -> PathBuf {
    env::var_os(DATA_DIR_ENV)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

#[cfg(not(target_family = "wasm"))]
fn data_file_path(file_name: &str) -> PathBuf {
    data_dir().join(file_name)
}

#[cfg(not(target_family = "wasm"))]
fn ensure_parent_exists(path: &PathBuf) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn log_file_path() -> PathBuf {
    let path = data_file_path("app.log");
    ensure_parent_exists(&path);
    path
}

// TODO: allow for save file naming and directory selection
// TODO: allow for configuring time based auto save?
/// Writes storage hash map to json
/// responsible for saving user data
pub fn save_data(s: &HashMap<NaiveDate, Vec<TextEvent>>) {
    if std::env::var_os(DISABLE_SAVE_ENV).is_some() {
        return;
    }

    let j = serde_json::to_string(&s).expect("could not serialize hashmap");

    #[cfg(target_family = "wasm")]
    {
        if let Some(storage) = window()
            .and_then(|value| value.local_storage().ok().flatten())
        {
            let _ = storage.set_item(STORAGE_KEY, &j);
        }
        return;
    }

    #[cfg(not(target_family = "wasm"))]
    {
    let path = data_file_path("test.json");
    ensure_parent_exists(&path);
    fs::write(path, j).expect("Unable to write file");
    }
}

// Converts json save file back into a hashmap
// that is used to load previous data
pub fn read() -> HashMap<NaiveDate, Vec<TextEvent>> {
    #[cfg(target_family = "wasm")]
    {
        return window()
            .and_then(|value| value.local_storage().ok().flatten())
            .and_then(|storage| storage.get_item(STORAGE_KEY).ok().flatten())
            .and_then(|contents| {
                serde_json::from_str::<HashMap<NaiveDate, Vec<TextEvent>>>(&contents).ok()
            })
            .unwrap_or_default();
    }

    #[cfg(not(target_family = "wasm"))]
    {
    let path = data_file_path("test.json");

    if let Ok(mut f) = File::open(path) {
        let mut contents = String::new();
        if f.read_to_string(&mut contents).is_err() {
            return HashMap::new();
        }
        if let Ok(p) = serde_json::from_str::<HashMap<NaiveDate, Vec<TextEvent>>>(&contents) {
            return p;
        } else {
            return HashMap::new();
        }
    } else {
        HashMap::new()
    }
    }
}
