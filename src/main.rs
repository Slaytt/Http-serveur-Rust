use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread;


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer){
        Ok(size) => {
            println!("{} bytes read", size);
            let request = String::from_utf8_lossy(&buffer[..]);
            if let Some(first_line) = request.lines().next() {
                println!("Request: {}", first_line);
            }
        }
        Err(e) => {
            eprintln!("Error when reading from stream: {}", e);
            return;
        }
    }
    let html_content = "<html><body><h1>Signal received</h1></body></html>";

    let response = format!(
        "HTTP/1.1 200 ok\r\n\
        content-type: text/html; charset=utf-8\r\n\
        \r\n{}",html_content);

    match stream.write(response.as_bytes()) {
        Ok(_) => println!("Response sent"),
        Err(e) => eprintln!("Error when writing to stream: {}", e),
    }
}

fn main() {
    let listener = match TcpListener::bind("0.0.0.0:8080") {
        Ok(listener) => {
            println!("Server listening on 0.0.0.0:8080");
            listener
        },
        Err(e) => {
            eprintln!("Error when server launched: {}", e);
            return;
        }
    };

    let mut count = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                count += 1;
                let peer_addr = stream.peer_addr().unwrap();
                let local_addr = peer_addr.ip().is_loopback() || peer_addr.ip().to_string() == "127.0.0.1" || peer_addr.ip().to_string() == "::1";
                //is.loopback return a bool if the ip is a loopback ip
                println!("New connexion num {} from: {} ({})",
                         count, peer_addr, if local_addr { "local" } else { "remote" });
                println!("Connexion from {} to dock {}",
                         stream.peer_addr().unwrap(),
                         stream.local_addr().unwrap().port());
                std::thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Erreur de connexion: {}", e);
                continue;  // continue the loop even if it failed
            }
        }
    }
}

