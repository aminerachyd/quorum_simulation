mod server;

use std::{
    fmt::{Debug, Display},
    time,
};

use rand::{seq::SliceRandom, thread_rng};
use server::Server;

#[derive(Debug)]
pub struct ServerPool<T: 'static> {
    majority: usize,
    pub servers: Vec<Server<T>>,
}

impl<T: Send + Copy + Sync + Debug + Display> ServerPool<T> {
    pub fn new(size: usize) -> ServerPool<T> {
        let mut servers = Vec::with_capacity(size);

        for i in 0..size {
            servers.push(Server::new(i).run());
        }
        ServerPool {
            majority: size / 2 + 1,
            servers,
        }
    }

    pub fn read(&mut self) -> Option<T> {
        let mut answers = Vec::with_capacity(self.majority);

        self.servers.shuffle(&mut thread_rng());

        for server in &mut self.servers {
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
        answers
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
            .0
    }

    pub fn write(&mut self, value: T) -> Result<(), ()> {
        let mut answers = Vec::with_capacity(self.majority);

        self.servers.shuffle(&mut thread_rng());

        let instant_now = time::Instant::now();

        for server in &mut self.servers {
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
                return Ok(());
            }
        }

        let ok_count = answers
            .into_iter()
            .filter(|&e| e.is_ok())
            .collect::<Vec<Result<_, _>>>()
            .len();

        if ok_count >= self.majority {
            return Ok(());
        } else {
            Err(())
        }
    }
}
