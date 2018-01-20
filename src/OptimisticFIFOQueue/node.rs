#[derive(Debug, Copy, Clone)]
pub struct point_t {
    pub ptr :&node_t
    pub tag :u64
}

#[derive(Debug, Copy, Clone)]
pub struct queue_t {
    pub tail :point_t
    pub head :point_t
}

#[derive(Debug, Copy, Clone)]
pub struct node_t {
    data :data_t
    next :point_t
    prev :point_t
}

enum data_t {
    Op(Operator),
    Number(i32)
}