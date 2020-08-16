
use std::sync::{Arc, Mutex};
use std::net::{TcpStream, TcpListener};
use std::time::Duration;
use std::sync::atomic::{Ordering, AtomicBool};
use std::io::Read;
use std::thread;
use std::thread::JoinHandle;

lazy_static!{
    static ref VALUES: Mutex<Vec<Arc<Mutex<(f32,f32,f32,f32)>>>> = Mutex::new(Vec::new());
    static ref RUNNING: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    static ref SENDERS: Mutex<Vec<JoinHandle<()>>> = Mutex::new(Vec::new());
    static ref CONNECTION_THREAD: Mutex<Option<JoinHandle<()>>> = Mutex::new(None);
}


pub fn start_ct(){
    *(CONNECTION_THREAD.lock().unwrap()) = Some(thread::spawn(||{handle_incoming_connection();}));
}

pub fn shut_down(){

    RUNNING.swap(false,Ordering::Relaxed);

    println!("joining threads ...");
    match CONNECTION_THREAD.lock().unwrap().take() {
        Some(ct) => {ct.join();},
        None => {}
    }
    println!("CONNECTION_THREAD joined");

    {
        let mut vs = SENDERS.lock().unwrap();
        vs.drain(0..).for_each(|t|{t.join();});
    }
    println!("SENDERS joined");

}

pub fn get_value(index: usize) -> Option<(f32,f32,f32,f32)>{

    match VALUES.lock().unwrap().get(index){
        Some(r) => {
            let v = r.lock().unwrap().clone();
            Some(v)
        }
        None => None
    }

}

pub fn handle_incoming_connection(){
    println!("handle_incoming_connection");

    let listener = TcpListener::bind("10.0.0.30:8000").unwrap();
    listener.set_nonblocking(true);

    for stream in listener.incoming() {
        if !RUNNING.load(Ordering::Relaxed){
            break;
        }
        match stream {
            Ok(incoming_stream) => {
                println!("-----------accepted---------------");

                let new_value = Arc::new(Mutex::new((0.0, 0.0, 0.0, 0.0)));
                let value_clone = new_value.clone();

                VALUES.lock().unwrap().push(new_value);

                let r = RUNNING.clone();

                SENDERS.lock().unwrap().push(
                    thread::spawn(move || {
                        handle_client(incoming_stream, value_clone, r);
                    })
                );
            }
            Err(e) => {
                println!("handle_incoming_connection error: \n{}",e);
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
    println!("incoming_connection - Thread ends");
}

fn handle_client(mut stream: TcpStream, value: Arc<Mutex<(f32, f32, f32, f32)>>, is_running: Arc<AtomicBool>) {
    println!("handle_client");

    let mut buf: [u8; 16] = [0; 16];
    stream.set_read_timeout(Some(Duration::from_millis(1000)));
    stream.set_nonblocking(false).unwrap();
    while is_running.load(Ordering::Relaxed) {
        match stream.read_exact(buf.as_mut()) {
            Result::Err(e) => {
                println!("handle_client\n{}\n", e);
                break;
            }
            _ => {
                let f1 = f32::from_ne_bytes([buf[0], buf[1], buf[2], buf[3]]);
                let f2 = f32::from_ne_bytes([buf[4], buf[5], buf[6], buf[7]]);
                let f3 = f32::from_ne_bytes([buf[8], buf[9], buf[10], buf[11]]);
                let f4 = f32::from_ne_bytes([buf[12], buf[13], buf[14], buf[15]]);
                let f = (f1, f2, f3, f4);

                {
                    let mut v = value.lock().unwrap();
                    *v = f;
                }

            }
        };
    }
    println!("Connection closed - Thread ends");
}