use std::sync::atomic::AtomicPtr;

#[derive(Debug)]
pub struct Point_t<T> {
    pub ptr: AtomicPtr<Node_t<T>>,
    pub tag: u64,
}

#[derive(Debug)]
pub struct Queue_t<T> {
    pub tail: AtomicPtr<Point_t<T>>,
    pub head: AtomicPtr<Point_t<T>>,
}

#[derive(Debug)]
pub struct Node_t<T> {
    pub data: Option<T>,
    pub next: AtomicPtr<Point_t<T>>,
    pub prev: AtomicPtr<Point_t<T>>,
}