use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::mpsc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    for i in 0..5 {
        let received = rx.try_recv().unwrap();
        println!("Got: {}", received);
        println!("{}", i * 2);
    }

    // new_thread.join().unwrap();
}
