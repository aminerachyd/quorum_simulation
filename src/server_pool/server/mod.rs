use std::{
    fmt::{Debug, Display},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    thread::JoinHandle,
    time::{self, Duration, Instant},
};

#[derive(Debug)]
pub struct Server<T> {
    pub id: usize,
    _thread: JoinHandle<()>,
    event: Arc<Mutex<EventType<T>>>,
    current_data: Arc<Mutex<(Option<T>, Instant)>>,
    read_channel: (Sender<(Option<T>, Instant)>, Receiver<(Option<T>, Instant)>),
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
    pub fn new(id: usize) -> Server<T> {
        let _thread = thread::spawn(|| {});
        Server {
            id,
            current_data: Arc::new(Mutex::new((None, time::Instant::now()))),
            event: Arc::new(Mutex::new(EventType::INIT)),
            _thread,
            read_channel: channel(),
            write_channel: channel(),
        }
    }

    pub fn run(mut self) -> Server<T> {
        let id = self.id.clone();
        let event = Arc::clone(&mut self.event);
        let current_data = Arc::clone(&self.current_data);
        let read_sender = self.read_channel.0.clone();
        let write_sender = self.write_channel.0.clone();

        let _thread = thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(50));
            let mut event = event.lock().unwrap();
            match &*event {
                EventType::INIT => {
                    println!("[{}] Started server", id);
                    *event = EventType::IDLE;
                }
                EventType::IDLE => {
                    println!("[{}] Idling", id);
                    thread::sleep(Duration::from_secs(3));
                }
                EventType::READ => {
                    let current_data = current_data.lock().unwrap();
                    read_sender.send((*current_data).clone()).unwrap();

                    *event = EventType::IDLE;
                }
                EventType::WRITE(data, instant) => {
                    let mut current_data = current_data.lock().unwrap();
                    if instant.ge(&current_data.1) {
                        *current_data = (Some(data.clone()), instant.clone());
                    }
                    write_sender.send(Ok(())).unwrap();

                    *event = EventType::IDLE;
                }
            }
        });
        Server { _thread, ..self }
    }

    pub fn read(&mut self) -> Result<(Option<T>, Instant), ()> {
        let mut event = (&self.event).lock().unwrap();
        println!("[{}] Reading...", self.id);
        *event = EventType::READ;
        let receiver = &self.read_channel.1;
        // try_recv instead of recv because recv is a blocking call and we're holding a mutex
        match receiver.try_recv() {
            Ok(data) => Ok(data),
            Err(_) => Err(()),
        }
    }

    pub fn write(&mut self, value: (T, Instant)) -> Result<(), ()> {
        println!("[{}] Writing...", self.id);
        let mut event = self.event.lock().unwrap();
        *event = EventType::WRITE(value.0, value.1);
        let receiver = &self.write_channel.1;
        receiver.try_recv().unwrap_or(Err(()))
    }
}
