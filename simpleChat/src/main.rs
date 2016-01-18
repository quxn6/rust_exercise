use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

type Sockets = BTreeMap<i32, TcpStream>;
type Messages = Vec<String>;

fn handle_client(mut socket: TcpStream, message_queue : Arc<Mutex<Messages>> ) {
    // ...
	println!("client connected");
    let mut buf : [u8; 128 ]= [0; 128];
    
    loop {
	    match socket.read(&mut buf) {
	    	Ok(len) => {println!("recieved {}", len);}
	    	Err(e) => { 
	    		println!("socket closed");
	    		break; 
	    	}
	    }

	    let message = String::from_utf8_lossy(&buf).into_owned();
	    let mut mq = message_queue.lock().unwrap();
	    mq.push(message);
    }
}

fn broadcasting(users : Arc<Mutex<Sockets>>, message_queue : Arc<Mutex<Messages>> ) {
	// users loop돌면서 messageQueue write
    // socket.write(&mut buf).ok();
    // socket.flush().ok();
    loop {
    	thread::sleep_ms(100);
    	let mut mq = message_queue.lock().unwrap();
    	let mut us = users.lock().unwrap();

    	for (id, mut user_socket) in us.iter() {
    		for msg in mq.iter() {
    			user_socket.write(&mut msg.as_bytes()).ok();
    		}
    		user_socket.flush().ok();
    	}
    	mq.clear();
    }
}

fn main() {
	// start listen sockets
	let listener = TcpListener::bind("127.0.0.1:9000").unwrap();

	// 공유자원 선언
	let users = Arc::new(Mutex::new(Sockets::new()));
	let message_queue = Arc::new(Mutex::new(Messages::new()));

	// broadcasting 쓰레드 시작.
	let cloned_user = users.clone();
	let cloned_message_queue = message_queue.clone();
	let broadcasting_thread_handle = thread::spawn( move|| {
		broadcasting(cloned_user, cloned_message_queue);
	});

	// accept connections and process them, spawning a new thread for each one
	let mut user_count : i64 = 0;
	for handle in listener.incoming() {
	    match handle {
	        Ok(socket) => {
	        	user_count += 1;
	        	let cloned_message_queue = message_queue.clone();
	            thread::spawn(move|| {
	                // connection succeeded
	                handle_client(socket, cloned_message_queue)
	            });
	            println!("test spawn end");
	        }
	        Err(e) => { println!("{:?}", e); }
	    }
	}

	// close the socket server
	drop(listener);
	broadcasting_thread_handle.join().ok();
}
