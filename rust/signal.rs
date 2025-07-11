use crate::decode::lgetenv;
use ::libc;
extern "C" {
    fn getpid() -> __pid_t;
    fn quit(status: std::ffi::c_int);
    fn secure_allow(features: std::ffi::c_int) -> std::ffi::c_int;
    fn raw_mode(on: std::ffi::c_int);
    fn screen_size_changed();
    fn get_term();
    fn init();
    fn deinit();
    fn bell();
    fn clear_bot();
    fn screen_trashed();
    fn ungetsc(s: *const std::ffi::c_char);
    fn isnullenv(s: *const std::ffi::c_char) -> lbool;
    fn intio();
    fn flush();
    fn set_filter_pattern(pattern: *const std::ffi::c_char, search_type: std::ffi::c_int);
    fn signal(__sig: std::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: std::ffi::c_int) -> std::ffi::c_int;
    static mut sc_width: std::ffi::c_int;
    static mut sc_height: std::ffi::c_int;
    static mut wscroll: std::ffi::c_int;
    static mut quit_on_intr: std::ffi::c_int;
}
pub type __pid_t = std::ffi::c_int;
pub type lbool = std::ffi::c_uint;
pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;
pub type __sighandler_t = Option<unsafe extern "C" fn(std::ffi::c_int) -> ()>;
#[no_mangle]
pub static mut sigs: std::ffi::c_int = 0;
unsafe extern "C" fn u_interrupt(mut type_0: std::ffi::c_int) {
    bell();
    signal(
        2 as std::ffi::c_int,
        Some(u_interrupt as unsafe extern "C" fn(std::ffi::c_int) -> ()),
    );
    sigs |= (1 as std::ffi::c_int) << 0 as std::ffi::c_int;
    set_filter_pattern(0 as *const std::ffi::c_char, 0 as std::ffi::c_int);
    intio();
}
unsafe extern "C" fn stop(mut type_0: std::ffi::c_int) {
    signal(
        20 as std::ffi::c_int,
        Some(stop as unsafe extern "C" fn(std::ffi::c_int) -> ()),
    );
    sigs |= (1 as std::ffi::c_int) << 2 as std::ffi::c_int;
    intio();
}
#[no_mangle]
pub unsafe extern "C" fn winch(mut type_0: std::ffi::c_int) {
    signal(
        28 as std::ffi::c_int,
        Some(winch as unsafe extern "C" fn(std::ffi::c_int) -> ()),
    );
    sigs |= (1 as std::ffi::c_int) << 3 as std::ffi::c_int;
    intio();
}
unsafe extern "C" fn terminate(mut type_0: std::ffi::c_int) {
    quit(15 as std::ffi::c_int);
}

unsafe extern "C" fn sigusr(var: &str) {
    if let Ok(cmd) = lgetenv(var) {
        ungetsc(cmd);
        intio();
    }
}

unsafe extern "C" fn sigusr1(mut type_0: std::ffi::c_int) {
    signal(
        10 as std::ffi::c_int,
        Some(sigusr1 as unsafe extern "C" fn(std::ffi::c_int) -> ()),
    );
    sigusr("LESS_SIGUSR1");
}
#[no_mangle]
pub unsafe extern "C" fn init_signals(mut on: std::ffi::c_int) {
    if on != 0 {
        signal(
            2 as std::ffi::c_int,
            Some(u_interrupt as unsafe extern "C" fn(std::ffi::c_int) -> ()),
        );
        signal(
            20 as std::ffi::c_int,
            if secure_allow((1 as std::ffi::c_int) << 10 as std::ffi::c_int) == 0 {
                ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                    1 as std::ffi::c_int as libc::intptr_t,
                )
            } else {
                Some(stop as unsafe extern "C" fn(std::ffi::c_int) -> ())
            },
        );
        signal(
            28 as std::ffi::c_int,
            Some(winch as unsafe extern "C" fn(std::ffi::c_int) -> ()),
        );
        signal(
            3 as std::ffi::c_int,
            ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                1 as std::ffi::c_int as libc::intptr_t,
            ),
        );
        signal(
            15 as std::ffi::c_int,
            Some(terminate as unsafe extern "C" fn(std::ffi::c_int) -> ()),
        );
        signal(
            10 as std::ffi::c_int,
            Some(sigusr1 as unsafe extern "C" fn(std::ffi::c_int) -> ()),
        );
    } else {
        signal(2 as std::ffi::c_int, None);
        signal(20 as std::ffi::c_int, None);
        signal(
            28 as std::ffi::c_int,
            ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                1 as std::ffi::c_int as libc::intptr_t,
            ),
        );
        signal(3 as std::ffi::c_int, None);
        signal(15 as std::ffi::c_int, None);
        signal(10 as std::ffi::c_int, None);
    };
}
#[no_mangle]
pub unsafe extern "C" fn psignals() {
    let mut tsignals: std::ffi::c_int = 0;
    tsignals = sigs;
    if tsignals == 0 as std::ffi::c_int {
        return;
    }
    sigs = 0 as std::ffi::c_int;
    if tsignals & (1 as std::ffi::c_int) << 2 as std::ffi::c_int != 0 {
        signal(
            22 as std::ffi::c_int,
            ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                1 as std::ffi::c_int as libc::intptr_t,
            ),
        );
        clear_bot();
        deinit();
        flush();
        raw_mode(0 as std::ffi::c_int);
        signal(22 as std::ffi::c_int, None);
        signal(20 as std::ffi::c_int, None);
        kill(getpid(), 20 as std::ffi::c_int);
        signal(
            20 as std::ffi::c_int,
            Some(stop as unsafe extern "C" fn(std::ffi::c_int) -> ()),
        );
        raw_mode(1 as std::ffi::c_int);
        init();
        screen_trashed();
        tsignals |= (1 as std::ffi::c_int) << 3 as std::ffi::c_int;
    }
    if tsignals & (1 as std::ffi::c_int) << 3 as std::ffi::c_int != 0 {
        let mut old_width: std::ffi::c_int = 0;
        let mut old_height: std::ffi::c_int = 0;
        old_width = sc_width;
        old_height = sc_height;
        get_term();
        if sc_width != old_width || sc_height != old_height {
            wscroll = (sc_height + 1 as std::ffi::c_int) / 2 as std::ffi::c_int;
            screen_size_changed();
        }
        screen_trashed();
    }
    if tsignals & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
        if quit_on_intr != 0 {
            quit(1 as std::ffi::c_int + 1 as std::ffi::c_int);
        }
    }
}
