use ::libc;
extern "C" {
    fn open(
        __file: *const std::ffi::c_char,
        __oflag: std::ffi::c_int,
        _: ...
    ) -> std::ffi::c_int;
    fn ttyname(__fd: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn quit(status: std::ffi::c_int);
    fn iread(fd: std::ffi::c_int, buf: *mut std::ffi::c_uchar, len: size_t) -> ssize_t;
    fn flush();
}
pub type __ssize_t = std::ffi::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = std::ffi::c_ulong;
#[no_mangle]
pub static mut tty: std::ffi::c_int = 0;
unsafe extern "C" fn open_tty_device(
    mut dev: *const std::ffi::c_char,
) -> std::ffi::c_int {
    return open(dev, 0 as std::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn open_tty() -> std::ffi::c_int {
    let mut fd: std::ffi::c_int = -(1 as std::ffi::c_int);
    if fd < 0 as std::ffi::c_int {
        let mut dev: *const std::ffi::c_char = ttyname(2 as std::ffi::c_int);
        if !dev.is_null() {
            fd = open_tty_device(dev);
        }
    }
    if fd < 0 as std::ffi::c_int {
        fd = open_tty_device(b"/dev/tty\0" as *const u8 as *const std::ffi::c_char);
    }
    if fd < 0 as std::ffi::c_int {
        fd = 2 as std::ffi::c_int;
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn open_getchr() {
    tty = open_tty();
}
#[no_mangle]
pub unsafe extern "C" fn close_getchr() {}
#[no_mangle]
pub unsafe extern "C" fn default_wheel_lines() -> std::ffi::c_int {
    let mut lines: std::ffi::c_int = 1 as std::ffi::c_int;
    return lines;
}
#[no_mangle]
pub unsafe extern "C" fn getchr() -> std::ffi::c_int {
    let mut c: std::ffi::c_char = 0;
    let mut result: ssize_t = 0;
    loop {
        flush();
        let mut uc: std::ffi::c_uchar = 0;
        result = iread(
            tty,
            &mut uc,
            ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
        );
        c = uc as std::ffi::c_char;
        if result == -(2 as std::ffi::c_int) as ssize_t {
            return -(2 as std::ffi::c_int);
        }
        if result < 0 as std::ffi::c_int as ssize_t {
            quit(1 as std::ffi::c_int);
        }
        if c as std::ffi::c_int == '\0' as i32 {
            c = -32i32 as std::ffi::c_char;
        }
        if !(result != 1 as std::ffi::c_int as ssize_t) {
            break;
        }
    }
    return c as std::ffi::c_int & 0xff as std::ffi::c_int;
}
