const FD_STDOUT: usize = 1;

/// write buf of length 'len' to a file with 'file_descriptor'
pub fn sys_write(file_descriptor: usize, buf: *const u8, len: usize) -> isize {
    match file_descriptor {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!")
        }
    }
}

