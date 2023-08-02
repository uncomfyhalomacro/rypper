// SPDX-License-Identifier: MPL-2.0

use ini_core as ini;
use regex::Regex;
use std::{
    default::Default,
    error,
    fmt::{
        self,
        Display,
    },
    fs,
    path::{
        self,
        PathBuf,
    },
};

pub const ZYPP_CONFIG_PATH: &str = "/etc/zypp";
pub const ZYPP_REPO_PATH: &str = "/etc/zypp/repos.d";

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
/// # Config Validity
/// Minimal valid config only requires a section AND URI.
pub struct RepoConfig
{
    pub alias: Option<String>,
    pub autorefresh: Option<bool>,
    pub baseurl: Option<String>,
    pub enabled: Option<bool>,
    pub gpgcheck: Option<bool>,
    pub gpgkey: Option<String>,
    pub name: Option<String>,
    pub path: Option<PathBuf>,
    pub priority: Option<u32>,
    pub typemd: Option<String>,
}

#[allow(dead_code)]
enum RepoOptions
{
    AutoRefresh(bool),
    Baseurl(String),
    Enabled(bool),
    Name(String),
    Path(PathBuf),
    Priority(bool),
    Section(String),
    TypeMd(String),
}

#[derive(Debug, PartialEq)]
pub enum RepoConfigErrors
{
    /// Default ZYpper will error if a section is just `[]`, thus,
    /// `MissingAliasError`.
    MissingAliasError,
    /// There is no way to request for a file or mirror if there is no URI.
    MissingUriError,
    /// There is no way to read a non-existing config file.
    MissingConfigError,
    /// Invalid config files such as typos or missing brackets or even just an
    /// empty file.
    InvalidConfigError,
    /// A key that requires a URI string may contain a non-URI string.
    InvalidUriString,
}

impl error::Error for RepoConfigErrors {}

impl Display for RepoConfigErrors
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            RepoConfigErrors::MissingAliasError =>
            {
                write!(f, "A variant of RepoConfigError occured")
            }
            RepoConfigErrors::MissingUriError =>
            {
                write!(f, "A variant of RepoConfigError occured")
            }
            RepoConfigErrors::MissingConfigError =>
            {
                write!(f, "A variant of RepoConfigError occured")
            }
            RepoConfigErrors::InvalidConfigError =>
            {
                write!(f, "A variant of RepoConfigError occured")
            }
            RepoConfigErrors::InvalidUriString => write!(f, "A variant of RepoConfigError occured"),
        }
    }
}

impl Default for RepoConfig
{
    /// # NOTE
    /// ZYpper's behavior is to assume that the following parameters are the
    /// default Those with None aren't but are just placeholders since there
    /// is no way to define defaults for aliases and autorefresh
    fn default() -> Self
    {
        RepoConfig {
            alias: None,
            autorefresh: Some(false),
            baseurl: None,
            enabled: Some(true),
            gpgcheck: Some(true),
            gpgkey: None,
            name: None,
            path: None,
            priority: Some(99u32),
            typemd: None,
        }
    }
}

impl RepoConfig
{
    pub fn from(document: &str) -> Result<RepoConfig, RepoConfigErrors>
    {
        let document = ini::Parser::new(document.trim_start());
        RepoConfig::read_config(document)
    }

    pub fn load_config_file(p: &str) -> Result<Self, RepoConfigErrors>
    {
        let path_buf: PathBuf = path::PathBuf::from(p);
        validate_file_metadata(&path_buf)?;

        let conf = match fs::read_to_string(path_buf)
        {
            Ok(file) => file,
            Err(_) => return Err(RepoConfigErrors::InvalidConfigError),
        };
        let config = ini::Parser::new(&conf.trim_start());

        RepoConfig::read_config(config)
    }

    pub fn read_config(document: ini::Parser) -> Result<RepoConfig, RepoConfigErrors>
    {
        let mut repoconfig = RepoConfig::default();

        for item in document.skip(1)
        {
            match item
            {
                ini::Item::SectionEnd => break,
                ini::Item::Blank => continue,
                ini::Item::Comment(_) => continue,
                ini::Item::Error(err) =>
                {
                    eprintln!("Ini format error: {}", err);
                    return Err(RepoConfigErrors::InvalidConfigError);
                }
                ini::Item::Section(section) => match validate_alias(section)
                {
                    Ok(_) =>
                    {
                        repoconfig.alias = Some(section.to_string());
                    }
                    Err(err) => return Err(err),
                },
                ini::Item::Property(key, some_value) => match key
                {
                    "name" => repoconfig.name = some_value.map(|name| name.to_string()),
                    "gpgkey" => match validate_uri(some_value)
                    {
                        Ok(s) => repoconfig.gpgkey = Some(s.to_string()),
                        Err(err) => return Err(err),
                    },
                    "baseurl" => match validate_uri(some_value)
                    {
                        Ok(s) => repoconfig.baseurl = Some(s.to_string()),
                        Err(err) => return Err(err),
                    },
                    "autorefresh" =>
                    {
                        repoconfig.autorefresh = match some_value
                        {
                            Some(input_string) => match input_string.parse::<usize>()
                            {
                                Ok(parsed_number) =>
                                {
                                    if parsed_number >= 1
                                    {
                                        Some(true)
                                    }
                                    else
                                    {
                                        Some(false)
                                    }
                                }
                                Err(err) =>
                                {
                                    eprintln!("Trouble parsing string to type usize: {}", err);
                                    panic!()
                                }
                            },
                            None => Some(false),
                        }
                    }

                    "enabled" =>
                    {
                        repoconfig.enabled = match some_value
                        {
                            Some(input_string) => match input_string.parse::<usize>()
                            {
                                Ok(parsed_number) =>
                                {
                                    if parsed_number >= 1
                                    {
                                        Some(true)
                                    }
                                    else
                                    {
                                        Some(false)
                                    }
                                }
                                Err(err) =>
                                {
                                    eprintln!("Trouble parsing string to type usize: {}", err);
                                    panic!()
                                }
                            },
                            None => Some(true),
                        }
                    }
                    "gpgcheck" =>
                    {
                        repoconfig.gpgcheck = match some_value
                        {
                            Some(input_string) => match input_string.parse::<usize>()
                            {
                                Ok(parsed_number) =>
                                {
                                    if parsed_number >= 1
                                    {
                                        Some(true)
                                    }
                                    else
                                    {
                                        Some(false)
                                    }
                                }
                                Err(err) =>
                                {
                                    eprintln!("Trouble parsing string to type usize: {}", err);
                                    panic!()
                                }
                            },
                            None => Some(true),
                        }
                    }
                    "priority" =>
                    {
                        repoconfig.priority = match some_value
                        {
                            Some(input_string) => match input_string.parse::<u32>()
                            {
                                Ok(parsed_number) => Some(parsed_number),
                                Err(err) =>
                                {
                                    eprintln!("Trouble parsing string to u32: {}", err);
                                    panic!()
                                }
                            },
                            None => Some(99u32),
                        }
                    }
                    _ =>
                    {}
                },
            }
        }

        if repoconfig.alias.is_none()
        {
            eprintln!("Repository has no alias defined!");
            return Err(RepoConfigErrors::MissingAliasError);
        }
        if repoconfig.baseurl.is_none()
        {
            eprintln!("No URI found!");
            return Err(RepoConfigErrors::MissingUriError);
        }
        Ok(repoconfig)
    }

    /// TODO:
    /// The following layout should look like
    /// ```ini
    /// [alias]
    /// name
    /// autorefresh
    /// ```
    /// something like that
    pub fn _write_config() {}
}

fn validate_alias(alias: &str) -> Result<&str, RepoConfigErrors>
{
    match alias
    {
        "" => Err(RepoConfigErrors::MissingAliasError),
        _ => Ok(alias),
    }
}

/// Checking if a URI is valid is essential for
/// reading `baseurl` and `gpgkey` parameters.
///
/// # A valid URI should be like this:
/// ```
/// use rypper_reader::repoconfig;
///
/// let uri = Some("https://example.com");
/// match repoconfig::validate_uri(uri) {
///   Ok(c) => {
///     assert_eq!(c, "https://example.com");
///   }
///   Err(_) => {}
/// };
/// ```
pub fn validate_uri(uristring: Option<&str>) -> Result<&str, RepoConfigErrors>
{
    match uristring
    {
        Some(s) => match s
        {
            "" => Err(RepoConfigErrors::MissingUriError),
            _ =>
            {
                let uri_pattern =
                    Regex::new(r"^[a-zA-Z][a-zA-Z0-9+.-]*://[^\s/$.?#].[^\s]*$").unwrap();
                match uri_pattern.is_match(s)
                {
                    true => Ok(s),
                    false => Err(RepoConfigErrors::InvalidUriString),
                }
            }
        },
        None => Err(RepoConfigErrors::MissingUriError),
    }
}

pub fn validate_file_metadata(file_path: &PathBuf) -> Result<&PathBuf, RepoConfigErrors>
{
    let file_md = match std::fs::metadata(file_path)
    {
        Ok(c) => c,
        Err(err) =>
        {
            eprintln!("Error reading file: {}", err);
            return Err(RepoConfigErrors::InvalidConfigError);
        }
    };

    if file_md.len() == 0
    {
        eprintln!("File contains no content");
        return Err(RepoConfigErrors::InvalidConfigError);
    }
    Ok(file_path)
}

// TODO: Move tests to separate file or folder
#[cfg(test)]
mod tests
{

    use super::*;
    use std::env;

    // Test equality from config file with another config file and/or variable
    // declared config + default.
    #[test]
    fn equal_repoconfig() -> Result<(), RepoConfigErrors>
    {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let file_path = format!("{}/samples/home_uncomfyhalomacro.repo", manifest_dir);
        let another_file_path =
            format!("{}/samples/another_home_uncomfyhalomacro.repo", manifest_dir);
        let example_config = RepoConfig::load_config_file(&file_path)?;
        // This one contains a file which should have same key-values but of different
        // arrangements.
        let another_example_config = RepoConfig::load_config_file(&another_file_path)?;
        let config = RepoConfig {
            alias:  Some(String::from("home_uncomfyhalomacro")),
            autorefresh:  Some(true),
            baseurl:  Some(String::from("https://download.opensuse.org/repositories/home:/uncomfyhalomacro/openSUSE_Tumbleweed/")),
            enabled:  Some(true),
            gpgcheck:  Some(true),
            gpgkey:  Some(String::from("https://download.opensuse.org/repositories/home:/uncomfyhalomacro/openSUSE_Tumbleweed/repodata/repomd.xml.key")),
            priority:  Some(99u32),
            typemd:  None,
            .. Default::default()
        };
        assert_eq!(example_config, config);
        assert_eq!(another_example_config, example_config);
        Ok(())
    }

    // Test if it will panic with an invalid URI string gained from a config file.
    #[test]
    fn invalid_baseurl()
    {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let file_path = format!("{}/samples/invalid_baseurl.repo", manifest_dir);
        assert_eq!(true, RepoConfig::load_config_file(&file_path).is_err());
    }
    #[test]
    fn invalid_gpgkey_uri()
    {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let file_path = format!("{}/samples/invalid_gpgkey_uri.repo", manifest_dir);
        assert_eq!(true, RepoConfig::load_config_file(&file_path).is_err());
    }

    #[test]
    fn errors_without_baseurl()
    {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let file_path = format!("{}/samples/no_baseurl.repo", manifest_dir);
        assert_eq!(true, RepoConfig::load_config_file(&file_path).is_err());
    }

    #[test]
    fn works_with_only_section_and_baseurl() -> Result<(), RepoConfigErrors>
    {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let file_path = format!("{}/samples/only_section_and_baseurl.repo", manifest_dir);
        let config = RepoConfig::load_config_file(&file_path)?;
        assert_eq!(Some("a_section".to_string()), config.alias);
        assert_eq!(Some("https://example.com".to_string()), config.baseurl);
        Ok(())
    }

    #[test]
    fn works_with_only_section_and_baseurl_from_str() -> Result<(), RepoConfigErrors>
    {
        let document = r#"[section]
baseurl=https://example.com
"#;
        let config = RepoConfig::from(&document)?;
        println!("{:#?}", config);
        assert_eq!(Some("section".to_string()), config.alias);
        assert_eq!(Some("https://example.com".to_string()), config.baseurl);
        Ok(())
    }

    #[test]
    fn works_with_too_many_whitespaces_for_valid_config() -> Result<(), RepoConfigErrors>
    {
        let document = r#"





[section]
baseurl=https://example.com
"#;
        let config = RepoConfig::from(&document)?;
        println!("{:#?}", config);
        assert_eq!(Some("section".to_string()), config.alias);
        assert_eq!(Some("https://example.com".to_string()), config.baseurl);
        Ok(())
    }

    #[test]
    fn invalid_if_with_non_existent_file()
    {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let file_path = format!("{}/samples/dummy.repo", manifest_dir);
        assert_eq!(true, RepoConfig::load_config_file(&file_path).is_err());
        if let Err(err) = RepoConfig::load_config_file(&file_path)
        {
            assert_eq!(RepoConfigErrors::InvalidConfigError, err)
        }
    }

    #[test]
    fn invalid_if_no_section()
    {
        let document = "baseurl=https://example.com";
        let config = RepoConfig::from(&document);
        assert_eq!(true, config.is_err());
    }

    #[test]
    fn invalid_if_empty_str()
    {
        let document = "";
        let config = RepoConfig::from(&document);
        assert_eq!(true, config.is_err());
    }

    #[test]
    fn invalid_if_empty_config()
    {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let file_path = format!("{}/samples/empty_config.repo", manifest_dir);
        assert_eq!(true, RepoConfig::load_config_file(&file_path).is_err());
    }
}
