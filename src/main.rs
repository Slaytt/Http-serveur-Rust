use std::net::TcpListener;
use std::net::TcpStream;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080");
    match TcpListener::bind("127.0.0.1:8080") {
        Ok(listener ) => {
         println!({:?},listener);
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    println!("listening on 127.0.0.1:8080");
}
