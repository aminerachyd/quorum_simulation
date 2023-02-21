pub mod client_pool;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    sync::Arc,
};

use axum::{
    extract::Path,
    routing::{get, post},
    Extension, Router,
};

use crate::{http::client_pool::ClientPool, server_pool::ServerPool};

struct Pools {
    client_pool: ClientPool<u32>,
    server_pool: ServerPool<u32>,
}

pub async fn run_http_server<T: Send + Sync + Copy + Display + Debug>(
    client_pool: ClientPool<u32>,
    server_pool: ServerPool<u32>,
    port: u32,
) {
    let app_state = Arc::new(Pools {
        client_pool,
        server_pool,
    });
    let app = Router::new()
        .route("/", get(root))
        .route("/quorum", get(read_quorum))
        .route("/quorum/:data", post(write_quorum))
        .layer(Extension(app_state));

    axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    async fn root() -> String {
        "This is a quorum test".to_string()
    }
}

async fn read_quorum(Extension(pools): Extension<Arc<Pools>>) -> Result<String, String> {
    let client_pool = &pools.client_pool;
    let server_pool = &pools.server_pool;
    let answer = client_pool.read(server_pool);
    match answer {
        Some(data) => Ok(data.to_string()),
        None => Err("No data".to_string()),
    }
}

async fn write_quorum(
    Extension(pools): Extension<Arc<Pools>>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<String, String> {
    let client_pool = &pools.client_pool;
    let server_pool = &pools.server_pool;

    match params.get("data") {
        Some(data) => {
            let data: u32 = data.parse().unwrap();
            let write_result = client_pool.write(server_pool, data);
            match write_result {
                Ok(_) => Ok("Written data".to_string()),
                Err(_) => Err("Error writing data".to_string()),
            }
        }
        None => Err("Wrong input".to_string()),
    }
}
