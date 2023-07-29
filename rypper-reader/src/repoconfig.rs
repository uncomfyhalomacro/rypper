// SPDX-License-Identifier: MPL-2.0

use ini_core as ini;
use regex::Regex;
use std::{
    fs,
    path::{self, PathBuf},
};

pub const ZYPP_CONFIG_PATH: &'static str = "/etc/zypp";
pub const ZYPP_REPO_PATH: &'static str = "/etc/zypp/repos.d";

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RepoConfig {
    pub autorefresh: Option<bool>,
    pub baseurl: Option<String>,
    pub enabled: Option<bool>,
    pub gpgcheck: Option<bool>,
    pub name: Option<String>,
    pub path: Option<PathBuf>,
    pub priority: Option<u8>,
    pub alias: Option<String>,
    pub gpgkey: Option<String>,
    pub typemd: Option<String>,
}

// TODO: When to use these? Worry later.
// enum RepoOptions {
//     Section(String),
//     AutoRefresh(bool),
//     BaseUrl(String),
//     Enabled(bool),
//     Name(String),
//     Priority(bool),
//     TypeMd(String),
//     Path(PathBuf),
// }

#[derive(Debug)]
pub enum RepoConfigErrors {
    AliasError,
    KeyValueError,
    MissingConfigError,
}

impl RepoConfig {
    
    // TODO: make this receive parameters.
    pub fn new() -> Self {
        RepoConfig {
            alias: Some(String::new()),
            autorefresh: Some(false),
            baseurl: Some(String::new()),
            enabled: Some(false),
            name: Some(String::new()),
            gpgcheck: Some(true),
            gpgkey: Some(String::new()),
            priority: Some(99_u8),
            typemd: None,
            path: None,
        }
    }
    pub fn load_from_file(p: &str) -> Self {
        let path_buf: PathBuf = path::PathBuf::from(p);
        let conf = match fs::read_to_string(path_buf) {
            Ok(c) => c,
            Err(err) => {
                eprintln!("Error reading file to string: {}", err);
                panic!()
            }
        };
        // TODO: Replace this with something else. Change this into something that holds a Result type?
        let mut repoconfig = RepoConfig::new();
        let read_config = ini::Parser::new(&conf);
        for item in read_config {
            // There should only be one section. HOW?
            match item {
                ini::Item::Section(section) => {
                    repoconfig.alias = match section {
                        "" => {
                            eprintln!(
                                r#"
Repository has no alias defined!
Please file a bug report about this.
See http://en.opensuse.org/Zypper/Troubleshooting for instructions."#
                            );
                            // TODO: Improve error handling using Result types across the codebase. Panic is discouraged :D
                            panic!()
                        }
                        _ => Some(section.to_string()),
                    }
                }
                ini::Item::Property(key, value) => {
                    match key {
                        "name" => {
                            repoconfig.name = match value {
                                Some(name) => Some(name.to_string()),
                                None => None,
                            }
                        }
                        "autorefresh" => {
                            repoconfig.autorefresh = match value {
                                Some(input_string) => match input_string.parse::<u8>() {
                                    Ok(parsed_number) => {
                                        if parsed_number == 1 {
                                            Some(true)
                                        } else if parsed_number == 0 {
                                            Some(false)
                                        } else {
                                            eprintln!("autorefresh value not 1 or 0");
                                            panic!()
                                        }
                                    }
                                    Err(err) => {
                                        eprintln!("Cannot parse string to type u8: {}", err);
                                        panic!()
                                    }
                                },
                                None => Some(false),
                            }
                        }
                        "enabled" => {
                            repoconfig.enabled = match value {
                                Some(input_string) => match input_string.parse::<u8>() {
                                    Ok(parsed_number) => {
                                        if parsed_number == 1 {
                                            Some(true)
                                        } else if parsed_number == 0 {
                                            Some(false)
                                        } else {
                                            eprintln!("enabled value not 1 or 0");
                                            panic!()
                                        }
                                    }
                                    Err(err) => {
                                        eprintln!("Cannot parse string to type u8: {}", err);
                                        panic!()
                                    }
                                },
                                None => Some(true),
                            }
                        }
                        "gpgcheck" => {
                            repoconfig.gpgcheck = match value {
                                Some(input_string) => match input_string.parse::<u8>() {
                                    Ok(parsed_number) => {
                                        if parsed_number == 1 {
                                            Some(true)
                                        } else if parsed_number == 0 {
                                            Some(false)
                                        } else {
                                            eprintln!("gpgcheck value not 1 or 0");
                                            panic!()
                                        }
                                    }
                                    Err(err) => {
                                        eprintln!("Cannot parse string to type u8: {}", err);
                                        panic!()
                                    }
                                },
                                None => Some(true),
                            }
                        }
                        "priority" => {
                            repoconfig.priority = match value {
                                Some(input_string) => match input_string.parse::<u8>() {
                                    Ok(parsed_number) => {
                                        if parsed_number <= 99_u8 {
                                            Some(parsed_number)
                                        } else {
                                            eprintln!("Value not between 99 and 0");
                                            panic!()
                                        }
                                    }
                                    Err(err) => {
                                        eprintln!("Cannot parse string to type u8: {}", err);
                                        panic!()
                                    }
                                },
                                None => Some(99_u8),
                            }
                        }
                        // If this key-value does not have a value, raise error and panic.
                        // If duplicated key-value pairs exist, use latest.
                        "baseurl" => {
                            repoconfig.baseurl = match value {
                                Some(url) => match is_valid_url(url) {
                                    true => Some(url.to_string()),
                                    false => {
                                        eprintln!("String not valid URL: {}", url);
                                        panic!()
                                    }
                                },
                                None => {
                                    eprintln!("No URL for repository");
                                    panic!()
                                }
                            }
                        }
                        "gpgkey" => {
                            repoconfig.gpgkey = match value {
                                Some(url) => match is_valid_url(url) {
                                    true => Some(url.to_string()),
                                    false => {
                                        eprintln!("String not valid URL: {}", url);
                                        panic!()
                                    }
                                },
                                None => {
                                    eprintln!("No URL for repository");
                                    panic!()
                                }
                            }
                        }
                        // "type" => todo!(),
                        // "path" => todo!(),
                        // "name" => todo!(),
                        _ => {
                            // TODO: maybe change println! to a logger function?
                            println!("Ignoring unknown key `{}` with value: {:?}", key, value);
                        }
                    }
                }

                ini::Item::Error(err) => {
                    eprintln!("Error reading configuration. Failed to parse: {}", err);
                    panic!()
                }
                _ => {} // Skip comments and blanks
            }
        }
        // No other hacky way to do this.
        if repoconfig.name == Some("".to_string()) {
            repoconfig.name = repoconfig.alias.clone();
        }
        repoconfig
    }
    // TODO: Finish this write_config function.
    pub fn _write_config() {}
}

// TODO: Move tests to separate file or folder
#[cfg(test)]
mod tests {
    use super::RepoConfig;
    use std::env;

    #[test]
    fn read_repoconfig() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let file_path = format!("{}/samples/home_uncomfyhalomacro.repo", manifest_dir);
        let example_config = RepoConfig::load_from_file(&file_path);
        let mut config = RepoConfig::new();
        config.alias = Some(String::from("home_uncomfyhalomacro"));
        config.autorefresh = Some(true);
        config.baseurl = Some(String::from("https://download.opensuse.org/repositories/home:/uncomfyhalomacro/openSUSE_Tumbleweed/"));
        config.enabled = Some(true);
        config.gpgcheck = Some(true);
        config.gpgkey = Some(String::from("https://download.opensuse.org/repositories/home:/uncomfyhalomacro/openSUSE_Tumbleweed/repodata/repomd.xml.key"));
        config.name = config.alias.clone();
        config.path = None;
        config.priority = Some(99_u8);
        config.typemd = None;
        assert_eq!(example_config, config);
    }
}

// TODO: move functions below to rypper-utils?
fn is_valid_url(input: &str) -> bool {
    // Define a regular expression pattern to match URLs
    let url_pattern = Regex::new(r"^(https?|ftp)://[^\s/$.?#].[^\s]*$").unwrap();
    url_pattern.is_match(input)
}
