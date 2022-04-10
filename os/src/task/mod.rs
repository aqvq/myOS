use crate::config::*;
use crate::loader::load_apps;
use crate::safe_cell::SafeCell;
use crate::trap::*;
use lazy_static::*;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};
static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        println!("[kernel] push_context Done!");
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}
#[derive(Debug)]
struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM+1],
}

impl AppManager {
    pub fn print_app_info(&self) {
        for i in 0..self.num_app {
            println!(
                "[kernel] app_{} [{:#x}, {:#x})", // 左闭右开区间
                i,
                APP_BASE_ADDRESS + i * APP_SIZE_LIMIT,
                APP_BASE_ADDRESS + (i + 1) * APP_SIZE_LIMIT
            );
        }
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }

    pub fn get_num_app(&self) -> usize {
        self.num_app
    }

}


// TODO: lazy_static用法
lazy_static! {
    static ref APP_MANAGER: SafeCell<AppManager> = unsafe{
        SafeCell::new({
            // TODO: extern "C"如何知道_num_app的位置？难道是因为它们被链接到同一个文件里面？
            extern "C" { fn _num_app(); }
            let num_app_ptr = _num_app as *const usize;
            let num_app = num_app_ptr.read_volatile(); // 内存读取操作
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] =
                core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
            app_start[..=num_app].copy_from_slice(app_start_raw);

            // println!("app_start: {:#?}", app_start);
            // TODO: rust的slice语法

            AppManager{
                num_app,
                current_app: 0,
                app_start,
            }
        })
    };
}

pub fn init() {
    println!("[kernel] print_app_info Done!");
    let app_manager = APP_MANAGER.exclusive_access();
    app_manager.print_app_info();
    drop(app_manager);
}

pub fn run_next_app() -> ! {
    println!("run_app");
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
    app_manager.move_to_next_app();
    // println!("{:#?}", app_manager);
    drop(app_manager);

    // if app_manager.get_current_app() >= app_manager.get_num_app() {
    //     println!("[kernel] All tasks are completed!");
    //     sys_exit(0);
    // }
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        __restore( KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDRESS + (current_app-1) * APP_SIZE_LIMIT,
            USER_STACK.get_sp(),
        )) as *const _ as usize);
    }
    panic!("Unreachable in task::run_app!");
}
