use std::fmt::Error;

pub struct Client {}
#[derive(Debug)]
pub struct ServerPool<T> {
    servers: Vec<Server<T>>,
}

#[derive(Debug, Clone)]
struct Server<T> {
    current_data: Option<T>,
}

// Client > Writes to ServerPool > Writes to Servers
// Client > Reads from ServerPool > Reads from Servers

impl Client {
    pub fn new() -> Self {
        Client {}
    }

    pub fn read<'a, T: Clone>(&'a self, server_pool: &'a ServerPool<T>) -> Option<&T> {
        server_pool.read()
    }

    pub fn write<T: Clone>(
        &self,
        server_pool: &mut ServerPool<T>,
        value: T,
    ) -> Result<String, Error> {
        server_pool.write(value)
    }
}

impl<T: Clone> ServerPool<T> {
    pub fn new(size: usize) -> ServerPool<T> {
        let servers = vec![Server::new(); size];
        ServerPool { servers }
    }

    pub fn read(&self) -> Option<&T> {
        let mut result = None;
        for server in &self.servers {
            result = server.read()
        }
        result
    }

    pub fn write(&mut self, value: T) -> Result<String, Error> {
        for server in &mut self.servers {
            server.write(value.clone());
        }
        Ok(String::from("Success"))
    }
}

impl<T> Server<T> {
    fn new() -> Server<T> {
        Server { current_data: None }
    }

    fn read(&self) -> Option<&T> {
        self.current_data.as_ref()
    }

    fn write(&mut self, value: T) {
        self.current_data = Some(value);
    }
}
