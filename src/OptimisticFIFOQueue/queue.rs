use OptimisticFIFOQueue::node::Queue;
use OptimisticFIFOQueue::node::Point;
use OptimisticFIFOQueue::node::Node;
use std::sync::atomic::AtomicPtr;

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue::default()          
    }
}