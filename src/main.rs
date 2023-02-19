use std::thread;
use std::time::Duration;

use quorum_simulation::client::Client;
use quorum_simulation::server_pool::ServerPool;

fn main() {
    // Create a client
    let client = Client::new();
    // Create a pool of servers
    let mut server_pool: ServerPool<u32> = ServerPool::new(3);

    loop {
        client.read(&mut server_pool);
        thread::sleep(Duration::from_secs(3));
        client.write(&mut server_pool, 3).unwrap();
        thread::sleep(Duration::from_secs(3));
        client.read(&mut server_pool);
        thread::sleep(Duration::from_secs(3));
        client.write(&mut server_pool, 5).unwrap();
        thread::sleep(Duration::from_secs(3));
    }
}
