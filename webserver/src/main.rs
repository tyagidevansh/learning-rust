use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream); //bind the tcp stream to a buffered reader so reading
                                              //its contents becomes easier
    let http_request: Vec<_> = buf_reader 
        .lines() // turns BufReaderinto an iterator over lines and returns 'Results'
        .map(|result| result.unwrap()) // each res can obv be either Ok or Error so gotta unwrap
        .take_while(|line| !line.is_empty()) // now we have unwrapped strings
        .collect(); // turn the strings into vec<string>

    //println!("Request {http_request:#?}"); //pretty print + debug
 
    if http_request.is_empty() {
        return;
    }

    let request_line = &http_request[0]; // borrow(&) not move otherwise vector would grow smaller

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    
}
