use std::env;
use std::fs::File;
use std::io::prelude::*;

// use std::cell::RefCell;
// use std::default::Default;
// use std::rc::{Rc, Weak};
// use std::borrow::Cow;
// use std::ops::{Deref, DerefMut};
// use std::collections::HashMap;

// pub enum NodeEnum {
// 	STRING(String),
// 	NUMBER(i32),
// 	DICTIONARY(HashMap<&'static str, Handle>),
// 	ARRAY(Vec<Handle>),
// }

// pub struct Node {
//     pub node: NodeEnum,
//     pub parent: Option<WeakHandle>,
// }

// impl Node {
//     fn new(node: NodeEnum) -> Node {
//         Node {
//             node: node,
//             parent: None,
//         }
//     }
// }

// /// Reference to a DOM node.
// #[derive(Clone)]
// pub struct Handle(Rc<RefCell<Node>>);

// impl Deref for Handle {
//     type Target = Rc<RefCell<Node>>;
//     fn deref(&self) -> &Rc<RefCell<Node>> { &self.0 }
// }

// /// Weak reference to a DOM node, used for parent pointers.
// pub type WeakHandle = Weak<RefCell<Node>>;

// fn new_node(node: NodeEnum) -> Handle {
//     Handle(Rc::new(RefCell::new(Node::new(node))))
// }

// fn append(new_parent: &Handle, child: Handle) {
//     	new_parent.borrow_mut().node.push(child.clone());
// 	    let parent = &mut child.borrow_mut().parent;
// 	    assert!(parent.is_none());
// 	    *parent = Some(new_parent.downgrade());

    
// }

fn main() {
    // let args: Vec<_> = env::args().collect();
    // if args.len() == 0 {
    //     println!("ERROR : there is no argument");
    //     return ;
    // } else if ( args.len() > 1 ) {
    // 	println!("ERROR : too many arguments.");
    // 	return ; 
    // }
	let mut file = match File::open("./testJson.txt") {
	    Ok(file) => file,
	    Err(why)  => panic!(why),
	};

	let mut s = String::new();
	file.read_to_string(&mut s);

	// println!("{:?}", s);

	let mut testString = String::new();
	let mut isWriting = false;
	// iterator 정의.. weak pointer로..

    for each_char in s.chars() {
    	if each_char.is_whitespace() {
    		continue;
    	}

    	if s == "\"" {
    		isWriting = !isWriting;
    	}

    	// if s == "{" {
    		
    	// }
    }

}