use super::error;

use std::net::TcpListener;

pub struct Port {
    name: String,
    listener: TcpListener,
}

impl Port {
    pub fn new() -> error::Result<Port> {
        let listener = match TcpListener::bind("127.0.0.1:0") {
            Ok(listener) => listener,
            Err(io) => return Err(error::Error::IoError(io)),
        };

        Ok(Port {
            name: format!("{}", listener.local_addr()?),
            listener,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
