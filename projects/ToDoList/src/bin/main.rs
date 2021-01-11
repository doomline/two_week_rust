use ToDoList::ThreadPool;
use std::thread;
use std::time::Duration;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); 
    // bind works like the new function, it returns a new TCPListener instance
    // bind also returns a <T, E> (Try, Error) 

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // this code spawns a single thread every time a new request is made
        // this is inefficient becuase if you receive many requests for actions it will use a lot of resources
        // we will switch to using a threadpool 
        //  std::thread::spawn(move || {
        //   handle_connection(stream);

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    // buffer holds the data that is read in. 

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    // this establishes the GET request
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);


    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

/* Something to look into 

Other options you might explore are the fork/join model and the single-threaded async I/O model.
*/