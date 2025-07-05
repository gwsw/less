use ::libc;
extern "C" {
    pub type ansi_state_0;
    fn iswupper(__wc: wint_t) -> std::ffi::c_int;
    fn towlower(__wc: wint_t) -> wint_t;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
    fn put_wchar(pp: *mut *mut std::ffi::c_char, ch: LWCHAR);
    fn step_charc(
        pp: *mut *const std::ffi::c_char,
        dir: std::ffi::c_int,
        limit: *const std::ffi::c_char,
    ) -> LWCHAR;
    fn ansi_start(ch: LWCHAR) -> *mut ansi_state_0;
    fn ansi_step(pansi: *mut ansi_state_0, ch: LWCHAR) -> ansi_state;
    fn ansi_done(pansi: *mut ansi_state_0);
    static mut utf_mode: std::ffi::c_int;
}
pub type size_t = std::ffi::c_ulong;
pub type wint_t = std::ffi::c_uint;
pub type LWCHAR = std::ffi::c_ulong;
pub type ansi_state = std::ffi::c_uint;
pub const ANSI_END: ansi_state = 3;
pub const ANSI_ERR: ansi_state = 2;
pub const ANSI_MID: ansi_state = 1;
pub const ANSI_NULL: ansi_state = 0;
#[no_mangle]
pub unsafe extern "C" fn cvt_length(
    mut len: size_t,
    mut ops: std::ffi::c_int,
) -> size_t {
    if utf_mode != 0 {
        len = len * 4 as std::ffi::c_int as size_t;
    }
    return len.wrapping_add(1 as std::ffi::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn cvt_alloc_chpos(mut len: size_t) -> *mut std::ffi::c_int {
    let mut i: size_t = 0;
    let mut chpos: *mut std::ffi::c_int = ecalloc(
        len,
        ::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_int;
    i = 0 as std::ffi::c_int as size_t;
    while i < len {
        *chpos.offset(i as isize) = -(1 as std::ffi::c_int);
        i = i.wrapping_add(1);
        i;
    }
    return chpos;
}
#[no_mangle]
pub unsafe extern "C" fn cvt_text(
    mut odst: *mut std::ffi::c_char,
    mut osrc: *const std::ffi::c_char,
    mut chpos: *mut std::ffi::c_int,
    mut lenp: *mut size_t,
    mut ops: std::ffi::c_int,
) {
    let mut dst: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut edst: *mut std::ffi::c_char = odst;
    let mut src: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut src_end: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut ch: LWCHAR = 0;
    if !lenp.is_null() {
        src_end = osrc.offset(*lenp as isize);
    } else {
        src_end = osrc.offset(strlen(osrc) as isize);
    }
    src = osrc;
    dst = odst;
    while src < src_end {
        let mut src_pos: size_t = src.offset_from(osrc) as std::ffi::c_long as size_t;
        let mut dst_pos: size_t = dst.offset_from(odst) as std::ffi::c_long as size_t;
        let mut pansi: *mut ansi_state_0 = 0 as *mut ansi_state_0;
        ch = step_charc(&mut src, 1 as std::ffi::c_int, src_end);
        if ops & 0o2 as std::ffi::c_int != 0 && ch == '\u{8}' as i32 as LWCHAR
            && dst > odst
        {
            loop {
                dst = dst.offset(-1);
                dst;
                if !(dst > odst && utf_mode != 0
                    && !(*dst as std::ffi::c_int & 0x80 as std::ffi::c_int
                        == 0 as std::ffi::c_int)
                    && !(*dst as std::ffi::c_int & 0xc0 as std::ffi::c_int
                        == 0xc0 as std::ffi::c_int
                        && !(*dst as std::ffi::c_int & 0xfe as std::ffi::c_int
                            == 0xfe as std::ffi::c_int)))
                {
                    break;
                }
            }
        } else if ops & 0o10 as std::ffi::c_int != 0
            && {
                pansi = ansi_start(ch);
                !pansi.is_null()
            }
        {
            while src < src_end {
                if ansi_step(pansi, ch) as std::ffi::c_uint
                    != ANSI_MID as std::ffi::c_int as std::ffi::c_uint
                {
                    break;
                }
                let fresh0 = src;
                src = src.offset(1);
                ch = *fresh0 as LWCHAR;
            }
            ansi_done(pansi);
        } else {
            let mut cdst: *mut std::ffi::c_char = dst;
            if ops & 0o1 as std::ffi::c_int != 0 && iswupper(ch as wint_t) != 0 {
                ch = towlower(ch as wint_t) as LWCHAR;
            }
            put_wchar(&mut dst, ch);
            if !chpos.is_null() {
                loop {
                    let fresh1 = cdst;
                    cdst = cdst.offset(1);
                    if !(fresh1 < dst) {
                        break;
                    }
                    let fresh2 = dst_pos;
                    dst_pos = dst_pos.wrapping_add(1);
                    *chpos.offset(fresh2 as isize) = src_pos as std::ffi::c_int;
                }
            }
        }
        if dst > edst {
            edst = dst;
        }
    }
    if ops & 0o4 as std::ffi::c_int != 0 && edst > odst
        && *edst.offset(-(1 as std::ffi::c_int) as isize) as std::ffi::c_int
            == '\r' as i32
    {
        edst = edst.offset(-1);
        edst;
    }
    *edst = '\0' as i32 as std::ffi::c_char;
    if !lenp.is_null() {
        *lenp = edst.offset_from(odst) as std::ffi::c_long as size_t;
    }
}
