use alloc::string::String;

#[repr(transparent)]
pub struct Path {
    inner: str,
}

impl Path {
    pub fn new(s: &str) -> &Path {
        unsafe { &*(s as *const str as *const Path) }
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathBuf {
    inner: String,
}

impl PathBuf {
    pub fn new() -> Self {
        Self {
            inner: String::new(),
        }
    }

    pub fn from<S: AsRef<Path>>(path: S) -> Self {
        Self {
            inner: String::from(path.as_ref().as_str()),
        }
    }

    pub fn as_path(&self) -> &Path {
        Path::new(&self.inner)
    }

    pub fn into_string(self) -> String {
        self.inner
    }
}

impl AsRef<Path> for Path {
    fn as_ref(&self) -> &Path {
        self
    }
}

impl AsRef<Path> for PathBuf {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl AsRef<Path> for str {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

impl AsRef<Path> for String {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}
