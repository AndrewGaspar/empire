use super::{error, registrar, Comm, port::Port};

use std::{env, collections::HashMap, num::ParseIntError, str::FromStr, sync::{Arc, RwLock}};

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

pub struct CommRegistration(registrar::Registration<Comm>);

impl CommRegistration {
    pub fn unwrap(&self) -> Arc<Comm> {
        self.0
            .get()
            .upgrade()
            .expect("The communicator has already been freed.")
    }
}

pub struct Universe {
    // ports
    ports: HashMap<String, Box<Port>>,

    // standard communicators
    comm_self: Option<CommRegistration>,
    comm_world: Option<CommRegistration>,
    comm_parent: Option<CommRegistration>,

    // communicator strong references are held by Universe so that if the Universe is destroyed
    // these communicators get cleaned up correctly, and destroying user references will result in
    // a reasonable
    registrar: registrar::Registrar<Comm>,
}

impl Universe {
    fn new() -> error::Result<Self> {
        Ok(Self {
            ports: HashMap::new(),
            comm_self: None,
            comm_world: None,
            comm_parent: None,
            registrar: registrar::Registrar::new(),
        })
    }

    fn initialize_comm_self(universe: &Arc<RwLock<Self>>) -> error::Result<()> {
        let comm_self_universe = Arc::downgrade(&universe);

        let mut locked = universe.write().unwrap();
        let registration = locked.register_comm(Comm::intracomm(comm_self_universe, 0, 1)?);
        locked.comm_self = Some(registration);

        Ok(())
    }

    fn initialize_comm_world(
        universe: &Arc<RwLock<Self>>,
        rank: usize,
        size: usize,
    ) -> error::Result<()> {
        let comm_world_universe = Arc::downgrade(&universe);

        let mut locked = universe.write().unwrap();
        let registration = locked.register_comm(Comm::intracomm(comm_world_universe, rank, size)?);
        locked.comm_world = Some(registration);

        Ok(())
    }

    pub fn root() -> error::Result<Arc<RwLock<Self>>> {
        let universe = Arc::new(RwLock::new(Universe::new()?));

        Self::initialize_comm_self(&universe)?;
        Self::initialize_comm_world(&universe, 0, 1)?;

        Ok(universe)
    }

    pub fn from_env() -> error::Result<Arc<RwLock<Self>>> {
        let universe = Arc::new(RwLock::new(Universe::new()?));

        Self::initialize_comm_self(&universe)?;

        let rank = read_integer_variable("EMPIRE_COMM_WORLD_RANK", 0usize);
        let size = read_integer_variable("EMPIRE_COMM_WORLD_SIZE", 1usize);

        Self::initialize_comm_world(&universe, rank, size)?;

        Ok(universe)
    }

    pub fn comm_self(&self) -> Arc<Comm> {
        self.comm_self.as_ref().unwrap().unwrap()
    }

    pub fn comm_world(&self) -> Arc<Comm> {
        self.comm_world.as_ref().unwrap().unwrap()
    }

    pub fn register_comm(&mut self, comm: Comm) -> CommRegistration {
        CommRegistration(self.registrar.track_object(comm))
    }

    pub fn free_comm(&mut self, registration: CommRegistration) {
        self.registrar.free_object(registration.0)
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
