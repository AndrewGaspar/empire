use super::{Info, Universe, error::{self, Error}, port::Port};

use std::{io, ffi::OsString, process::Command, sync::{Arc, RwLock, Weak}};

use futures::future;

use tokio::prelude::*;
use tokio_process::CommandExt;

pub struct SpawnCommandInfo {
    command: OsString,
    args: Vec<OsString>,
    max_procs: usize,

    #[allow(dead_code)]
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

pub struct SpawnMultipleResult {
    pub comm: Comm,
    pub results: Vec<super::Result<()>>,
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
    child_commands: Vec<Command>,
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
            child_commands: Vec::new(),
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
            child_commands: Vec::new(),
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

    pub fn port(&self, idx: usize) -> &Option<Port> {
        &self.ports[idx]
    }

    pub fn attach_children(&mut self, commands: Vec<Command>) {
        self.child_commands = commands;
    }

    pub(crate) fn universe(&self) -> Arc<RwLock<Universe>> {
        self.universe
            .upgrade()
            .expect("The MPI Universe has been destroyed - this Comm object was leaked")
    }

    pub fn spawn_multiple_async<'a, I: IntoIterator<Item = SpawnCommandInfo>>(
        &'a self,
        commands: Option<I>,
        root: usize,
    ) -> impl Future<Item = SpawnMultipleResult, Error = super::Error> + 'a {
        // This needs to eventually support intracomms other than MPI_COMM_SELF
        assert!(root == self.rank());
        assert!(self.size() == 1);

        let commands: Vec<_> = commands
            .expect("The root rank must supply commands to run.")
            .into_iter()
            .map(|command| Arc::new(command))
            .collect();

        future::result(Comm::intercomm(self.universe.clone(), 0)).and_then(move |intercomm| {
            let world_size: usize = commands.iter().map(|command| command.max_procs).sum();

            let children: Vec<_> = commands
                .iter()
                .flat_map(|spawn_command| {
                    (0..spawn_command.max_procs)
                        .map(|_| spawn_command.clone())
                        .collect::<Vec<_>>()
                        .into_iter()
                })
                .enumerate()
                .map(|(world_rank, spawn_command)| {
                    future::result(
                        Command::new(&spawn_command.command)
                            .env("EMPIRE_COMM_WORLD_RANK", format!("{}", world_rank))
                            .env("EMPIRE_COMM_WORLD_SIZE", format!("{}", world_size))
                            .env(
                                "EMPIRE_COMM_WORLD_PARENT_PORT",
                                format!("{}", intercomm.port(0).as_ref().unwrap().name()),
                            )
                            .args(&spawn_command.args)
                            .spawn_async(),
                    ).flatten()
                        .map_err(move |err| {
                            if err.kind() == io::ErrorKind::NotFound {
                                Error::CommandNotFound(spawn_command.command.clone())
                            } else {
                                err.into()
                            }
                        })
                        .then(|child_result| {
                            // This is a Result<Result<_>>. This is intentional: we want to track
                            // the result of each rank separately.
                            future::done(Ok(child_result))
                        })
                })
                .collect();

            // Wait for all children to either exit or call MPI_Init (unimplemented)
            future::join_all(children).map(|children_results| SpawnMultipleResult {
                comm: intercomm,
                results: children_results
                    .into_iter()
                    .map(|child_result| -> super::Result<()> {
                        let status = child_result?;

                        if status.success() {
                            Ok(())
                        } else {
                            Err(Error::FailExitCode(status.code().unwrap_or(-1)))
                        }
                    })
                    .collect(),
            })
        })
    }

    pub fn spawn_multiple<I: IntoIterator<Item = SpawnCommandInfo>>(
        &self,
        commands: Option<I>,
        root: usize,
    ) -> super::Result<SpawnMultipleResult> {
        self.spawn_multiple_async(commands, root).wait()
    }

    pub fn spawn_multiple_root_async<'a, I: IntoIterator<Item = SpawnCommandInfo> + 'a>(
        &'a self,
        commands: I,
        root: usize,
    ) -> impl Future<Item = SpawnMultipleResult, Error = super::Error> + 'a {
        assert!(root == self.rank());
        self.spawn_multiple_async(Some(commands), root)
    }

    pub fn spawn_multiple_root<I: IntoIterator<Item = SpawnCommandInfo>>(
        &self,
        commands: I,
        root: usize,
    ) -> super::Result<SpawnMultipleResult> {
        self.spawn_multiple_root_async(commands, root).wait()
    }
}
