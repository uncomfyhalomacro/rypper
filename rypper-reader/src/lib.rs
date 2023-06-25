use ini::Ini;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug)]
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
        let autorefresh: bool = false;
        let enabled: bool = false;
        let name: String = String::new();
        let gpgcheck: bool = false;
        let priority: u8 = 99_u8;
        let baseurl: String = String::new();
        let type_md = String::new();
        let path = PathBuf::from("/");
        RepoConfig { autorefresh, enabled, name, gpgcheck, priority, baseurl, type_md, path }
    }
}

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
        let mut autorefresh: bool = false;
        let mut enabled: bool = false;
        let mut name: String = String::new();
        let mut gpgcheck: bool = false;
        let mut priority: u8 = 99_u8;
        let mut baseurl: String = String::new();
        let mut type_md = String::new();
        let mut path = PathBuf::from("/");
        for (key, value) in properties.iter() {
            match key {
                "autorefresh" => {
                    autorefresh = value.parse::<u8>().unwrap() != 0;
                }
                "enabled" => {
                    enabled = value.parse::<u8>().unwrap() != 0;
                }
                "gpgcheck" => {
                    gpgcheck = value.parse::<u8>().unwrap() != 0;
                }
                "priority" => {
                    let check_priority = value.parse::<u8>().unwrap();
                    priority = match check_priority {
                        1..=99 => check_priority,
                        _ => panic!("Key {} has value {}. That's not between 1-99", key, value),
                    }
                }
                "baseurl" => {
                    baseurl = value.to_string();
                }
                "name" => {
                    name = value.to_string();
                }
                "type" => {
                    type_md = value.to_string();
                }
                "path" => {
                    path = std::path::PathBuf::from(value);
                }
                _ => {
                    println!("Ignoring key {} with value {}", key, value);
                }
            }
        }
        RepoConfig { autorefresh, enabled, name, gpgcheck, priority, baseurl, type_md, path }
    }
    /* TODO: Use builder pattern to fix
    https://rust-lang.github.io/rust-clippy/master/index.html#/too_many_arguments */
    pub fn new(
        autorefresh: bool,
        baseurl: String,
        enabled: bool,
        gpgcheck: bool,
        name: String,
        priority: u8,
        type_md: String,
        path: PathBuf,
    ) -> Self {
        Self { autorefresh, baseurl, enabled, gpgcheck, name, priority, type_md, path }
    }
}
