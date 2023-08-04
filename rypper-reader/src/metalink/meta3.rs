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
pub struct MetaLink
{
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "@xmlns")]
    xmlns: String,
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "@generator")]
    generator: String,
    #[serde(rename = "@pubdate")]
    pubdate: String,
    #[serde(rename = "@origin")]
    origin: String,
    #[serde(rename = "publisher")]
    publisher: Publisher,
    #[serde(rename = "files")]
    files: Files,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
struct Publisher
{
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "url")]
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
struct Files
{
    file: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
struct File
{
    #[serde(rename = "@name")]
    name: String,
    size: usize,
    #[serde(rename = "resources")]
    resources: Resources,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
struct Resources
{
    url: Vec<Url>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
struct Url
{
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "@location")]
    location: String,
    #[serde(rename = "@preference")]
    preference: usize,
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
        let file_path = format!("{}/samples/valid-metalink3-file.xml", &manifest_path);
        let read_file = MetaLink::from_file(file_path);
        println!("{:#?}", read_file);
        assert_eq!(true, read_file.is_ok());
    }
}
