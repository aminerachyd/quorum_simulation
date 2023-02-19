use std::fmt::{Debug, Display};

use crate::server_pool::ServerPool;

pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Client {}
    }

    pub fn read<T: Send + Copy + Sync + Debug + Display>(
        &self,
        server_pool: &mut ServerPool<T>,
    ) -> Option<T> {
        server_pool.read()
    }

    pub fn write<T: Send + Copy + Sync + Debug + Display>(
        &self,
        server_pool: &mut ServerPool<T>,
        value: T,
    ) -> Result<(), ()> {
        server_pool.write(value)
    }
}
