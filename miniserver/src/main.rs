use miniserver::ThreadPool;
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    path::Path,
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            if let Err(e) = handle_connection(stream) {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf_reader = BufReader::new(&stream);
    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line)?;

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return Err("Invalid request line".into());
    }

    let method = parts[0];
    let path = parts[1];

    if method != "GET" {
        let response = "HTTP/1.1 405 Method Not Allowed\r\n\r\n";
        stream.write_all(response.as_bytes())?;
        return Ok(());
    }

    let (status_line, filename) = match path {
        "/" => ("HTTP/1.1 200 OK", "hello.html"),
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let full_path = Path::new(filename);
    if !full_path.exists() {
        let response = "HTTP/1.1 500 Internal Server Error\r\n\r\nFile not found on server.";
        stream.write_all(response.as_bytes())?;
        return Ok(());
    }

    let contents = fs::read_to_string(filename)?;
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes())?;
    Ok(())
}
