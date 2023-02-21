pub mod client_pool;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    sync::Arc,
};

use axum::{
    extract::Path,
    http::Method,
    routing::{get, post},
    Extension, Json, Router,
};
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};

use crate::{http::client_pool::ClientPool, server_pool::ServerPool};

struct Pools {
    client_pool: ClientPool<u32>,
    server_pool: ServerPool<u32>,
}

#[derive(Serialize)]
struct ServerPoolState {
    response: Option<u32>,
    info: String,
    servers_state: Vec<(usize, Option<u32>, Option<String>)>,
}

pub async fn run_http_server<T: Send + Sync + Copy + Display + Debug>(
    client_pool: ClientPool<u32>,
    server_pool: ServerPool<u32>,
    port: u32,
) {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app_state = Arc::new(Pools {
        client_pool,
        server_pool,
    });
    let app = Router::new()
        .route("/", get(root))
        .route("/quorum", get(read_quorum))
        .route("/quorum/:data", post(write_quorum))
        .layer(Extension(app_state))
        .layer(cors);

    axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    async fn root() -> String {
        "This is a quorum test".to_string()
    }
}

async fn read_quorum(Extension(pools): Extension<Arc<Pools>>) -> Json<ServerPoolState> {
    let client_pool = &pools.client_pool;
    let server_pool = &pools.server_pool;
    let answer = client_pool.read(server_pool);

    let servers_state = server_pool.get_state();

    match answer {
        Some(data) => Json(ServerPoolState {
            response: Some(data),
            info: "Fetched data".to_string(),
            servers_state,
        }),
        None => Json(ServerPoolState {
            response: None,
            info: "No data".to_string(),
            servers_state,
        }),
    }
}

async fn write_quorum(
    Extension(pools): Extension<Arc<Pools>>,
    Path(params): Path<HashMap<String, String>>,
) -> Json<ServerPoolState> {
    let client_pool = &pools.client_pool;
    let server_pool = &pools.server_pool;

    match params.get("data") {
        Some(data) => {
            let data: u32 = data.parse().unwrap();
            let write_result = client_pool.write(server_pool, data);
            let servers_state = server_pool.get_state();
            match write_result {
                Ok(_) => Json(ServerPoolState {
                    response: None,
                    info: "Written data to server".to_string(),
                    servers_state,
                }),
                Err(_) => Json(ServerPoolState {
                    response: None,
                    info: "Error writing data".to_string(),
                    servers_state,
                }),
            }
        }
        None => {
            let servers_state = server_pool.get_state();
            Json(ServerPoolState {
                response: None,
                info: "Wrong input".to_string(),
                servers_state,
            })
        }
    }
}
