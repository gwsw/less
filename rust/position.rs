use ::libc;
extern "C" {
    fn free(_: *mut std::ffi::c_void);
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
    fn ch_seek(pos: POSITION) -> std::ffi::c_int;
    fn ch_tell() -> POSITION;
    fn ch_forw_get() -> std::ffi::c_int;
    fn ch_back_get() -> std::ffi::c_int;
    fn screen_trashed();
    fn cvt_length(len: size_t, ops: std::ffi::c_int) -> size_t;
    fn cvt_text(
        odst: *mut std::ffi::c_char,
        osrc: *const std::ffi::c_char,
        chpos: *mut std::ffi::c_int,
        lenp: *mut size_t,
        ops: std::ffi::c_int,
    );
    fn forw_raw_line_len(
        curr_pos: POSITION,
        read_len: size_t,
        linep: *mut *const std::ffi::c_char,
        line_lenp: *mut size_t,
    ) -> POSITION;
    fn get_cvt_ops(search_type: std::ffi::c_int) -> std::ffi::c_int;
    static mut sc_height: std::ffi::c_int;
    static mut hshift: std::ffi::c_int;
    static mut shell_lines: std::ffi::c_int;
}
pub type __off_t = std::ffi::c_long;
pub type off_t = __off_t;
pub type size_t = std::ffi::c_ulong;
pub type lbool = std::ffi::c_uint;
pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;
pub type less_off_t = off_t;
pub type POSITION = less_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scrpos {
    pub pos: POSITION,
    pub ln: std::ffi::c_int,
}
static mut table: *mut POSITION = 0 as *const POSITION as *mut POSITION;
static mut table_size: std::ffi::c_int = 0 as std::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn position(mut sindex: std::ffi::c_int) -> POSITION {
    match sindex {
        -1 => {
            sindex = sc_height - 2 as std::ffi::c_int;
        }
        -2 => {
            sindex = sc_height - 1 as std::ffi::c_int;
        }
        -4 => {
            sindex = sc_height - shell_lines;
        }
        -3 => {
            sindex = (sc_height - 1 as std::ffi::c_int) / 2 as std::ffi::c_int;
        }
        _ => {}
    }
    return *table.offset(sindex as isize);
}
#[no_mangle]
pub unsafe extern "C" fn add_forw_pos(mut pos: POSITION, mut no_scroll: lbool) {
    let mut i: std::ffi::c_int = 0;
    if no_scroll as u64 == 0 {
        i = 1 as std::ffi::c_int;
        while i < sc_height {
            *table
                .offset((i - 1 as std::ffi::c_int) as isize) = *table.offset(i as isize);
            i += 1;
            i;
        }
    }
    *table.offset((sc_height - 1 as std::ffi::c_int) as isize) = pos;
}
#[no_mangle]
pub unsafe extern "C" fn add_back_pos(mut pos: POSITION) {
    let mut i: std::ffi::c_int = 0;
    i = sc_height - 1 as std::ffi::c_int;
    while i > 0 as std::ffi::c_int {
        *table.offset(i as isize) = *table.offset((i - 1 as std::ffi::c_int) as isize);
        i -= 1;
        i;
    }
    *table.offset(0 as std::ffi::c_int as isize) = pos;
}
#[no_mangle]
pub unsafe extern "C" fn pos_clear() {
    let mut i: std::ffi::c_int = 0;
    i = 0 as std::ffi::c_int;
    while i < sc_height {
        *table.offset(i as isize) = -(1 as std::ffi::c_int) as POSITION;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn pos_init() {
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    if sc_height <= table_size {
        return;
    }
    if !table.is_null() {
        get_scrpos(&mut scrpos, 0 as std::ffi::c_int);
        free(table as *mut std::ffi::c_char as *mut std::ffi::c_void);
    } else {
        scrpos.pos = -(1 as std::ffi::c_int) as POSITION;
    }
    table = ecalloc(
        sc_height as size_t,
        ::core::mem::size_of::<POSITION>() as std::ffi::c_ulong,
    ) as *mut POSITION;
    table_size = sc_height;
    pos_clear();
    if scrpos.pos != -(1 as std::ffi::c_int) as POSITION {
        *table.offset((scrpos.ln - 1 as std::ffi::c_int) as isize) = scrpos.pos;
    }
}
#[no_mangle]
pub unsafe extern "C" fn onscreen(mut pos: POSITION) -> std::ffi::c_int {
    let mut i: std::ffi::c_int = 0;
    if pos < *table.offset(0 as std::ffi::c_int as isize) {
        return -(1 as std::ffi::c_int);
    }
    i = 1 as std::ffi::c_int;
    while i < sc_height {
        if pos < *table.offset(i as isize) {
            return i - 1 as std::ffi::c_int;
        }
        i += 1;
        i;
    }
    return -(1 as std::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn empty_screen() -> std::ffi::c_int {
    return empty_lines(0 as std::ffi::c_int, sc_height - 1 as std::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn empty_lines(
    mut s: std::ffi::c_int,
    mut e: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut i: std::ffi::c_int = 0;
    i = s;
    while i <= e {
        if *table.offset(i as isize) != -(1 as std::ffi::c_int) as POSITION
            && *table.offset(i as isize) != 0 as std::ffi::c_int as POSITION
        {
            return 0 as std::ffi::c_int;
        }
        i += 1;
        i;
    }
    return 1 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_scrpos(
    mut scrpos: *mut scrpos,
    mut where_0: std::ffi::c_int,
) {
    let mut i: std::ffi::c_int = 0;
    let mut dir: std::ffi::c_int = 0;
    let mut last: std::ffi::c_int = 0;
    match where_0 {
        0 => {
            i = 0 as std::ffi::c_int;
            dir = 1 as std::ffi::c_int;
            last = sc_height - 2 as std::ffi::c_int;
        }
        -1 | -2 => {
            i = sc_height - 2 as std::ffi::c_int;
            dir = -(1 as std::ffi::c_int);
            last = 0 as std::ffi::c_int;
        }
        _ => {
            i = where_0;
            if *table.offset(i as isize) == -(1 as std::ffi::c_int) as POSITION {
                (*scrpos).pos = -(1 as std::ffi::c_int) as POSITION;
                return;
            }
        }
    }
    loop {
        if *table.offset(i as isize) != -(1 as std::ffi::c_int) as POSITION {
            (*scrpos).ln = i + 1 as std::ffi::c_int;
            (*scrpos).pos = *table.offset(i as isize);
            return;
        }
        if i == last {
            break;
        }
        i += dir;
    }
    (*scrpos).pos = -(1 as std::ffi::c_int) as POSITION;
}
#[no_mangle]
pub unsafe extern "C" fn sindex_from_sline(
    mut sline: std::ffi::c_int,
) -> std::ffi::c_int {
    if sline < 0 as std::ffi::c_int {
        sline += sc_height;
    }
    if sline <= 0 as std::ffi::c_int {
        sline = 1 as std::ffi::c_int;
    }
    if sline > sc_height {
        sline = sc_height;
    }
    return sline - 1 as std::ffi::c_int;
}
unsafe extern "C" fn pos_shift(
    mut linepos: POSITION,
    mut choff: size_t,
) -> std::ffi::c_int {
    let mut line: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut line_len: size_t = 0;
    let mut pos: POSITION = 0;
    let mut cvt_ops: std::ffi::c_int = 0;
    let mut cline: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    pos = forw_raw_line_len(linepos, choff, &mut line, &mut line_len);
    if pos == -(1 as std::ffi::c_int) as POSITION || line_len != choff {
        return -(1 as std::ffi::c_int);
    }
    cvt_ops = get_cvt_ops(0 as std::ffi::c_int);
    cline = ecalloc(1 as std::ffi::c_int as size_t, cvt_length(line_len, cvt_ops))
        as *mut std::ffi::c_char;
    cvt_text(cline, line, 0 as *mut std::ffi::c_int, &mut line_len, cvt_ops);
    free(cline as *mut std::ffi::c_void);
    return line_len as std::ffi::c_int;
}
unsafe extern "C" fn beginning_of_line(mut tpos: POSITION) -> POSITION {
    ch_seek(tpos);
    while ch_tell() != 0 as std::ffi::c_int as POSITION {
        let mut ch: std::ffi::c_int = ch_back_get();
        if !(ch == '\n' as i32) {
            continue;
        }
        ch_forw_get();
        break;
    }
    return ch_tell();
}
#[no_mangle]
pub unsafe extern "C" fn pos_rehead() {
    let mut linepos: POSITION = 0;
    let mut tpos: POSITION = *table.offset(0 as std::ffi::c_int as isize);
    if tpos == -(1 as std::ffi::c_int) as POSITION {
        return;
    }
    linepos = beginning_of_line(tpos);
    if linepos == tpos {
        return;
    }
    *table.offset(0 as std::ffi::c_int as isize) = linepos;
    hshift = pos_shift(linepos, (tpos - linepos) as size_t);
    screen_trashed();
}
