use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct DnsPacket {
    pub transaction_id: u16,
    pub flags: u16,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsResourceRecord>,
    pub authorities: Vec<DnsResourceRecord>,
    pub additionals: Vec<DnsResourceRecord>,
}

#[derive(Debug, Clone)]
pub struct DnsQuestion {
    pub name: String,
    pub qtype: u16,
    pub qclass: u16,
}

#[derive(Debug, Clone)]
pub struct DnsResourceRecord {
    pub name: String,
    pub rtype: u16,
    pub rclass: u16,
    pub ttl: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct SrvData {
    pub priority: u16,
    pub weight: u16,
    pub port: u16,
    pub target: String,
}

impl DnsPacket {
    pub fn new_response(tid: u16) -> Self {
        Self {
            transaction_id: tid,
            flags: 0x8400,
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            additionals: Vec::new(),
        }
    }

    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 12 {
            return None;
        }

        let tid = u16::from_be_bytes([data[0], data[1]]);
        let flags = u16::from_be_bytes([data[2], data[3]]);
        let qcount = u16::from_be_bytes([data[4], data[5]]) as usize;
        let ancount = u16::from_be_bytes([data[6], data[7]]) as usize;
        let nscount = u16::from_be_bytes([data[8], data[9]]) as usize;
        let arcount = u16::from_be_bytes([data[10], data[11]]) as usize;

        let mut pos = 12usize;
        let mut questions = Vec::with_capacity(qcount);
        for _ in 0..qcount {
            let (name, next_pos) = parse_name(data, pos)?;
            pos = next_pos;
            if pos + 4 > data.len() {
                return None;
            }
            let qtype = u16::from_be_bytes([data[pos], data[pos + 1]]);
            let qclass = u16::from_be_bytes([data[pos + 2], data[pos + 3]]);
            pos += 4;
            questions.push(DnsQuestion {
                name,
                qtype,
                qclass,
            });
        }

        let (answers, next_pos) = parse_records(data, pos, ancount)?;
        pos = next_pos;
        let (authorities, next_pos) = parse_records(data, pos, nscount)?;
        pos = next_pos;
        let (additionals, pos) = parse_records(data, pos, arcount)?;
        if pos > data.len() {
            return None;
        }

        Some(Self {
            transaction_id: tid,
            flags,
            questions,
            answers,
            authorities,
            additionals,
        })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.transaction_id.to_be_bytes());
        out.extend_from_slice(&self.flags.to_be_bytes());
        out.extend_from_slice(&(self.questions.len() as u16).to_be_bytes());
        out.extend_from_slice(&(self.answers.len() as u16).to_be_bytes());
        out.extend_from_slice(&(self.authorities.len() as u16).to_be_bytes());
        out.extend_from_slice(&(self.additionals.len() as u16).to_be_bytes());

        for q in &self.questions {
            encode_name(&mut out, &q.name);
            out.extend_from_slice(&q.qtype.to_be_bytes());
            out.extend_from_slice(&q.qclass.to_be_bytes());
        }

        for rr in &self.answers {
            encode_rr(&mut out, rr);
        }
        for rr in &self.authorities {
            encode_rr(&mut out, rr);
        }
        for rr in &self.additionals {
            encode_rr(&mut out, rr);
        }

        out
    }

    pub fn is_query(&self) -> bool {
        (self.flags & 0x8000) == 0
    }

    pub fn all_records<'a>(&'a self) -> impl Iterator<Item = &'a DnsResourceRecord> {
        self.answers
            .iter()
            .chain(self.authorities.iter())
            .chain(self.additionals.iter())
    }
}

fn parse_records(
    data: &[u8],
    mut pos: usize,
    count: usize,
) -> Option<(Vec<DnsResourceRecord>, usize)> {
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let (name, next_pos) = parse_name(data, pos)?;
        pos = next_pos;
        if pos + 10 > data.len() {
            return None;
        }
        let rtype = u16::from_be_bytes([data[pos], data[pos + 1]]);
        let rclass = u16::from_be_bytes([data[pos + 2], data[pos + 3]]);
        let ttl = u32::from_be_bytes([data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]]);
        let rdlen = u16::from_be_bytes([data[pos + 8], data[pos + 9]]) as usize;
        pos += 10;
        if pos + rdlen > data.len() {
            return None;
        }
        out.push(DnsResourceRecord {
            name,
            rtype,
            rclass,
            ttl,
            data: data[pos..pos + rdlen].to_vec(),
        });
        pos += rdlen;
    }
    Some((out, pos))
}

fn encode_rr(out: &mut Vec<u8>, rr: &DnsResourceRecord) {
    encode_name(out, &rr.name);
    out.extend_from_slice(&rr.rtype.to_be_bytes());
    out.extend_from_slice(&rr.rclass.to_be_bytes());
    out.extend_from_slice(&rr.ttl.to_be_bytes());
    out.extend_from_slice(&(rr.data.len() as u16).to_be_bytes());
    out.extend_from_slice(&rr.data);
}

fn parse_name(data: &[u8], mut pos: usize) -> Option<(String, usize)> {
    let mut out = String::new();
    let mut jumps = 0usize;
    let mut return_pos: Option<usize> = None;

    loop {
        if pos >= data.len() || jumps > 16 {
            return None;
        }

        let len = data[pos] as usize;
        if len == 0 {
            pos += 1;
            break;
        }

        if (len & 0xC0) == 0xC0 {
            if pos + 1 >= data.len() {
                return None;
            }
            let ptr = (((len & 0x3F) << 8) | data[pos + 1] as usize) as usize;
            if return_pos.is_none() {
                return_pos = Some(pos + 2);
            }
            pos = ptr;
            jumps += 1;
            continue;
        }

        if pos + 1 + len > data.len() {
            return None;
        }
        if !out.is_empty() {
            out.push('.');
        }
        let label = core::str::from_utf8(&data[pos + 1..pos + 1 + len]).ok()?;
        out.push_str(label);
        pos += 1 + len;
    }

    Some((out, return_pos.unwrap_or(pos)))
}

pub fn encode_name(out: &mut Vec<u8>, name: &str) {
    if name.is_empty() || name == "." {
        out.push(0);
        return;
    }
    for label in name.trim_end_matches('.').split('.') {
        out.push(label.len() as u8);
        out.extend_from_slice(label.as_bytes());
    }
    out.push(0);
}

pub fn decode_name_from_rdata(packet: &[u8], rr: &DnsResourceRecord) -> Option<String> {
    let start = find_rdata_start(packet, rr)?;
    let (name, _) = parse_name(packet, start)?;
    Some(name)
}

fn find_rdata_start(packet: &[u8], rr: &DnsResourceRecord) -> Option<usize> {
    if packet.len() < 12 {
        return None;
    }
    let qcount = u16::from_be_bytes([packet[4], packet[5]]) as usize;
    let ancount = u16::from_be_bytes([packet[6], packet[7]]) as usize;
    let nscount = u16::from_be_bytes([packet[8], packet[9]]) as usize;
    let arcount = u16::from_be_bytes([packet[10], packet[11]]) as usize;

    let mut i = 12usize;
    for _ in 0..qcount {
        let (_, next) = parse_name(packet, i)?;
        i = next;
        if i + 4 > packet.len() {
            return None;
        }
        i += 4;
    }

    let total_rrs = ancount + nscount + arcount;
    for _ in 0..total_rrs {
        let (_, next) = parse_name(packet, i)?;
        i = next;
        if i + 10 > packet.len() {
            return None;
        }
        let rtype = u16::from_be_bytes([packet[i], packet[i + 1]]);
        let rclass = u16::from_be_bytes([packet[i + 2], packet[i + 3]]);
        let ttl = u32::from_be_bytes([packet[i + 4], packet[i + 5], packet[i + 6], packet[i + 7]]);
        let rdlen = u16::from_be_bytes([packet[i + 8], packet[i + 9]]) as usize;
        i += 10;
        if i + rdlen > packet.len() {
            return None;
        }
        if rtype == rr.rtype
            && rclass == rr.rclass
            && ttl == rr.ttl
            && packet[i..i + rdlen] == rr.data
        {
            return Some(i);
        }
        i += rdlen;
    }
    None
}

pub fn parse_ptr_target(rr: &DnsResourceRecord, packet: &[u8]) -> Option<String> {
    if rr.rtype != 12 {
        return None;
    }
    decode_name_from_rdata(packet, rr)
}

pub fn parse_srv(rr: &DnsResourceRecord, packet: &[u8]) -> Option<SrvData> {
    if rr.rtype != 33 || rr.data.len() < 6 {
        return None;
    }
    let priority = u16::from_be_bytes([rr.data[0], rr.data[1]]);
    let weight = u16::from_be_bytes([rr.data[2], rr.data[3]]);
    let port = u16::from_be_bytes([rr.data[4], rr.data[5]]);
    let start = find_rdata_start(packet, rr)?.saturating_add(6);
    let (target, _) = parse_name(packet, start)?;
    Some(SrvData {
        priority,
        weight,
        port,
        target,
    })
}

pub fn parse_txt_kvs(rr: &DnsResourceRecord) -> Vec<(String, String)> {
    let mut out = Vec::new();
    if rr.rtype != 16 {
        return out;
    }
    let mut i = 0usize;
    while i < rr.data.len() {
        let len = rr.data[i] as usize;
        i += 1;
        if i + len > rr.data.len() {
            break;
        }
        let chunk = &rr.data[i..i + len];
        i += len;
        if let Ok(s) = core::str::from_utf8(chunk) {
            if let Some((k, v)) = s.split_once('=') {
                out.push((k.into(), v.into()));
            } else {
                out.push((s.into(), String::new()));
            }
        }
    }
    out
}

pub fn parse_a(rr: &DnsResourceRecord) -> Option<[u8; 4]> {
    if rr.rtype != 1 || rr.data.len() != 4 {
        return None;
    }
    Some([rr.data[0], rr.data[1], rr.data[2], rr.data[3]])
}

pub fn parse_aaaa(rr: &DnsResourceRecord) -> Option<[u8; 16]> {
    if rr.rtype != 28 || rr.data.len() != 16 {
        return None;
    }
    let mut out = [0u8; 16];
    out.copy_from_slice(&rr.data);
    Some(out)
}

pub fn encode_ptr_rdata(name: &str) -> Vec<u8> {
    let mut out = Vec::new();
    encode_name(&mut out, name);
    out
}

pub fn encode_srv_rdata(priority: u16, weight: u16, port: u16, target: &str) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&priority.to_be_bytes());
    out.extend_from_slice(&weight.to_be_bytes());
    out.extend_from_slice(&port.to_be_bytes());
    encode_name(&mut out, target);
    out
}

pub fn encode_txt_rdata(txt_items: &[String]) -> Vec<u8> {
    let mut out = Vec::new();
    for item in txt_items {
        let bytes = item.as_bytes();
        let len = core::cmp::min(bytes.len(), 255);
        out.push(len as u8);
        out.extend_from_slice(&bytes[..len]);
    }
    out
}
