use super::{Info, Universe};
use super::error::{self, Error};
use super::port::Port;

use std::process::Command;
use std::ffi::OsString;
use std::io;
use std::sync::{RwLock, Weak};

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
    universe: Weak<RwLock<Universe>>,

    // properties
    name: Option<String>,
    rank: usize,
    size: usize,
    is_intercomm: bool,

    // tracking state
    ports: Vec<Option<Port>>,
}

impl Comm {
    pub(crate) fn intracomm(
        universe: Weak<RwLock<Universe>>,
        rank: usize,
        size: usize,
    ) -> error::Result<Self> {
        assert!(rank < size);

        let mut ports = Vec::new();
        for _ in 0..size {
            ports.push(None);
        }
        ports[rank] = Some(Port::new()?);

        Ok(Self {
            universe,
            name: None,
            rank,
            size,
            is_intercomm: false,
            ports,
        })
    }

    pub(crate) fn intercomm(universe: Weak<RwLock<Universe>>, rank: usize) -> error::Result<Self> {
        assert!(rank < 2);

        Ok(Self {
            universe,
            name: None,
            rank,
            size: 2,
            is_intercomm: true,
            ports: vec![Some(Port::new()?), None],
        })
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|name| name.as_str())
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn rank(&self) -> usize {
        self.rank
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_intercomm(&self) -> bool {
        self.is_intercomm
    }

    pub fn spawn_multiple<'b, I: IntoIterator<Item = SpawnCommandInfo>>(
        &self,
        commands: Option<I>,
        root: usize,
    ) -> super::Result<Comm> {
        let mut spawned = Vec::new();

        if root == self.rank {
            let commands = commands.expect("The root rank must supply commands to run.");

            let commands: Vec<_> = commands.into_iter().collect();

            let world_size: usize = commands.iter().map(|command| command.max_procs).sum();

            let mut world_rank = 0;
            for spawn_command in &commands {
                for _ in 0..spawn_command.max_procs {
                    let child = match Command::new(&spawn_command.command)
                        .env("EMPIRE_COMM_WORLD_RANK", format!("{}", world_rank))
                        .env("EMPIRE_COMM_WORLD_SIZE", format!("{}", world_size))
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

                    world_rank += 1;
                }
            }
        }

        for mut child in spawned {
            child.wait().unwrap();
        }

        Comm::intercomm(self.universe.clone(), 0)
    }

    pub fn spawn_multiple_root<'b, I: IntoIterator<Item = SpawnCommandInfo>>(
        &self,
        commands: I,
        root: usize,
    ) -> super::Result<Comm> {
        self.spawn_multiple(Some(commands), root)
    }
}
