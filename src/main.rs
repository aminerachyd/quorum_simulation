use quorum_simulation::http::client_pool::ClientPool;
use quorum_simulation::http::run_http_server;
use quorum_simulation::server_pool::ServerPool;

#[tokio::main]
async fn main() {
    // Create a pool of servers
    let mut server_pool: ServerPool<u32> = ServerPool::new(10, 0.20);
    let client_pool = ClientPool::new(2, &mut server_pool);

    let port = 8080;
    run_http_server::<u32>(client_pool, server_pool, port).await;
}
