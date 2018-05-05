use super::{Comm, port::Port};
use super::error;

use std::collections::HashMap;
use std::ffi::OsString;
use std::sync::{Arc, RwLock};

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

    fn initialize(self_lock: &Arc<RwLock<Self>>) {
        let comm_self_universe = Arc::downgrade(&self_lock);
        let comm_world_universe = Arc::downgrade(&self_lock);

        let mut locked = self_lock.write().unwrap();

        locked.comm_self = Some(Comm::intracomm(comm_self_universe, 0, 1));
        locked.comm_world = Some(Comm::intracomm(comm_world_universe, 0, 1));
    }

    pub fn new() -> Arc<RwLock<Self>> {
        let universe = Arc::new(RwLock::new(Universe::empty()));

        Universe::initialize(&universe);

        universe
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
