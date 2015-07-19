use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::io;

fn write_thread(mut socket : TcpStream) {
	let mut input = String::new();

	loop {
	    io::stdin().read_line(&mut input)
	        .ok()
	        .expect("failed to read line");
	    socket.write(&mut input.as_bytes());
	    socket.flush();
	    input.clear();
	}
}

fn read_thread(mut socket : TcpStream ) {
    let mut read_buf : [u8; 128] = [0;128];

	loop {
		//let result = socket.try_clone().unwrap().read(&mut read_buf); // ignore here too
    	let result = socket.read(&mut read_buf); // ignore here too
    	match result {
    		Ok(size) => {
    			if size > 0 {
    				println!("{:?}", String::from_utf8_lossy(&read_buf) );
    			}
    		}
    		Err(e) => { println!("error {:?}", e);}
    	}
    }
}

fn main()
{
	let socket = TcpStream::connect("127.0.0.1:9000").unwrap();
	let cloned_socket = socket.try_clone().unwrap();

	let write_thread_handle = thread::spawn(move|| {
    	write_thread(cloned_socket)
	});

	let read_thread_handle = thread::spawn(move|| {
		read_thread(socket);
	});
    
	write_thread_handle.join();
	read_thread_handle.join();
//    rx.recv().ok().expect("Could not receive answer");
} // the stream is closed here