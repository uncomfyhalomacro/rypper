use md5::Md5;
use sha1::Sha1;
use sha2::{
    Sha256,
    Sha512,
};
use std::sync::Arc;
use url::Url;

pub struct MirrorList {
    pub mirrors: Arc<Mirror>
}
// We don't really care about this but for uniformity reasons
pub struct Publisher
{
    pub name: String,
    pub url: Url,
}

// We really care about this because lol
pub struct FileData
{
    pub name: String,
    pub origin: Url,
    pub hash_md5: Md5,
    pub hash_sha1: Sha1,
    pub hash_sha256: Sha256,
    pub hash_sha512: Sha512,
    pub pieces: Arc<Sha1>,
    pub mirrorlist: MirrorList
}

pub struct Mirror
{
    // I may use https://github.com/sifton/isocountry-rs/blob/master/src/lib.rs for this but eh not now
    pub location: String,
    pub priority: u8,
    pub url: Url,
}

// This is the struct that we want to construct
pub struct MetaLinkData
{
    pub generator: String,
    pub published: String,
    pub publisher: Publisher,
    pub file: FileData,
}

impl FileData {}
impl MetaLinkData {}
impl Mirror {}
impl MirrorList {}
impl Publisher {}





