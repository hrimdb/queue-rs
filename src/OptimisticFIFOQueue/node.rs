use std::sync::atomic::AtomicPtr;
use std::ptr;

#[derive(Debug)]
pub struct Point<T> {
    pub ptr: AtomicPtr<Node<T>>,
    pub tag: u64,
}

#[derive(Debug)]
pub struct Queue<T> {
    pub tail: AtomicPtr<Point<T>>,
    pub head: AtomicPtr<Point<T>>,
}

#[derive(Debug)]
pub struct Node<T> {
    pub data: Option<T>,
    pub next: AtomicPtr<Point<T>>,
    pub prev: AtomicPtr<Point<T>>,
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        let mut tail_n = Node::default();
        Queue {
            tail: AtomicPtr::new(&mut Point {
                tag: 0,
                ptr: AtomicPtr::new(&mut tail_n),
            }),
            head: AtomicPtr::new(&mut Point {
                tag: 0,
                ptr: AtomicPtr::new(&mut tail_n),
            }),
        }
    }
}

impl<T> Default for Node<T> {
    fn default() -> Node<T> {
        Node {
            next: AtomicPtr::new(ptr::null_mut()),
            prev: AtomicPtr::new(ptr::null_mut()),
            data: None,
        }
    }
}