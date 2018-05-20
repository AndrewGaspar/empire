use super::error;

use futures::sync::oneshot;
use std::thread;
use tokio::{self, net::{TcpListener, TcpStream}, prelude::*};

enum ServerEvent {
    Connection(TcpStream),
    Terminal,
}

pub struct Port {
    name: String,
    terminal: Option<oneshot::Sender<()>>,
    server_thread: Option<thread::JoinHandle<()>>,
}

fn process_new_connection(connection: TcpStream) -> impl Future<Item = (), Error = ()> {
    future::done::<(), ()>(Ok(()))
}

impl Port {
    pub fn new() -> error::Result<Port> {
        let listener = match TcpListener::bind(&("127.0.0.1:0".parse().unwrap())) {
            Ok(listener) => listener,
            Err(io) => return Err(error::Error::IoError(io)),
        };

        let name = format!("{}", listener.local_addr()?);

        let (terminal, receiver) = oneshot::channel();

        let server_thread = thread::spawn(move || {
            let server = listener
                .incoming()
                .map(|stream| ServerEvent::Connection(stream))
                .select(
                    receiver
                        .then(|_| future::done(Ok(ServerEvent::Terminal)))
                        .into_stream(),
                )
                .map_err(|err| {
                    println!("Server encountered error: {}", err);
                })
                .take_while(|event| {
                    future::ok(match event {
                        ServerEvent::Terminal => false,
                        _ => true,
                    })
                })
                .map(|event| match event {
                    ServerEvent::Connection(connection) => connection,
                    _ => panic!("EMPIRE internal error: Terminal events should be filtered out"),
                })
                .for_each(|connection| tokio::spawn(process_new_connection(connection)));

            tokio::run(server);
        });

        Ok(Port {
            name,
            terminal: Some(terminal),
            server_thread: Some(server_thread),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Drop for Port {
    fn drop(&mut self) {
        self.terminal.take().unwrap().send(()).unwrap();
        self.server_thread
            .take()
            .unwrap()
            .join()
            .expect("Server thread did not exit successfully.");
    }
}
