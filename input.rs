use ::libc;
extern "C" {
    fn ch_seek(pos: POSITION) -> std::ffi::c_int;
    fn ch_tell() -> POSITION;
    fn ch_forw_get() -> std::ffi::c_int;
    fn ch_back_get() -> std::ffi::c_int;
    fn line_position() -> POSITION;
    fn is_line_contig_pos(pos: POSITION) -> lbool;
    fn set_line_contig_pos(pos: POSITION);
    fn prewind(contig: lbool);
    fn plinestart(pos: POSITION);
    fn pshift_all();
    fn savec();
    fn loadc();
    fn pappend_b(
        c: std::ffi::c_char,
        pos: POSITION,
        before_pendc: lbool,
    ) -> std::ffi::c_int;
    fn pappend(c: std::ffi::c_char, pos: POSITION) -> std::ffi::c_int;
    fn pflushmbc() -> std::ffi::c_int;
    fn pdone(endline: lbool, chopped: lbool, forw: lbool);
    fn set_status_col(c: std::ffi::c_char, attr: std::ffi::c_int);
    fn null_line();
    fn chop_line() -> std::ffi::c_int;
    fn is_filtered(pos: POSITION) -> lbool;
    fn is_hilited_attr(
        pos: POSITION,
        epos: POSITION,
        nohide: std::ffi::c_int,
        p_matches: *mut std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn prep_hilite(spos: POSITION, epos: POSITION, maxlines: std::ffi::c_int);
    fn is_filtering() -> lbool;
    static mut squeeze: std::ffi::c_int;
    static mut hshift: std::ffi::c_int;
    static mut quit_if_one_screen: std::ffi::c_int;
    static mut status_col: std::ffi::c_int;
    static mut wordwrap: std::ffi::c_int;
    static mut start_attnpos: POSITION;
    static mut end_attnpos: POSITION;
    static mut hilite_search: std::ffi::c_int;
    static mut show_attn: std::ffi::c_int;
}
pub type __off_t = std::ffi::c_long;
pub type off_t = __off_t;
pub type lbool = std::ffi::c_uint;
pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;
pub type less_off_t = off_t;
pub type POSITION = less_off_t;
unsafe extern "C" fn init_status_col(
    mut base_pos: POSITION,
    mut disp_pos: POSITION,
    mut edisp_pos: POSITION,
    mut eol_pos: POSITION,
) {
    let mut hl_before: std::ffi::c_int = if chop_line() != 0
        && disp_pos != -(1 as std::ffi::c_int) as POSITION
    {
        is_hilited_attr(
            base_pos,
            disp_pos,
            LTRUE as std::ffi::c_int,
            0 as *mut std::ffi::c_int,
        )
    } else {
        0 as std::ffi::c_int
    };
    let mut hl_after: std::ffi::c_int = if chop_line() != 0
        && edisp_pos != -(1 as std::ffi::c_int) as POSITION
    {
        is_hilited_attr(
            edisp_pos,
            eol_pos,
            LTRUE as std::ffi::c_int,
            0 as *mut std::ffi::c_int,
        )
    } else {
        0 as std::ffi::c_int
    };
    let mut attr: std::ffi::c_int = 0;
    let mut ch: std::ffi::c_char = 0;
    if hl_before != 0 && hl_after != 0 {
        attr = hl_after;
        ch = '=' as i32 as std::ffi::c_char;
    } else if hl_before != 0 {
        attr = hl_before;
        ch = '<' as i32 as std::ffi::c_char;
    } else if hl_after != 0 {
        attr = hl_after;
        ch = '>' as i32 as std::ffi::c_char;
    } else if disp_pos != -(1 as std::ffi::c_int) as POSITION {
        attr = is_hilited_attr(
            disp_pos,
            edisp_pos,
            LTRUE as std::ffi::c_int,
            0 as *mut std::ffi::c_int,
        );
        ch = '*' as i32 as std::ffi::c_char;
    } else {
        attr = 0 as std::ffi::c_int;
    }
    if attr != 0 {
        set_status_col(ch, attr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn forw_line_seg(
    mut curr_pos: POSITION,
    mut skipeol: lbool,
    mut rscroll: lbool,
    mut nochop: lbool,
    mut p_linepos: *mut POSITION,
    mut p_newline: *mut lbool,
) -> POSITION {
    let mut base_pos: POSITION = 0;
    let mut new_pos: POSITION = 0;
    let mut edisp_pos: POSITION = 0;
    let mut c: std::ffi::c_int = 0;
    let mut blankline: lbool = LFALSE;
    let mut endline: lbool = LFALSE;
    let mut chopped: lbool = LFALSE;
    let mut backchars: std::ffi::c_int = 0;
    let mut wrap_pos: POSITION = 0;
    let mut skipped_leading: lbool = LFALSE;
    if !p_linepos.is_null() {
        *p_linepos = -(1 as std::ffi::c_int) as POSITION;
    }
    loop {
        if curr_pos == -(1 as std::ffi::c_int) as POSITION {
            null_line();
            return -(1 as std::ffi::c_int) as POSITION;
        }
        if hilite_search == 2 as std::ffi::c_int
            || is_filtering() as std::ffi::c_uint != 0
            || status_col != 0 && hilite_search != 1 as std::ffi::c_int
        {
            prep_hilite(
                curr_pos,
                -(1 as std::ffi::c_int) as POSITION,
                1 as std::ffi::c_int,
            );
        }
        if ch_seek(curr_pos) != 0 {
            null_line();
            return -(1 as std::ffi::c_int) as POSITION;
        }
        base_pos = curr_pos;
        loop {
            c = ch_back_get();
            if c == -(1 as std::ffi::c_int) {
                break;
            }
            if c == '\n' as i32 {
                ch_forw_get();
                break;
            } else {
                base_pos -= 1;
                base_pos;
            }
        }
        if is_line_contig_pos(curr_pos) as u64 != 0 {
            prewind(LTRUE);
            plinestart(base_pos);
            ch_seek(curr_pos);
            new_pos = curr_pos;
        } else {
            prewind(LFALSE);
            plinestart(base_pos);
            ch_seek(base_pos);
            new_pos = base_pos;
            while new_pos < curr_pos {
                c = ch_forw_get();
                if c == -(1 as std::ffi::c_int) {
                    null_line();
                    return -(1 as std::ffi::c_int) as POSITION;
                }
                backchars = pappend(c as std::ffi::c_char, new_pos);
                new_pos += 1;
                new_pos;
                if backchars > 0 as std::ffi::c_int {
                    pshift_all();
                    if wordwrap != 0 && (c == ' ' as i32 || c == '\t' as i32) {
                        loop {
                            new_pos += 1;
                            new_pos;
                            c = ch_forw_get();
                            if !(c == ' ' as i32 || c == '\t' as i32) {
                                break;
                            }
                        }
                        backchars = 1 as std::ffi::c_int;
                    }
                    new_pos -= backchars as POSITION;
                    loop {
                        backchars -= 1;
                        if !(backchars >= 0 as std::ffi::c_int) {
                            break;
                        }
                        ch_back_get();
                    }
                }
            }
            pshift_all();
        }
        pflushmbc();
        c = ch_forw_get();
        if c == -(1 as std::ffi::c_int) {
            null_line();
            return -(1 as std::ffi::c_int) as POSITION;
        }
        blankline = (c == '\n' as i32 || c == '\r' as i32) as std::ffi::c_int as lbool;
        wrap_pos = -(1 as std::ffi::c_int) as POSITION;
        skipped_leading = LFALSE;
        chopped = LFALSE;
        loop {
            if c == '\n' as i32 || c == -(1 as std::ffi::c_int) {
                backchars = pflushmbc();
                new_pos = ch_tell();
                if backchars > 0 as std::ffi::c_int
                    && (nochop as std::ffi::c_uint != 0 || chop_line() == 0)
                    && hshift == 0 as std::ffi::c_int
                {
                    new_pos -= (backchars + 1 as std::ffi::c_int) as POSITION;
                    endline = LFALSE;
                } else {
                    endline = LTRUE;
                }
                edisp_pos = new_pos;
                break;
            } else {
                if c != '\r' as i32 {
                    blankline = LFALSE;
                }
                backchars = pappend(
                    c as std::ffi::c_char,
                    ch_tell() - 1 as std::ffi::c_int as POSITION,
                );
                if backchars > 0 as std::ffi::c_int {
                    if skipeol as u64 != 0 {
                        edisp_pos = ch_tell() - backchars as POSITION;
                        loop {
                            c = ch_forw_get();
                            if !(c != '\n' as i32 && c != -(1 as std::ffi::c_int)) {
                                break;
                            }
                        }
                        new_pos = ch_tell();
                        endline = LTRUE;
                        quit_if_one_screen = LFALSE as std::ffi::c_int;
                        chopped = LTRUE;
                    } else {
                        if wordwrap == 0 {
                            new_pos = ch_tell() - backchars as POSITION;
                        } else if c == ' ' as i32 || c == '\t' as i32 {
                            loop {
                                new_pos = ch_tell();
                                c = ch_forw_get();
                                if !(c == ' ' as i32 || c == '\t' as i32) {
                                    break;
                                }
                            }
                            if c == '\r' as i32 {
                                c = ch_forw_get();
                            }
                            if c == '\n' as i32 {
                                new_pos = ch_tell();
                            }
                        } else if wrap_pos == -(1 as std::ffi::c_int) as POSITION {
                            new_pos = ch_tell() - backchars as POSITION;
                        } else {
                            new_pos = wrap_pos;
                            loadc();
                        }
                        endline = LFALSE;
                        edisp_pos = new_pos;
                    }
                    break;
                } else {
                    if wordwrap != 0 {
                        if c == ' ' as i32 || c == '\t' as i32 {
                            if skipped_leading as u64 != 0 {
                                wrap_pos = ch_tell();
                                savec();
                            }
                        } else {
                            skipped_leading = LTRUE;
                        }
                    }
                    c = ch_forw_get();
                }
            }
        }
        if blankline as std::ffi::c_uint != 0 && show_attn != 0 {
            pappend_b(
                ' ' as i32 as std::ffi::c_char,
                ch_tell() - 1 as std::ffi::c_int as POSITION,
                LTRUE,
            );
        }
        pdone(
            endline,
            (rscroll as std::ffi::c_uint != 0 && chopped as std::ffi::c_uint != 0)
                as std::ffi::c_int as lbool,
            LTRUE,
        );
        if !(is_filtered(base_pos) as u64 != 0) {
            break;
        }
        curr_pos = new_pos;
    }
    if status_col != 0 {
        init_status_col(base_pos, line_position(), edisp_pos, new_pos);
    }
    if squeeze != 0 && blankline as std::ffi::c_uint != 0 {
        loop {
            c = ch_forw_get();
            if !(c == '\n' as i32 || c == '\r' as i32) {
                break;
            }
        }
        if c != -(1 as std::ffi::c_int) {
            ch_back_get();
        }
        new_pos = ch_tell();
    }
    if !p_linepos.is_null() {
        *p_linepos = curr_pos;
    }
    if !p_newline.is_null() {
        *p_newline = endline;
    }
    set_line_contig_pos(new_pos);
    return new_pos;
}
#[no_mangle]
pub unsafe extern "C" fn forw_line(
    mut curr_pos: POSITION,
    mut p_linepos: *mut POSITION,
    mut p_newline: *mut lbool,
) -> POSITION {
    return forw_line_seg(
        curr_pos,
        (chop_line() != 0 || hshift > 0 as std::ffi::c_int) as std::ffi::c_int as lbool,
        LTRUE,
        LFALSE,
        p_linepos,
        p_newline,
    );
}
#[no_mangle]
pub unsafe extern "C" fn back_line(
    mut curr_pos: POSITION,
    mut p_newline: *mut lbool,
) -> POSITION {
    let mut base_pos: POSITION = 0;
    let mut new_pos: POSITION = 0;
    let mut edisp_pos: POSITION = 0;
    let mut begin_new_pos: POSITION = 0;
    let mut c: std::ffi::c_int = 0;
    let mut endline: lbool = LFALSE;
    let mut chopped: lbool = LFALSE;
    let mut backchars: std::ffi::c_int = 0;
    let mut wrap_pos: POSITION = 0;
    let mut skipped_leading: lbool = LFALSE;
    loop {
        if curr_pos == -(1 as std::ffi::c_int) as POSITION
            || curr_pos <= 0 as std::ffi::c_int as POSITION
        {
            null_line();
            return -(1 as std::ffi::c_int) as POSITION;
        }
        if ch_seek(curr_pos - 1 as std::ffi::c_int as POSITION) != 0 {
            null_line();
            return -(1 as std::ffi::c_int) as POSITION;
        }
        if squeeze != 0 {
            ch_forw_get();
            c = ch_forw_get();
            ch_back_get();
            ch_back_get();
            if c == '\n' as i32 || c == '\r' as i32 {
                loop {
                    c = ch_back_get();
                    if !(c == '\n' as i32 || c == '\r' as i32) {
                        break;
                    }
                }
                if c == -(1 as std::ffi::c_int) {
                    null_line();
                    return -(1 as std::ffi::c_int) as POSITION;
                }
                ch_forw_get();
            }
        }
        loop {
            c = ch_back_get();
            if c == '\n' as i32 {
                base_pos = ch_tell() + 1 as std::ffi::c_int as POSITION;
                break;
            } else {
                if !(c == -(1 as std::ffi::c_int)) {
                    continue;
                }
                base_pos = ch_tell();
                break;
            }
        }
        if hilite_search == 2 as std::ffi::c_int
            || is_filtering() as std::ffi::c_uint != 0
            || status_col != 0 && hilite_search != 1 as std::ffi::c_int
        {
            prep_hilite(
                base_pos,
                -(1 as std::ffi::c_int) as POSITION,
                1 as std::ffi::c_int,
            );
        }
        new_pos = base_pos;
        if ch_seek(new_pos) != 0 {
            null_line();
            return -(1 as std::ffi::c_int) as POSITION;
        }
        endline = LFALSE;
        prewind(LFALSE);
        plinestart(new_pos);
        if !p_newline.is_null() {
            *p_newline = LTRUE;
        }
        '_loop: loop {
            wrap_pos = -(1 as std::ffi::c_int) as POSITION;
            skipped_leading = LFALSE;
            begin_new_pos = new_pos;
            ch_seek(new_pos);
            chopped = LFALSE;
            loop {
                c = ch_forw_get();
                if c == -(1 as std::ffi::c_int) {
                    null_line();
                    return -(1 as std::ffi::c_int) as POSITION;
                }
                new_pos += 1;
                new_pos;
                if c == '\n' as i32 {
                    backchars = pflushmbc();
                    if backchars > 0 as std::ffi::c_int && chop_line() == 0
                        && hshift == 0 as std::ffi::c_int
                    {
                        backchars += 1;
                        backchars;
                    } else {
                        endline = LTRUE;
                        edisp_pos = new_pos;
                        break '_loop;
                    }
                } else {
                    backchars = pappend(
                        c as std::ffi::c_char,
                        ch_tell() - 1 as std::ffi::c_int as POSITION,
                    );
                    if backchars > 0 as std::ffi::c_int {
                        if chop_line() != 0 || hshift > 0 as std::ffi::c_int {
                            endline = LTRUE;
                            chopped = LTRUE;
                            quit_if_one_screen = LFALSE as std::ffi::c_int;
                            edisp_pos = new_pos;
                            break '_loop;
                        } else if !p_newline.is_null() {
                            *p_newline = LFALSE;
                        }
                    } else {
                        if wordwrap != 0 {
                            if c == ' ' as i32 || c == '\t' as i32 {
                                if skipped_leading as u64 != 0 {
                                    wrap_pos = new_pos;
                                }
                            } else {
                                skipped_leading = LTRUE;
                            }
                        }
                        if !(new_pos >= curr_pos) {
                            continue;
                        }
                        edisp_pos = new_pos;
                        break '_loop;
                    }
                }
                if wordwrap == 0 {
                    pshift_all();
                    new_pos -= backchars as POSITION;
                    break;
                } else if c == ' ' as i32 || c == '\t' as i32 {
                    loop {
                        c = ch_forw_get();
                        if c == ' ' as i32 || c == '\t' as i32 {
                            new_pos += 1;
                            new_pos;
                        } else {
                            if c == '\r' as i32 {
                                c = ch_forw_get();
                                if c == '\n' as i32 {
                                    new_pos += 1;
                                    new_pos;
                                }
                            }
                            if c == '\n' as i32 {
                                new_pos += 1;
                                new_pos;
                            }
                            edisp_pos = new_pos;
                            break;
                        }
                    }
                    if new_pos >= curr_pos {
                        edisp_pos = new_pos;
                        break '_loop;
                    } else {
                        pshift_all();
                        break;
                    }
                } else {
                    pshift_all();
                    if wrap_pos == -(1 as std::ffi::c_int) as POSITION {
                        new_pos -= backchars as POSITION;
                    } else {
                        new_pos = wrap_pos;
                    }
                    break;
                }
            }
        }
        pdone(endline, chopped, LFALSE);
        if !(is_filtered(base_pos) as u64 != 0) {
            break;
        }
        curr_pos = begin_new_pos;
    }
    if status_col != 0 {
        init_status_col(base_pos, line_position(), edisp_pos, new_pos);
    }
    return begin_new_pos;
}
#[no_mangle]
pub unsafe extern "C" fn set_attnpos(mut pos: POSITION) {
    let mut c: std::ffi::c_int = 0;
    if pos != -(1 as std::ffi::c_int) as POSITION {
        if ch_seek(pos) != 0 {
            return;
        }
        loop {
            c = ch_forw_get();
            if c == -(1 as std::ffi::c_int) {
                break;
            }
            if c == '\n' as i32 || c == '\r' as i32 {
                ch_back_get();
                break;
            } else {
                pos += 1;
                pos;
            }
        }
        end_attnpos = pos;
        loop {
            c = ch_back_get();
            if c == -(1 as std::ffi::c_int) || c == '\n' as i32 || c == '\r' as i32 {
                break;
            }
            pos -= 1;
            pos;
        }
    }
    start_attnpos = pos;
}
