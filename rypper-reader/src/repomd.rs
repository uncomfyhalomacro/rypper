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
#[serde(rename_all = "kebab-case")]
pub struct RepoMd
{
    #[serde(rename = "@xmlns")]
    xmlns: String,
    #[serde(rename = "@xmlns:rpm")]
    xmlns_rpm: String,
    revision: usize,
    #[serde(rename = "tags")]
    tags: Tags,
    #[serde(rename = "data")]
    data: Vec<Data>,
}

impl RepoMd
{
    pub fn from_file<P>(p: P) -> Result<Self, DeError>
    where
        P: AsRef<Path>,
    {
        match std::fs::read_to_string(p).map(|content| from_str::<RepoMd>(&content))
        {
            Ok(c) => c,
            Err(err) =>
            {
                panic!("Error reading file for deserializing: {}", err);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct Tags
{
    content: Vec<Content>,
    repo: String,
    distro: Distro,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct Distro
{
    #[serde(rename = "@cpeid")]
    cpeid: String,
    #[serde(rename = "$text")]
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct Content
{
    #[serde(rename = "$text")]
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct Data
{
    #[serde(rename = "@type")]
    r#type: String,
    checksum: CheckSum,
    #[serde(rename = "open-checksum")]
    open_checksum: OpenCheckSum,
    location: Location,
    timestamp: usize,
    size: usize,
    #[serde(rename = "open-size")]
    open_size: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct CheckSum
{
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "$text")]
    text: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct OpenCheckSum
{
    #[serde(rename = "@type")]
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct Location
{
    #[serde(rename = "@href")]
    href: String,
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn read_from_file()
    {
        let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let file_path = format!("{}/samples/valid-repomd-file.xml", &manifest_path);
        assert_eq!(true, RepoMd::from_file(file_path).is_ok());
    }

    #[test]
    #[should_panic]
    fn panic_at_non_existent_file()
    {
        let somefile = "";
        let _ = RepoMd::from_file(&somefile);
    }
}
