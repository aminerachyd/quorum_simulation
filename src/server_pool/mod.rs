mod server;

use crossbeam_channel::{unbounded, Receiver, Sender};
use rand::{seq::SliceRandom, thread_rng};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    time,
};

use server::Server;

use crate::client::Client;

#[derive(Debug)]
pub struct ServerPool<T: 'static> {
    majority: usize,
    pub servers: Vec<Server<T>>,
    connections: HashMap<u32, (Sender<Option<T>>, Sender<Result<(), ()>>)>,
}

impl<T: Send + Copy + Sync + Debug + Display> ServerPool<T> {
    pub fn new(size: usize, failure_probability: f32) -> ServerPool<T> {
        let mut servers = Vec::with_capacity(size);

        for i in 0..size {
            servers.push(Server::new(i, failure_probability).run());
        }
        ServerPool {
            connections: HashMap::new(),
            majority: size / 2 + 1,
            servers,
        }
    }

    pub fn connect(
        &mut self,
        client: &Client<T>,
    ) -> (Receiver<Option<T>>, Receiver<Result<(), ()>>) {
        let new_read_connection = unbounded();
        let new_write_connection = unbounded();

        self.connections
            .insert(client.id, (new_read_connection.0, new_write_connection.0));

        println!("[ServerPool] Client `{}` connected", &client.id);

        (new_read_connection.1, new_write_connection.1)
    }

    pub fn disconnect(&mut self, client: &Client<T>) {
        self.connections.remove_entry(&client.id);
        println!("[ServerPool] Client `{}` disconnected", &client.id);
    }

    pub fn read(&self, client_id: u32) {
        let mut answers = Vec::with_capacity(self.majority);

        let mut vector_indexes = (0..self.servers.len()).into_iter().collect::<Vec<usize>>();

        vector_indexes.shuffle(&mut thread_rng());

        for index in vector_indexes {
            let server = self.servers.get(index).unwrap();

            match server.read() {
                Ok(data) => {
                    match data.0 {
                        Some(data) => println!("[{}] Read data {}", server.id, data),
                        None => println!("[{}] No data available", server.id),
                    }
                    answers.push(data)
                }
                Err(_) => {
                    println!("[{}] Error reading data", server.id);
                }
            };
            if answers.len() >= self.majority {
                break;
            }
        }

        // Get most recent answer based on timestamps
        let answer = answers
            .iter()
            .fold(&(None, None), |a, b| {
                if a.0.is_none() && a.1.is_none() || a.1.is_none() {
                    b
                } else {
                    if b.1.gt(&a.1) {
                        b
                    } else {
                        a
                    }
                }
            })
            .0;

        // Send the answer to the client
        self.connections.get(&client_id).unwrap().0.send(answer);
    }

    pub fn write(&self, client_id: u32, value: T) {
        let mut answers = Vec::with_capacity(self.majority);

        let mut vector_indexes = (0..self.servers.len()).into_iter().collect::<Vec<usize>>();

        vector_indexes.shuffle(&mut thread_rng());

        let instant_now = time::Instant::now();

        for index in vector_indexes {
            let server = self.servers.get(index).unwrap();

            match server.write((value, instant_now)) {
                Ok(_) => {
                    println!("[{}] Written data {}", server.id, value);
                    answers.push(Ok(()));
                }
                Err(_) => {
                    println!("[{}] Error writing data {}, retrying...", server.id, value);
                    answers.push(Err(()));
                }
            }

            let ok_count = answers
                .clone()
                .into_iter()
                .filter(|&e| e.is_ok())
                .collect::<Vec<Result<_, _>>>()
                .len();

            if ok_count >= self.majority {
                self.connections.get(&client_id).unwrap().1.send(Ok(()));
                return;
            }
        }

        let ok_count = answers
            .into_iter()
            .filter(|&e| e.is_ok())
            .collect::<Vec<Result<_, _>>>()
            .len();

        if ok_count >= self.majority {
            self.connections.get(&client_id).unwrap().1.send(Ok(()));
        } else {
            self.connections.get(&client_id).unwrap().1.send(Err(()));
        }
    }

    pub fn get_state(&self) -> Vec<(usize, Option<T>, Option<String>)> {
        let mut result = Vec::new();
        for server in &self.servers {
            let current_data = *server.current_data.lock().unwrap();

            if current_data.1.is_none() {
                result.push((server.id, current_data.0, None));
            } else {
                result.push((
                    server.id,
                    current_data.0,
                    Some(format!("{:?}", (current_data.1.unwrap()))),
                ));
            }
        }

        result
    }
}
