use quorum_simulation::Client;
use quorum_simulation::ServerPool;

fn main() {
    // Create a client
    let client = Client::new();
    // Create a pool of servers
    let mut server_pool = ServerPool::new(5);

    assert_eq!(None, client.read::<i32>(&server_pool));
    client.write(&mut server_pool, 3).unwrap();
    assert_eq!(Some(&3), client.read(&server_pool));
    client.write(&mut server_pool, 5).unwrap();
    assert_eq!(Some(&5), client.read(&server_pool));
}
