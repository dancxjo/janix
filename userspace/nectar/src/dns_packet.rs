use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;

pub struct DnsPacket {
    pub transaction_id: u16,
    pub flags: u16,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsResourceRecord>,
}

pub struct DnsQuestion {
    pub name: String,
    pub qtype: u16,
    pub qclass: u16,
}

pub struct DnsResourceRecord {
    pub name: String,
    pub rtype: u16,
    pub rclass: u16,
    pub ttl: u32,
    pub data: Vec<u8>,
}

impl DnsPacket {
    pub fn new_response(tid: u16) -> Self {
        Self {
            transaction_id: tid,
            flags: 0x8400, // Response, Authoritative, Success
            questions: Vec::new(),
            answers: Vec::new(),
        }
    }

    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 12 { return None; }
        
        let tid = u16::from_be_bytes([data[0], data[1]]);
        let flags = u16::from_be_bytes([data[2], data[3]]);
        let qcount = u16::from_be_bytes([data[4], data[5]]);
        
        let mut pos = 12;
        let mut questions = Vec::new();
        
        for _ in 0..qcount {
            let (name, next_pos) = parse_name(data, pos)?;
            pos = next_pos;
            if pos + 4 > data.len() { return None; }
            let qtype = u16::from_be_bytes([data[pos], data[pos+1]]);
            let qclass = u16::from_be_bytes([data[pos+2], data[pos+3]]);
            pos += 4;
            questions.push(DnsQuestion { name, qtype, qclass });
        }
        
        // We generally don't need to parse answers in the incoming mDNS queries for our simple responder
        Some(Self {
            transaction_id: tid,
            flags,
            questions,
            answers: Vec::new(),
        })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.transaction_id.to_be_bytes());
        out.extend_from_slice(&self.flags.to_be_bytes());
        out.extend_from_slice(&(self.questions.len() as u16).to_be_bytes());
        out.extend_from_slice(&(self.answers.len() as u16).to_be_bytes());
        out.extend_from_slice(&0u16.to_be_bytes()); // Authority
        out.extend_from_slice(&0u16.to_be_bytes()); // Additional
        
        for q in &self.questions {
            encode_name(&mut out, &q.name);
            out.extend_from_slice(&q.qtype.to_be_bytes());
            out.extend_from_slice(&q.qclass.to_be_bytes());
        }
        
        for a in &self.answers {
            encode_name(&mut out, &a.name);
            out.extend_from_slice(&a.rtype.to_be_bytes());
            out.extend_from_slice(&a.rclass.to_be_bytes());
            out.extend_from_slice(&a.ttl.to_be_bytes());
            out.extend_from_slice(&(a.data.len() as u16).to_be_bytes());
            out.extend_from_slice(&a.data);
        }
        
        out
    }
}

fn parse_name(data: &[u8], mut pos: usize) -> Option<(String, usize)> {
    let mut name = String::new();
    let mut loop_count = 0;
    let mut saved_pos = None;

    loop {
        if loop_count > 10 { return None; } // Prevent infinite loops
        if pos >= data.len() { return None; }
        
        let len = data[pos] as usize;
        if len == 0 {
            pos += 1;
            break;
        } else if len >= 192 {
            // Compression pointer
            if pos + 1 >= data.len() { return None; }
            let pointer = (((len & 0x3F) << 8) | (data[pos+1] as usize)) as usize;
            if saved_pos.is_none() {
                saved_pos = Some(pos + 2);
            }
            pos = pointer;
            loop_count += 1;
            continue;
        } else {
            if !name.is_empty() { name.push('.'); }
            if pos + 1 + len > data.len() { return None; }
            let s = core::str::from_utf8(&data[pos+1..pos+1+len]).ok()?;
            name.push_str(s);
            pos += 1 + len;
        }
    }
    
    let final_pos = saved_pos.unwrap_or(pos);
    Some((name, final_pos))
}

fn encode_name(out: &mut Vec<u8>, name: &str) {
    if name == "_" {
        out.push(0);
        return;
    }
    for part in name.split('.') {
        out.push(part.len() as u8);
        out.extend_from_slice(part.as_bytes());
    }
    out.push(0);
}
