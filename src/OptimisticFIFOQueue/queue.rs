use std::ptr;
use OptimisticFIFOQueue::node::Queue;
use OptimisticFIFOQueue::node::Point;
use OptimisticFIFOQueue::node::Node;
use std::sync::atomic::AtomicPtr;

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        let mut tail_n = Node {
            next: AtomicPtr::new(ptr::null_mut()),
            prev: AtomicPtr::new(ptr::null_mut()),
            data: None,
        };
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

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue::default()          
    }
}