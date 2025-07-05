use ::libc;
extern "C" {
    fn home();
    fn bell();
    fn clear();
    fn ch_seek(pos: POSITION) -> std::ffi::c_int;
    fn ch_end_seek() -> std::ffi::c_int;
    fn ch_end_buffer_seek() -> std::ffi::c_int;
    fn ch_beg_seek() -> std::ffi::c_int;
    fn ch_length() -> POSITION;
    fn ch_tell() -> POSITION;
    fn ch_forw_get() -> std::ffi::c_int;
    fn ch_back_get() -> std::ffi::c_int;
    fn screen_trashed_num(trashed: std::ffi::c_int);
    fn eof_bell();
    fn forw(
        n: std::ffi::c_int,
        pos: POSITION,
        force: lbool,
        only_last: lbool,
        to_newline: lbool,
        nblank: std::ffi::c_int,
    );
    fn back(
        n: std::ffi::c_int,
        pos: POSITION,
        force: lbool,
        only_last: lbool,
        to_newline: lbool,
    );
    fn forw_line(
        curr_pos: POSITION,
        p_linepos: *mut POSITION,
        p_newline: *mut lbool,
    ) -> POSITION;
    fn back_line(curr_pos: POSITION, p_newline: *mut lbool) -> POSITION;
    fn set_attnpos(pos: POSITION);
    fn find_pos(linenum: LINENUM) -> POSITION;
    fn lastmark();
    fn get_time() -> time_t;
    fn percent_pos(
        pos: POSITION,
        percent: std::ffi::c_int,
        fraction: std::ffi::c_long,
    ) -> POSITION;
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn ierror(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn position(sindex: std::ffi::c_int) -> POSITION;
    fn add_back_pos(pos: POSITION);
    fn pos_clear();
    fn onscreen(pos: POSITION) -> std::ffi::c_int;
    fn get_scrpos(scrpos: *mut scrpos, where_0: std::ffi::c_int);
    fn sindex_from_sline(sline: std::ffi::c_int) -> std::ffi::c_int;
    fn repaint_hilite(on: lbool);
    fn next_unfiltered(pos: POSITION) -> POSITION;
    static mut jump_sline: std::ffi::c_int;
    static mut squished: lbool;
    static mut sc_height: std::ffi::c_int;
    static mut show_attn: std::ffi::c_int;
    static mut top_scroll: std::ffi::c_int;
    static mut header_start_pos: POSITION;
}
pub type __off_t = std::ffi::c_long;
pub type __time_t = std::ffi::c_long;
pub type off_t = __off_t;
pub type time_t = __time_t;
pub type lbool = std::ffi::c_uint;
pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;
pub type less_off_t = off_t;
pub type POSITION = less_off_t;
pub type LINENUM = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scrpos {
    pub pos: POSITION,
    pub ln: std::ffi::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn jump_forw() {
    let mut pos: POSITION = 0;
    let mut end_pos: POSITION = 0;
    if ch_end_seek() != 0 {
        error(
            b"Cannot seek to end of file\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        return;
    }
    end_pos = ch_tell();
    if position(sc_height - 1 as std::ffi::c_int) == end_pos {
        eof_bell();
        return;
    }
    lastmark();
    pos_clear();
    pos = back_line(end_pos, 0 as *mut lbool);
    if pos == -(1 as std::ffi::c_int) as POSITION {
        jump_loc(0 as std::ffi::c_int as POSITION, sc_height - 1 as std::ffi::c_int);
    } else {
        jump_loc(pos, sc_height - 1 as std::ffi::c_int);
        if position(sc_height - 1 as std::ffi::c_int) != end_pos {
            repaint();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn jump_forw_buffered() {
    let mut end: POSITION = 0;
    if ch_end_buffer_seek() != 0 {
        error(
            b"Cannot seek to end of buffers\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        return;
    }
    end = ch_tell();
    if end != -(1 as std::ffi::c_int) as POSITION
        && end > 0 as std::ffi::c_int as POSITION
    {
        jump_line_loc(
            end - 1 as std::ffi::c_int as POSITION,
            sc_height - 1 as std::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn jump_back(mut linenum: LINENUM) {
    let mut pos: POSITION = 0;
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    pos = find_pos(linenum);
    if pos != -(1 as std::ffi::c_int) as POSITION && ch_seek(pos) == 0 as std::ffi::c_int
    {
        if show_attn != 0 {
            set_attnpos(pos);
        }
        jump_loc(pos, jump_sline);
    } else if linenum <= 1 as std::ffi::c_int as LINENUM
        && ch_beg_seek() == 0 as std::ffi::c_int
    {
        jump_loc(ch_tell(), jump_sline);
        error(
            b"Cannot seek to beginning of file\0" as *const u8
                as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
    } else {
        parg.p_linenum = linenum;
        error(
            b"Cannot seek to line number %n\0" as *const u8 as *const std::ffi::c_char,
            &mut parg,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn repaint() {
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    get_scrpos(&mut scrpos, 0 as std::ffi::c_int);
    pos_clear();
    if scrpos.pos == -(1 as std::ffi::c_int) as POSITION {
        jump_loc(0 as std::ffi::c_int as POSITION, 1 as std::ffi::c_int);
    } else {
        jump_loc(scrpos.pos, scrpos.ln);
    };
}
#[no_mangle]
pub unsafe extern "C" fn jump_percent(
    mut percent: std::ffi::c_int,
    mut fraction: std::ffi::c_long,
) {
    let mut pos: POSITION = 0;
    let mut len: POSITION = 0;
    len = ch_length();
    if len == -(1 as std::ffi::c_int) as POSITION {
        ierror(
            b"Determining length of file\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        ch_end_seek();
    }
    len = ch_length();
    if len == -(1 as std::ffi::c_int) as POSITION {
        error(
            b"Don't know length of file\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        return;
    }
    pos = percent_pos(len, percent, fraction);
    if pos >= len {
        pos = len - 1 as std::ffi::c_int as POSITION;
    }
    jump_line_loc(pos, jump_sline);
}
#[no_mangle]
pub unsafe extern "C" fn jump_line_loc(mut pos: POSITION, mut sline: std::ffi::c_int) {
    let mut c: std::ffi::c_int = 0;
    if ch_seek(pos) == 0 as std::ffi::c_int {
        loop {
            c = ch_back_get();
            if !(c != '\n' as i32 && c != -(1 as std::ffi::c_int)) {
                break;
            }
        }
        if c == '\n' as i32 {
            ch_forw_get();
        }
        pos = ch_tell();
    }
    if show_attn != 0 {
        set_attnpos(pos);
    }
    jump_loc(pos, sline);
}
unsafe extern "C" fn after_header_message() {
    static mut last_msg: time_t = 0 as std::ffi::c_int as time_t;
    let mut now: time_t = get_time();
    if now < last_msg + 1 as std::ffi::c_int as time_t {
        return;
    }
    last_msg = now;
    bell();
}
#[no_mangle]
pub unsafe extern "C" fn after_header_pos(mut pos: POSITION) -> POSITION {
    if header_start_pos != -(1 as std::ffi::c_int) as POSITION && pos < header_start_pos
    {
        after_header_message();
        pos = header_start_pos;
    }
    return pos;
}
#[no_mangle]
pub unsafe extern "C" fn jump_loc(mut pos: POSITION, mut sline: std::ffi::c_int) {
    let mut nline: std::ffi::c_int = 0;
    let mut sindex: std::ffi::c_int = 0;
    let mut tpos: POSITION = 0;
    let mut bpos: POSITION = 0;
    pos = after_header_pos(pos);
    pos = next_unfiltered(pos);
    sindex = sindex_from_sline(sline);
    nline = onscreen(pos);
    if nline >= 0 as std::ffi::c_int {
        nline -= sindex;
        if nline > 0 as std::ffi::c_int {
            forw(
                nline,
                position(-(2 as std::ffi::c_int)),
                LTRUE,
                LFALSE,
                LFALSE,
                0 as std::ffi::c_int,
            );
        } else {
            back(-nline, position(0 as std::ffi::c_int), LTRUE, LFALSE, LFALSE);
        }
        if show_attn != 0 {
            repaint_hilite(LTRUE);
        }
        return;
    }
    if ch_seek(pos) != 0 {
        error(
            b"Cannot seek to that file position\0" as *const u8
                as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        return;
    }
    tpos = position(0 as std::ffi::c_int);
    bpos = position(-(2 as std::ffi::c_int));
    if tpos == -(1 as std::ffi::c_int) as POSITION || pos >= tpos {
        nline = 0 as std::ffi::c_int;
        while nline < sindex {
            if bpos != -(1 as std::ffi::c_int) as POSITION && pos <= bpos {
                forw(
                    sc_height - sindex + nline - 1 as std::ffi::c_int,
                    bpos,
                    LTRUE,
                    LFALSE,
                    LFALSE,
                    0 as std::ffi::c_int,
                );
                if show_attn != 0 {
                    repaint_hilite(LTRUE);
                }
                return;
            }
            pos = back_line(pos, 0 as *mut lbool);
            if pos == -(1 as std::ffi::c_int) as POSITION {
                break;
            }
            nline += 1;
            nline;
        }
        lastmark();
        squished = LFALSE;
        screen_trashed_num(0 as std::ffi::c_int);
        forw(
            sc_height - 1 as std::ffi::c_int,
            pos,
            LTRUE,
            LFALSE,
            LFALSE,
            sindex - nline,
        );
    } else {
        nline = sindex;
        while nline < sc_height - 1 as std::ffi::c_int {
            let mut linepos: POSITION = 0;
            pos = forw_line(pos, &mut linepos, 0 as *mut lbool);
            if pos == -(1 as std::ffi::c_int) as POSITION {
                break;
            }
            if linepos >= tpos {
                back(nline, tpos, LTRUE, LFALSE, LFALSE);
                if show_attn != 0 {
                    repaint_hilite(LTRUE);
                }
                return;
            }
            nline += 1;
            nline;
        }
        lastmark();
        if top_scroll == 0 {
            clear();
        } else {
            home();
        }
        screen_trashed_num(0 as std::ffi::c_int);
        add_back_pos(pos);
        back(sc_height - 1 as std::ffi::c_int, pos, LTRUE, LFALSE, LFALSE);
    };
}
