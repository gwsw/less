use crate::defs::*;
extern "C" {
    fn ch_seek(pos: POSITION) -> std::ffi::c_int;
    fn ch_tell() -> POSITION;
    fn ch_forw_get() -> std::ffi::c_int;
    fn ch_back_get() -> std::ffi::c_int;
    fn jump_line_loc(pos: POSITION, sline: std::ffi::c_int);
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn position(sindex: std::ffi::c_int) -> POSITION;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union parg {
    pub p_string: *const std::ffi::c_char,
    pub p_int: std::ffi::c_int,
    pub p_linenum: LINENUM,
    pub p_char: std::ffi::c_char,
}
pub type PARG = parg;

/*
 * Try to match the n-th open bracket
 *  which appears in the top displayed line (forwdir),
 * or the n-th close bracket
 *  which appears in the bottom displayed line (!forwdir).
 * The characters which serve as "open bracket" and
 * "close bracket" are given.
 */
#[no_mangle]
pub unsafe extern "C" fn match_brac(mut obrac: u8, mut cbrac: u8, forwdir: bool, mut n: i32) {
    let mut c = 0;
    let mut nest = 0;
    let mut pos: POSITION = 0;
    let mut chget: Option<unsafe extern "C" fn() -> std::ffi::c_int> = None;

    /*
     * Seek to the line containing the open bracket.
     * This is either the top or bottom line on the screen,
     * depending on the type of bracket.
     */
    pos = position(if forwdir { 0 } else { -1 });
    if pos == -1 || ch_seek(pos) != 0 {
        if forwdir {
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

    /*
     * Look thru the line to find the open bracket to match.
     */
    loop {
        c = ch_forw_get();
        if c == '\n' as i32 || c == -1 {
            if forwdir {
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
        if !(c != obrac as i32 || {
            n -= 1;
            n > 0 as std::ffi::c_int
        }) {
            break;
        }
    }

    /*
     * Position the file just "after" the open bracket
     * (in the direction in which we will be searching).
     * If searching forward, we are already after the bracket.
     * If searching backward, skip back over the open bracket.
     */
    if !forwdir {
        ch_back_get();
    }
    /*
     * Search the file for the matching bracket.
     */
    chget = ::core::mem::transmute::<
        Option<unsafe extern "C" fn() -> std::ffi::c_int>,
        Option<unsafe extern "C" fn() -> std::ffi::c_int>,
    >(if forwdir {
        Some(ch_forw_get as unsafe extern "C" fn() -> std::ffi::c_int)
    } else {
        Some(ch_back_get as unsafe extern "C" fn() -> std::ffi::c_int)
    });
    nest = 0;
    loop {
        c = ::core::mem::transmute::<_, fn() -> std::ffi::c_int>(
            (Some(chget.expect("non-null function pointer"))).expect("non-null function pointer"),
        )();
        if !(c != -(1 as std::ffi::c_int)) {
            break;
        }
        if c == obrac as std::ffi::c_int {
            if nest == i32::MAX {
                break;
            }
            nest += 1;
        } else if c == cbrac as std::ffi::c_int && {
            nest -= 1;
            nest < 0 as std::ffi::c_int
        } {
            /*
             * Found the matching bracket.
             * If searching backward, put it on the top line.
             * If searching forward, put it on the bottom line.
             */
            jump_line_loc(
                ch_tell(),
                if forwdir {
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
