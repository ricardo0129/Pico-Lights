use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

struct Worker {
    id: i32,
}

struct State {
    workers: Vec<Worker>,
}
struct Request {
    color: u32,
    position: u32,
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
    state.workers.push(Worker { id });
    id
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:4242").unwrap();
    println!("listening started, ready to accept");
    let state = Arc::new(Mutex::new(State { workers: vec![] }));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let state_ref = state.clone();
                thread::spawn(move || {
                    handle_client(stream, state_ref);
                });
            }
            Err(e) => {
                eprintln!("failed: {}", e);
            }
        }
    }
}
const BUFFER_SIZE: usize = 8;

fn handle_client(mut stream: std::net::TcpStream, mut state: Arc<Mutex<State>>) {
    let colors: [u32; 3] = [
        urgb_u32(255, 154, 0), // Pumpkin Orange
        urgb_u32(9, 255, 0),   // Spooky Green
        urgb_u32(201, 0, 255), // Witchy Purple
    ];
    // Read from stream
    let worker_id = add_worker(&mut state.lock().unwrap());
    println!("Worker {} connected", worker_id);
    for pos in 0..50 {
        let req: Request = Request::new(colors[(pos as usize) % 3], pos);
        let mut buffer: [u8; BUFFER_SIZE] = req.to_bytes();
        println!("Worker {} sent data", worker_id);
        stream.write(&buffer).unwrap();
        //sleep 1 second
        std::thread::sleep(std::time::Duration::from_millis(100));
        //stream.read_exact(&mut buffer).unwrap();
        //println!("Worker {} received data", worker_id);
    }
}
