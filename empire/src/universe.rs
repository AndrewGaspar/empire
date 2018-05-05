use super::{Comm, port::Port};
use super::error;

use std::collections::HashMap;
use std::env;
use std::num::ParseIntError;
use std::str::FromStr;
use std::sync::{Arc, RwLock};

fn read_integer_variable<F: FromStr<Err = ParseIntError>>(var_name: &str, default: F) -> F {
    match env::var(var_name) {
        Ok(world_size) => world_size
            .parse()
            .expect(&format!("{} was not an integer.", var_name)),
        Err(err) => match err {
            env::VarError::NotPresent => {
                eprintln!("Warning: the environment is not correctly configured.");
                default
            }
            env::VarError::NotUnicode(_) => panic!("{} could not be interepreted.", var_name),
        },
    }
}

pub struct Universe {
    // ports
    ports: HashMap<String, Box<Port>>,

    // standard communicators
    comm_self: Option<Comm>,
    comm_world: Option<Comm>,
}

impl Universe {
    fn empty() -> Self {
        Self {
            ports: HashMap::new(),
            comm_self: None,
            comm_world: None,
        }
    }

    fn initialize_comm_self(universe: &Arc<RwLock<Self>>) -> error::Result<()> {
        let comm_self_universe = Arc::downgrade(&universe);

        let mut locked = universe.write().unwrap();
        locked.comm_self = Some(Comm::intracomm(comm_self_universe, 0, 1)?);

        Ok(())
    }

    fn initialize_comm_world(
        universe: &Arc<RwLock<Self>>,
        rank: usize,
        size: usize,
    ) -> error::Result<()> {
        let comm_world_universe = Arc::downgrade(&universe);

        let mut locked = universe.write().unwrap();
        locked.comm_world = Some(Comm::intracomm(comm_world_universe, rank, size)?);

        Ok(())
    }

    pub fn new() -> error::Result<Arc<RwLock<Self>>> {
        let universe = Arc::new(RwLock::new(Universe::empty()));

        Self::initialize_comm_self(&universe)?;
        Self::initialize_comm_world(&universe, 0, 1)?;

        Ok(universe)
    }

    pub fn from_env() -> error::Result<Arc<RwLock<Self>>> {
        let universe = Arc::new(RwLock::new(Universe::empty()));

        Self::initialize_comm_self(&universe)?;

        let rank = read_integer_variable("EMPIRE_COMM_WORLD_RANK", 0usize);
        let size = read_integer_variable("EMPIRE_COMM_WORLD_SIZE", 1usize);

        Self::initialize_comm_world(&universe, rank, size)?;

        Ok(universe)
    }

    pub fn comm_self_opt(&self) -> &Option<Comm> {
        &self.comm_self
    }

    pub fn comm_world_opt(&self) -> &Option<Comm> {
        &self.comm_world
    }

    pub fn comm_self(&self) -> &Comm {
        self.comm_self.as_ref().unwrap()
    }

    pub fn comm_world(&self) -> &Comm {
        self.comm_world.as_ref().unwrap()
    }

    pub fn open_port(&mut self) -> error::Result<&Port> {
        let port = Box::new(Port::new()?);
        let port_name = port.name().to_owned();
        self.ports.insert(port_name.clone(), port);
        Ok(self.ports.get(&port_name).unwrap())
    }

    pub fn close_port(&mut self, port_name: &str) -> error::Result<()> {
        match self.ports.remove(port_name) {
            Some(_) => Ok(()),
            None => Err(error::Error::NoSuchPort(port_name.to_owned())),
        }
    }
}
