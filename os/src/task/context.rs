// os/src/task/context.rs

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TaskContext {
    _ra: usize,
    _sp: usize,
    _s: [usize; 12],
}

impl TaskContext {
    pub fn zero_init() -> Self {
        Self {
            _ra: 0,
            _sp: 0,
            _s: [0; 12],
        }
    }

    pub fn goto_restore(kstack_ptr: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            _ra: __restore as usize,
            _sp: kstack_ptr,
            _s: [0; 12],
        }
    }
}
