use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::PathBuf;
use abi::SymbolId;
use kernel_core::symbols::store::{SymbolStore, SymbolStoreError};

pub struct FileSymbolStore {
    path: PathBuf,
}

impl FileSymbolStore {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

const MAGIC: &[u8] = b"TSYM\0\0\0\x01";

impl SymbolStore for FileSymbolStore {
    fn load(&mut self) -> Result<Vec<(SymbolId, Vec<u8>)>, SymbolStoreError> {
        if !self.path.exists() {
            // If file doesn't exist, return empty list (not error) so we can seed fresh
            return Ok(Vec::new());
        }

        let mut file = File::open(&self.path).map_err(|_| SymbolStoreError::Io)?;
        
        let mut magic = [0u8; 8];
        if file.read_exact(&mut magic).is_err() || magic != MAGIC {
            // If file is empty or magic wrong, assume empty/corrupt?
            // If completely empty, maybe fine. If magic mismatch, Corrupt.
            // For v0 simplicity, if read fails or magic mismatch, treat as empty or error.
            // Let's treat valid file with different magic as Corrupt.
            // But if file is 0 bytes?
            if file.metadata().map(|m| m.len()).unwrap_or(0) == 0 {
                 return Ok(Vec::new());
            }
            return Err(SymbolStoreError::Corrupt);
        }

        let mut entries = Vec::new();
        let mut len_buf = [0u8; 4];
        let mut id_buf = [0u8; 8];

        loop {
            // Read ID
            match file.read_exact(&mut id_buf) {
                Ok(_) => {},
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(_) => return Err(SymbolStoreError::Io),
            }
            let id = u64::from_le_bytes(id_buf);

            // Read Len
            file.read_exact(&mut len_buf).map_err(|_| SymbolStoreError::Io)?;
            let len = u32::from_le_bytes(len_buf) as usize;

            // Read Bytes
            let mut bytes = vec![0u8; len];
            file.read_exact(&mut bytes).map_err(|_| SymbolStoreError::Io)?;

            entries.push((SymbolId(id), bytes));
        }

        Ok(entries)
    }

    fn save(&mut self, entries: &[(SymbolId, &[u8])]) -> Result<(), SymbolStoreError> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent).map_err(|_| SymbolStoreError::Io)?;
        }

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
            .map_err(|_| SymbolStoreError::Io)?;

        file.write_all(MAGIC).map_err(|_| SymbolStoreError::Io)?;

        for (id, bytes) in entries {
            file.write_all(&id.0.to_le_bytes()).map_err(|_| SymbolStoreError::Io)?;
            file.write_all(&(bytes.len() as u32).to_le_bytes()).map_err(|_| SymbolStoreError::Io)?;
            file.write_all(*bytes).map_err(|_| SymbolStoreError::Io)?;
        }

        Ok(())
    }
}
