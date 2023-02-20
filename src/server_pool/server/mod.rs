use std::{
    fmt::{Debug, Display},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    thread::JoinHandle,
    time::{Duration, Instant},
};

use rand::Rng;

#[derive(Debug)]
pub struct Server<T> {
    pub id: usize,
    failure_probability: f32,
    _thread: JoinHandle<()>,
    event: Arc<Mutex<EventType<T>>>,
    pub current_data: Arc<Mutex<(Option<T>, Option<Instant>)>>,
    read_channel: (
        Sender<(Option<T>, Option<Instant>)>,
        Receiver<(Option<T>, Option<Instant>)>,
    ),
    write_channel: (Sender<Result<(), ()>>, Receiver<Result<(), ()>>),
}
#[derive(Debug, Clone)]
enum EventType<T> {
    INIT,
    READ,
    IDLE,
    WRITE(T, Instant),
}

// Client > Writes to ServerPool > Writes to Servers
// Client > Reads from ServerPool > Reads from Servers
impl<T: Sync + Send + Display + Clone + Debug + 'static> Server<T> {
    pub fn new(id: usize, failure_probability: f32) -> Server<T> {
        Server {
            id,
            failure_probability,
            current_data: Arc::new(Mutex::new((None, None))),
            event: Arc::new(Mutex::new(EventType::INIT)),
            _thread: thread::spawn(|| {}),
            read_channel: channel(),
            write_channel: channel(),
        }
    }

    pub fn run(mut self) -> Self {
        let id = self.id.clone();
        let event = Arc::clone(&mut self.event);
        let current_data = Arc::clone(&self.current_data);
        let read_sender = self.read_channel.0.clone();
        let write_sender = self.write_channel.0.clone();

        let _thread = thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(50));

            // Failure simulation
            let mut rng = rand::thread_rng();
            let failure = rng.gen_range(0.0..1.0);
            if failure > self.failure_probability {
                continue;
            }

            let mut event = event.lock().unwrap();
            match &*event {
                EventType::INIT => {
                    println!("[{}] Started server", id);
                    *event = EventType::IDLE;
                }
                EventType::IDLE => {
                    thread::sleep(Duration::from_secs(3));
                }
                EventType::READ => {
                    let current_data = current_data.lock().unwrap();
                    read_sender.send((*current_data).clone()).unwrap();

                    *event = EventType::IDLE;
                }
                EventType::WRITE(data, instant) => {
                    let mut current_data = current_data.lock().unwrap();
                    match current_data.1 {
                        Some(data_instant) => {
                            if instant.ge(&data_instant) {
                                *current_data = (Some(data.clone()), Some(instant.clone()));
                            }
                        }
                        None => {
                            *current_data = (Some(data.clone()), Some(instant.clone()));
                        }
                    }
                    write_sender.send(Ok(())).unwrap();

                    *event = EventType::IDLE;
                }
            }
        });
        Server { _thread, ..self }
    }

    pub fn read(&self) -> Result<(Option<T>, Option<Instant>), ()> {
        let mut event = (&self.event).lock().unwrap();
        println!("[{}] Reading...", self.id);
        *event = EventType::READ;
        drop(event);
        let receiver = &self.read_channel.1;
        match receiver.recv() {
            Ok(data) => Ok(data),
            Err(_) => Err(()),
        }
    }

    pub fn write(&self, value: (T, Instant)) -> Result<(), ()> {
        println!("[{}] Writing...", self.id);
        let mut event = self.event.lock().unwrap();
        *event = EventType::WRITE(value.0, value.1);
        drop(event);
        let receiver = &self.write_channel.1;
        receiver.recv().unwrap_or(Err(()))
    }
}
