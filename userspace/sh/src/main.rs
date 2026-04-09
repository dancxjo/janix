#![feature(restricted_std)]
#![no_main]

extern crate alloc;
use abi::syscall::vfs_flags;
use alloc::string::String;
use alloc::vec::Vec;
use stem::syscall::{dup2, pipe, vfs_close, vfs_open, vfs_read, vfs_write};

fn prompt() {
    let mut buf = [0u8; 256];
    match stem::syscall::vfs_getcwd(&mut buf) {
        Ok(n) => {
            let cwd = core::str::from_utf8(&buf[..n]).unwrap_or("/");
            let out = alloc::format!("thing {} # ", cwd);
            let _ = vfs_write(1, out.as_bytes());
        }
        Err(_) => {
            let _ = vfs_write(1, b"thing-os # ");
        }
    }
}

fn read_line() -> String {
    let mut buf = [0u8; 1024];
    let mut bytes = Vec::new();
    loop {
        match vfs_read(0, &mut buf) {
            Ok(0) => break,
            Ok(n) => {
                for &b in &buf[..n] {
                    if b == 0x03 {
                        bytes.clear();
                        return String::new(); // Ctrl-C
                    }
                    bytes.push(b);
                    if b == b'\n' {
                        let s = String::from_utf8(bytes).unwrap_or_default();
                        return s;
                    }
                }
            }
            Err(_) => break,
        }
    }
    String::from_utf8(bytes).unwrap_or_default()
}

/// A single command segment: program name, args, optional stdin/stdout redirects.
struct Cmd<'a> {
    program: &'a str,
    args: Vec<&'a str>,
    /// Override stdin with this path (< file).
    stdin_file: Option<&'a str>,
    /// Override stdout with this path (> file or >> file).
    stdout_file: Option<&'a str>,
    /// Append mode for stdout.
    stdout_append: bool,
}

impl<'a> Cmd<'a> {
    fn parse(tokens: &[&'a str]) -> Option<Self> {
        if tokens.is_empty() {
            return None;
        }
        let mut program: Option<&'a str> = None;
        let mut args = Vec::new();
        let mut stdin_file: Option<&'a str> = None;
        let mut stdout_file: Option<&'a str> = None;
        let mut stdout_append = false;

        let mut i = 0;
        while i < tokens.len() {
            let tok = tokens[i];
            match tok {
                "<" => {
                    i += 1;
                    if i < tokens.len() {
                        stdin_file = Some(tokens[i]);
                    }
                }
                ">>" => {
                    i += 1;
                    if i < tokens.len() {
                        stdout_file = Some(tokens[i]);
                        stdout_append = true;
                    }
                }
                ">" => {
                    i += 1;
                    if i < tokens.len() {
                        stdout_file = Some(tokens[i]);
                        stdout_append = false;
                    }
                }
                _ => {
                    if program.is_none() {
                        program = Some(tok);
                    } else {
                        args.push(tok);
                    }
                }
            }
            i += 1;
        }
        program.map(|p| Cmd {
            program: p,
            args,
            stdin_file,
            stdout_file,
            stdout_append,
        })
    }
}

/// Spawn one command with explicit stdin/stdout fds; wait for it to finish.
///
/// `stdin_fd` and `stdout_fd` are the fds that the child should inherit as 0
/// and 1 respectively.  Pass 0/1 to keep the shell's own stdio.
fn spawn_cmd(cmd: &Cmd, stdin_fd: u32, stdout_fd: u32) -> abi::errors::SysResult<()> {
    let path = if cmd.program.starts_with('/') {
        String::from(cmd.program)
    } else {
        alloc::format!("/bin/{}", cmd.program)
    };

    let mut argv: Vec<Vec<u8>> = Vec::new();
    argv.push(path.as_bytes().to_vec());
    for arg in &cmd.args {
        argv.push(arg.as_bytes().to_vec());
    }
    let argv_slices: Vec<&[u8]> = argv.iter().map(|v| v.as_slice()).collect();
    let env = alloc::collections::BTreeMap::new();

    match stem::syscall::spawn_process_ex(
        &path,
        &argv_slices,
        &env,
        stdin_fd,
        stdout_fd,
        1,  // stderr → shell's own stdout
        0,  // boot_arg
        &[] // No handles to inherit
    ) {
        Ok(resp) => {
            let _ = stem::syscall::task_wait(resp.child_tid);
            Ok(())
        }
        Err(e) => {
            let out = alloc::format!("sh: {}: command not found\n", cmd.program);
            let _ = vfs_write(1, out.as_bytes());
            Err(e)
        }
    }
}

/// Open a file for use as stdin redirect.  Returns the fd on success.
fn open_stdin_file(path: &str) -> Option<u32> {
    vfs_open(path, vfs_flags::O_RDONLY).ok()
}

/// Open a file for use as stdout redirect.  Returns the fd on success.
fn open_stdout_file(path: &str, append: bool) -> Option<u32> {
    let flags = if append {
        vfs_flags::O_WRONLY | vfs_flags::O_CREAT | vfs_flags::O_APPEND
    } else {
        vfs_flags::O_WRONLY | vfs_flags::O_CREAT | vfs_flags::O_TRUNC
    };
    vfs_open(path, flags).ok()
}

/// Execute a pipeline of commands.
///
/// `cmds` contains the parsed command segments separated by `|`.  Pipes are
/// created between adjacent commands; the first and last segment may also have
/// file redirections.
fn run_pipeline(cmds: &[Cmd]) {
    if cmds.is_empty() {
        return;
    }

    if cmds.len() == 1 {
        // Simple case: single command, possibly with redirects.
        let cmd = &cmds[0];
        let stdin_fd = cmd.stdin_file.and_then(|p| open_stdin_file(p)).unwrap_or(0);
        let stdout_fd = cmd
            .stdout_file
            .and_then(|p| open_stdout_file(p, cmd.stdout_append))
            .unwrap_or(1);

        let _ = spawn_cmd(cmd, stdin_fd, stdout_fd);

        if stdin_fd != 0 {
            let _ = vfs_close(stdin_fd);
        }
        if stdout_fd != 1 {
            let _ = vfs_close(stdout_fd);
        }
        return;
    }

    // Pipeline: create N-1 pipes for N commands.
    // prev_read_fd tracks the read end of the previous pipe.
    let mut prev_read_fd: Option<u32> = None;

    for (i, cmd) in cmds.iter().enumerate() {
        let is_last = i == cmds.len() - 1;

        // Determine stdin for this stage.
        let stdin_fd = if i == 0 {
            // First stage: honour < redirection, else use shell stdin.
            cmd.stdin_file.and_then(|p| open_stdin_file(p)).unwrap_or(0)
        } else {
            // Middle/last stage: read from the previous pipe.
            prev_read_fd.unwrap_or(0)
        };

        // Determine stdout for this stage.
        let (stdout_fd, next_read_fd) = if is_last {
            // Last stage: honour > / >> redirection, else use shell stdout.
            let fd = cmd
                .stdout_file
                .and_then(|p| open_stdout_file(p, cmd.stdout_append))
                .unwrap_or(1);
            (fd, None)
        } else {
            // Non-last: create a new pipe; write end → stdout of this stage.
            let mut pipefd = [0u32; 2];
            if pipe(&mut pipefd).is_err() {
                // Failed to create pipe; abort pipeline.
                if stdin_fd != 0 {
                    let _ = vfs_close(stdin_fd);
                }
                break;
            }
            (pipefd[1], Some(pipefd[0]))
        };

        let _ = spawn_cmd(cmd, stdin_fd, stdout_fd);

        // Close fds that the shell opened but no longer needs after spawn.
        // Any fd that is not the shell's own stdin (0) or stdout (1) was
        // created specifically for this pipeline stage and must be closed here
        // so file descriptors are not leaked.
        if stdin_fd != 0 {
            let _ = vfs_close(stdin_fd);
        }
        if !is_last {
            // Close the write end of the pipe we just used (child inherited it).
            let _ = vfs_close(stdout_fd);
        } else if stdout_fd != 1 {
            let _ = vfs_close(stdout_fd);
        }

        prev_read_fd = next_read_fd;
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let _ = vfs_write(1, b"janix sh\n");

    loop {
        prompt();
        let line = read_line();
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Tokenise by whitespace first (simple shell-level tokeniser — no
        // quoting support yet).
        let tokens: Vec<&str> = trimmed.split_whitespace().collect();

        if tokens.first().copied() == Some("exit") {
            break;
        }

        // Split into pipeline segments on `|`.
        let mut segments: Vec<Vec<&str>> = Vec::new();
        let mut current: Vec<&str> = Vec::new();
        for &tok in &tokens {
            if tok == "|" {
                segments.push(current);
                current = Vec::new();
            } else {
                current.push(tok);
            }
        }
        segments.push(current);

        // Parse each segment into a Cmd.
        let cmds: Vec<Cmd> = segments.iter().filter_map(|seg| Cmd::parse(seg)).collect();

        if cmds.len() == 1 && cmds[0].program == "cd" {
            let target = if !cmds[0].args.is_empty() {
                cmds[0].args[0]
            } else {
                "/"
            };
            if let Err(e) = stem::syscall::vfs_chdir(target) {
                let out = alloc::format!("cd: {}: {:?}\n", target, e);
                let _ = vfs_write(1, out.as_bytes());
            }
            continue;
        }

        run_pipeline(&cmds);
    }

    stem::syscall::exit(0)
}
