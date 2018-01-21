use std::ptr;
use OptimisticFIFOQueue::node::Queue_t;
use OptimisticFIFOQueue::node::Point_t;
use OptimisticFIFOQueue::node::Node_t;
use std::sync::atomic::AtomicPtr;

impl<T> Queue_t<T> {
    pub fn new() -> Queue_t<T> {
        let mut tail_n = Node_t {
            next: AtomicPtr::new(ptr::null_mut()),
            prev: AtomicPtr::new(ptr::null_mut()),
            data: None,
        };
        return Queue_t {
            tail: AtomicPtr::new(&mut Point_t {
                tag: 0,
                ptr: AtomicPtr::new(&mut tail_n),
            }),
            head: AtomicPtr::new(&mut Point_t {
                tag: 0,
                ptr: AtomicPtr::new(&mut tail_n),
            }),
        };
    }
}