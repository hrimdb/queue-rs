use std::sync::atomic::AtomicPtr;

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