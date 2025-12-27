// Stub for ISO9660 reader
// Currently disabled due to AHCI driver issues and model refactoring.

pub struct Iso9660Reader<F> {
    _marker: core::marker::PhantomData<F>,
}

impl<F> Iso9660Reader<F> {
    pub fn new(_f: F) -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
    }

    pub fn scan_root(&mut self) -> Option<()> {
        None
    }
}
