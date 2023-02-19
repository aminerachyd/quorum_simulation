use std::fmt::{Debug, Display};

use crate::server_pool::ServerPool;

pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Client {}
    }

    pub fn read<'a, T: Send + Copy + Sync + Debug + Display>(
        &'a self,
        server_pool: &'a mut ServerPool<T>,
    ) -> Option<&T> {
        server_pool.read();
        None
    }

    pub fn write<T: Send + Copy + Sync + Debug + Display>(
        &self,
        server_pool: &mut ServerPool<T>,
        value: T,
    ) -> Result<String, String> {
        server_pool.write(value)
    }
}
