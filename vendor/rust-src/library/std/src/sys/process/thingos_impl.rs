
use crate::ffi::{OsStr, OsString};
use crate::io;
use crate::path::Path;
use crate::sys::fs::File;
use crate::sys::pipe::AnonPipe;
use crate::num::NonZero;
use crate::collections::HashMap;
use crate::vec::Vec;
use crate::fmt;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
pub struct ExitStatus(pub i32);

impl ExitStatus {
    pub fn exit_ok(&self) -> Result<(), ExitStatusError> {
        Ok(())
    }
    pub fn code(&self) -> Option<i32> {
        Some(self.0)
    }
    pub fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
    pub fn success(&self) -> bool { self.0 == 0 }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitStatusError(pub i32);

impl Into<ExitStatus> for ExitStatusError {
    fn into(self) -> ExitStatus { ExitStatus(self.0) }
}

impl ExitStatusError {
    pub fn code(self) -> Option<NonZero<i32>> { NonZero::new(self.0) }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitCode(pub i32);

impl ExitCode {
    pub const SUCCESS: ExitCode = ExitCode(0);
    pub const FAILURE: ExitCode = ExitCode(1);
    pub fn as_i32(&self) -> i32 { self.0 }
}

impl From<i32> for ExitCode {
    fn from(code: i32) -> Self { ExitCode(code) }
}
impl From<u8> for ExitCode {
    fn from(code: u8) -> Self { ExitCode(code as i32) }
}

pub struct Process;
pub type ChildPipe = AnonPipe;

pub(crate) use crate::process::StdioPipes;

#[derive(Debug)]
pub struct Command {
    program: OsString,
    args: Vec<OsString>,
    env: CommandEnv,
    cwd: Option<OsString>,
}

pub struct CommandArgs<'a> {
    iter: crate::slice::Iter<'a, OsString>,
}

impl<'a> Iterator for CommandArgs<'a> {
    type Item = &'a OsStr;
    fn next(&mut self) -> Option<&'a OsStr> {
        self.iter.next().map(|s| s.as_os_str())
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for CommandArgs<'a> {
    fn len(&self) -> usize { self.iter.len() }
}

impl<'a> fmt::Debug for CommandArgs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter.clone()).finish()
    }
}

impl Command {
    pub fn new(program: &OsStr) -> Command {
        Command {
            program: program.to_os_string(),
            args: Vec::new(),
            env: CommandEnv::default(),
            cwd: None,
        }
    }
    pub fn arg(&mut self, arg: &OsStr) {
        self.args.push(arg.to_os_string());
    }
    pub fn env_mut(&mut self) -> &mut CommandEnv {
        &mut self.env
    }
    pub fn cwd(&mut self, dir: &OsStr) {
        self.cwd = Some(dir.to_os_string());
    }
    pub fn stdin(&mut self, _stdin: Stdio) {}
    pub fn stdout(&mut self, _stdout: Stdio) {}
    pub fn stderr(&mut self, _stderr: Stdio) {}
    pub fn spawn(&mut self, _default: Stdio, _needs_stdin: bool) -> io::Result<(Process, StdioPipes)> {
        Err(io::Error::UNSUPPORTED_PLATFORM)
    }

    pub fn get_program(&self) -> &OsStr {
        &self.program
    }
    pub fn get_args(&self) -> CommandArgs<'_> {
        CommandArgs { iter: self.args.iter() }
    }
    pub fn get_envs(&self) -> CommandEnvs<'_> {
        self.env.iter()
    }
    pub fn get_current_dir(&self) -> Option<&Path> {
        self.cwd.as_deref().map(Path::new)
    }
    pub fn get_env_clear(&self) -> bool {
        self.env.is_cleared()
    }
}

#[derive(Debug, Default, Clone)]
pub struct CommandEnv {
    clear: bool,
    vars: HashMap<OsString, Option<OsString>>,
}
impl CommandEnv {
    pub fn set(&mut self, key: &OsStr, value: &OsStr) {
        self.vars.insert(key.to_os_string(), Some(value.to_os_string()));
    }
    pub fn remove(&mut self, key: &OsStr) {
        self.vars.insert(key.to_os_string(), None);
    }
    pub fn clear(&mut self) {
        self.clear = true;
        self.vars.clear();
    }
    pub fn capture(&self) -> HashMap<OsString, OsString> {
         let mut map = HashMap::new();
         for (k, v) in &self.vars {
             if let Some(val) = v {
                 map.insert(k.clone(), val.clone());
             }
         }
         map
    }
    pub fn iter(&self) -> CommandEnvs<'_> {
        CommandEnvs { iter: self.vars.iter() }
    }
    pub fn is_cleared(&self) -> bool {
        self.clear
    }
}

pub struct CommandEnvs<'a> {
    iter: crate::collections::hash_map::Iter<'a, OsString, Option<OsString>>,
}

impl<'a> Iterator for CommandEnvs<'a> {
    type Item = (&'a OsStr, Option<&'a OsStr>);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, v)| (k.as_os_str(), v.as_deref()))
    }
}

impl<'a> ExactSizeIterator for CommandEnvs<'a> {
    fn len(&self) -> usize { self.iter.len() }
}

impl<'a> fmt::Debug for CommandEnvs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandEnvs").finish()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash, Default)]
pub struct EnvKey; 
impl From<&OsStr> for EnvKey { fn from(_: &OsStr) -> EnvKey { EnvKey } }
impl From<OsString> for EnvKey { fn from(_: OsString) -> EnvKey { EnvKey } }
impl From<EnvKey> for OsString { fn from(_: EnvKey) -> OsString { OsString::new() } }
impl AsRef<OsStr> for EnvKey { fn as_ref(&self) -> &OsStr { OsStr::new("") } }

#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
pub struct Stdio;
impl Stdio {
    pub const Null: Stdio = Stdio;
    pub const MakePipe: Stdio = Stdio;
    pub const Inherit: Stdio = Stdio;
}
impl From<AnonPipe> for Stdio {
    fn from(_pipe: AnonPipe) -> Stdio { Stdio }
}
impl From<File> for Stdio {
    fn from(_file: File) -> Stdio { Stdio }
}
impl From<crate::sys::stdio::Stdin> for Stdio { fn from(_: crate::sys::stdio::Stdin) -> Stdio { Stdio } }
impl From<crate::sys::stdio::Stdout> for Stdio { fn from(_: crate::sys::stdio::Stdout) -> Stdio { Stdio } }
impl From<crate::sys::stdio::Stderr> for Stdio { fn from(_: crate::sys::stdio::Stderr) -> Stdio { Stdio } }

impl From<crate::io::Stdout> for Stdio { fn from(_: crate::io::Stdout) -> Stdio { Stdio } }
impl From<crate::io::Stderr> for Stdio { fn from(_: crate::io::Stderr) -> Stdio { Stdio } }
impl From<crate::io::Stdin> for Stdio { fn from(_: crate::io::Stdin) -> Stdio { Stdio } }

impl Process {
    pub fn id(&self) -> u32 { 0 }
    pub fn kill(&mut self) -> io::Result<()> { Ok(()) }
    pub fn wait(&mut self) -> io::Result<ExitStatus> { Ok(ExitStatus(0)) }
    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> { Ok(Some(ExitStatus(0))) }
}

pub fn id() -> u32 { 0 }

pub fn output(cmd: &mut Command) -> io::Result<(ExitStatus, Vec<u8>, Vec<u8>)> {
    let (mut p, _) = cmd.spawn(Stdio::Null, false)?;
    let status = p.wait()?;
    Ok((status, Vec::new(), Vec::new()))
}

pub fn read_output(_pipe: AnonPipe, _out: &mut Vec<u8>, _pipe2: AnonPipe, _out2: &mut Vec<u8>) -> io::Result<()> {
    Ok(())
}
