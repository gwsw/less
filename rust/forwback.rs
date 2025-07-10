use ::libc;
extern "C" {
    fn quit(status: std::ffi::c_int);
    fn home();
    fn add_line();
    fn lower_left();
    fn vbell();
    fn bell();
    fn clear();
    fn clear_eol();
    fn ch_length() -> POSITION;
    fn ch_getflags() -> std::ffi::c_int;
    fn edit_next(n: std::ffi::c_int) -> std::ffi::c_int;
    fn forw_line_seg(
        curr_pos: POSITION,
        skipeol: lbool,
        rscroll: lbool,
        nochop: lbool,
        p_linepos: *mut POSITION,
        p_newline: *mut lbool,
    ) -> POSITION;
    fn forw_line(curr_pos: POSITION, p_linepos: *mut POSITION, p_newline: *mut lbool) -> POSITION;
    fn back_line(curr_pos: POSITION, p_newline: *mut lbool) -> POSITION;
    fn repaint();
    fn after_header_pos(pos: POSITION) -> POSITION;
    fn line_pfx_width() -> std::ffi::c_int;
    fn line_is_ff() -> lbool;
    fn set_attr_line(a: std::ffi::c_int);
    fn currline(where_0: std::ffi::c_int) -> LINENUM;
    fn get_quit_at_eof() -> std::ffi::c_int;
    fn get_time() -> time_t;
    fn put_line(forw_scroll_0: lbool);
    fn putchr(ch: std::ffi::c_int) -> std::ffi::c_int;
    fn putstr(s: *const std::ffi::c_char);
    fn position(sindex: std::ffi::c_int) -> POSITION;
    fn add_forw_pos(pos: POSITION, no_scroll: lbool);
    fn add_back_pos(pos: POSITION);
    fn pos_clear();
    fn empty_screen() -> std::ffi::c_int;
    fn empty_lines(s: std::ffi::c_int, e: std::ffi::c_int) -> std::ffi::c_int;
    fn is_filtering() -> lbool;
    static mut sigs: std::ffi::c_int;
    static mut top_scroll: std::ffi::c_int;
    static mut quiet: std::ffi::c_int;
    static mut sc_width: std::ffi::c_int;
    static mut sc_height: std::ffi::c_int;
    static mut hshift: std::ffi::c_int;
    static mut auto_wrap: std::ffi::c_int;
    static mut plusoption: lbool;
    static mut forw_scroll: std::ffi::c_int;
    static mut back_scroll: std::ffi::c_int;
    static mut ignore_eoi: std::ffi::c_int;
    static mut header_lines: std::ffi::c_int;
    static mut header_cols: std::ffi::c_int;
    static mut full_screen: std::ffi::c_int;
    static mut stop_on_form_feed: std::ffi::c_int;
    static mut header_start_pos: POSITION;
    static mut getting_one_screen: lbool;
    static mut tagoption: *mut std::ffi::c_char;
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
#[no_mangle]
pub static mut squished: lbool = LFALSE;
#[no_mangle]
pub static mut no_back_scroll: std::ffi::c_int = 0 as std::ffi::c_int;
#[no_mangle]
pub static mut forw_prompt: std::ffi::c_int = 0;
#[no_mangle]
pub static mut first_time: lbool = LTRUE;
#[no_mangle]
pub static mut shell_lines: std::ffi::c_int = 1 as std::ffi::c_int;
#[no_mangle]
pub static mut soft_eof: POSITION = -(1 as std::ffi::c_int) as POSITION;
#[no_mangle]
pub unsafe extern "C" fn eof_bell() {
    static mut last_eof_bell: time_t = 0 as std::ffi::c_int as time_t;
    let mut now: time_t = get_time();
    if now == last_eof_bell {
        return;
    }
    last_eof_bell = now;
    if quiet == 0 as std::ffi::c_int {
        bell();
    } else {
        vbell();
    };
}
#[no_mangle]
pub unsafe extern "C" fn eof_displayed(mut offset: lbool) -> lbool {
    let mut pos: POSITION = 0;
    if ignore_eoi != 0 {
        return LFALSE;
    }
    if ch_length() == -(1 as std::ffi::c_int) as POSITION {
        return LFALSE;
    }
    pos = position(if offset as std::ffi::c_uint != 0 {
        -(4 as std::ffi::c_int)
    } else {
        -(2 as std::ffi::c_int)
    });
    return (pos == -(1 as std::ffi::c_int) as POSITION || pos == ch_length() || pos == soft_eof)
        as std::ffi::c_int as lbool;
}
#[no_mangle]
pub unsafe extern "C" fn entire_file_displayed() -> lbool {
    let mut pos: POSITION = 0;
    if eof_displayed(LTRUE) as u64 == 0 {
        return LFALSE;
    }
    pos = position(0 as std::ffi::c_int);
    return (pos == -(1 as std::ffi::c_int) as POSITION || pos == 0 as std::ffi::c_int as POSITION)
        as std::ffi::c_int as lbool;
}
#[no_mangle]
pub unsafe extern "C" fn squish_check() {
    if squished as u64 == 0 {
        return;
    }
    squished = LFALSE;
    repaint();
}
unsafe extern "C" fn forw_line_pfx(
    mut pos: POSITION,
    mut pfx: std::ffi::c_int,
    mut skipeol: std::ffi::c_int,
) -> POSITION {
    let mut save_sc_width: std::ffi::c_int = sc_width;
    let mut save_auto_wrap: std::ffi::c_int = auto_wrap;
    let mut save_hshift: std::ffi::c_int = hshift;
    sc_width = pfx + line_pfx_width();
    auto_wrap = 0 as std::ffi::c_int;
    hshift = 0 as std::ffi::c_int;
    pos = forw_line_seg(
        pos,
        skipeol as lbool,
        LFALSE,
        LFALSE,
        0 as *mut POSITION,
        0 as *mut lbool,
    );
    sc_width = save_sc_width;
    auto_wrap = save_auto_wrap;
    hshift = save_hshift;
    return pos;
}
unsafe extern "C" fn set_attr_header(mut ln: std::ffi::c_int) {
    set_attr_line((9 as std::ffi::c_int) << 8 as std::ffi::c_int);
    if ln + 1 as std::ffi::c_int == header_lines
        && position(0 as std::ffi::c_int) != header_start_pos
    {
        set_attr_line((1 as std::ffi::c_int) << 0 as std::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn overlay_header() -> std::ffi::c_int {
    let mut ln: std::ffi::c_int = 0;
    let mut moved: lbool = LFALSE;
    if header_lines > 0 as std::ffi::c_int {
        let mut pos: POSITION = header_start_pos;
        home();
        ln = 0 as std::ffi::c_int;
        while ln < header_lines {
            pos = forw_line(pos, 0 as *mut POSITION, 0 as *mut lbool);
            set_attr_header(ln);
            clear_eol();
            put_line(LFALSE);
            ln += 1;
        }
        moved = LTRUE;
    }
    if header_cols > 0 as std::ffi::c_int {
        let mut pos_0: POSITION = header_start_pos;
        home();
        ln = 0 as std::ffi::c_int;
        while ln < sc_height - 1 as std::ffi::c_int {
            if ln >= header_lines {
                pos_0 = position(ln);
            }
            if pos_0 == -(1 as std::ffi::c_int) as POSITION {
                putchr('\n' as i32);
            } else {
                pos_0 = forw_line_pfx(
                    pos_0,
                    header_cols,
                    ((ln + 1 as std::ffi::c_int) < header_lines) as std::ffi::c_int,
                );
                set_attr_header(ln);
                put_line(LFALSE);
            }
            ln += 1;
        }
        moved = LTRUE;
    }
    if moved as u64 != 0 {
        lower_left();
    }
    return moved as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn forw(
    mut n: std::ffi::c_int,
    mut pos: POSITION,
    mut force: lbool,
    mut only_last: lbool,
    mut to_newline: lbool,
    mut nblank: std::ffi::c_int,
) {
    let mut nlines: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut do_repaint: lbool = LFALSE;
    let mut newline: lbool = LFALSE;
    let mut first_line: lbool = LTRUE;
    if pos != -(1 as std::ffi::c_int) as POSITION {
        pos = after_header_pos(pos);
    }
    squish_check();
    do_repaint = (only_last as std::ffi::c_uint != 0 && n > sc_height - 1 as std::ffi::c_int
        || forw_scroll >= 0 as std::ffi::c_int
            && n > forw_scroll
            && n != sc_height - 1 as std::ffi::c_int) as std::ffi::c_int as lbool;
    if do_repaint as u64 == 0 {
        if top_scroll != 0 && n >= sc_height - 1 as std::ffi::c_int && pos != ch_length() {
            pos_clear();
            force = LTRUE;
            clear();
            home();
        }
        if pos != position(-(2 as std::ffi::c_int)) || empty_screen() != 0 {
            pos_clear();
            force = LTRUE;
            if top_scroll != 0 {
                clear();
                home();
            } else if first_time as u64 == 0 && is_filtering() as u64 == 0 && full_screen != 0 {
                putstr(b"...skipping...\n\0" as *const u8 as *const std::ffi::c_char);
            }
        }
    }
    loop {
        n -= 1;
        if !(n >= 0 as std::ffi::c_int) {
            break;
        }
        let mut linepos: POSITION = -(1 as std::ffi::c_int) as POSITION;
        if nblank > 0 as std::ffi::c_int {
            nblank -= 1;
            if nblank == 0 as std::ffi::c_int {
                pos = 0 as std::ffi::c_int as POSITION;
            }
        } else {
            let mut opos: POSITION = pos;
            pos = forw_line(pos, &mut linepos, &mut newline);
            if to_newline as std::ffi::c_uint != 0 && newline as u64 == 0 {
                n += 1;
            }
            if pos == -(1 as std::ffi::c_int) as POSITION {
                soft_eof = opos;
                linepos = opos;
                if sigs
                    & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                        | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                        | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
                    != 0
                    || force as u64 == 0
                        && position(0 as std::ffi::c_int) != -(1 as std::ffi::c_int) as POSITION
                    || empty_lines(0 as std::ffi::c_int, 0 as std::ffi::c_int) == 0
                        && empty_lines(1 as std::ffi::c_int, 1 as std::ffi::c_int) == 0
                        && empty_lines(2 as std::ffi::c_int, sc_height - 1 as std::ffi::c_int) != 0
                {
                    pos = opos;
                    break;
                }
            }
        }
        add_forw_pos(linepos, first_line);
        first_line = LFALSE;
        nlines += 1;
        if do_repaint as u64 != 0 {
            continue;
        }
        if first_time as std::ffi::c_uint != 0
            && pos == -(1 as std::ffi::c_int) as POSITION
            && top_scroll == 0
            && header_lines == 0 as std::ffi::c_int
            && header_cols == 0 as std::ffi::c_int
            && tagoption.is_null()
            && plusoption as u64 == 0
        {
            squished = LTRUE;
        } else {
            put_line(LTRUE);
            if stop_on_form_feed != 0
                && do_repaint as u64 == 0
                && line_is_ff() as std::ffi::c_uint != 0
                && position(0 as std::ffi::c_int) != -(1 as std::ffi::c_int) as POSITION
            {
                break;
            }
            forw_prompt = 1 as std::ffi::c_int;
        }
    }
    if first_line as u64 == 0 {
        add_forw_pos(pos, LFALSE);
    }
    if nlines == 0 as std::ffi::c_int && ignore_eoi == 0 {
        eof_bell();
    } else if do_repaint as u64 != 0 {
        repaint();
    } else {
        overlay_header();
    }
    first_time = LFALSE;
    currline(-(1 as std::ffi::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn back(
    mut n: std::ffi::c_int,
    mut pos: POSITION,
    mut force: lbool,
    mut only_last: lbool,
    mut to_newline: lbool,
) {
    let mut nlines: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut do_repaint: lbool = LFALSE;
    let mut newline: lbool = LFALSE;
    squish_check();
    do_repaint = (n > get_back_scroll()
        || only_last as std::ffi::c_uint != 0 && n > sc_height - 1 as std::ffi::c_int
        || header_lines > 0 as std::ffi::c_int) as std::ffi::c_int as lbool;
    loop {
        n -= 1;
        if !(n >= 0 as std::ffi::c_int) {
            break;
        }
        pos = back_line(pos, &mut newline);
        if to_newline as std::ffi::c_uint != 0 && newline as u64 == 0 {
            n += 1;
        }
        if pos == -(1 as std::ffi::c_int) as POSITION {
            if force as u64 == 0 {
                break;
            }
        }
        if pos != after_header_pos(pos) {
            break;
        }
        add_back_pos(pos);
        nlines += 1;
        if !(do_repaint as u64 == 0) {
            continue;
        }
        home();
        add_line();
        put_line(LFALSE);
        if stop_on_form_feed != 0 && line_is_ff() as std::ffi::c_uint != 0 {
            break;
        }
    }
    if nlines == 0 as std::ffi::c_int {
        eof_bell();
    } else if do_repaint as u64 != 0 {
        repaint();
    } else {
        overlay_header();
        lower_left();
    }
    currline(-(1 as std::ffi::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn forward(
    mut n: std::ffi::c_int,
    mut force: lbool,
    mut only_last: lbool,
    mut to_newline: lbool,
) {
    let mut pos: POSITION = 0;
    if get_quit_at_eof() != 0
        && eof_displayed(LFALSE) as std::ffi::c_uint != 0
        && ch_getflags() & 0o10 as std::ffi::c_int == 0
    {
        if edit_next(1 as std::ffi::c_int) != 0 {
            quit(0 as std::ffi::c_int);
        }
        return;
    }
    pos = position(-(2 as std::ffi::c_int));
    if pos == -(1 as std::ffi::c_int) as POSITION
        && (force as u64 == 0
            || empty_lines(2 as std::ffi::c_int, sc_height - 1 as std::ffi::c_int) != 0)
    {
        if ignore_eoi != 0 {
            if empty_screen() != 0 {
                pos = 0 as std::ffi::c_int as POSITION;
            } else {
                loop {
                    back(
                        1 as std::ffi::c_int,
                        position(0 as std::ffi::c_int),
                        LTRUE,
                        LFALSE,
                        LFALSE,
                    );
                    pos = position(-(2 as std::ffi::c_int));
                    if !(pos == -(1 as std::ffi::c_int) as POSITION
                        && sigs
                            & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                                | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                                | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
                            == 0)
                    {
                        break;
                    }
                }
            }
        } else {
            eof_bell();
            return;
        }
    }
    forw(n, pos, force, only_last, to_newline, 0 as std::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn backward(
    mut n: std::ffi::c_int,
    mut force: lbool,
    mut only_last: lbool,
    mut to_newline: lbool,
) {
    let mut pos: POSITION = 0;
    pos = position(0 as std::ffi::c_int);
    if pos == -(1 as std::ffi::c_int) as POSITION
        && (force as u64 == 0
            || position(-(1 as std::ffi::c_int)) == 0 as std::ffi::c_int as POSITION)
    {
        eof_bell();
        return;
    }
    back(n, pos, force, only_last, to_newline);
}
#[no_mangle]
pub unsafe extern "C" fn get_back_scroll() -> std::ffi::c_int {
    if no_back_scroll != 0 {
        return 0 as std::ffi::c_int;
    }
    if back_scroll >= 0 as std::ffi::c_int {
        return back_scroll;
    }
    if top_scroll != 0 {
        return sc_height - 2 as std::ffi::c_int;
    }
    return 10000 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_one_screen() -> lbool {
    let mut nlines: std::ffi::c_int = 0;
    let mut pos: POSITION = 0 as std::ffi::c_int as POSITION;
    let mut ret: lbool = LFALSE;
    getting_one_screen = LTRUE;
    nlines = 0 as std::ffi::c_int;
    while nlines + shell_lines <= sc_height {
        pos = forw_line(pos, 0 as *mut POSITION, 0 as *mut lbool);
        if sigs
            & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
            != 0
        {
            break;
        }
        if pos == -(1 as std::ffi::c_int) as POSITION {
            ret = LTRUE;
            break;
        } else {
            nlines += 1;
        }
    }
    getting_one_screen = LFALSE;
    return ret;
}
