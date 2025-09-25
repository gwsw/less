use crate::defs::*;
use crate::line::{pappend, pappend_b, pdone, prewind};

extern "C" {
    fn ch_seek(pos: POSITION) -> std::ffi::c_int;
    fn ch_tell() -> POSITION;
    fn ch_forw_get() -> std::ffi::c_int;
    fn ch_back_get() -> std::ffi::c_int;
    fn line_position() -> POSITION;
    fn is_line_contig_pos(pos: POSITION) -> lbool;
    fn set_line_contig_pos(pos: POSITION);
    fn plinestart(pos: POSITION);
    fn pshift_all();
    fn savec();
    fn loadc();
    fn pflushmbc() -> std::ffi::c_int;
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
    static mut quit_if_one_screen: bool;
    static mut status_col: std::ffi::c_int;
    static mut wordwrap: std::ffi::c_int;
    static mut start_attnpos: POSITION;
    static mut end_attnpos: POSITION;
    static mut hilite_search: std::ffi::c_int;
    static mut show_attn: std::ffi::c_int;
}

/*
 * High level routines dealing with getting lines of input
 * from the file being viewed.
 *
 * When we speak of "lines" here, we mean PRINTABLE lines;
 * lines processed with respect to the screen width.
 * We use the term "raw line" to refer to lines simply
 * delimited by newlines; not processed with respect to screen width.
 */

/*
 * Set the status column.
 *  base  Position of first char in line.
 *  disp  First visible char.
 *        Different than base_pos if line is shifted.
 *  edisp Last visible char.
 *  eol   End of line. Normally the newline.
 *        Different than edisp if line is chopped.
 */
unsafe extern "C" fn init_status_col(
    base_pos: POSITION,
    disp_pos: POSITION,
    edisp_pos: POSITION,
    eol_pos: POSITION,
) {
    let mut hl_before = if chop_line() != 0 && disp_pos != NULL_POSITION {
        is_hilited_attr(
            base_pos,
            disp_pos,
            LTRUE as std::ffi::c_int,
            0 as *mut std::ffi::c_int,
        )
    } else {
        0
    };
    let mut hl_after = if chop_line() != 0 && edisp_pos != NULL_POSITION {
        is_hilited_attr(
            edisp_pos,
            eol_pos,
            LTRUE as std::ffi::c_int,
            0 as *mut std::ffi::c_int,
        )
    } else {
        0
    };
    let mut attr = 0;
    let mut ch = b'\0';
    if hl_before != 0 && hl_after != 0 {
        attr = hl_after;
        ch = b'=';
    } else if hl_before != 0 {
        attr = hl_before;
        ch = b'<';
    } else if hl_after != 0 {
        attr = hl_after;
        ch = b'>';
    } else if disp_pos != NULL_POSITION {
        attr = is_hilited_attr(
            disp_pos,
            edisp_pos,
            LTRUE as std::ffi::c_int,
            0 as *mut std::ffi::c_int,
        );
        ch = b'*';
    } else {
        attr = 0;
    }
    if attr != 0 {
        set_status_col(ch as i8, attr);
    }
}

/*
 * Get the next line.
 * A "current" position is passed and a "new" position is returned.
 * The current position is the position of the first character of
 * a line.  The new position is the position of the first character
 * of the NEXT line.  The line obtained is the line starting at curr_pos.
 */
#[no_mangle]
pub unsafe extern "C" fn forw_line_seg(
    curr_pos: POSITION,
    skipeol: bool,
    rscroll: bool,
    nochop: bool,
    p_linepos: &mut Option<POSITION>,
    p_newline: &mut Option<bool>,
) -> POSITION {
    let mut base_pos: POSITION = 0;
    let mut new_pos: POSITION = 0;
    let mut edisp_pos: POSITION = 0;
    let mut c: i32 = 0;
    let mut blankline: bool = false;
    let mut endline = false;
    let mut chopped = false;
    let mut backchars = 0;
    let mut wrap_pos: POSITION = 0;
    let mut skipped_leading = false;
    let mut curr_pos = curr_pos;

    if !p_linepos.is_none() {
        *p_linepos = Some(NULL_POSITION);
    }
    loop {
        if curr_pos == NULL_POSITION {
            null_line();
            return NULL_POSITION;
        }
        if hilite_search == OPT_ONPLUS
            || is_filtering() != 0
            || status_col != 0 && hilite_search != 1
        {
            /*
             * If we are ignoring EOI (command F), only prepare
             * one line ahead, to avoid getting stuck waiting for
             * slow data without displaying the data we already have.
             * If we're not ignoring EOI, we *could* do the same, but
             * for efficiency we prepare several lines ahead at once.
             */
            prep_hilite(curr_pos, NULL_POSITION, 1);
        }
        if ch_seek(curr_pos) != 0 {
            null_line();
            return NULL_POSITION;
        }

        /*
         * Step back to the beginning of the line.
         */
        base_pos = curr_pos;
        loop {
            c = ch_back_get();
            if c == EOI {
                break;
            }
            if c == b'\n' as i32 {
                ch_forw_get();
                break;
            } else {
                base_pos -= 1;
            }
        }

        /*
         * Read forward again to the position we should start at.
         */
        if is_line_contig_pos(curr_pos) != 0 {
            prewind(true);
            plinestart(base_pos);
            ch_seek(curr_pos);
            new_pos = curr_pos;
        } else {
            prewind(false);
            plinestart(base_pos);
            ch_seek(base_pos);
            new_pos = base_pos;
            while new_pos < curr_pos {
                c = ch_forw_get();
                if c == EOI {
                    null_line();
                    return -(1 as std::ffi::c_int) as POSITION;
                }
                backchars = pappend(c as u8, new_pos as i64);
                new_pos += 1;
                if backchars > 0 {
                    pshift_all();
                    if wordwrap != 0 && (c == b' ' as i32 || c == b'\t' as i32) {
                        loop {
                            new_pos += 1;
                            c = ch_forw_get();
                            if !(c == b' ' as i32 || c == b'\t' as i32) {
                                break;
                            }
                        }
                        backchars = 1;
                    }
                    new_pos -= backchars;
                    loop {
                        backchars -= 1;
                        if !(backchars >= 0) {
                            break;
                        }
                        ch_back_get();
                    }
                }
            }
            pshift_all();
        }
        pflushmbc();

        /*
         * Read the first character to display.
         */
        c = ch_forw_get();
        if c == EOI {
            null_line();
            return NULL_POSITION;
        }
        blankline = c == b'\n' as i32 || c == b'\r' as i32;
        wrap_pos = NULL_POSITION;
        skipped_leading = false;
        chopped = false;
        loop {
            if c == b'\n' as i32 || c == EOI {
                backchars = pflushmbc() as i64;
                new_pos = ch_tell();
                if backchars > 0 && (nochop || chop_line() == 0) && hshift == 0 {
                    new_pos -= (backchars + 1) as POSITION;
                    endline = false;
                } else {
                    endline = true;
                }
                edisp_pos = new_pos;
                break;
            } else {
                if c != b'\r' as i32 {
                    blankline = false;
                }

                /*
                 * Append the char to the line and get the next char.
                 */
                backchars = pappend(c as u8, (ch_tell() - 1) as i64);
                if backchars > 0 {
                    /*
                     * The char won't fit in the line; the line
                     * is too long to print in the screen width.
                     * End the line here.
                     */
                    if skipeol {
                        /* Read to end of line. */
                        edisp_pos = ch_tell() - backchars as POSITION;
                        loop {
                            c = ch_forw_get();
                            if !(c != b'\n' as i32 && c != EOI) {
                                break;
                            }
                        }
                        new_pos = ch_tell();
                        endline = true;
                        quit_if_one_screen = false;
                        chopped = false;
                    } else {
                        if wordwrap == 0 {
                            new_pos = ch_tell() - backchars as POSITION;
                        } else if c == b' ' as i32 || c == b'\t' as i32 {
                            /*
                             * We're word-wrapping, so go back to the last space.
                             * However, if it's the space itself that couldn't fit,
                             * simply ignore it and any subsequent spaces.
                             */
                            loop {
                                new_pos = ch_tell();
                                c = ch_forw_get();
                                if !(c == b' ' as i32 || c == b'\t' as i32) {
                                    break;
                                }
                            }
                            if c == b'\r' as i32 {
                                c = ch_forw_get();
                            }
                            if c == b'\n' as i32 {
                                new_pos = ch_tell();
                            }
                        } else if wrap_pos == NULL_POSITION {
                            new_pos = ch_tell() - backchars as POSITION;
                        } else {
                            new_pos = wrap_pos;
                            loadc();
                        }
                        endline = false;
                        edisp_pos = new_pos;
                    }
                    break;
                } else {
                    if wordwrap != 0 {
                        if c == b' ' as i32 || c == b'\t' as i32 {
                            if skipped_leading {
                                wrap_pos = ch_tell();
                                savec();
                            }
                        } else {
                            skipped_leading = true;
                        }
                    }
                    c = ch_forw_get();
                }
            }
        }
        if blankline && show_attn != 0 {
            /* Add spurious space to carry possible attn hilite.
             * Use pappend_b so that if line ended with \r\n,
             * we insert the space before the \r. */
            pappend_b(b' ', (ch_tell() - 1) as POSITION, true);
        }
        pdone(endline, rscroll && chopped, true);
        if !(is_filtered(base_pos) as u64 != 0) {
            /*
             * We don't want to display this line.
             * Get the next line.
             */
            break;
        }
        curr_pos = new_pos;
    }
    if status_col != 0 {
        init_status_col(base_pos, line_position(), edisp_pos, new_pos);
    }
    if squeeze != 0 && blankline {
        loop {
            c = ch_forw_get();
            if !(c == b'\n' as i32 || c == b'\r' as i32) {
                break;
            }
        }
        if c != EOI {
            ch_back_get();
        }
        new_pos = ch_tell();
    }
    if !p_linepos.is_none() {
        *p_linepos = Some(curr_pos);
    }
    if !p_newline.is_none() {
        *p_newline = Some(endline);
    }
    set_line_contig_pos(new_pos);
    new_pos
}

#[no_mangle]
pub unsafe extern "C" fn forw_line(
    curr_pos: POSITION,
    p_linepos: &mut Option<POSITION>,
    p_newline: &mut Option<bool>,
) -> POSITION {
    return forw_line_seg(
        curr_pos,
        chop_line() != 0 || hshift > 0,
        true,
        false,
        p_linepos,
        p_newline,
    );
}

/*
 * Get the previous line.
 * A "current" position is passed and a "new" position is returned.
 * The current position is the position of the first character of
 * a line.  The new position is the position of the first character
 * of the PREVIOUS line.  The line obtained is the one starting at new_pos.
 */
#[no_mangle]
pub unsafe extern "C" fn back_line(
    mut curr_pos: POSITION,
    p_newline: &mut Option<bool>,
) -> POSITION {
    let mut base_pos: POSITION = 0;
    let mut new_pos: POSITION = 0;
    let mut edisp_pos: POSITION = 0;
    let mut begin_new_pos: POSITION = 0;
    let mut c = 0;
    let mut endline: bool = false;
    let mut chopped: bool = false;
    let mut backchars = 0;
    let mut wrap_pos: POSITION = 0;
    let mut skipped_leading: bool = false;
    loop {
        if curr_pos == NULL_POSITION || curr_pos <= 0 {
            null_line();
            return NULL_POSITION;
        }
        if ch_seek(curr_pos - 1) != 0 {
            null_line();
            return NULL_POSITION;
        }
        if squeeze != 0 {
            /*
             * Find out if the "current" line was blank.
             */
            ch_forw_get(); /* Skip the newline */
            c = ch_forw_get(); /* First char of the current line */
            /* {{ what if c == EOI? }} */
            ch_back_get(); /* Restore our position */
            ch_back_get();
            if c == '\n' as i32 || c == '\r' as i32 {
                /*
                 * The "current" line was blank.
                 * Skip over any preceding blank lines,
                 * since we skipped them in forw_line().
                 */
                loop {
                    c = ch_back_get();
                    if !(c == '\n' as i32 || c == '\r' as i32) {
                        break;
                    }
                }
                if c == EOI {
                    null_line();
                    return NULL_POSITION;
                }
                ch_forw_get();
            }
        }

        /*
         * Scan backwards until we hit the beginning of the line.
         */
        loop {
            c = ch_back_get();
            if c == '\n' as i32 {
                /*
                 * This is the newline ending the previous line.
                 * We have hit the beginning of the line.
                 */
                base_pos = ch_tell() + 1;
                break;
            } else {
                if !(c == EOI) {
                    continue;
                }
                /*
                 * We have hit the beginning of the file.
                 * This must be the first line in the file.
                 * This must, of course, be the beginning of the line.
                 */
                base_pos = ch_tell();
                break;
            }
        }
        if hilite_search == OPT_ONPLUS
            || is_filtering() != 0
            || status_col != 0 && hilite_search != 1
        {
            prep_hilite(base_pos, NULL_POSITION, 1);
        }

        /*
         * Now scan forwards from the beginning of this line.
         * We keep discarding "printable lines" (based on screen width)
         * until we reach the curr_pos.
         *
         * {{ This algorithm is pretty inefficient if the lines
         *    are much longer than the screen width,
         *    but I don't know of any better way. }}
         */
        new_pos = base_pos;
        if ch_seek(new_pos) != 0 {
            null_line();
            return NULL_POSITION;
        }
        endline = false;
        prewind(false);
        plinestart(new_pos);
        if !p_newline.is_none() {
            *p_newline = Some(true);
        }
        '_loop: loop {
            wrap_pos = NULL_POSITION;
            skipped_leading = false;
            begin_new_pos = new_pos;
            ch_seek(new_pos);
            chopped = false;
            loop {
                c = ch_forw_get();
                if c == EOI {
                    null_line();
                    return NULL_POSITION;
                }
                new_pos += 1;
                if c == '\n' as i32 {
                    backchars = pflushmbc();
                    if backchars > 0 && chop_line() == 0 && hshift == 0 {
                        backchars += 1;
                    } else {
                        endline = true;
                        edisp_pos = new_pos;
                        break '_loop;
                    }
                } else {
                    backchars = pappend(c as u8, (ch_tell() - 1) as i64) as i32;
                    if backchars > 0 {
                        /*
                         * Got a full printable line, but we haven't
                         * reached our curr_pos yet.  Discard the line
                         * and start a new one.
                         */
                        if chop_line() != 0 || hshift > 0 {
                            endline = true;
                            chopped = true;
                            quit_if_one_screen = false;
                            edisp_pos = new_pos;
                            break '_loop;
                        } else if !p_newline.is_none() {
                            *p_newline = Some(false);
                        }
                    } else {
                        if wordwrap != 0 {
                            if c == ' ' as i32 || c == '\t' as i32 {
                                if skipped_leading {
                                    wrap_pos = new_pos;
                                }
                            } else {
                                skipped_leading = true;
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
                        } else {
                            if c == '\r' as i32 {
                                c = ch_forw_get();
                                if c == '\n' as i32 {
                                    new_pos += 1;
                                }
                            }
                            if c == '\n' as i32 {
                                new_pos += 1;
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
                    if wrap_pos == NULL_POSITION {
                        new_pos -= backchars as POSITION;
                    } else {
                        new_pos = wrap_pos;
                    }
                    break;
                }
            }
        }
        pdone(endline, chopped, false);
        if !(is_filtered(base_pos) as u64 != 0) {
            break;
        }
        curr_pos = begin_new_pos;
    }
    if status_col != 0 {
        init_status_col(base_pos, line_position(), edisp_pos, new_pos);
    }
    begin_new_pos
}

/*
 * Set attnpos.
 */
#[no_mangle]
pub unsafe extern "C" fn set_attnpos(mut pos: POSITION) {
    let mut c = 0;
    if pos != NULL_POSITION {
        if ch_seek(pos) != 0 {
            return;
        }
        loop {
            c = ch_forw_get();
            if c == EOI {
                break;
            }
            if c == '\n' as i32 || c == '\r' as i32 {
                ch_back_get();
                break;
            } else {
                pos += 1;
            }
        }
        end_attnpos = pos;
        loop {
            c = ch_back_get();
            if c == EOI || c == '\n' as i32 || c == '\r' as i32 {
                break;
            }
            pos -= 1;
        }
    }
    start_attnpos = pos;
}
