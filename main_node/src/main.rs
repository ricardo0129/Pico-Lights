use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Barrier, Mutex};
use std::thread;

const NUM_PIXELS: u32 = 50;

struct Request {
    color: u32,
    position: u32,
}
struct Worker {
    id: i32,
    pixel_count: u32,
    queue: Vec<Request>,
    state: i32,
}

impl Worker {
    fn new(id: i32, state: i32) -> Self {
        Worker {
            id: id,
            queue: vec![],
            pixel_count: NUM_PIXELS,
            state: state,
        }
    }
}

struct State {
    workers: BTreeMap<i32, Worker>,
    color: [u32; 3],
    global_index: i32,
    state: i32,
}

impl State {
    fn new() -> Self {
        State {
            workers: BTreeMap::new(),
            color: [
                urgb_u32(255, 64, 0), // Pumpkin Orange
                urgb_u32(93, 4, 217), // Witchy Purple
                urgb_u32(0, 200, 0),  // Spooky Green
            ],
            global_index: 0,
            state: 1,
        }
    }
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
    state.global_index += 1;
    let id = state.global_index;
    let new_worker = Worker::new(id, 1);
    state.workers.insert(id, new_worker);
    id
}

fn update_loop(state: &mut State, tick: u32, worker_id: i32) {
    // If state is inactive, return
    if state.state == 0 && state.workers.get(&worker_id).unwrap().state != 0 {
        //Send black to all pixels
        let worker = state.workers.get_mut(&worker_id).unwrap();
        for i in 0..worker.pixel_count {
            let color = 0x000000;
            worker.queue.push(Request::new(color, i));
        }
        state.workers.get_mut(&worker_id).unwrap().state = 0;
        return;
    }
    if state.state == 1 && state.workers.get(&worker_id).unwrap().state != 1 {
        state.workers.get_mut(&worker_id).unwrap().state = 1;
    }
    if state.workers.get(&worker_id).unwrap().state == 0 {
        return;
    }
    let worker = state.workers.get_mut(&worker_id).unwrap();
    for i in 0..worker.pixel_count {
        let colors = state.color;
        let group = ((i + tick) / 4) as usize;
        let color = colors[(group % colors.len()) as usize];
        worker.queue.push(Request::new(color, i));
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:4242").unwrap();
    let other_listener = TcpListener::bind("0.0.0.0:4243").unwrap();
    let state = Arc::new(Mutex::new(State::new()));
    let state_ref = state.clone();
    let listener_handle = thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let state_ref = state_ref.clone();
                    thread::spawn(move || {
                        handle_client(stream, state_ref);
                    });
                }
                Err(e) => eprintln!("failed: {}", e),
            }
        }
    });

    let state_ref2 = state.clone();
    let other_handle = thread::spawn(move || {
        for stream in other_listener.incoming() {
            match stream {
                Ok(stream) => {
                    let state_ref = state_ref2.clone();
                    thread::spawn(move || {
                        handle_state(stream, state_ref);
                    });
                }
                Err(e) => eprintln!("failed: {}", e),
            }
        }
    });
    listener_handle.join().unwrap();
    other_handle.join().unwrap();
}

fn handle_state(mut stream: std::net::TcpStream, state: Arc<Mutex<State>>) {
    let mut buffer: [u8; 1] = [0; 1];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Connection closed
                println!("Client disconnected");
                break;
            }
            Ok(n) => {
                println!("Read {} bytes: {:?}", n, buffer);
                let new_state = buffer[0] as i32;
                println!("Setting state to {}", new_state);
                state.lock().unwrap().state = new_state;
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}

fn handle_client(mut stream: std::net::TcpStream, mut state: Arc<Mutex<State>>) {
    // Read from stream
    let worker_id = add_worker(&mut state.lock().unwrap());
    println!("Worker {} connected", worker_id);
    let mut start_time = std::time::Instant::now();
    let mut updates: u64 = 0;
    loop {
        let mut buffer: Vec<u8> = vec![];
        let queue_len = state.lock().unwrap().workers[&worker_id].queue.len();
        buffer.extend_from_slice(&(queue_len as u32).to_le_bytes());
        for update in state
            .lock()
            .unwrap()
            .workers
            .get_mut(&worker_id)
            .unwrap()
            .queue
            .drain(..)
        {
            buffer.extend_from_slice(&update.to_bytes());
        }

        if queue_len >= 0 {
            if let Ok(_) = stream.write(&buffer) {
                // Successfully sent data
            } else {
                println!("Worker {} disconnected", worker_id);
                state.lock().unwrap().workers.remove(&worker_id);
                break;
            }
            let mut receive_buffer = [0; 2];
            if let Err(_) = stream.read_exact(&mut receive_buffer) {
                println!("Worker {} disconnected", worker_id);
                state.lock().unwrap().workers.remove(&worker_id);
                break;
            }
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
        update_loop(&mut state.lock().unwrap(), updates as u32, worker_id);
        std::thread::sleep(std::time::Duration::from_millis(100));

        updates += 1;
    }
}
