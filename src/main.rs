use quorum_simulation::http::client_pool::ClientPool;
use quorum_simulation::http::run_http_server;
use quorum_simulation::server_pool::ServerPool;

#[tokio::main]
async fn main() {
    // Create a pool of servers
    let mut server_pool: ServerPool<u32> = ServerPool::new(7, 0.20);
    let client_pool = ClientPool::new(2, &mut server_pool);

    // // Create clients
    // let mut client1 = Client::new(1);
    // let mut client2 = Client::new(2);

    // client1.connect(&mut server_pool);
    // client2.connect(&mut server_pool);

    // let mut i = 1;
    // loop {
    //     w_write(&client1, &mut server_pool, i);

    //     w_read(&client2, &mut server_pool);

    //     i += 1;
    // }

    let port = 3000;
    run_http_server::<u32>(client_pool, server_pool, port).await;
}
