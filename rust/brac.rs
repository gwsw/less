use ::libc;
extern "C" {
    fn ch_seek(pos: POSITION) -> std::ffi::c_int;
    fn ch_tell() -> POSITION;
    fn ch_forw_get() -> std::ffi::c_int;
    fn ch_back_get() -> std::ffi::c_int;
    fn jump_line_loc(pos: POSITION, sline: std::ffi::c_int);
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn position(sindex: std::ffi::c_int) -> POSITION;
}
pub type __off_t = std::ffi::c_long;
pub type off_t = __off_t;
pub type less_off_t = off_t;
pub type POSITION = less_off_t;
pub type LINENUM = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union parg {
    pub p_string: *const std::ffi::c_char,
    pub p_int: std::ffi::c_int,
    pub p_linenum: LINENUM,
    pub p_char: std::ffi::c_char,
}
pub type PARG = parg;
#[no_mangle]
pub unsafe extern "C" fn match_brac(
    mut obrac: std::ffi::c_char,
    mut cbrac: std::ffi::c_char,
    mut forwdir: std::ffi::c_int,
    mut n: std::ffi::c_int,
) {
    let mut c: std::ffi::c_int = 0;
    let mut nest: std::ffi::c_int = 0;
    let mut pos: POSITION = 0;
    let mut chget: Option<unsafe extern "C" fn() -> std::ffi::c_int> = None;
    pos = position(if forwdir != 0 {
        0 as std::ffi::c_int
    } else {
        -(1 as std::ffi::c_int)
    });
    if pos == -(1 as std::ffi::c_int) as POSITION || ch_seek(pos) != 0 {
        if forwdir != 0 {
            error(
                b"Nothing in top line\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
        } else {
            error(
                b"Nothing in bottom line\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
        }
        return;
    }
    loop {
        c = ch_forw_get();
        if c == '\n' as i32 || c == -(1 as std::ffi::c_int) {
            if forwdir != 0 {
                error(
                    b"No bracket in top line\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
            } else {
                error(
                    b"No bracket in bottom line\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
            }
            return;
        }
        if !(c != obrac as std::ffi::c_int || {
            n -= 1;
            n > 0 as std::ffi::c_int
        }) {
            break;
        }
    }
    if forwdir == 0 {
        ch_back_get();
    }
    chget = ::core::mem::transmute::<
        Option<unsafe extern "C" fn() -> std::ffi::c_int>,
        Option<unsafe extern "C" fn() -> std::ffi::c_int>,
    >(if forwdir != 0 {
        Some(ch_forw_get as unsafe extern "C" fn() -> std::ffi::c_int)
    } else {
        Some(ch_back_get as unsafe extern "C" fn() -> std::ffi::c_int)
    });
    nest = 0 as std::ffi::c_int;
    loop {
        c = ::core::mem::transmute::<_, fn() -> std::ffi::c_int>(
            (Some(chget.expect("non-null function pointer"))).expect("non-null function pointer"),
        )();
        if !(c != -(1 as std::ffi::c_int)) {
            break;
        }
        if c == obrac as std::ffi::c_int {
            if nest == 2147483647 as std::ffi::c_int {
                break;
            }
            nest += 1;
            nest;
        } else if c == cbrac as std::ffi::c_int && {
            nest -= 1;
            nest < 0 as std::ffi::c_int
        } {
            jump_line_loc(
                ch_tell(),
                if forwdir != 0 {
                    -(1 as std::ffi::c_int)
                } else {
                    1 as std::ffi::c_int
                },
            );
            return;
        }
    }
    error(
        b"No matching bracket\0" as *const u8 as *const std::ffi::c_char,
        0 as *mut std::ffi::c_void as *mut PARG,
    );
}
