mod server;

use std::fmt::{Debug, Display};

use server::Server;

#[derive(Debug)]
pub struct ServerPool<T: 'static> {
    servers: Vec<Server<T>>,
}

impl<T: Send + Copy + Sync + Debug + Display> ServerPool<T> {
    pub fn new(size: usize) -> ServerPool<T> {
        let mut servers = Vec::with_capacity(size);
        for i in 0..size {
            servers.push(Server::new(i).run());
        }
        ServerPool { servers }
    }

    pub fn read(&mut self) -> Option<&T> {
        for server in &mut self.servers {
            server.read();
        }
        None
    }

    pub fn write(&mut self, value: T) -> Result<String, String> {
        for server in &mut self.servers {
            server.write(value);
        }

        Ok(String::from("Success"))
    }
}
