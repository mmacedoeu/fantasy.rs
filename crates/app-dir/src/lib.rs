//! * App-dir
//! This is the directories library.
//! It's responsability is to manage where to load and save
//! App data on diferent environments and Operating Systems
//! It brings the ability to load configuration stored on a 
//! mounted drive shared across a cluster of machines

extern crate app_dirs;

use app_dirs::{get_app_root, AppDataType, AppInfo};
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::{env, fs};

const AUTHOR: &'static str = "mmacedoeu";
const PRODUCT: &'static str = "fantasy";

/// Structure to hold the base path and sub-folders
#[derive(Debug, PartialEq)]
pub struct Directories {
    pub base: String,
    pub db: String,
}

impl Default for Directories {
    fn default() -> Self {
        let data_dir = default_data_path();
        let base = replace_home(&data_dir, "$BASE");
        Directories {
            db: db_root_path(&base).into_string().unwrap(),
            base: base,
        }
    }
}

impl Directories {
    /// Used to initialize creating missing folders
    pub fn create_dirs(&self) -> Result<(), String> {
        fs::create_dir_all(&self.base).map_err(|e| e.to_string())?;
        Ok(())
    }
}

/// Standard data path according to current environment
pub fn default_data_path() -> String {
    let app_info = AppInfo {
        name: PRODUCT,
        author: AUTHOR,
    };
    get_app_root(AppDataType::UserData, &app_info)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "$HOME/.fantasy".to_owned())
}

/// Replaces `$HOME` str with home directory path.
pub fn replace_home(base: &str, arg: &str) -> String {
    // the $HOME directory on mac os should be `~/Library` or `~/Library/Application Support`
    #![allow(deprecated)]
    let r = arg.replace("$HOME", env::home_dir().unwrap().to_str().unwrap());
    let r = r.replace("$BASE", base);
    r.replace("/", &::std::path::MAIN_SEPARATOR.to_string())
}

/// Database storage path relative with base
pub fn db_root_path<B: AsRef<OsStr> + Sized>(base: B) -> OsString {
    let mut dir = Path::new(&base).to_path_buf();
    dir.push("db");
    dir.into_os_string()
}

pub fn get_base_file<B, F>(base: B, file_name: F) -> OsString
where
    B: AsRef<OsStr> + Sized,
    F: AsRef<OsStr>,
{
    let mut dir = Path::new(&base).to_path_buf();
    dir.set_file_name(file_name);
    dir.into_os_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
