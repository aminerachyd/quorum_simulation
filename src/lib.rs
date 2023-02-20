pub mod client;
pub mod server_pool;
pub mod status;

use std::fmt::{Debug, Display};

use client::Client;
use server_pool::ServerPool;

// Wrappers on top of client read/write to print informations

pub fn w_read<T: Send + Sync + Copy + Debug + Display>(
    client: &Client,
    server_pool: &mut ServerPool<T>,
) {
    println!("[Client] Reading data from server pool");
    match client.read(server_pool) {
        Some(data) => {
            println!("[Client] Read from server pool : `{}`", data)
        }
        None => {
            println!("[Client] No data yet available on server pool")
        }
    }
}

pub fn w_write<T: Send + Sync + Copy + Debug + Display>(
    client: &Client,
    server_pool: &mut ServerPool<T>,
    value: T,
) {
    println!("[Client] Writing data on server pool : `{}`", value);
    match client.write(server_pool, value) {
        Ok(_) => {
            println!(
                "[Client] Successfully written data on server pool : `{}`",
                value
            )
        }
        Err(_) => {
            println!("[Client] Error writing data on server pool : `{}`", value)
        }
    }
}
