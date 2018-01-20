use std::ptr;

impl Default for Queue_t {
    #[inline]
    fn default() -> Queue_t {
        VecDeque::new()
    }
}

impl Queue_t {
    pub fn new() -> Queue_t {
        let tail_n :&node_t 
        tail_n.next = point_t{ptr: ptr::null(),tag:0}
        Queue_t {
            tail: point_t{tag:0,ptr:tail_n}
            head: point_t{tag:0,ptr:tail_n}
        }
    }
}