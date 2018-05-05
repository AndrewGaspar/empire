use super::Comm;

use std::ffi::OsString;
use std::sync::{Arc, RwLock};

pub struct UniverseBuilder {}

impl UniverseBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self) -> Arc<RwLock<Universe>> {
        let universe = Arc::new(RwLock::new(Universe::empty()));

        Universe::initialize(&universe);

        universe
    }
}

pub struct Universe {
    comm_self: Option<Comm>,
    comm_world: Option<Comm>,
}

impl Universe {
    fn empty() -> Self {
        Self {
            comm_self: None,
            comm_world: None,
        }
    }

    fn initialize(self_lock: &Arc<RwLock<Self>>) {
        let comm_self_universe = Arc::downgrade(&self_lock);
        let comm_world_universe = Arc::downgrade(&self_lock);

        let mut locked = self_lock.write().unwrap();

        locked.comm_self = Some(Comm::new(comm_self_universe, 0, 1));
        locked.comm_world = Some(Comm::new(comm_world_universe, 0, 1));
    }

    pub fn new() -> Arc<RwLock<Self>> {
        UniverseBuilder::new().build()
    }

    pub fn from_args<I: IntoIterator<Item = String>>(_: I) -> Arc<RwLock<Self>> {
        UniverseBuilder::new().build()
    }

    pub fn from_args_os<I: IntoIterator<Item = OsString>>(_: I) -> Arc<RwLock<Self>> {
        UniverseBuilder::new().build()
    }

    pub fn comm_self(&self) -> &Comm {
        self.comm_self.as_ref().unwrap()
    }

    pub fn comm_world(&self) -> &Comm {
        self.comm_world.as_ref().unwrap()
    }
}
