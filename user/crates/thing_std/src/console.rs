pub trait Console {
    fn write_str(&self, s: &str);
}

pub struct StdoutConsole;

impl Console for StdoutConsole {
    fn write_str(&self, s: &str) {
        // v0: route to whatever exists.
        let _ = s;
        // TODO: implement (sys_log or graph op).
    }
}
