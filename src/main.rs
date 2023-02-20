use axum::routing::{get, post};
use quorum_simulation::client::Client;
use quorum_simulation::server_pool::ServerPool;
use quorum_simulation::{w_read, w_write};

fn main() {
    // Create a pool of servers
    let mut server_pool: ServerPool<u32> = ServerPool::new(3, 0.20);
    // Create a client
    let mut client = Client::new(1);
    client.connect(&mut server_pool);

    let mut i = 1;
    loop {
        w_write(&client, &mut server_pool, i);

        w_read(&client, &mut server_pool);

        i += 1;
    }
}
