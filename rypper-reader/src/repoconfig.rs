use ini::Ini;
use std::path::{Path, PathBuf};

#[derive(Debug)]
// NOTE: Maybe rearrange the struct fields by moving them into categories such as `RepoDescription` and `RepoOptions`. Also use `Option` instead of using empty
// strings. Remove `Default` after.
pub struct RepoConfig {
    pub autorefresh: bool,
    pub baseurl: String,
    pub enabled: bool,
    pub gpgcheck: bool,
    pub name: String,
    pub priority: u8,
    pub type_md: String,
    pub path: PathBuf,
}

impl Default for RepoConfig {
    fn default() -> RepoConfig {
        RepoConfig {
            autorefresh: false,
            baseurl: String::new(),
            enabled: false,
            gpgcheck: false,
            name: String::new(),
            priority: 99_u8,
            type_md: String::new(),
            path: PathBuf::from("/"),
        }
    }
}

/*
REDO: Refactor to allow `Option`.
TODO: Write your own ini parser. This crate sucks.
*/
impl RepoConfig {
    pub fn load_from_file(p: &Path) -> RepoConfig {
        let conf = Ini::load_from_file(p).unwrap();
        let mut properties: &ini::Properties = &ini::Properties::new();
        for (sec, prop) in conf.iter() {
            if sec.is_some() {
                properties = prop
            }
        }

        // Set default values. Ugly but Ugly
        let mut default = RepoConfig::default();
        for (key, value) in properties.iter() {
            match key {
                "autorefresh" => {
                    default.autorefresh = value.parse::<u8>().unwrap() != 0;
                }
                "enabled" => {
                    default.enabled = value.parse::<u8>().unwrap() != 0;
                }
                "gpgcheck" => {
                    default.gpgcheck = value.parse::<u8>().unwrap() != 0;
                }
                "priority" => {
                    let check_priority = value.parse::<u8>().unwrap();
                    default.priority = match check_priority {
                        1..=99 => check_priority,
                        _ => panic!("Key {} has value {}. That's not between 1-99", key, value),
                    }
                }
                "baseurl" => {
                    default.baseurl = value.to_string();
                }
                "name" => {
                    default.name = value.to_string();
                }
                "type" => {
                    default.type_md = value.to_string();
                }
                "path" => {
                    default.path = std::path::PathBuf::from(value);
                }
                _ => {
                    println!("Ignoring key {} with value {}", key, value);
                }
            }
        }
        default
    }
    // TODO: Finish this write_config function.
    pub fn _write_config() {}
}

// TODO: either rypper-core or rypper-utils will do the web requests
