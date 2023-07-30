use md5::Md5;
use sha1::Sha1;
use sha2::{
    Sha256,
    Sha512,
};
use std::sync::Arc;
use url::Url;

// We don't really care about this but for uniformity reasons
pub struct Publisher
{
    name: String,
    url: Url,
}

// We really care about this because lol
pub struct FileData
{
    name: String,
    origin: Url,
    hash_md5: Md5,
    hash_sha1: Sha1,
    hash_sha256: Sha256,
    hash_sha512: Sha512,
    pieces: Arc<Sha1>,
    mirrors: Location,
}

pub struct Location
{
    // I may use https://github.com/sifton/isocountry-rs/blob/master/src/lib.rs for this but eh not now
    location: String,
    priority: u8,
    url: Url,
}

// This is the struct that we want to construct
pub struct MetaLinkData
{
    pub generator: String,
    pub published: String,
    pub publisher: Publisher,
    pub file: FileData,
}

impl MetaLinkData {}

impl Location {}
impl FileData {}
impl Publisher {}
