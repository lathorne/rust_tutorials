//in a real application, error handling would be used instead of .unwrap() when an error is possible

use std::io::prelude::*; //lets us use handle_connection
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs; //this is used for the HTML, the standard filesystem module
use std::thread; //used to multi-thread
use std::time::Duration; //to add a sleeping duration

use hello::ThreadPool;

fn main() {
    //listening to a TCP connection at the specified number
    //.unwrap() returns an object if there is one (between T or failure)
    //bind in this case works like the new function (returns Result<T,E>)
    //unwrap stops the program if an error happens
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4); //creating a new threadpool for a multi-threaded server

    //TcpListener incoming method returns an iteration over a sequence of streams
    //A single stream represents an open connection
    //This for loop processes connections so we can handle them 
    for stream in listener.incoming() { //iterates over connection attempts, not connections!
        let stream = stream.unwrap(); //wrap makes sure the stream doesn't have any errors

        pool.execute(|| { //only handling connections up to the number of threads in the pool
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) { //steam must be mutable because data can be continually added to it
    let mut buffer = [0; 512]; //creating a buffer of 512 bytes to store data from the stream

    stream.read(&mut buffer).unwrap(); //puts info into the buffer

    let get = b"GET / HTTP/1.1\r\n"; //b" transforms this to bytes

    //the string in this portion contains the request data that is sent to the server
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..])); //reads the string

    // if buffer.starts_with(get){ //GET request
    //     let contents = fs::read_to_string("hello.html").unwrap(); //format the HTML to string

    //     let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents); //contains response string, format! adds contents to body of response

    //     stream.write(response.as_bytes()).unwrap(); //converts string to bytes, write sends the bytes down the connection
    //     stream.flush().unwrap(); //flush waits until all bytes are sent and flushed the internal buffer of TcpStream
    // }else{
    //     //some other sort of request, returning 404 for content not found
    //     let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    //     let contents = fs::read_to_string("404.html").unwrap();

    //     let response = format!("{}{}", status_line, contents);

    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }

    //more concise code to get rid of duplicate statements

    let get = b"GET / HTTP/1.1\r\n";
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
