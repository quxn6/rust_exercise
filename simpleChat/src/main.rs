use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;

// fn broadcasting(mut socket : TcpStream) {

// }

fn handle_client(mut socket: TcpStream) {
    // ...
	println!("client connected");
    let mut buf : [u8; 128 ]= [0; 128];

    loop {
	    socket.read(&mut buf);
	    socket.write(&mut buf);
	    socket.flush();
    }
}

fn main() {
	let listener = TcpListener::bind("127.0.0.1:9000").unwrap();



	// accept connections and process them, spawning a new thread for each one
	for handle in listener.incoming() {
	    match handle {
	        Ok(mut socket) => {
	            thread::spawn(move|| {
	                // connection succeeded
	                handle_client(socket)
	            });
	        }
	        Err(e) => { /* connection failed */ }
	    }
	}

	// close the socket server
	drop(listener);
}
