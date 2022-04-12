

use crate::task;


/// task exits and submit an exit code
pub fn sys_exit(exit_state: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_state);
    task::run_next_app()
}


