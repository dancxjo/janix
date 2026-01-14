#![no_std]

pub struct Guess {
    pub mime: &'static str,
    pub extension: &'static str,
    pub confidence: u16,
    pub bytes_needed: usize,
}

pub fn sniff(data: &[u8]) -> Option<Guess> {
    let kind = infer::get(data)?;
    Some(Guess {
        mime: kind.mime_type(),
        extension: kind.extension(),
        confidence: 1000, 
        bytes_needed: 0,
    })
}
