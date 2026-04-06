use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub enum ShellAction {
    PrintLine(String),
    PrintError(String),
    ClearScreen,
    ListDir(String),
    ShowTasks,
    ShowMem,
    DumpGraph,
    RunProgram(String),
    RunGql(String),
}

pub struct ShellSession {
    cwd: String,
}

impl ShellSession {
    pub fn new() -> Self {
        Self {
            cwd: "/".to_string(),
        }
    }

    pub fn banner_lines(&self) -> [&'static str; 2] {
        ["ThingOS Petals", "type 'help' for commands"]
    }

    pub fn prompt(&self) -> &str {
        "petals> "
    }

    pub fn handle_line(&mut self, line: &str) -> Vec<ShellAction> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return Vec::new();
        }

        let mut parts = trimmed.split_whitespace();
        let cmd = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        match cmd {
            "help" => vec![
                ShellAction::PrintLine("help  - show this command list".to_string()),
                ShellAction::PrintLine("echo  - print arguments".to_string()),
                ShellAction::PrintLine("clear - clear the console".to_string()),
                ShellAction::PrintLine("pwd   - show current location".to_string()),
                ShellAction::PrintLine("ls    - list graph-backed directory entries".to_string()),
                ShellAction::PrintLine("tasks - list known tasks".to_string()),
                ShellAction::PrintLine("mem   - show memory diagnostics".to_string()),
                ShellAction::PrintLine("graph - dump the root graph".to_string()),
                ShellAction::PrintLine("run   - launch a foreground program".to_string()),
            ],
            "echo" => vec![ShellAction::PrintLine(args.join(" "))],
            "clear" => vec![ShellAction::ClearScreen],
            "pwd" => vec![ShellAction::PrintLine(self.cwd.clone())],
            "ls" => {
                let path = args
                    .first()
                    .map(|path| normalize_path(&self.cwd, path))
                    .unwrap_or_else(|| self.cwd.clone());
                vec![ShellAction::ListDir(path)]
            }
            "tasks" => vec![ShellAction::ShowTasks],
            "mem" => vec![ShellAction::ShowMem],
            "graph" => vec![ShellAction::DumpGraph],
            "run" => match args.first() {
                Some(path) => vec![ShellAction::RunProgram(normalize_program(path))],
                None => vec![ShellAction::PrintError("usage: run <program>".to_string())],
            },
            "match" => vec![ShellAction::RunGql(line.to_string())],
            _ => vec![ShellAction::PrintError(format!("unknown command: {}", cmd))],
        }
    }
}

fn normalize_path(cwd: &str, path: &str) -> String {
    if path.starts_with('/') {
        path.to_string()
    } else if cwd == "/" {
        format!("/{}", path)
    } else {
        format!("{}/{}", cwd.trim_end_matches('/'), path)
    }
}

fn normalize_program(path: &str) -> String {
    if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{}", path)
    }
}
