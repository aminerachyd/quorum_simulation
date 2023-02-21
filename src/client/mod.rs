use std::fmt::{Debug, Display};

use crossbeam_channel::Receiver;

use crate::server_pool::ServerPool;

#[derive(Clone)]
pub struct Client<T> {
    pub id: u32,
    receiver: Option<(Receiver<Option<T>>, Receiver<Result<(), ()>>)>,
}

impl<T: Send + Copy + Sync + Debug + Display> Client<T> {
    pub fn new(id: u32) -> Self {
        Client { id, receiver: None }
    }

    pub fn read(&self, server_pool: &ServerPool<T>) -> Option<T> {
        server_pool.read(self.id);
        self.receiver.as_ref().unwrap().0.recv().unwrap()
    }

    pub fn write(&self, server_pool: &ServerPool<T>, value: T) -> Result<(), ()> {
        server_pool.write(self.id, value);
        self.receiver.as_ref().unwrap().1.recv().unwrap()
    }

    pub fn connect(&mut self, server_pool: &mut ServerPool<T>) {
        self.receiver = Some(server_pool.connect(self));
    }

    pub fn disconnect(&mut self, server_pool: &mut ServerPool<T>) {
        server_pool.disconnect(self);
        self.receiver = None;
    }
}
