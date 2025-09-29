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
const BUFFER_SIZE: usize = 2048;

fn handle_client(mut stream: std::net::TcpStream, mut state: Arc<Mutex<State>>) {
    // Read from stream
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    buffer[0] = 'H' as u8;
    buffer[1] = 'e' as u8;
    buffer[2] = 'l' as u8;
    buffer[3] = 'l' as u8;
    buffer[4] = 'o' as u8;
    buffer[5] = '\n' as u8;

    let worker_id = add_worker(&mut state.lock().unwrap());
    println!("Worker {} connected", worker_id);
    for _ in 0..10 {
        stream.write(&buffer).unwrap();
        println!("Worker {} sent data", worker_id);
        stream.read(&mut buffer).unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    }
}
