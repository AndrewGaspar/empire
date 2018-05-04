use super::{Error, Info, Universe};

use std::process::Command;
use std::ffi::OsString;
use std::io;
use std::sync::{Arc, RwLock};

pub struct SpawnCommandInfo {
    command: OsString,
    args: Vec<OsString>,
    max_procs: usize,
    info: Option<Info>,
}

impl SpawnCommandInfo {
    pub fn new<I: IntoIterator<Item = T>, T: Into<OsString> + Clone>(
        command: OsString,
        itr: I,
    ) -> Self {
        Self {
            command,
            args: itr.into_iter().map(|arg| arg.into()).collect(),
            max_procs: 1,
            info: None,
        }
    }

    pub fn max_procs(&mut self, max_procs: usize) -> &mut Self {
        self.max_procs = max_procs;
        self
    }
}

pub struct Comm {
    universe: Arc<RwLock<Universe>>,
    rank: usize,
    size: usize,
}

impl Comm {
    pub(crate) fn new(universe: Arc<RwLock<Universe>>, rank: usize, size: usize) -> Self {
        Self {
            universe,
            rank,
            size,
        }
    }

    pub fn rank(&self) -> usize {
        self.rank
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn spawn_multiple<'b, I: IntoIterator<Item = SpawnCommandInfo>>(
        &self,
        commands: Option<I>,
        root: usize,
    ) -> super::Result<Comm> {
        let mut spawned = Vec::new();

        if root == self.rank {
            let commands = commands.expect("The root rank must supply commands to run.");

            for spawn_command in commands.into_iter() {
                for _ in 0..spawn_command.max_procs {
                    let child = match Command::new(&spawn_command.command)
                        .args(&spawn_command.args)
                        .spawn()
                    {
                        Ok(child) => child,
                        Err(ref err) if err.kind() == io::ErrorKind::NotFound => {
                            return Err(Error::CommandNotFound(spawn_command.command.clone()));
                        }
                        Err(err) => return Err(err.into()),
                    };

                    spawned.push(child);
                }
            }
        }

        for mut child in spawned {
            child.wait().unwrap();
        }

        Ok(Comm::new(self.universe.clone(), 0, 2))
    }

    pub fn spawn_multiple_root<'b, I: IntoIterator<Item = SpawnCommandInfo>>(
        &self,
        commands: I,
        root: usize,
    ) -> super::Result<Comm> {
        self.spawn_multiple(Some(commands), root)
    }
}
