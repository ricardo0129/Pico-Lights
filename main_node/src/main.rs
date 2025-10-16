use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Barrier, Mutex};
use std::thread;

const NUM_PIXELS: u32 = 50;
const WORKER_COUNT: usize = 2;

struct Request {
    color: u32,
    position: u32,
}
struct Worker {
    id: i32,
    pixel_count: u32,
    queue: Vec<Request>,
}

impl Worker {
    fn new(id: i32) -> Self {
        Worker {
            id: id,
            queue: vec![],
            pixel_count: NUM_PIXELS,
        }
    }
}

struct State {
    workers: Vec<Worker>,
}

fn urgb_u32(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

impl Request {
    fn new(color: u32, position: u32) -> Self {
        Request { color, position }
    }
    fn to_bytes(&self) -> [u8; 8] {
        let mut bytes = [0; 8];
        bytes[0..4].copy_from_slice(&self.position.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.color.to_le_bytes());
        bytes
    }
}

fn add_worker(state: &mut State) -> i32 {
    let id = state.workers.len() as i32 + 1;
    state.workers.push(Worker::new(id));
    id
}
fn update_loop(state: &mut State, tick: u32) {
    let colors: [u32; 3] = [
        urgb_u32(255, 154, 0), // Pumpkin Orange
        urgb_u32(9, 255, 0),   // Spooky Green
        urgb_u32(201, 0, 255), // Witchy Purple
    ];
    for worker in &mut state.workers {
        for i in 0..worker.pixel_count {
            let color = colors[((tick + i) as usize) % colors.len()];
            worker.queue.push(Request::new(color, i));
        }
    }
}

fn main() {
    let barrier = Arc::new(std::sync::Barrier::new(WORKER_COUNT + 1));

    let listener = TcpListener::bind("0.0.0.0:4242").unwrap();
    println!("listening started, ready to accept");
    let state = Arc::new(Mutex::new(State { workers: vec![] }));
    //Start coordinator thread
    {
        let state_ref = state.clone();
        let barrier_ref = barrier.clone();
        thread::spawn(move || {
            coordinator_loop(state_ref, barrier_ref);
        });
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let barrier = barrier.clone();
                let state_ref = state.clone();
                thread::spawn(move || {
                    handle_client(stream, state_ref, barrier);
                });
            }
            Err(e) => {
                eprintln!("failed: {}", e);
            }
        }
    }
}
fn coordinator_loop(mut state: Arc<Mutex<State>>, mut barrier: Arc<Barrier>) {
    let mut tick: u32 = 0;
    loop {
        barrier.wait();
        update_loop(&mut state.lock().unwrap(), tick);
        tick += 1;
        thread::sleep(std::time::Duration::from_millis(500));
        barrier.wait();
    }
}

fn handle_client(
    mut stream: std::net::TcpStream,
    mut state: Arc<Mutex<State>>,
    mut barrier: Arc<Barrier>,
) {
    // Read from stream
    let worker_id = add_worker(&mut state.lock().unwrap());
    while state.lock().unwrap().workers.len() < worker_id as usize {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    println!("Worker {} connected", worker_id);
    let mut start_time = std::time::Instant::now();
    let mut updates: u64 = 0;
    loop {
        let mut buffer: Vec<u8> = vec![];
        let queue_len = state.lock().unwrap().workers[worker_id as usize - 1]
            .queue
            .len();
        buffer.extend_from_slice(&(queue_len as u32).to_le_bytes());

        for update in state.lock().unwrap().workers[worker_id as usize - 1]
            .queue
            .drain(..)
        {
            buffer.extend_from_slice(&update.to_bytes());
        }
        if queue_len > 0 {
            stream.write(&buffer).unwrap();
            let mut receive_buffer = [0; 2];
            stream.read(&mut receive_buffer).unwrap();
        }
        let elapsed = start_time.elapsed();
        if elapsed.as_secs() >= 1 {
            println!(
                "Worker {}: Sent {} updates in the last {} seconds",
                worker_id,
                updates,
                elapsed.as_secs()
            );
            updates = 0;
            start_time = std::time::Instant::now();
        }

        updates += 1;
        barrier.wait();
        barrier.wait();
    }
}
