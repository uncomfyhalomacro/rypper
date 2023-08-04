use quick_xml::{
    de::from_str,
    DeError,
};
use serde::{
    Deserialize,
    Serialize,
};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
/// Uses metalink+xml version 4 as defined in <https://datatracker.ietf.org/doc/html/rfc5854>
/// There are subtle differences differences with metalink version 3.0 but
/// not that much.
pub struct MetaLink
{
    #[serde(rename = "@xmlns")]
    xmlns: String,
    #[serde(rename = "generator")]
    generator: String,
    #[serde(rename = "origin")]
    origin: Origin,
    #[serde(rename = "published")]
    published: String,
    #[serde(rename = "publisher")]
    publisher: Publisher,
    #[serde(rename = "file")]
    file: File,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
struct Origin
{
    #[serde(rename = "@dynamic")]
    dynamic: bool,
    #[serde(rename = "$text")]
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
struct Publisher
{
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
struct File
{
    #[serde(rename = "@name")]
    name: String,
    size: usize,
    url: Vec<Url>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
struct Url
{
    #[serde(rename = "@location")]
    location: String,
    #[serde(rename = "@priority")]
    priority: usize,
    #[serde(rename = "$text")]
    text: String,
}

impl MetaLink
{
    pub fn from_file<P: AsRef<Path>>(p: P) -> Result<Self, DeError>
    {
        match std::fs::read_to_string(p).map(|content| from_str::<MetaLink>(&content))
        {
            Ok(c) => c,
            Err(err) =>
            {
                panic!("Error reading file for deserializing: {}", err);
            }
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn read_from_file()
    {
        let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let file_path = format!("{}/samples/valid-metalink4-file.xml", &manifest_path);
        assert_eq!(true, MetaLink::from_file(file_path).is_ok());
    }
}
