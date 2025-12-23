#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use thing_models::{InputCharEvent, KeyScanEvent, MousePacketEvent};
use thing_os::prelude::*;

#[thing_os::main]
fn main() {
    println!("debug_input_events: starting");

    if !ensure_schema_exists_for::<KeyScanEvent>() {
        println!("debug_input_events: KeyScanEvent schema missing");
    }
    if !ensure_schema_exists_for::<InputCharEvent>() {
        println!("debug_input_events: InputCharEvent schema missing");
    }
    if !ensure_schema_exists_for::<MousePacketEvent>() {
        println!("debug_input_events: MousePacketEvent schema missing");
    }

    let mut last_key = max_seq::<KeyScanEvent>();
    let mut last_char = max_seq::<InputCharEvent>();
    let mut last_mouse = max_seq::<MousePacketEvent>();

    loop {
        process_events(
            "KEY",
            &mut last_key,
            list_things_by_kind::<KeyScanEvent>(),
            |e| {
                println!(
                    "debug_input_events: KEY scancode={:#x} released={} extended={} seq={}",
                    e.scancode, e.released, e.extended, e.sequence_index
                );
            },
        );

        process_events(
            "CHAR",
            &mut last_char,
            list_things_by_kind::<InputCharEvent>(),
            |e| {
                println!(
                    "debug_input_events: CHAR '{}' seq={}",
                    escape_char(e.ch),
                    e.sequence_index
                );
            },
        );

        process_events(
            "MOUSE",
            &mut last_mouse,
            list_things_by_kind::<MousePacketEvent>(),
            |e| {
                println!(
                    "debug_input_events: MOUSE dx={} dy={} btns={:#x} seq={}",
                    e.delta_x, e.delta_y, e.buttons, e.sequence_index
                );
            },
        );

        sleep_ms(10);
    }
}

fn max_seq<T: Thing + Sequenced>() -> Option<u64> {
    list_things_by_kind::<T>()
        .iter()
        .map(|e| e.sequence())
        .max()
}

fn process_events<T, F>(tag: &str, last_seq: &mut Option<u64>, mut events: Vec<T>, f: F)
where
    T: Thing + Sequenced,
    F: Fn(&T),
{
    events.sort_by_key(|e| e.sequence());
    for e in events {
        if last_seq.map_or(true, |last| e.sequence() > last) {
            f(&e);
            *last_seq = Some(e.sequence());
        }
    }
}

fn escape_char(ch: char) -> &'static str {
    match ch {
        '\n' => "\\n",
        '\r' => "\\r",
        '\t' => "\\t",
        _ => {
            // We return static str; fallback to "." for non-printable
            if ch.is_control() {
                "."
            } else {
                // Best effort single-char; leaking is not allowed, so limit to ASCII printable
                // If not ASCII printable, just return "."
                if ch.is_ascii_graphic() || ch == ' ' {
                    // Use small static table for common printable ASCII
                    match ch {
                        ' ' => " ",
                        '!' => "!",
                        '"' => "\"",
                        '#' => "#",
                        '$' => "$",
                        '%' => "%",
                        '&' => "&",
                        '\'' => "'",
                        '(' => "(",
                        ')' => ")",
                        '*' => "*",
                        '+' => "+",
                        ',' => ",",
                        '-' => "-",
                        '.' => ".",
                        '/' => "/",
                        '0' => "0",
                        '1' => "1",
                        '2' => "2",
                        '3' => "3",
                        '4' => "4",
                        '5' => "5",
                        '6' => "6",
                        '7' => "7",
                        '8' => "8",
                        '9' => "9",
                        ':' => ":",
                        ';' => ";",
                        '<' => "<",
                        '=' => "=",
                        '>' => ">",
                        '?' => "?",
                        '@' => "@",
                        'A' => "A",
                        'B' => "B",
                        'C' => "C",
                        'D' => "D",
                        'E' => "E",
                        'F' => "F",
                        'G' => "G",
                        'H' => "H",
                        'I' => "I",
                        'J' => "J",
                        'K' => "K",
                        'L' => "L",
                        'M' => "M",
                        'N' => "N",
                        'O' => "O",
                        'P' => "P",
                        'Q' => "Q",
                        'R' => "R",
                        'S' => "S",
                        'T' => "T",
                        'U' => "U",
                        'V' => "V",
                        'W' => "W",
                        'X' => "X",
                        'Y' => "Y",
                        'Z' => "Z",
                        '[' => "[",
                        '\\' => "\\",
                        ']' => "]",
                        '^' => "^",
                        '_' => "_",
                        '`' => "`",
                        'a' => "a",
                        'b' => "b",
                        'c' => "c",
                        'd' => "d",
                        'e' => "e",
                        'f' => "f",
                        'g' => "g",
                        'h' => "h",
                        'i' => "i",
                        'j' => "j",
                        'k' => "k",
                        'l' => "l",
                        'm' => "m",
                        'n' => "n",
                        'o' => "o",
                        'p' => "p",
                        'q' => "q",
                        'r' => "r",
                        's' => "s",
                        't' => "t",
                        'u' => "u",
                        'v' => "v",
                        'w' => "w",
                        'x' => "x",
                        'y' => "y",
                        'z' => "z",
                        '{' => "{",
                        '|' => "|",
                        '}' => "}",
                        '~' => "~",
                        _ => ".",
                    }
                } else {
                    "."
                }
            }
        }
    }
}

trait Sequenced {
    fn sequence(&self) -> u64;
}

impl Sequenced for KeyScanEvent {
    fn sequence(&self) -> u64 {
        self.sequence_index
    }
}
impl Sequenced for InputCharEvent {
    fn sequence(&self) -> u64 {
        self.sequence_index
    }
}
impl Sequenced for MousePacketEvent {
    fn sequence(&self) -> u64 {
        self.sequence_index
    }
}
