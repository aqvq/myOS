// os/src/task/context.rs

pub struct TaskContext{
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext{
    pub fn zero_init() -> TaskContext{
        TaskContext { ra: 0, sp: 0, s: [0;12] }
    }
}