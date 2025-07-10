use ::libc;
extern "C" {
    fn ch_seek(pos: POSITION) -> std::ffi::c_int;
    fn ch_length() -> POSITION;
    fn screen_trashed();
    fn forw_raw_line(
        curr_pos: POSITION,
        linep: *mut *const std::ffi::c_char,
        line_lenp: *mut size_t,
    ) -> POSITION;
    fn back_raw_line(
        curr_pos: POSITION,
        linep: *mut *const std::ffi::c_char,
        line_lenp: *mut size_t,
    ) -> POSITION;
    fn get_time() -> time_t;
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn ierror(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn position(sindex: std::ffi::c_int) -> POSITION;
    static mut linenums: std::ffi::c_int;
    static mut sigs: std::ffi::c_int;
    static mut sc_height: std::ffi::c_int;
    static mut header_lines: std::ffi::c_int;
    static mut nonum_headers: std::ffi::c_int;
    static mut header_start_pos: POSITION;
}
pub type __off_t = std::ffi::c_long;
pub type __time_t = std::ffi::c_long;
pub type off_t = __off_t;
pub type time_t = __time_t;
pub type size_t = std::ffi::c_ulong;
pub type lbool = std::ffi::c_uint;
pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linenum_info {
    pub next: *mut linenum_info,
    pub prev: *mut linenum_info,
    pub pos: POSITION,
    pub gap: POSITION,
    pub line: LINENUM,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delayed_msg {
    pub message: Option<unsafe extern "C" fn() -> ()>,
    pub loopcount: std::ffi::c_int,
    pub startime: time_t,
}
static mut anchor: linenum_info = linenum_info {
    next: 0 as *const linenum_info as *mut linenum_info,
    prev: 0 as *const linenum_info as *mut linenum_info,
    pos: 0,
    gap: 0,
    line: 0,
};
static mut freelist: *mut linenum_info = 0 as *const linenum_info as *mut linenum_info;
static mut pool: [linenum_info; 1024] = [linenum_info {
    next: 0 as *const linenum_info as *mut linenum_info,
    prev: 0 as *const linenum_info as *mut linenum_info,
    pos: 0,
    gap: 0,
    line: 0,
}; 1024];
static mut spare: *mut linenum_info = 0 as *const linenum_info as *mut linenum_info;
#[no_mangle]
pub static mut scanning_eof: lbool = LFALSE;
#[no_mangle]
pub unsafe extern "C" fn clr_linenum() {
    let mut p: *mut linenum_info = 0 as *mut linenum_info;
    p = pool.as_mut_ptr();
    while p < &mut *pool
        .as_mut_ptr()
        .offset((1024 as std::ffi::c_int - 2 as std::ffi::c_int) as isize)
        as *mut linenum_info
    {
        (*p).next = p.offset(1 as std::ffi::c_int as isize);
        p = p.offset(1);
    }
    pool[(1024 as std::ffi::c_int - 2 as std::ffi::c_int) as usize].next = 0 as *mut linenum_info;
    freelist = pool.as_mut_ptr();
    spare = &mut *pool
        .as_mut_ptr()
        .offset((1024 as std::ffi::c_int - 1 as std::ffi::c_int) as isize)
        as *mut linenum_info;
    anchor.prev = &mut anchor;
    anchor.next = anchor.prev;
    anchor.gap = 0 as std::ffi::c_int as POSITION;
    anchor.pos = 0 as std::ffi::c_int as POSITION;
    anchor.line = 1 as std::ffi::c_int as LINENUM;
}
unsafe extern "C" fn calcgap(mut p: *mut linenum_info) {
    if p == &mut anchor as *mut linenum_info || (*p).next == &mut anchor as *mut linenum_info {
        return;
    }
    (*p).gap = (*(*p).next).pos - (*(*p).prev).pos;
}
#[no_mangle]
pub unsafe extern "C" fn add_lnum(mut linenum: LINENUM, mut pos: POSITION) {
    let mut p: *mut linenum_info = 0 as *mut linenum_info;
    let mut new: *mut linenum_info = 0 as *mut linenum_info;
    let mut nextp: *mut linenum_info = 0 as *mut linenum_info;
    let mut prevp: *mut linenum_info = 0 as *mut linenum_info;
    let mut mingap: POSITION = 0;
    p = anchor.next;
    while p != &mut anchor as *mut linenum_info && (*p).pos < pos {
        if (*p).line == linenum {
            return;
        }
        p = (*p).next;
    }
    nextp = p;
    prevp = (*p).prev;
    if !freelist.is_null() {
        new = freelist;
        freelist = (*freelist).next;
    } else {
        new = spare;
        spare = 0 as *mut linenum_info;
    }
    (*new).next = nextp;
    (*new).prev = prevp;
    (*new).pos = pos;
    (*new).line = linenum;
    (*nextp).prev = new;
    (*prevp).next = new;
    calcgap(new);
    calcgap(nextp);
    calcgap(prevp);
    if spare.is_null() {
        mingap = (*anchor.next).gap;
        p = anchor.next;
        while (*p).next != &mut anchor as *mut linenum_info {
            if (*p).gap <= mingap {
                spare = p;
                mingap = (*p).gap;
            }
            p = (*p).next;
        }
        (*(*spare).next).prev = (*spare).prev;
        (*(*spare).prev).next = (*spare).next;
    }
}
unsafe extern "C" fn longloopmessage() {
    ierror(
        b"Calculating line numbers\0" as *const u8 as *const std::ffi::c_char,
        0 as *mut std::ffi::c_void as *mut PARG,
    );
}
unsafe extern "C" fn start_delayed_msg(
    mut dmsg: *mut delayed_msg,
    mut message: Option<unsafe extern "C" fn() -> ()>,
) {
    (*dmsg).loopcount = 0 as std::ffi::c_int;
    (*dmsg).message = message;
    (*dmsg).startime = get_time();
}
unsafe extern "C" fn delayed_msg(mut dmsg: *mut delayed_msg) {
    if (*dmsg).loopcount >= 0 as std::ffi::c_int && {
        (*dmsg).loopcount += 1;
        (*dmsg).loopcount > 100 as std::ffi::c_int
    } {
        (*dmsg).loopcount = 0 as std::ffi::c_int;
        if get_time() >= (*dmsg).startime + 2 as std::ffi::c_int as time_t {
            ((*dmsg).message).expect("non-null function pointer")();
            (*dmsg).loopcount = -(1 as std::ffi::c_int);
        }
    }
}
unsafe extern "C" fn abort_delayed_msg(mut dmsg: *mut delayed_msg) {
    if (*dmsg).loopcount >= 0 as std::ffi::c_int {
        return;
    }
    if linenums == 2 as std::ffi::c_int {
        screen_trashed();
    }
    linenums = 0 as std::ffi::c_int;
    error(
        b"Line numbers turned off\0" as *const u8 as *const std::ffi::c_char,
        0 as *mut std::ffi::c_void as *mut PARG,
    );
}
#[no_mangle]
pub unsafe extern "C" fn find_linenum(mut pos: POSITION) -> LINENUM {
    let mut p: *mut linenum_info = 0 as *mut linenum_info;
    let mut linenum: LINENUM = 0;
    let mut cpos: POSITION = 0;
    let mut dmsg: delayed_msg = delayed_msg {
        message: None,
        loopcount: 0,
        startime: 0,
    };
    if linenums == 0 {
        return 0 as std::ffi::c_int as LINENUM;
    }
    if pos == -(1 as std::ffi::c_int) as POSITION {
        return 0 as std::ffi::c_int as LINENUM;
    }
    if pos <= 0 as std::ffi::c_int as POSITION {
        return 1 as std::ffi::c_int as LINENUM;
    }
    p = anchor.next;
    while p != &mut anchor as *mut linenum_info && (*p).pos < pos {
        p = (*p).next;
    }
    if (*p).pos == pos {
        return (*p).line;
    }
    start_delayed_msg(
        &mut dmsg,
        Some(longloopmessage as unsafe extern "C" fn() -> ()),
    );
    if p == &mut anchor as *mut linenum_info || pos - (*(*p).prev).pos < (*p).pos - pos {
        p = (*p).prev;
        if ch_seek((*p).pos) != 0 {
            return 0 as std::ffi::c_int as LINENUM;
        }
        linenum = (*p).line;
        cpos = (*p).pos;
        while cpos < pos {
            cpos = forw_raw_line(cpos, 0 as *mut *const std::ffi::c_char, 0 as *mut size_t);
            if sigs
                & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
                != 0
            {
                abort_delayed_msg(&mut dmsg);
                return 0 as std::ffi::c_int as LINENUM;
            }
            if cpos == -(1 as std::ffi::c_int) as POSITION {
                return 0 as std::ffi::c_int as LINENUM;
            }
            delayed_msg(&mut dmsg);
            linenum += 1;
        }
        add_lnum(linenum, cpos);
        if cpos > pos {
            linenum -= 1;
        }
    } else {
        if ch_seek((*p).pos) != 0 {
            return 0 as std::ffi::c_int as LINENUM;
        }
        linenum = (*p).line;
        cpos = (*p).pos;
        while cpos > pos {
            cpos = back_raw_line(cpos, 0 as *mut *const std::ffi::c_char, 0 as *mut size_t);
            if sigs
                & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
                != 0
            {
                abort_delayed_msg(&mut dmsg);
                return 0 as std::ffi::c_int as LINENUM;
            }
            if cpos == -(1 as std::ffi::c_int) as POSITION {
                return 0 as std::ffi::c_int as LINENUM;
            }
            delayed_msg(&mut dmsg);
            linenum -= 1;
        }
        add_lnum(linenum, cpos);
    }
    return linenum;
}
#[no_mangle]
pub unsafe extern "C" fn find_pos(mut linenum: LINENUM) -> POSITION {
    let mut p: *mut linenum_info = 0 as *mut linenum_info;
    let mut cpos: POSITION = 0;
    let mut clinenum: LINENUM = 0;
    if linenum <= 1 as std::ffi::c_int as LINENUM {
        return 0 as std::ffi::c_int as POSITION;
    }
    p = anchor.next;
    while p != &mut anchor as *mut linenum_info && (*p).line < linenum {
        p = (*p).next;
    }
    if (*p).line == linenum {
        return (*p).pos;
    }
    if p == &mut anchor as *mut linenum_info || linenum - (*(*p).prev).line < (*p).line - linenum {
        p = (*p).prev;
        if ch_seek((*p).pos) != 0 {
            return -(1 as std::ffi::c_int) as POSITION;
        }
        clinenum = (*p).line;
        cpos = (*p).pos;
        while clinenum < linenum {
            cpos = forw_raw_line(cpos, 0 as *mut *const std::ffi::c_char, 0 as *mut size_t);
            if sigs
                & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
                != 0
            {
                return -(1 as std::ffi::c_int) as POSITION;
            }
            if cpos == -(1 as std::ffi::c_int) as POSITION {
                return -(1 as std::ffi::c_int) as POSITION;
            }
            clinenum += 1;
        }
    } else {
        if ch_seek((*p).pos) != 0 {
            return -(1 as std::ffi::c_int) as POSITION;
        }
        clinenum = (*p).line;
        cpos = (*p).pos;
        while clinenum > linenum {
            cpos = back_raw_line(cpos, 0 as *mut *const std::ffi::c_char, 0 as *mut size_t);
            if sigs
                & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
                != 0
            {
                return -(1 as std::ffi::c_int) as POSITION;
            }
            if cpos == -(1 as std::ffi::c_int) as POSITION {
                return -(1 as std::ffi::c_int) as POSITION;
            }
            clinenum -= 1;
        }
    }
    add_lnum(clinenum, cpos);
    return cpos;
}
#[no_mangle]
pub unsafe extern "C" fn currline(mut where_0: std::ffi::c_int) -> LINENUM {
    let mut pos: POSITION = 0;
    let mut len: POSITION = 0;
    let mut linenum: LINENUM = 0;
    pos = position(where_0);
    len = ch_length();
    while pos == -(1 as std::ffi::c_int) as POSITION
        && where_0 >= 0 as std::ffi::c_int
        && where_0 < sc_height
    {
        where_0 += 1;
        pos = position(where_0);
    }
    if pos == -(1 as std::ffi::c_int) as POSITION {
        pos = len;
    }
    linenum = find_linenum(pos);
    if pos == len {
        linenum -= 1;
    }
    return linenum;
}
unsafe extern "C" fn detlenmessage() {
    ierror(
        b"Determining length of file\0" as *const u8 as *const std::ffi::c_char,
        0 as *mut std::ffi::c_void as *mut PARG,
    );
}
#[no_mangle]
pub unsafe extern "C" fn scan_eof() {
    let mut pos: POSITION = 0 as std::ffi::c_int as POSITION;
    let mut linenum: LINENUM = 0 as std::ffi::c_int as LINENUM;
    let mut dmsg: delayed_msg = delayed_msg {
        message: None,
        loopcount: 0,
        startime: 0,
    };
    if ch_seek(0 as std::ffi::c_int as POSITION) != 0 {
        return;
    }
    start_delayed_msg(
        &mut dmsg,
        Some(detlenmessage as unsafe extern "C" fn() -> ()),
    );
    scanning_eof = LTRUE;
    while pos != -(1 as std::ffi::c_int) as POSITION {
        let fresh0 = linenum;
        linenum = linenum + 1;
        if fresh0 % 256 as std::ffi::c_int as LINENUM == 0 as std::ffi::c_int as LINENUM {
            add_lnum(linenum, pos);
        }
        pos = forw_raw_line(pos, 0 as *mut *const std::ffi::c_char, 0 as *mut size_t);
        if sigs
            & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
            != 0
        {
            abort_delayed_msg(&mut dmsg);
            break;
        } else {
            delayed_msg(&mut dmsg);
        }
    }
    scanning_eof = LFALSE;
}
#[no_mangle]
pub unsafe extern "C" fn vlinenum(mut linenum: LINENUM) -> LINENUM {
    if nonum_headers != 0 && header_lines > 0 as std::ffi::c_int {
        let mut header_start_line: LINENUM = find_linenum(header_start_pos);
        if header_start_line != 0 as std::ffi::c_int as LINENUM {
            let mut header_end_line: LINENUM = header_start_line + header_lines as LINENUM;
            linenum = if linenum < header_end_line {
                0 as std::ffi::c_int as LINENUM
            } else {
                linenum - header_end_line + 1 as std::ffi::c_int as LINENUM
            };
        }
    }
    return linenum;
}
