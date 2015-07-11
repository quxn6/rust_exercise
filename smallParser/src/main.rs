use std::env;
use std::fs::File;
use std::io::prelude::*;

use std::cell::RefCell;
use std::default::Default;
use std::rc::{Rc, Weak};
use std::borrow::Cow;
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;

pub enum NodeEnum {
	STRING(String),
	NUMBER(i32),
	DICTIONARY(HashMap<String, Handle>),
	ARRAY(Vec<Handle>),
}

pub struct Node {
    pub node: NodeEnum,
}

impl Node {
    fn new(node: NodeEnum) -> Node {
        Node {
            node: node,
        }
    }
}

/// Reference to a DOM node.
#[derive(Clone)]
pub struct Handle(Rc<RefCell<Node>>);

impl Deref for Handle {
    type Target = Rc<RefCell<Node>>;
    fn deref(&self) -> &Rc<RefCell<Node>> { &self.0 }
}

fn new_node(node: NodeEnum) -> Handle {
    Handle(Rc::new(RefCell::new(Node::new(node))))
}

// fn append(new_parent: &Handle, child: Handle) {
// 	new_parent.borrow_mut().node.push(child.clone());
//     let parent = &mut child.borrow_mut().parent;
//     assert!(parent.is_none());
//     *parent = Some(new_parent.downgrade());
// }


//fn parse_file()

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

//	println!("{:?}", s);

	let mut stack : Vec<Handle> = Vec::new();
	let mut temp_key = String::new();
	let mut temp_value = String::new();

	let mut is_value : bool = false;
	let mut is_key : bool = false;
	let mut is_root : bool = true;
	let mut is_writing : bool = false;
	// iterator 정의.. weak pointer로..

	let root : Handle = new_node(NodeEnum::DICTIONARY(HashMap::new()));
	let mut this_node = root;

    for each_char in s.chars() {
    	// white space는 무시
    	if each_char.is_whitespace() { //  상태 판단해야함.. 스트링 내부에..
    		continue;
    	}

    	// 첫 시작을 판단
    	if is_root && each_char == '{' {
    		is_root = false;
    		is_key = true;
    	}

    	if is_writing {
    		match each_char {
    			'\"' 	=> {	// 입력이 종료되면 value에 삽입
    				is_writing = false;
    				if !is_key {
    					// node_enum dictionary에 삽입
    				}
    			} 
    			_ 		=> {	// 입력중이면 key, value를 생성
    				if is_key {
    					temp_key.push(each_char);
    				} else {
    					temp_value.push(each_char);
    				}
    			}
    		}
    	} else {
	    	match each_char {
	    		'\"' 	=> { is_writing = true; },
	    		':' 	=> { is_key = false; },
	    		'{' 	=> {
	    			// 새로운 자식 노드 생성
	    			let temp_node = new_node(NodeEnum::DICTIONARY(HashMap::new()));
	    			
	    			// 자식 노드 연결 
	    			// this_node.borrow_mut().node.insert(temp_key, temp_node);
	    			match this_node.borrow_mut().node {
	    				NodeEnum::DICTIONARY(dic ) 	=> { dic.insert(temp_key, temp_node); },
	    				NodeEnum::ARRAY(array )		=> { array.push(temp_node); },
	    				_ => println!("something Wrong.."),
	    			}

	    			// 스택에 넣고 자식 노드 탐색 시작.
	    			stack.push(this_node);
	    			this_node = temp_node;
	    			is_key = true; 	// 자식 노드의 키
	    		},
	    		'}' 	=> {
	    			// 부모노드를 꺼내와서 탐색 시작
	    			this_node = stack.pop().unwrap();
	    			is_key = true; // 부모 노드의 키
	    		},
	    		// '[' 	=> 
	    		// ']' 	=>
	    		_ 		=> println!("W.T.H"),
    		}		
    	}
    	
    	print!("{:?}", each_char);
    }
}