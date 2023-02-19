use std::{
    fmt::{Debug, Display},
    sync::{Arc, Mutex},
    thread,
    thread::JoinHandle,
    time::Duration,
};

#[derive(Debug)]
pub struct Server<T> {
    id: usize,
    _thread: JoinHandle<()>,
    event: Arc<Mutex<EventType<T>>>,
    current_data: Arc<Mutex<Option<T>>>,
}
#[derive(Debug, Clone)]
enum EventType<T> {
    INIT,
    READ,
    IDLE,
    WRITE(T),
}
// Client > Writes to ServerPool > Writes to Servers
// Client > Reads from ServerPool > Reads from Servers

impl<T: Sync + Send + Display + Clone + Debug + 'static> Server<T> {
    pub fn new(id: usize) -> Server<T> {
        let _thread = thread::spawn(|| {});
        Server {
            id,
            current_data: Arc::new(Mutex::new(None)),
            event: Arc::new(Mutex::new(EventType::INIT)),
            _thread,
        }
    }

    pub fn run(mut self) -> Server<T> {
        let id = self.id.clone();
        let event = Arc::clone(&mut self.event);
        let current_data = Arc::clone(&self.current_data);
        let _thread = thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(50));
            let mut event = event.lock().unwrap();
            match &*event {
                EventType::INIT => {
                    println!("[{}] Started server", id);
                    *event = EventType::IDLE;
                }
                EventType::READ => {
                    let current_data = current_data.lock().unwrap();
                    match &*current_data {
                        Some(data) => {
                            println!("[{}] Read data {}", id, data)
                        }
                        None => {
                            println!("[{}] No data available", id)
                        }
                    };
                    *event = EventType::IDLE;
                }
                EventType::WRITE(data) => {
                    let mut current_data = current_data.lock().unwrap();
                    *current_data = Some(data.clone());
                    println!("[{}] Written data {}", id, data);
                    *event = EventType::IDLE;
                }
                EventType::IDLE => {
                    println!("[{}] Idling", id);
                    thread::sleep(Duration::from_secs(3));
                }
            }
        });
        Server { _thread, ..self }
    }

    pub fn read(&mut self) {
        let mut event = (&self.event).lock().unwrap();
        println!("[{}] Reading...", self.id);
        *event = EventType::READ;
    }

    pub fn write(&mut self, value: T) {
        println!("[{}] Writing...", self.id);
        let mut event = self.event.lock().unwrap();
        *event = EventType::WRITE(value);
    }
}
