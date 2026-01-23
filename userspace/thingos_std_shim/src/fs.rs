use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

use crate::io::{Error, ErrorKind, Result};
use crate::path::Path;

static FILES: Mutex<BTreeMap<String, Vec<u8>>> = Mutex::new(BTreeMap::new());

#[derive(Clone, Debug)]
pub struct Metadata {
    len: u64,
}

impl Metadata {
    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn is_file(&self) -> bool {
        true
    }

    pub fn is_dir(&self) -> bool {
        false
    }
}

pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<()> {
    let path = String::from(path.as_ref().as_str());
    let mut files = FILES.lock();
    files.insert(path, contents.as_ref().to_vec());
    Ok(())
}

pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref().as_str();
    let files = FILES.lock();
    let data = files
        .get(path)
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "file not found"))?;
    let text = core::str::from_utf8(data)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "file is not valid UTF-8"))?;
    Ok(String::from(text))
}

pub fn metadata<P: AsRef<Path>>(path: P) -> Result<Metadata> {
    let path = path.as_ref().as_str();
    let files = FILES.lock();
    let data = files
        .get(path)
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "file not found"))?;
    Ok(Metadata {
        len: data.len() as u64,
    })
}
