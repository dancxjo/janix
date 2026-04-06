#![feature(restricted_std)]
#![no_main]

extern crate alloc;

mod net_client;
mod packstream;
mod protocol;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use net_client::{NetClient, TcpRecvResult};
use packstream::Value;
use protocol::{create_chunked, decode_message, BoltMessage};
use stem::{info, warn};

const BOLT_PORT: u16 = 7687;

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("boltd: starting on port {}", BOLT_PORT);

    let net = loop {
        match NetClient::connect() {
            Some(net) => break net,
            None => {
                info!("boltd: waiting for network stack");
                stem::time::sleep_ms(500);
            }
        }
    };

    let listen_handle = loop {
        match net.tcp_listen(BOLT_PORT) {
            Some(handle) => break handle,
            None => {
                warn!("boltd: listen failed, retrying");
                stem::time::sleep_ms(1000);
            }
        }
    };

    info!(
        "boltd: listening on guest port {} (handle={})",
        BOLT_PORT, listen_handle
    );

    loop {
        if let Some(accept) = net.tcp_accept(listen_handle) {
            info!(
                "boltd: accepted connection conn_handle={}",
                accept.conn_handle
            );
            handle_connection(&net, accept.conn_handle);
            info!(
                "boltd: closing connection conn_handle={}",
                accept.conn_handle
            );
            net.tcp_close(accept.conn_handle);
        }
        stem::time::sleep_ms(10);
    }
}

fn handle_connection(net: &NetClient, conn_handle: u32) {
    let mut executor = phloem::GraphExecutor::new();
    let mut state = 0; // 0: waiting for magic, 1: streaming
    let mut recv_buffer = Vec::new();

    let mut stashed_result: Option<phloem::ExecutionResult> = None;

    loop {
        let mut data = match net.tcp_recv(conn_handle, NetClient::MAX_RECV_LEN) {
            TcpRecvResult::Data(data) => data,
            TcpRecvResult::Empty => {
                stem::time::sleep_ms(10);
                continue;
            }
            TcpRecvResult::Closed => break,
        };

        recv_buffer.append(&mut data);

        if state == 0 {
            // Handshake expects 20 bytes: 4 magic + 4*4 version
            if recv_buffer.len() >= 20 {
                let magic = &recv_buffer[0..4];
                if magic != [0x60, 0x60, 0xB0, 0x17] {
                    info!("boltd: invalid magic {:?}", magic);
                    return;
                }
                
                // version 4
                write_raw(net, conn_handle, &[0x00, 0x00, 0x00, 0x04]);
                recv_buffer.drain(0..20);
                state = 1;
            }
        }

        if state == 1 {
            while let Some((payload, consumed)) = try_parse_chunks(&recv_buffer) {
                recv_buffer.drain(0..consumed);
                
                if let Some(msg) = decode_message(&payload) {
                    handle_bolt_message(net, conn_handle, &mut executor, msg, &mut stashed_result);
                } else {
                    info!("boltd: failed to decode message from payload");
                }
            }
        }
    }
}

fn try_parse_chunks(buffer: &[u8]) -> Option<(Vec<u8>, usize)> {
    let mut payload = Vec::new();
    let mut pos = 0;

    loop {
        if buffer.len() < pos + 2 {
            return None; // Need more data for length
        }
        
        let chunk_len = u16::from_be_bytes([buffer[pos], buffer[pos+1]]) as usize;
        pos += 2;

        if chunk_len == 0 {
            // end of message
            return Some((payload, pos));
        }

        if buffer.len() < pos + chunk_len {
            return None; // Need more data for chunk payload
        }

        payload.extend_from_slice(&buffer[pos..pos+chunk_len]);
        pos += chunk_len;
    }
}

fn handle_bolt_message(net: &NetClient, conn_handle: u32, executor: &mut phloem::GraphExecutor, msg: BoltMessage, stashed_result: &mut Option<phloem::ExecutionResult>) {
    info!("boltd: msg {:?}", msg);
    match msg {
        BoltMessage::Hello { .. } | BoltMessage::Init { .. } => {
            let mut map = BTreeMap::new();
            map.insert("server".into(), Value::String("thingos-boltd/1.0".into()));
            map.insert("connection_id".into(), Value::String("conn-1".into()));
            let response = BoltMessage::Success(map);
            let bytes = create_chunked(&response);
            write_raw(net, conn_handle, &bytes);
        }
        BoltMessage::Run { query, .. } => {
            let result = match phloem::parse_with_metadata(&query) {
                Ok(parsed) => executor.execute(parsed.command),
                Err(e) => {
                    let mut err_map = BTreeMap::new();
                    err_map.insert("code".into(), Value::String("Neo.ClientError.Statement.SyntaxError".into()));
                    err_map.insert("message".into(), Value::String(alloc::format!("syntax error: {}", e)));
                    write_raw(net, conn_handle, &create_chunked(&BoltMessage::Failure(err_map)));
                    return;
                }
            };
            
            let mut map = BTreeMap::new();
            let mut fields = Vec::new();
            for col in &result.columns {
                fields.push(Value::String(col.clone()));
            }
            map.insert("fields".into(), Value::List(fields));
            
            *stashed_result = Some(result);
            
            let bytes = create_chunked(&BoltMessage::Success(map));
            write_raw(net, conn_handle, &bytes);
        }
        BoltMessage::PullAll | BoltMessage::Pull { .. } => {
            if let Some(result) = stashed_result.take() {
                for row in result.rows {
                    let mut rec = Vec::new();
                    for val in row {
                        match val {
                            phloem::ResultValue::Node(id) => {
                                // For node we should serialize as a node. But simplistic Bolt encodes as Map or Node Struct?
                                // Neo4j Node Struct is tag 'N' (0x4E). Fields: [id: int, labels: list_of_strings, properties: map]
                                let mut props = BTreeMap::new();
                                props.insert("_id".into(), Value::Integer(id as i64));
                                let node_struct = Value::Struct {
                                    tag: 0x4E, // 'N'
                                    fields: vec![
                                        Value::Integer(id as i64),
                                        Value::List(vec![]),
                                        Value::Map(props),
                                    ],
                                };
                                rec.push(node_struct);
                            }
                            phloem::ResultValue::String(s) => rec.push(Value::String(s)),
                            phloem::ResultValue::Number(n) => rec.push(Value::Integer(n as i64)), // GQL number is f64, but ResultValue::Number is usually an int
                        }
                    }
                    let bytes = create_chunked(&BoltMessage::Record(rec));
                    write_raw(net, conn_handle, &bytes);
                }
            }
            let mut map = BTreeMap::new();
            map.insert("has_more".into(), Value::Boolean(false));
            let bytes = create_chunked(&BoltMessage::Success(map));
            write_raw(net, conn_handle, &bytes);
        }
        BoltMessage::Discard { .. } => {
            let _ = stashed_result.take();
            let mut map = BTreeMap::new();
            map.insert("has_more".into(), Value::Boolean(false));
            let bytes = create_chunked(&BoltMessage::Success(map));
            write_raw(net, conn_handle, &bytes);
        }
        _ => {
            let mut err_map = BTreeMap::new();
            err_map.insert("code".into(), Value::String("Neo.ClientError.Request.Invalid".into()));
            err_map.insert("message".into(), Value::String("Unsupported message".into()));
            write_raw(net, conn_handle, &create_chunked(&BoltMessage::Failure(err_map)));
        }
    }
}

fn write_raw(net: &NetClient, conn_handle: u32, bytes: &[u8]) -> bool {
    let mut sent = 0usize;
    let mut stalled = 0u16;
    while sent < bytes.len() {
        let n = net.tcp_send(conn_handle, &bytes[sent..]);
        if n == 0 {
            stalled = stalled.saturating_add(1);
            if stalled >= 1000 {
                return false;
            }
            stem::time::sleep_ms(5);
            continue;
        }
        stalled = 0;
        sent += n;
    }
    true
}
