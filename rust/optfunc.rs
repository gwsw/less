use ::libc;
extern "C" {
    fn sprintf(_: *mut std::ffi::c_char, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn snprintf(
        _: *mut std::ffi::c_char,
        _: std::ffi::c_ulong,
        _: *const std::ffi::c_char,
        _: ...
    ) -> std::ffi::c_int;
    fn free(_: *mut std::ffi::c_void);
    fn strcpy(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strcat(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn save(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn skipspc(s: *const std::ffi::c_char) -> *const std::ffi::c_char;
    fn quit(status: std::ffi::c_int);
    fn secure_allow(features: std::ffi::c_int) -> std::ffi::c_int;
    fn init_mouse();
    fn deinit_mouse();
    fn init_bracketed_paste();
    fn deinit_bracketed_paste();
    fn sync_logfile();
    fn ch_length() -> POSITION;
    fn ch_setbufspace(bufspace_0: ssize_t);
    fn ch_getflags() -> std::ffi::c_int;
    fn setfmt(
        s: *const std::ffi::c_char,
        fmtvarptr: *mut *const std::ffi::c_char,
        attrptr: *mut std::ffi::c_int,
        default_fmt: *const std::ffi::c_char,
        for_printf: lbool,
    );
    fn prchar(c: LWCHAR) -> *const std::ffi::c_char;
    fn step_charc(
        pp: *mut *const std::ffi::c_char,
        dir: std::ffi::c_int,
        limit: *const std::ffi::c_char,
    ) -> LWCHAR;
    fn norm_search_type(st: std::ffi::c_int) -> std::ffi::c_int;
    fn dispversion();
    fn ungetcc_end_command();
    fn ungetsc(s: *const std::ffi::c_char);
    fn lesskey(filename: *const std::ffi::c_char, sysvar: lbool) -> std::ffi::c_int;
    fn lesskey_src(filename: *const std::ffi::c_char, sysvar: lbool) -> std::ffi::c_int;
    fn lesskey_content(content: *const std::ffi::c_char, sysvar: lbool) -> std::ffi::c_int;
    fn save_curr_ifile() -> *mut std::ffi::c_void;
    fn unsave_ifile(save_ifile: *mut std::ffi::c_void);
    fn reedit_ifile(save_ifile: *mut std::ffi::c_void);
    fn use_logfile(filename: *const std::ffi::c_char);
    fn shell_unquote(str: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn lglob(afilename: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn jump_loc(pos: POSITION, sline: std::ffi::c_int);
    fn pwidth(
        ch: LWCHAR,
        a: std::ffi::c_int,
        prev_ch: LWCHAR,
        prev_a: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn set_color_map(attr: std::ffi::c_int, colorstr: *const std::ffi::c_char) -> std::ffi::c_int;
    fn find_linenum(pos: POSITION) -> LINENUM;
    fn find_pos(linenum: LINENUM) -> POSITION;
    fn scan_eof();
    fn getnumc(
        sp: *mut *const std::ffi::c_char,
        printopt: *const std::ffi::c_char,
        errp: *mut lbool,
    ) -> std::ffi::c_int;
    fn getfraction(
        sp: *mut *const std::ffi::c_char,
        printopt: *const std::ffi::c_char,
        errp: *mut lbool,
    ) -> std::ffi::c_long;
    fn umuldiv(val: uintmax, num: uintmax, den: uintmax) -> uintmax;
    fn set_output(fd: std::ffi::c_int);
    fn putstr(s: *const std::ffi::c_char);
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn pattern_lib_name() -> *const std::ffi::c_char;
    fn position(sindex: std::ffi::c_int) -> POSITION;
    fn pos_rehead();
    fn set_header(pos: POSITION);
    fn chg_caseless();
    fn findtag(tag: *const std::ffi::c_char);
    fn tagsearch() -> POSITION;
    fn edit_tagfile() -> std::ffi::c_int;
    fn default_wheel_lines() -> std::ffi::c_int;
    static mut bufspace: std::ffi::c_int;
    static mut pr_type: std::ffi::c_int;
    static mut plusoption: lbool;
    static mut swindow: std::ffi::c_int;
    static mut sc_width: std::ffi::c_int;
    static mut sc_height: std::ffi::c_int;
    static mut dohelp: std::ffi::c_int;
    static mut openquote: std::ffi::c_char;
    static mut closequote: std::ffi::c_char;
    static mut prproto: [*mut std::ffi::c_char; 0];
    static mut eqproto: *mut std::ffi::c_char;
    static mut hproto: *mut std::ffi::c_char;
    static mut wproto: *mut std::ffi::c_char;
    static mut every_first_cmd: *mut std::ffi::c_char;
    static mut curr_ifile: *mut std::ffi::c_void;
    static mut version: [std::ffi::c_char; 0];
    static mut jump_sline: std::ffi::c_int;
    static mut jump_sline_fraction: std::ffi::c_long;
    static mut shift_count: std::ffi::c_int;
    static mut shift_count_fraction: std::ffi::c_long;
    static mut match_shift: std::ffi::c_int;
    static mut match_shift_fraction: std::ffi::c_long;
    static mut rscroll_char: LWCHAR;
    static mut rscroll_attr: std::ffi::c_int;
    static mut mousecap: std::ffi::c_int;
    static mut wheel_lines: std::ffi::c_int;
    static mut less_is_more: std::ffi::c_int;
    static mut linenum_width: std::ffi::c_int;
    static mut status_col_width: std::ffi::c_int;
    static mut use_color: std::ffi::c_int;
    static mut want_filesize: std::ffi::c_int;
    static mut header_lines: std::ffi::c_int;
    static mut header_cols: std::ffi::c_int;
    static mut def_search_type: std::ffi::c_int;
    static mut chopline: std::ffi::c_int;
    static mut tabstops: [std::ffi::c_int; 0];
    static mut ntabstops: std::ffi::c_int;
    static mut tabdefault: std::ffi::c_int;
    static mut no_paste: std::ffi::c_int;
    static mut intr_char: std::ffi::c_char;
    static mut nosearch_header_lines: std::ffi::c_int;
    static mut nosearch_header_cols: std::ffi::c_int;
    static mut header_start_pos: POSITION;
    static mut init_header: *mut std::ffi::c_char;
    static mut namelogfile: *mut std::ffi::c_char;
    static mut force_logfile: lbool;
    static mut logfile: std::ffi::c_int;
    static mut tags: *mut std::ffi::c_char;
    static mut ztags: [std::ffi::c_char; 0];
}
pub type __uintmax_t = std::ffi::c_ulong;
pub type __off_t = std::ffi::c_long;
pub type __ssize_t = std::ffi::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = std::ffi::c_ulong;
pub type uintmax_t = __uintmax_t;
pub type lbool = std::ffi::c_uint;
pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;
pub type uintmax = uintmax_t;
pub type LWCHAR = std::ffi::c_ulong;
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
pub static mut tagoption: *mut std::ffi::c_char =
    0 as *const std::ffi::c_char as *mut std::ffi::c_char;
#[no_mangle]
pub unsafe extern "C" fn opt_o(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    let mut filename: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    if secure_allow((1 as std::ffi::c_int) << 7 as std::ffi::c_int) == 0 {
        error(
            b"log file support is not available\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        return;
    }
    match type_0 {
        0 => {
            namelogfile = save(s);
        }
        2 => {
            if ch_getflags() & 0o1 as std::ffi::c_int != 0 {
                error(
                    b"Input is not a pipe\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                return;
            }
            if logfile >= 0 as std::ffi::c_int {
                error(
                    b"Log file is already in use\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                return;
            }
            s = skipspc(s);
            if !namelogfile.is_null() {
                free(namelogfile as *mut std::ffi::c_void);
            }
            filename = lglob(s);
            namelogfile = shell_unquote(filename);
            free(filename as *mut std::ffi::c_void);
            use_logfile(namelogfile);
            sync_logfile();
        }
        1 => {
            if logfile < 0 as std::ffi::c_int {
                error(
                    b"No log file\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
            } else {
                parg.p_string = namelogfile;
                error(
                    b"Log file \"%s\"\0" as *const u8 as *const std::ffi::c_char,
                    &mut parg,
                );
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt__O(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    force_logfile = LTRUE;
    opt_o(type_0, s);
}
unsafe extern "C" fn toggle_fraction(
    mut num: *mut std::ffi::c_int,
    mut frac: *mut std::ffi::c_long,
    mut s: *const std::ffi::c_char,
    mut printopt: *const std::ffi::c_char,
    mut calc: Option<unsafe extern "C" fn() -> ()>,
) -> std::ffi::c_int {
    let mut err: lbool = LFALSE;
    if s.is_null() {
        (Some(calc.expect("non-null function pointer"))).expect("non-null function pointer")();
    } else if *s as std::ffi::c_int == '.' as i32 {
        let mut tfrac: std::ffi::c_long = 0;
        s = s.offset(1);
        tfrac = getfraction(&mut s, printopt, &mut err);
        if err as u64 != 0 {
            error(
                b"Invalid fraction\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
            return -(1 as std::ffi::c_int);
        }
        *frac = tfrac;
        (Some(calc.expect("non-null function pointer"))).expect("non-null function pointer")();
    } else {
        let mut tnum: std::ffi::c_int = getnumc(&mut s, printopt, &mut err);
        if err as u64 != 0 {
            error(
                b"Invalid number\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
            return -(1 as std::ffi::c_int);
        }
        *frac = -(1 as std::ffi::c_int) as std::ffi::c_long;
        *num = tnum;
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn query_fraction(
    mut value: std::ffi::c_int,
    mut fraction: std::ffi::c_long,
    mut int_msg: *const std::ffi::c_char,
    mut frac_msg: *const std::ffi::c_char,
) {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    if fraction < 0 as std::ffi::c_int as std::ffi::c_long {
        parg.p_int = value;
        error(int_msg, &mut parg);
    } else {
        let mut buf: [std::ffi::c_char; 23] = [0; 23];
        let mut len: size_t = 0;
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[std::ffi::c_char; 23]>() as std::ffi::c_ulong,
            b".%06ld\0" as *const u8 as *const std::ffi::c_char,
            fraction,
        );
        len = strlen(buf.as_mut_ptr());
        while len > 2 as std::ffi::c_int as size_t
            && buf[len.wrapping_sub(1 as std::ffi::c_int as size_t) as usize] as std::ffi::c_int
                == '0' as i32
        {
            len = len.wrapping_sub(1);
        }
        buf[len as usize] = '\0' as i32 as std::ffi::c_char;
        parg.p_string = buf.as_mut_ptr();
        error(frac_msg, &mut parg);
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_j(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    match type_0 {
        0 | 2 => {
            toggle_fraction(
                &mut jump_sline,
                &mut jump_sline_fraction,
                s,
                b"j\0" as *const u8 as *const std::ffi::c_char,
                Some(calc_jump_sline as unsafe extern "C" fn() -> ()),
            );
        }
        1 => {
            query_fraction(
                jump_sline,
                jump_sline_fraction,
                b"Position target at screen line %d\0" as *const u8 as *const std::ffi::c_char,
                b"Position target at screen position %s\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn calc_jump_sline() {
    if jump_sline_fraction >= 0 as std::ffi::c_int as std::ffi::c_long {
        jump_sline = umuldiv(
            sc_height as uintmax,
            jump_sline_fraction as uintmax,
            1000000 as std::ffi::c_int as uintmax,
        ) as std::ffi::c_int;
    }
    if jump_sline <= header_lines {
        jump_sline = header_lines + 1 as std::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn opt_shift(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    match type_0 {
        0 | 2 => {
            toggle_fraction(
                &mut shift_count,
                &mut shift_count_fraction,
                s,
                b"#\0" as *const u8 as *const std::ffi::c_char,
                Some(calc_shift_count as unsafe extern "C" fn() -> ()),
            );
        }
        1 => {
            query_fraction(
                shift_count,
                shift_count_fraction,
                b"Horizontal shift %d columns\0" as *const u8 as *const std::ffi::c_char,
                b"Horizontal shift %s of screen width\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn calc_shift_count() {
    if shift_count_fraction < 0 as std::ffi::c_int as std::ffi::c_long {
        return;
    }
    shift_count = umuldiv(
        sc_width as uintmax,
        shift_count_fraction as uintmax,
        1000000 as std::ffi::c_int as uintmax,
    ) as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn opt_k(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    match type_0 {
        0 => {
            if lesskey(s, LFALSE) != 0 {
                parg.p_string = s;
                error(
                    b"Cannot use lesskey file \"%s\"\0" as *const u8 as *const std::ffi::c_char,
                    &mut parg,
                );
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_ks(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    match type_0 {
        0 => {
            if lesskey_src(s, LFALSE) != 0 {
                parg.p_string = s;
                error(
                    b"Cannot use lesskey source file \"%s\"\0" as *const u8
                        as *const std::ffi::c_char,
                    &mut parg,
                );
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_kc(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    match type_0 {
        0 => {
            if lesskey_content(s, LFALSE) != 0 {
                error(
                    b"Error in lesskey content\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt__S(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    match type_0 {
        2 => {
            pos_rehead();
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_t(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    let mut save_ifile: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    let mut pos: POSITION = 0;
    match type_0 {
        0 => {
            tagoption = save(s);
        }
        2 => {
            if secure_allow((1 as std::ffi::c_int) << 11 as std::ffi::c_int) == 0 {
                error(
                    b"tags support is not available\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
            } else {
                findtag(skipspc(s));
                save_ifile = save_curr_ifile();
                if edit_tagfile() != 0 || {
                    pos = tagsearch();
                    pos == -(1 as std::ffi::c_int) as POSITION
                } {
                    reedit_ifile(save_ifile);
                } else {
                    unsave_ifile(save_ifile);
                    jump_loc(pos, jump_sline);
                }
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt__T(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    let mut filename: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    match type_0 {
        0 => {
            tags = save(s);
        }
        2 => {
            s = skipspc(s);
            if !tags.is_null() && tags != ztags.as_mut_ptr() {
                free(tags as *mut std::ffi::c_void);
            }
            filename = lglob(s);
            tags = shell_unquote(filename);
            free(filename as *mut std::ffi::c_void);
        }
        1 => {
            parg.p_string = tags;
            error(
                b"Tags file \"%s\"\0" as *const u8 as *const std::ffi::c_char,
                &mut parg,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_p(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    match type_0 {
        0 => {
            if less_is_more != 0 {
                every_first_cmd = save(s);
            } else {
                plusoption = LTRUE;
                ungetsc(b"/\0" as *const u8 as *const std::ffi::c_char);
                ungetsc(s);
                ungetcc_end_command();
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt__P(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    let mut proto: *mut *mut std::ffi::c_char = 0 as *mut *mut std::ffi::c_char;
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    match type_0 {
        0 | 2 => {
            match *s as std::ffi::c_int {
                115 => {
                    proto = &mut *prproto.as_mut_ptr().offset(0 as std::ffi::c_int as isize)
                        as *mut *mut std::ffi::c_char;
                    s = s.offset(1);
                }
                109 => {
                    proto = &mut *prproto.as_mut_ptr().offset(1 as std::ffi::c_int as isize)
                        as *mut *mut std::ffi::c_char;
                    s = s.offset(1);
                }
                77 => {
                    proto = &mut *prproto.as_mut_ptr().offset(2 as std::ffi::c_int as isize)
                        as *mut *mut std::ffi::c_char;
                    s = s.offset(1);
                }
                61 => {
                    proto = &mut eqproto;
                    s = s.offset(1);
                }
                104 => {
                    proto = &mut hproto;
                    s = s.offset(1);
                }
                119 => {
                    proto = &mut wproto;
                    s = s.offset(1);
                }
                _ => {
                    proto = &mut *prproto.as_mut_ptr().offset(0 as std::ffi::c_int as isize)
                        as *mut *mut std::ffi::c_char;
                }
            }
            free(*proto as *mut std::ffi::c_void);
            *proto = save(s);
        }
        1 => {
            parg.p_string = *prproto.as_mut_ptr().offset(pr_type as isize);
            error(b"%s\0" as *const u8 as *const std::ffi::c_char, &mut parg);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_b(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    match type_0 {
        0 | 2 => {
            ch_setbufspace(bufspace as ssize_t);
        }
        1 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_i(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    match type_0 {
        2 => {
            chg_caseless();
        }
        1 | 0 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt__V(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    match type_0 {
        2 | 1 => {
            dispversion();
        }
        0 => {
            set_output(1 as std::ffi::c_int);
            putstr(b"less \0" as *const u8 as *const std::ffi::c_char);
            putstr(version.as_mut_ptr());
            putstr(b" (\0" as *const u8 as *const std::ffi::c_char);
            putstr(pattern_lib_name());
            putstr(b" regular expressions)\n\0" as *const u8 as *const std::ffi::c_char);
            let mut copyright: *const std::ffi::c_char =
                b"Copyright (C) 1984-2025  Mark Nudelman\n\n\0" as *const u8
                    as *const std::ffi::c_char;
            putstr(copyright);
            if *version.as_mut_ptr().offset(
                (strlen(version.as_mut_ptr()))
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                    as isize,
            ) as std::ffi::c_int
                == 'x' as i32
            {
                putstr(
                    b"** This is an EXPERIMENTAL build of the 'less' software,\n\0" as *const u8
                        as *const std::ffi::c_char,
                );
                putstr(
                    b"** and may not function correctly.\n\0" as *const u8
                        as *const std::ffi::c_char,
                );
                putstr(
                    b"** Obtain release builds from the web page below.\n\n\0" as *const u8
                        as *const std::ffi::c_char,
                );
            }
            putstr(
                b"less comes with NO WARRANTY, to the extent permitted by law.\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            putstr(
                b"For information about the terms of redistribution,\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            putstr(
                b"see the file named README in the less distribution.\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            putstr(
                b"Home page: https://greenwoodsoftware.com/less\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            quit(0 as std::ffi::c_int);
        }
        _ => {}
    };
}
unsafe extern "C" fn color_from_namechar(mut namechar: std::ffi::c_char) -> std::ffi::c_int {
    match namechar as std::ffi::c_int {
        66 => return (2 as std::ffi::c_int) << 8 as std::ffi::c_int,
        67 => return (3 as std::ffi::c_int) << 8 as std::ffi::c_int,
        69 => return (4 as std::ffi::c_int) << 8 as std::ffi::c_int,
        72 => return (9 as std::ffi::c_int) << 8 as std::ffi::c_int,
        77 => return (6 as std::ffi::c_int) << 8 as std::ffi::c_int,
        78 => return (5 as std::ffi::c_int) << 8 as std::ffi::c_int,
        80 => return (7 as std::ffi::c_int) << 8 as std::ffi::c_int,
        82 => return (8 as std::ffi::c_int) << 8 as std::ffi::c_int,
        83 => return (10 as std::ffi::c_int) << 8 as std::ffi::c_int,
        87 | 65 => return (1 as std::ffi::c_int) << 8 as std::ffi::c_int,
        110 => return 0 as std::ffi::c_int,
        115 => return (1 as std::ffi::c_int) << 3 as std::ffi::c_int,
        100 => return (1 as std::ffi::c_int) << 1 as std::ffi::c_int,
        117 => return (1 as std::ffi::c_int) << 0 as std::ffi::c_int,
        107 => return (1 as std::ffi::c_int) << 2 as std::ffi::c_int,
        _ => {
            if namechar as std::ffi::c_int >= '1' as i32
                && namechar as std::ffi::c_int
                    <= '0' as i32
                        + (16 as std::ffi::c_int - 10 as std::ffi::c_int - 1 as std::ffi::c_int)
            {
                return 10 as std::ffi::c_int + (namechar as std::ffi::c_int - '0' as i32)
                    << 8 as std::ffi::c_int;
            }
            return -(1 as std::ffi::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_D(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    let mut p: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    let mut attr: std::ffi::c_int = 0;
    match type_0 {
        0 | 2 => {
            attr = color_from_namechar(*s.offset(0 as std::ffi::c_int as isize));
            if attr < 0 as std::ffi::c_int {
                p.p_char = *s.offset(0 as std::ffi::c_int as isize);
                error(
                    b"Invalid color specifier '%c'\0" as *const u8 as *const std::ffi::c_char,
                    &mut p,
                );
                return;
            }
            if use_color == 0
                && attr & (16 as std::ffi::c_int - 1 as std::ffi::c_int) << 8 as std::ffi::c_int
                    != 0
            {
                error(
                    b"Set --use-color before changing colors\0" as *const u8
                        as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                return;
            }
            s = s.offset(1);
            if set_color_map(attr, s) < 0 as std::ffi::c_int {
                p.p_string = s;
                error(
                    b"Invalid color string \"%s\"\0" as *const u8 as *const std::ffi::c_char,
                    &mut p,
                );
                return;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn set_tabs(mut s: *const std::ffi::c_char, mut len: size_t) {
    let mut i: std::ffi::c_int = 0;
    let mut es: *const std::ffi::c_char = s.offset(len as isize);
    i = 1 as std::ffi::c_int;
    while i < 128 as std::ffi::c_int {
        let mut n: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut v: lbool = LFALSE;
        while s < es && *s as std::ffi::c_int == ' ' as i32 {
            s = s.offset(1);
        }
        while s < es && *s as std::ffi::c_int >= '0' as i32 && *s as std::ffi::c_int <= '9' as i32 {
            v = (v as std::ffi::c_uint != 0 || {
                let (fresh0, fresh1) = n.overflowing_mul(10 as std::ffi::c_int);
                *(&mut n as *mut std::ffi::c_int) = fresh0;
                fresh1 as std::ffi::c_int != 0
            }) as std::ffi::c_int as lbool;
            v = (v as std::ffi::c_uint != 0 || {
                let (fresh2, fresh3) = n.overflowing_add(*s as std::ffi::c_int - '0' as i32);
                *(&mut n as *mut std::ffi::c_int) = fresh2;
                fresh3 as std::ffi::c_int != 0
            }) as std::ffi::c_int as lbool;
            s = s.offset(1);
        }
        if v as u64 == 0
            && n > *tabstops
                .as_mut_ptr()
                .offset((i - 1 as std::ffi::c_int) as isize)
        {
            let fresh4 = i;
            i = i + 1;
            *tabstops.as_mut_ptr().offset(fresh4 as isize) = n;
        }
        while s < es && *s as std::ffi::c_int == ' ' as i32 {
            s = s.offset(1);
        }
        if s == es || {
            let fresh5 = s;
            s = s.offset(1);
            *fresh5 as std::ffi::c_int != ',' as i32
        } {
            break;
        }
    }
    if i < 2 as std::ffi::c_int {
        return;
    }
    ntabstops = i;
    tabdefault = *tabstops
        .as_mut_ptr()
        .offset((ntabstops - 1 as std::ffi::c_int) as isize)
        - *tabstops
            .as_mut_ptr()
            .offset((ntabstops - 2 as std::ffi::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn opt_x(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    let mut msg: [std::ffi::c_char; 1596] = [0; 1596];
    let mut i: std::ffi::c_int = 0;
    let mut p: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    match type_0 {
        0 | 2 => {
            set_tabs(s, strlen(s));
        }
        1 => {
            strcpy(
                msg.as_mut_ptr(),
                b"Tab stops \0" as *const u8 as *const std::ffi::c_char,
            );
            if ntabstops > 2 as std::ffi::c_int {
                i = 1 as std::ffi::c_int;
                while i < ntabstops {
                    if i > 1 as std::ffi::c_int {
                        strcat(
                            msg.as_mut_ptr(),
                            b",\0" as *const u8 as *const std::ffi::c_char,
                        );
                    }
                    sprintf(
                        msg.as_mut_ptr().offset(strlen(msg.as_mut_ptr()) as isize),
                        b"%d\0" as *const u8 as *const std::ffi::c_char,
                        *tabstops.as_mut_ptr().offset(i as isize),
                    );
                    i += 1;
                }
                sprintf(
                    msg.as_mut_ptr().offset(strlen(msg.as_mut_ptr()) as isize),
                    b" and then \0" as *const u8 as *const std::ffi::c_char,
                );
            }
            sprintf(
                msg.as_mut_ptr().offset(strlen(msg.as_mut_ptr()) as isize),
                b"every %d spaces\0" as *const u8 as *const std::ffi::c_char,
                tabdefault,
            );
            p.p_string = msg.as_mut_ptr();
            error(b"%s\0" as *const u8 as *const std::ffi::c_char, &mut p);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_quote(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    let mut buf: [std::ffi::c_char; 3] = [0; 3];
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    match type_0 {
        0 | 2 => {
            if *s.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int == '\0' as i32 {
                closequote = '\0' as i32 as std::ffi::c_char;
                openquote = closequote;
            } else {
                if *s.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int != '\0' as i32
                    && *s.offset(2 as std::ffi::c_int as isize) as std::ffi::c_int != '\0' as i32
                {
                    error(
                        b"-\" must be followed by 1 or 2 chars\0" as *const u8
                            as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void as *mut PARG,
                    );
                    return;
                }
                openquote = *s.offset(0 as std::ffi::c_int as isize);
                if *s.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int == '\0' as i32 {
                    closequote = openquote;
                } else {
                    closequote = *s.offset(1 as std::ffi::c_int as isize);
                }
            }
        }
        1 => {
            buf[0 as std::ffi::c_int as usize] = openquote;
            buf[1 as std::ffi::c_int as usize] = closequote;
            buf[2 as std::ffi::c_int as usize] = '\0' as i32 as std::ffi::c_char;
            parg.p_string = buf.as_mut_ptr();
            error(
                b"quotes %s\0" as *const u8 as *const std::ffi::c_char,
                &mut parg,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_rscroll(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    let mut p: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    match type_0 {
        0 | 2 => {
            let mut fmt: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
            let mut attr: std::ffi::c_int = (1 as std::ffi::c_int) << 3 as std::ffi::c_int;
            setfmt(
                s,
                &mut fmt,
                &mut attr,
                b"*s>\0" as *const u8 as *const std::ffi::c_char,
                LFALSE,
            );
            if strcmp(fmt, b"-\0" as *const u8 as *const std::ffi::c_char) == 0 as std::ffi::c_int {
                rscroll_char = 0 as std::ffi::c_int as LWCHAR;
            } else {
                rscroll_attr = attr | (8 as std::ffi::c_int) << 8 as std::ffi::c_int;
                if *fmt as std::ffi::c_int == '\0' as i32 {
                    rscroll_char = '>' as i32 as LWCHAR;
                } else {
                    let mut ch: LWCHAR = step_charc(
                        &mut fmt,
                        1 as std::ffi::c_int,
                        fmt.offset(strlen(fmt) as isize),
                    );
                    if pwidth(
                        ch,
                        rscroll_attr,
                        0 as std::ffi::c_int as LWCHAR,
                        0 as std::ffi::c_int,
                    ) > 1 as std::ffi::c_int
                    {
                        error(
                            b"cannot set rscroll to a wide character\0" as *const u8
                                as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void as *mut PARG,
                        );
                    } else {
                        rscroll_char = ch;
                    }
                }
            }
        }
        1 => {
            p.p_string = if rscroll_char != 0 {
                prchar(rscroll_char)
            } else {
                b"-\0" as *const u8 as *const std::ffi::c_char
            };
            error(
                b"rscroll character is %s\0" as *const u8 as *const std::ffi::c_char,
                &mut p,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_query(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    match type_0 {
        1 | 2 => {
            error(
                b"Use \"h\" for help\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
        }
        0 => {
            dohelp = 1 as std::ffi::c_int;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_match_shift(
    mut type_0: std::ffi::c_int,
    mut s: *const std::ffi::c_char,
) {
    match type_0 {
        0 | 2 => {
            toggle_fraction(
                &mut match_shift,
                &mut match_shift_fraction,
                s,
                b"--match-shift\0" as *const u8 as *const std::ffi::c_char,
                Some(calc_match_shift as unsafe extern "C" fn() -> ()),
            );
        }
        1 => {
            query_fraction(
                match_shift,
                match_shift_fraction,
                b"Search match shift is %d\0" as *const u8 as *const std::ffi::c_char,
                b"Search match shift is %s of screen width\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn calc_match_shift() {
    if match_shift_fraction < 0 as std::ffi::c_int as std::ffi::c_long {
        return;
    }
    match_shift = umuldiv(
        sc_width as uintmax,
        match_shift_fraction as uintmax,
        1000000 as std::ffi::c_int as uintmax,
    ) as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn opt_mousecap(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    match type_0 {
        2 => {
            if mousecap == 0 as std::ffi::c_int {
                deinit_mouse();
            } else {
                init_mouse();
            }
        }
        0 | 1 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_wheel_lines(
    mut type_0: std::ffi::c_int,
    mut s: *const std::ffi::c_char,
) {
    match type_0 {
        0 | 2 => {
            if wheel_lines <= 0 as std::ffi::c_int {
                wheel_lines = default_wheel_lines();
            }
        }
        1 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_linenum_width(
    mut type_0: std::ffi::c_int,
    mut s: *const std::ffi::c_char,
) {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    match type_0 {
        0 | 2 => {
            if linenum_width > 16 as std::ffi::c_int {
                parg.p_int = 16 as std::ffi::c_int;
                error(
                    b"Line number width must not be larger than %d\0" as *const u8
                        as *const std::ffi::c_char,
                    &mut parg,
                );
                linenum_width = 7 as std::ffi::c_int;
            }
        }
        1 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_status_col_width(
    mut type_0: std::ffi::c_int,
    mut s: *const std::ffi::c_char,
) {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    match type_0 {
        0 | 2 => {
            if status_col_width > 4 as std::ffi::c_int {
                parg.p_int = 4 as std::ffi::c_int;
                error(
                    b"Status column width must not be larger than %d\0" as *const u8
                        as *const std::ffi::c_char,
                    &mut parg,
                );
                status_col_width = 2 as std::ffi::c_int;
            }
        }
        1 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_filesize(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    match type_0 {
        0 | 2 => {
            if want_filesize != 0
                && !curr_ifile.is_null()
                && ch_length() == -(1 as std::ffi::c_int) as POSITION
            {
                scan_eof();
            }
        }
        1 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_intr(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    let mut p: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    match type_0 {
        0 | 2 => {
            intr_char = *s;
            if intr_char as std::ffi::c_int == '^' as i32
                && *s.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int != '\0' as i32
            {
                intr_char = (*s.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                    & 0o37 as std::ffi::c_int) as std::ffi::c_char;
            }
        }
        1 => {
            p.p_string = prchar(intr_char as LWCHAR);
            error(
                b"interrupt character is %s\0" as *const u8 as *const std::ffi::c_char,
                &mut p,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn next_cnum(
    mut sp: *mut *const std::ffi::c_char,
    mut printopt: *const std::ffi::c_char,
    mut errmsg: *const std::ffi::c_char,
    mut errp: *mut lbool,
) -> std::ffi::c_int {
    let mut n: std::ffi::c_int = 0;
    *errp = LFALSE;
    if **sp as std::ffi::c_int == '\0' as i32 {
        return -(1 as std::ffi::c_int);
    }
    if **sp as std::ffi::c_int == ',' as i32 {
        *sp = (*sp).offset(1);
        return -(1 as std::ffi::c_int);
    }
    n = getnumc(sp, printopt, errp);
    if *errp as u64 != 0 {
        let mut parg: PARG = parg {
            p_string: 0 as *const std::ffi::c_char,
        };
        parg.p_string = errmsg;
        error(
            b"invalid %s\0" as *const u8 as *const std::ffi::c_char,
            &mut parg,
        );
        return -(1 as std::ffi::c_int);
    }
    if **sp as std::ffi::c_int == ',' as i32 {
        *sp = (*sp).offset(1);
    }
    return n;
}
unsafe extern "C" fn parse_header(
    mut s: *const std::ffi::c_char,
    mut lines: *mut std::ffi::c_int,
    mut cols: *mut std::ffi::c_int,
    mut start_pos: *mut POSITION,
) -> lbool {
    let mut n: std::ffi::c_int = 0;
    let mut err: lbool = LFALSE;
    if *s as std::ffi::c_int == '-' as i32 {
        s = b"0,0\0" as *const u8 as *const std::ffi::c_char;
    }
    n = next_cnum(
        &mut s,
        b"header\0" as *const u8 as *const std::ffi::c_char,
        b"number of lines\0" as *const u8 as *const std::ffi::c_char,
        &mut err,
    );
    if err as u64 != 0 {
        return LFALSE;
    }
    if n >= 0 as std::ffi::c_int {
        *lines = n;
    }
    n = next_cnum(
        &mut s,
        b"header\0" as *const u8 as *const std::ffi::c_char,
        b"number of columns\0" as *const u8 as *const std::ffi::c_char,
        &mut err,
    );
    if err as u64 != 0 {
        return LFALSE;
    }
    if n >= 0 as std::ffi::c_int {
        *cols = n;
    }
    n = next_cnum(
        &mut s,
        b"header\0" as *const u8 as *const std::ffi::c_char,
        b"line number\0" as *const u8 as *const std::ffi::c_char,
        &mut err,
    );
    if err as u64 != 0 {
        return LFALSE;
    }
    if n > 0 as std::ffi::c_int {
        let mut lnum: LINENUM = n as LINENUM;
        if lnum < 1 as std::ffi::c_int as LINENUM {
            lnum = 1 as std::ffi::c_int as LINENUM;
        }
        *start_pos = find_pos(lnum);
    }
    return LTRUE;
}
#[no_mangle]
pub unsafe extern "C" fn opt_header(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    match type_0 {
        0 => {
            init_header = save(s);
        }
        2 => {
            let mut lines: std::ffi::c_int = header_lines;
            let mut cols: std::ffi::c_int = header_cols;
            let mut start_pos: POSITION = if type_0 == 0 as std::ffi::c_int {
                0 as std::ffi::c_int as POSITION
            } else {
                position(0 as std::ffi::c_int)
            };
            if start_pos == -(1 as std::ffi::c_int) as POSITION {
                start_pos = 0 as std::ffi::c_int as POSITION;
            }
            if !(parse_header(s, &mut lines, &mut cols, &mut start_pos) as u64 == 0) {
                header_lines = lines;
                header_cols = cols;
                set_header(start_pos);
                calc_jump_sline();
            }
        }
        1 => {
            let mut buf: [std::ffi::c_char; 66] = [0; 66];
            let mut parg: PARG = parg {
                p_string: 0 as *const std::ffi::c_char,
            };
            snprintf(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[std::ffi::c_char; 66]>() as std::ffi::c_ulong,
                b"%ld,%ld,%ld\0" as *const u8 as *const std::ffi::c_char,
                header_lines as std::ffi::c_long,
                header_cols as std::ffi::c_long,
                find_linenum(header_start_pos),
            );
            parg.p_string = buf.as_mut_ptr();
            error(
                b"Header (lines,columns,line-number) is %s\0" as *const u8
                    as *const std::ffi::c_char,
                &mut parg,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_search_type(
    mut type_0: std::ffi::c_int,
    mut s: *const std::ffi::c_char,
) {
    let mut st: std::ffi::c_int = 0;
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    let mut buf: [std::ffi::c_char; 16] = [0; 16];
    let mut bp: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut i: std::ffi::c_int = 0;
    match type_0 {
        0 | 2 => {
            st = 0 as std::ffi::c_int;
            while *s as std::ffi::c_int != '\0' as i32 {
                match *s as std::ffi::c_int {
                    69 | 101 | 5 => {
                        st |= (1 as std::ffi::c_int) << 9 as std::ffi::c_int;
                    }
                    70 | 102 | 6 => {
                        st |= (1 as std::ffi::c_int) << 10 as std::ffi::c_int;
                    }
                    75 | 107 | 11 => {
                        st |= (1 as std::ffi::c_int) << 2 as std::ffi::c_int;
                    }
                    78 | 110 | 14 => {
                        st |= (1 as std::ffi::c_int) << 8 as std::ffi::c_int;
                    }
                    82 | 114 | 18 => {
                        st |= (1 as std::ffi::c_int) << 12 as std::ffi::c_int;
                    }
                    87 | 119 | 23 => {
                        st |= (1 as std::ffi::c_int) << 15 as std::ffi::c_int;
                    }
                    45 => {
                        st = 0 as std::ffi::c_int;
                    }
                    94 => {}
                    _ => {
                        if *s as std::ffi::c_int >= '1' as i32
                            && *s as std::ffi::c_int
                                <= '0' as i32
                                    + (16 as std::ffi::c_int
                                        - 10 as std::ffi::c_int
                                        - 1 as std::ffi::c_int)
                        {
                            st |= (1 as std::ffi::c_int)
                                << 17 as std::ffi::c_int + (*s as std::ffi::c_int - '0' as i32);
                        } else {
                            parg.p_char = *s;
                            error(
                                b"invalid search option '%c'\0" as *const u8
                                    as *const std::ffi::c_char,
                                &mut parg,
                            );
                            return;
                        }
                    }
                }
                s = s.offset(1);
            }
            def_search_type = norm_search_type(st);
        }
        1 => {
            bp = buf.as_mut_ptr();
            if def_search_type & (1 as std::ffi::c_int) << 9 as std::ffi::c_int != 0 {
                let fresh6 = bp;
                bp = bp.offset(1);
                *fresh6 = 'E' as i32 as std::ffi::c_char;
            }
            if def_search_type & (1 as std::ffi::c_int) << 10 as std::ffi::c_int != 0 {
                let fresh7 = bp;
                bp = bp.offset(1);
                *fresh7 = 'F' as i32 as std::ffi::c_char;
            }
            if def_search_type & (1 as std::ffi::c_int) << 2 as std::ffi::c_int != 0 {
                let fresh8 = bp;
                bp = bp.offset(1);
                *fresh8 = 'K' as i32 as std::ffi::c_char;
            }
            if def_search_type & (1 as std::ffi::c_int) << 8 as std::ffi::c_int != 0 {
                let fresh9 = bp;
                bp = bp.offset(1);
                *fresh9 = 'N' as i32 as std::ffi::c_char;
            }
            if def_search_type & (1 as std::ffi::c_int) << 12 as std::ffi::c_int != 0 {
                let fresh10 = bp;
                bp = bp.offset(1);
                *fresh10 = 'R' as i32 as std::ffi::c_char;
            }
            if def_search_type & (1 as std::ffi::c_int) << 15 as std::ffi::c_int != 0 {
                let fresh11 = bp;
                bp = bp.offset(1);
                *fresh11 = 'W' as i32 as std::ffi::c_char;
            }
            i = 1 as std::ffi::c_int;
            while i <= 16 as std::ffi::c_int - 10 as std::ffi::c_int - 1 as std::ffi::c_int {
                if def_search_type & (1 as std::ffi::c_int) << 17 as std::ffi::c_int + i != 0 {
                    let fresh12 = bp;
                    bp = bp.offset(1);
                    *fresh12 = ('0' as i32 + i) as std::ffi::c_char;
                }
                i += 1;
            }
            if bp == buf.as_mut_ptr() {
                let fresh13 = bp;
                bp = bp.offset(1);
                *fresh13 = '-' as i32 as std::ffi::c_char;
            }
            *bp = '\0' as i32 as std::ffi::c_char;
            parg.p_string = buf.as_mut_ptr();
            error(
                b"search options: %s\0" as *const u8 as *const std::ffi::c_char,
                &mut parg,
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn do_nosearch_headers(
    mut type_0: std::ffi::c_int,
    mut no_header_lines: std::ffi::c_int,
    mut no_header_cols: std::ffi::c_int,
) {
    let mut current_block_8: u64;
    match type_0 {
        0 | 2 => {
            nosearch_header_lines = no_header_lines;
            nosearch_header_cols = no_header_cols;
            if type_0 != 2 as std::ffi::c_int {
                current_block_8 = 13109137661213826276;
            } else {
                current_block_8 = 4311149068773253642;
            }
        }
        1 => {
            current_block_8 = 4311149068773253642;
        }
        _ => {
            current_block_8 = 13109137661213826276;
        }
    }
    match current_block_8 {
        4311149068773253642 => {
            if nosearch_header_lines != 0 && nosearch_header_cols != 0 {
                error(
                    b"Search does not include header lines or columns\0" as *const u8
                        as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
            } else if nosearch_header_lines != 0 {
                error(
                    b"Search includes header columns but not header lines\0" as *const u8
                        as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
            } else if nosearch_header_cols != 0 {
                error(
                    b"Search includes header lines but not header columns\0" as *const u8
                        as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
            } else {
                error(
                    b"Search includes header lines and columns\0" as *const u8
                        as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_nosearch_headers(
    mut type_0: std::ffi::c_int,
    mut s: *const std::ffi::c_char,
) {
    do_nosearch_headers(type_0, 1 as std::ffi::c_int, 1 as std::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn opt_nosearch_header_lines(
    mut type_0: std::ffi::c_int,
    mut s: *const std::ffi::c_char,
) {
    do_nosearch_headers(type_0, 1 as std::ffi::c_int, 0 as std::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn opt_nosearch_header_cols(
    mut type_0: std::ffi::c_int,
    mut s: *const std::ffi::c_char,
) {
    do_nosearch_headers(type_0, 0 as std::ffi::c_int, 1 as std::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn opt_no_paste(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    match type_0 {
        2 => {
            if no_paste != 0 {
                init_bracketed_paste();
            } else {
                deinit_bracketed_paste();
            }
        }
        0 | 1 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn chop_line() -> std::ffi::c_int {
    return (chopline != 0
        || header_cols > 0 as std::ffi::c_int
        || header_lines > 0 as std::ffi::c_int) as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_swindow() -> std::ffi::c_int {
    if swindow > 0 as std::ffi::c_int {
        return swindow;
    }
    return sc_height - header_lines + swindow;
}
