use std::fmt::{Debug, Display};

use rand::Rng;

use crate::{client::Client, server_pool::ServerPool};

pub struct ClientPool<T: 'static> {
    clients: Vec<Client<T>>,
}

impl<T: Send + Sync + Display + Debug + Copy> ClientPool<T> {
    pub fn new(size: usize, server_pool: &mut ServerPool<T>) -> ClientPool<T> {
        let mut clients = Vec::with_capacity(size);

        for i in 0..size {
            let mut client = Client::new(i as u32);
            client.connect(server_pool);
            clients.push(client);
        }

        ClientPool { clients }
    }

    pub fn get_client(&self) -> &Client<T> {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..self.clients.len());

        self.clients.get(random_index).unwrap()
    }

    pub fn read(&self, server_pool: &ServerPool<T>) -> Option<T> {
        self.get_client().read(server_pool)
    }

    pub fn write(&self, server_pool: &ServerPool<T>, value: T) -> Result<(), ()> {
        self.get_client().write(server_pool, value)
    }
}
