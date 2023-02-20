use std::thread;
use std::time::Duration;

use quorum_simulation::client::Client;
use quorum_simulation::server_pool::ServerPool;
use quorum_simulation::{w_read, w_write};

fn main() {
    // Create a client
    let client = Client::new();
    // Create a pool of servers
    let mut server_pool: ServerPool<u32> = ServerPool::new(7);

    let mut i = 0;

    loop {
        i += 1;

        w_read(&client, &mut server_pool);
        thread::sleep(Duration::from_secs(1));

        w_write(&client, &mut server_pool, i);
        thread::sleep(Duration::from_secs(1));
    }
}
