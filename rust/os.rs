use ::libc;
extern "C" {
    fn sprintf(
        _: *mut std::ffi::c_char,
        _: *const std::ffi::c_char,
        _: ...
    ) -> std::ffi::c_int;
    fn snprintf(
        _: *mut std::ffi::c_char,
        _: std::ffi::c_ulong,
        _: *const std::ffi::c_char,
        _: ...
    ) -> std::ffi::c_int;
    fn open(
        __file: *const std::ffi::c_char,
        __oflag: std::ffi::c_int,
        _: ...
    ) -> std::ffi::c_int;
    fn read(
        __fd: std::ffi::c_int,
        __buf: *mut std::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    fn strtol(
        _: *const std::ffi::c_char,
        _: *mut *mut std::ffi::c_char,
        _: std::ffi::c_int,
    ) -> std::ffi::c_long;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn strerror(_: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn strsignal(__sig: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
    fn ungetcc_back(c: std::ffi::c_char);
    fn lgetenv(var: *const std::ffi::c_char) -> *const std::ffi::c_char;
    fn flush();
    fn psignals();
    fn getchr() -> std::ffi::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> std::ffi::c_int;
    fn sigprocmask(
        __how: std::ffi::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> std::ffi::c_int;
    fn __sigsetjmp(
        __env: *mut __jmp_buf_tag,
        __savemask: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn siglongjmp(_: *mut __jmp_buf_tag, _: std::ffi::c_int) -> !;
    fn time(__timer: *mut time_t) -> time_t;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> std::ffi::c_int;
    fn __errno_location() -> *mut std::ffi::c_int;
    fn poll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: std::ffi::c_int,
    ) -> std::ffi::c_int;
    static mut sigs: std::ffi::c_int;
    static mut ignore_eoi: std::ffi::c_int;
    static mut exit_F_on_close: std::ffi::c_int;
    static mut follow_mode: std::ffi::c_int;
    static mut scanning_eof: std::ffi::c_int;
    static mut intr_char: std::ffi::c_char;
    static mut is_tty: std::ffi::c_int;
    static mut quit_if_one_screen: std::ffi::c_int;
    static mut one_screen: std::ffi::c_int;
    static mut less_start_time: time_t;
    static mut tty: std::ffi::c_int;
}
pub type __uintmax_t = std::ffi::c_ulong;
pub type __off_t = std::ffi::c_long;
pub type __time_t = std::ffi::c_long;
pub type __ssize_t = std::ffi::c_long;
pub type __syscall_slong_t = std::ffi::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = std::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [std::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uintmax_t = __uintmax_t;
pub type lbool = std::ffi::c_uint;
pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;
pub type uintmax = uintmax_t;
pub type less_off_t = off_t;
pub type POSITION = less_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: std::ffi::c_int,
    pub __saved_mask: __sigset_t,
}
pub type __jmp_buf = [std::ffi::c_long; 8];
pub type sigjmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: std::ffi::c_int,
    pub events: std::ffi::c_short,
    pub revents: std::ffi::c_short,
}
pub type nfds_t = std::ffi::c_ulong;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const std::ffi::c_char) -> std::ffi::c_int {
    return strtol(
        __nptr,
        0 as *mut std::ffi::c_void as *mut *mut std::ffi::c_char,
        10 as std::ffi::c_int,
    ) as std::ffi::c_int;
}
static mut use_poll: lbool = LTRUE;
static mut any_data: lbool = LFALSE;
static mut reading: lbool = LFALSE;
static mut opening: lbool = LFALSE;
#[no_mangle]
pub static mut waiting_for_data: lbool = LFALSE;
#[no_mangle]
pub static mut consecutive_nulls: std::ffi::c_int = 0 as std::ffi::c_int;
#[no_mangle]
pub static mut getting_one_screen: lbool = LFALSE;
#[no_mangle]
pub static mut no_poll: lbool = LFALSE;
static mut waiting_for_data_delay: std::ffi::c_int = 4000 as std::ffi::c_int;
#[no_mangle]
pub static mut screenfill_ms: std::ffi::c_int = 3000 as std::ffi::c_int;
static mut read_label: sigjmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
static mut open_label: sigjmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
#[no_mangle]
pub unsafe extern "C" fn init_poll() {
    let mut delay: *const std::ffi::c_char = lgetenv(
        b"LESS_DATA_DELAY\0" as *const u8 as *const std::ffi::c_char,
    );
    let mut idelay: std::ffi::c_int = if delay.is_null() {
        0 as std::ffi::c_int
    } else {
        atoi(delay)
    };
    if idelay > 0 as std::ffi::c_int {
        waiting_for_data_delay = idelay;
    }
    delay = lgetenv(b"LESS_SCREENFILL_TIME\0" as *const u8 as *const std::ffi::c_char);
    idelay = if delay.is_null() { 0 as std::ffi::c_int } else { atoi(delay) };
    if idelay > 0 as std::ffi::c_int {
        screenfill_ms = idelay;
    }
}
unsafe extern "C" fn check_poll(
    mut fd: std::ffi::c_int,
    mut tty_0: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut poller: [pollfd; 2] = [
        {
            let mut init = pollfd {
                fd: fd,
                events: 0x1 as std::ffi::c_int as std::ffi::c_short,
                revents: 0 as std::ffi::c_int as std::ffi::c_short,
            };
            init
        },
        {
            let mut init = pollfd {
                fd: tty_0,
                events: 0x1 as std::ffi::c_int as std::ffi::c_short,
                revents: 0 as std::ffi::c_int as std::ffi::c_short,
            };
            init
        },
    ];
    let mut timeout: std::ffi::c_int = if waiting_for_data as std::ffi::c_uint != 0
        && !(scanning_eof != 0 && follow_mode == 1 as std::ffi::c_int)
    {
        -(1 as std::ffi::c_int)
    } else if ignore_eoi != 0 && waiting_for_data as u64 == 0 {
        0 as std::ffi::c_int
    } else {
        waiting_for_data_delay
    };
    if getting_one_screen as std::ffi::c_uint != 0
        && get_time()
            < less_start_time + (screenfill_ms / 1000 as std::ffi::c_int) as time_t
    {
        return 0 as std::ffi::c_int;
    }
    if any_data as u64 == 0 {
        return 0 as std::ffi::c_int;
    }
    poll(poller.as_mut_ptr(), 2 as std::ffi::c_int as nfds_t, timeout);
    if poller[1 as std::ffi::c_int as usize].revents as std::ffi::c_int
        & 0x1 as std::ffi::c_int != 0
    {
        let mut ch: std::ffi::c_int = getchr();
        if ch < 0 as std::ffi::c_int || ch == intr_char as std::ffi::c_int {
            return -(2 as std::ffi::c_int);
        }
        ungetcc_back(ch as std::ffi::c_char);
        return -(2 as std::ffi::c_int);
    }
    if ignore_eoi != 0 && exit_F_on_close != 0
        && poller[0 as std::ffi::c_int as usize].revents as std::ffi::c_int
            & (0x10 as std::ffi::c_int | 0x1 as std::ffi::c_int)
            == 0x10 as std::ffi::c_int
    {
        return -(2 as std::ffi::c_int);
    }
    if poller[0 as std::ffi::c_int as usize].revents as std::ffi::c_int
        & (0x1 as std::ffi::c_int | 0x10 as std::ffi::c_int | 0x8 as std::ffi::c_int)
        == 0 as std::ffi::c_int
    {
        return -(3 as std::ffi::c_int);
    }
    return 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn supports_ctrl_x() -> std::ffi::c_int {
    return use_poll as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn iread(
    mut fd: std::ffi::c_int,
    mut buf: *mut std::ffi::c_uchar,
    mut len: size_t,
) -> ssize_t {
    let mut n: ssize_t = 0;
    loop {
        if reading as u64 == 0
            && __sigsetjmp(read_label.as_mut_ptr(), 1 as std::ffi::c_int) != 0
        {
            reading = LFALSE;
            let mut mask: sigset_t = __sigset_t { __val: [0; 16] };
            sigemptyset(&mut mask);
            sigprocmask(2 as std::ffi::c_int, &mut mask, 0 as *mut sigset_t);
            if fd != tty
                && sigs
                    & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                        | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                        | (1 as std::ffi::c_int) << 2 as std::ffi::c_int) == 0
            {
                return -(3 as std::ffi::c_int) as ssize_t;
            }
            return -(2 as std::ffi::c_int) as ssize_t;
        }
        flush();
        reading = LTRUE;
        if is_tty != 0 && fd != tty && use_poll as std::ffi::c_uint != 0
            && no_poll as u64 == 0 && !(quit_if_one_screen != 0 && one_screen != 0)
        {
            let mut ret: std::ffi::c_int = check_poll(fd, tty);
            if ret != 0 as std::ffi::c_int {
                if ret == -(2 as std::ffi::c_int) {
                    sigs |= (1 as std::ffi::c_int) << 1 as std::ffi::c_int;
                }
                reading = LFALSE;
                return ret as ssize_t;
            }
        }
        n = read(fd, buf as *mut std::ffi::c_void, len);
        reading = LFALSE;
        if n < 0 as std::ffi::c_int as ssize_t {
            if *__errno_location() == 4 as std::ffi::c_int {
                continue;
            }
            if *__errno_location() == 11 as std::ffi::c_int {
                continue;
            }
            return -(1 as std::ffi::c_int) as ssize_t;
        } else {
            if fd != tty && n > 0 as std::ffi::c_int as ssize_t {
                any_data = LTRUE;
            }
            return n;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn iopen(
    mut filename: *const std::ffi::c_char,
    mut flags: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut r: std::ffi::c_int = 0;
    while opening as u64 == 0
        && __sigsetjmp(open_label.as_mut_ptr(), 1 as std::ffi::c_int) != 0
    {
        opening = LFALSE;
        if sigs
            & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 1 as std::ffi::c_int) != 0
        {
            sigs = 0 as std::ffi::c_int;
            *__errno_location() = 4 as std::ffi::c_int;
            return -(1 as std::ffi::c_int);
        }
        psignals();
    }
    opening = LTRUE;
    r = open(filename, flags);
    opening = LFALSE;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn intio() {
    if opening as u64 != 0 {
        siglongjmp(open_label.as_mut_ptr(), 1 as std::ffi::c_int);
    }
    if reading as u64 != 0 {
        siglongjmp(read_label.as_mut_ptr(), 1 as std::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_time() -> time_t {
    let mut t: time_t = 0;
    time(&mut t);
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn errno_message(
    mut filename: *const std::ffi::c_char,
) -> *mut std::ffi::c_char {
    let mut p: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut m: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut len: size_t = 0;
    p = strerror(*__errno_location());
    len = (strlen(filename))
        .wrapping_add(strlen(p))
        .wrapping_add(3 as std::ffi::c_int as std::ffi::c_ulong);
    m = ecalloc(len, ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong)
        as *mut std::ffi::c_char;
    snprintf(m, len, b"%s: %s\0" as *const u8 as *const std::ffi::c_char, filename, p);
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn signal_message(
    mut sig: std::ffi::c_int,
) -> *const std::ffi::c_char {
    static mut sigbuf: [std::ffi::c_char; 20] = [0; 20];
    let mut description: *const std::ffi::c_char = strsignal(sig);
    if !description.is_null() {
        return description;
    }
    sprintf(
        sigbuf.as_mut_ptr(),
        b"Signal %d\0" as *const u8 as *const std::ffi::c_char,
        sig,
    );
    return sigbuf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn umuldiv(
    mut val: uintmax,
    mut num: uintmax,
    mut den: uintmax,
) -> uintmax {
    let mut q: uintmax = val / den;
    let mut r: uintmax = val % den;
    let mut qnum: uintmax = q * num;
    let mut rnum: uintmax = r * num;
    let mut quot: uintmax = qnum.wrapping_add(rnum / den);
    let mut rem: uintmax = rnum % den;
    return quot
        .wrapping_add(
            ((den / 2 as std::ffi::c_int as uintmax)
                < rem.wrapping_add(quot & !den & 1 as std::ffi::c_int as uintmax))
                as std::ffi::c_int as uintmax,
        );
}
#[no_mangle]
pub unsafe extern "C" fn percentage(
    mut num: POSITION,
    mut den: POSITION,
) -> std::ffi::c_int {
    return umuldiv(num as uintmax, 100 as std::ffi::c_int as uintmax, den as uintmax)
        as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn percent_pos(
    mut pos: POSITION,
    mut percent: std::ffi::c_int,
    mut fraction: std::ffi::c_long,
) -> POSITION {
    let mut pctden: POSITION = (percent * 1000000 as std::ffi::c_int) as std::ffi::c_long
        + fraction;
    return umuldiv(
        pos as uintmax,
        pctden as uintmax,
        (100 as std::ffi::c_int * 1000000 as std::ffi::c_int) as uintmax,
    ) as POSITION;
}
#[no_mangle]
pub unsafe extern "C" fn sleep_ms(mut ms: std::ffi::c_int) {
    let mut sec: std::ffi::c_int = ms / 1000 as std::ffi::c_int;
    let mut t: timespec = {
        let mut init = timespec {
            tv_sec: sec as __time_t,
            tv_nsec: ((ms - sec * 1000 as std::ffi::c_int) * 1000000 as std::ffi::c_int)
                as __syscall_slong_t,
        };
        init
    };
    nanosleep(&mut t, 0 as *mut timespec);
}
