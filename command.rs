use ::libc;
extern "C" {
    fn snprintf(
        _: *mut std::ffi::c_char,
        _: std::ffi::c_ulong,
        _: *const std::ffi::c_char,
        _: ...
    ) -> std::ffi::c_int;
    fn free(_: *mut std::ffi::c_void);
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn save(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
    fn quit(status: std::ffi::c_int);
    fn secure_allow(features: std::ffi::c_int) -> std::ffi::c_int;
    fn check_winch();
    fn bell();
    fn clear();
    fn clear_eol();
    fn clear_bot();
    fn at_enter(attr: std::ffi::c_int);
    fn at_exit();
    fn match_brac(
        obrac: std::ffi::c_char,
        cbrac: std::ffi::c_char,
        forwdir: std::ffi::c_int,
        n: std::ffi::c_int,
    );
    fn ch_length() -> POSITION;
    fn ch_flush();
    fn ch_set_eof();
    fn ch_getflags() -> std::ffi::c_int;
    fn cmd_reset();
    fn clear_cmd();
    fn cmd_putstr(s: *const std::ffi::c_char);
    fn len_cmdbuf() -> std::ffi::c_int;
    fn cmdbuf_empty() -> lbool;
    fn cmd_repaint(old_cp: *const std::ffi::c_char);
    fn set_mlist(mlist: *mut std::ffi::c_void, cmdflags: std::ffi::c_int);
    fn save_updown_match() -> ssize_t;
    fn restore_updown_match(udm: ssize_t);
    fn cmd_accept();
    fn cmd_char(c: std::ffi::c_char) -> std::ffi::c_int;
    fn cmd_setstring(s: *const std::ffi::c_char, uc: lbool) -> std::ffi::c_int;
    fn cmd_int(frac: *mut std::ffi::c_long) -> LINENUM;
    fn get_cmdbuf() -> *const std::ffi::c_char;
    fn fcmd_decode(
        cmd: *const std::ffi::c_char,
        sp: *mut *const std::ffi::c_char,
    ) -> std::ffi::c_int;
    fn editchar(c: std::ffi::c_char, flags: std::ffi::c_int) -> std::ffi::c_int;
    fn edit(filename: *const std::ffi::c_char) -> std::ffi::c_int;
    fn edit_ifile(ifile: *mut std::ffi::c_void) -> std::ffi::c_int;
    fn edit_list(filelist: *mut std::ffi::c_char) -> std::ffi::c_int;
    fn edit_first() -> std::ffi::c_int;
    fn edit_last() -> std::ffi::c_int;
    fn edit_next(n: std::ffi::c_int) -> std::ffi::c_int;
    fn edit_prev(n: std::ffi::c_int) -> std::ffi::c_int;
    fn edit_index(n: std::ffi::c_int) -> std::ffi::c_int;
    fn save_curr_ifile() -> *mut std::ffi::c_void;
    fn unsave_ifile(save_ifile: *mut std::ffi::c_void);
    fn reedit_ifile(save_ifile: *mut std::ffi::c_void);
    fn reopen_curr_ifile();
    fn fexpand(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn eof_displayed(offset: lbool) -> lbool;
    fn entire_file_displayed() -> lbool;
    fn forward(n: std::ffi::c_int, force: lbool, only_last: lbool, to_newline: lbool);
    fn backward(n: std::ffi::c_int, force: lbool, only_last: lbool, to_newline: lbool);
    fn del_ifile(h: *mut std::ffi::c_void);
    fn next_ifile(h: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn getoff_ifile(ifile: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn get_filename(ifile: *mut std::ffi::c_void) -> *const std::ffi::c_char;
    fn get_altfilename(ifile: *mut std::ffi::c_void) -> *mut std::ffi::c_char;
    fn set_attnpos(pos: POSITION);
    fn jump_forw();
    fn jump_forw_buffered();
    fn jump_back(linenum: LINENUM);
    fn repaint();
    fn jump_percent(percent: std::ffi::c_int, fraction_0: std::ffi::c_long);
    fn jump_line_loc(pos: POSITION, sline: std::ffi::c_int);
    fn jump_loc(pos: POSITION, sline: std::ffi::c_int);
    fn set_line_contig_pos(pos: POSITION);
    fn load_line(str: *const std::ffi::c_char);
    fn rrshift() -> std::ffi::c_int;
    fn clr_linenum();
    fn lsystem(cmd: *const std::ffi::c_char, donemsg: *const std::ffi::c_char);
    fn pipe_mark(c: std::ffi::c_char, cmd: *const std::ffi::c_char) -> std::ffi::c_int;
    fn badmark(c: std::ffi::c_char) -> std::ffi::c_int;
    fn setmark(c: std::ffi::c_char, where_0: std::ffi::c_int);
    fn clrmark(c: std::ffi::c_char);
    fn gomark(c: std::ffi::c_char);
    fn get_swindow() -> std::ffi::c_int;
    fn propt(c: std::ffi::c_char) -> *const std::ffi::c_char;
    fn toggle_option(
        o: *mut loption,
        lower: lbool,
        s: *const std::ffi::c_char,
        how_toggle: std::ffi::c_int,
    );
    fn opt_has_param(o: *mut loption) -> std::ffi::c_int;
    fn opt_prompt(o: *mut loption) -> *const std::ffi::c_char;
    fn opt_toggle_disallowed(c: std::ffi::c_int) -> *const std::ffi::c_char;
    fn get_quit_at_eof() -> std::ffi::c_int;
    fn findopt(c: std::ffi::c_int) -> *mut loption;
    fn findopt_name(
        p_optname: *mut *const std::ffi::c_char,
        p_oname: *mut *const std::ffi::c_char,
        p_ambig: *mut lbool,
    ) -> *mut loption;
    fn get_time() -> time_t;
    fn put_line(forw_scroll: lbool);
    fn flush();
    fn putchr(ch: std::ffi::c_int) -> std::ffi::c_int;
    fn putstr(s: *const std::ffi::c_char);
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn position(sindex: std::ffi::c_int) -> POSITION;
    fn empty_screen() -> std::ffi::c_int;
    fn pos_rehead();
    fn pr_expand(proto: *const std::ffi::c_char) -> *const std::ffi::c_char;
    fn eq_message() -> *const std::ffi::c_char;
    fn pr_string() -> *const std::ffi::c_char;
    fn clear_attn();
    fn undo_search(clear_0: lbool);
    fn clr_hilite();
    fn osc8_search(
        search_type_0: std::ffi::c_int,
        param: *const std::ffi::c_char,
        matches: std::ffi::c_int,
    );
    fn osc8_open();
    fn osc8_jump();
    fn search(
        search_type_0: std::ffi::c_int,
        pattern: *const std::ffi::c_char,
        n: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn set_filter_pattern(
        pattern: *const std::ffi::c_char,
        search_type_0: std::ffi::c_int,
    );
    fn is_filtering() -> lbool;
    fn psignals();
    fn cleantags();
    fn tagsearch() -> POSITION;
    fn nexttag(n: std::ffi::c_int) -> *const std::ffi::c_char;
    fn prevtag(n: std::ffi::c_int) -> *const std::ffi::c_char;
    fn ntags() -> std::ffi::c_int;
    fn getchr() -> std::ffi::c_int;
    static mut erase_char: std::ffi::c_int;
    static mut erase2_char: std::ffi::c_int;
    static mut kill_char: std::ffi::c_int;
    static mut sigs: std::ffi::c_int;
    static mut quit_if_one_screen: std::ffi::c_int;
    static mut one_screen: std::ffi::c_int;
    static mut sc_width: std::ffi::c_int;
    static mut sc_height: std::ffi::c_int;
    static mut kent: *mut std::ffi::c_char;
    static mut swindow: std::ffi::c_int;
    static mut jump_sline: std::ffi::c_int;
    static mut quitting: lbool;
    static mut wscroll: std::ffi::c_int;
    static mut top_scroll: std::ffi::c_int;
    static mut ignore_eoi: std::ffi::c_int;
    static mut hshift: std::ffi::c_int;
    static mut bs_mode: std::ffi::c_int;
    static mut proc_backspace: std::ffi::c_int;
    static mut show_attn: std::ffi::c_int;
    static mut chopline: std::ffi::c_int;
    static mut highest_hilite: POSITION;
    static mut every_first_cmd: *mut std::ffi::c_char;
    static mut version: [std::ffi::c_char; 0];
    static mut initial_scrpos: scrpos;
    static mut curr_ifile: *mut std::ffi::c_void;
    static mut ml_search: *mut std::ffi::c_void;
    static mut ml_examine: *mut std::ffi::c_void;
    static mut wheel_lines: std::ffi::c_int;
    static mut def_search_type: std::ffi::c_int;
    static mut search_wrapped: lbool;
    static mut no_poll: lbool;
    static mut no_paste: std::ffi::c_int;
    static mut pasting: lbool;
    static mut no_edit_warn: std::ffi::c_int;
    static mut soft_eof: POSITION;
    static mut ml_shell: *mut std::ffi::c_void;
    static mut editproto: *const std::ffi::c_char;
    static mut osc8_uri: *mut std::ffi::c_char;
    static mut shift_count: std::ffi::c_int;
    static mut forw_prompt: std::ffi::c_int;
    static mut incr_search: std::ffi::c_int;
    static mut full_screen: std::ffi::c_int;
}
pub type __off_t = std::ffi::c_long;
pub type __time_t = std::ffi::c_long;
pub type __ssize_t = std::ffi::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loption {
    pub oletter: std::ffi::c_char,
    pub onames: *mut optname,
    pub otype: std::ffi::c_int,
    pub odefault: std::ffi::c_int,
    pub ovar: *mut std::ffi::c_int,
    pub ofunc: Option::<
        unsafe extern "C" fn(std::ffi::c_int, *const std::ffi::c_char) -> (),
    >,
    pub odesc: [*const std::ffi::c_char; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct optname {
    pub oname: *const std::ffi::c_char,
    pub onext: *mut optname,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ungot {
    pub ug_next: *mut ungot,
    pub ug_char: std::ffi::c_char,
    pub ug_end_command: lbool,
}
static mut shellcmd: *mut std::ffi::c_char = 0 as *const std::ffi::c_char
    as *mut std::ffi::c_char;
static mut mca: std::ffi::c_int = 0;
static mut search_type: std::ffi::c_int = 0;
static mut last_search_type: std::ffi::c_int = 0;
static mut number: LINENUM = 0;
static mut fraction: std::ffi::c_long = 0;
static mut curropt: *mut loption = 0 as *const loption as *mut loption;
static mut opt_lower: lbool = LFALSE;
static mut optflag: std::ffi::c_int = 0;
static mut optgetname: lbool = LFALSE;
static mut bottompos: POSITION = 0;
static mut save_hshift: std::ffi::c_int = 0;
static mut save_bs_mode: std::ffi::c_int = 0;
static mut save_proc_backspace: std::ffi::c_int = 0;
static mut screen_trashed_value: std::ffi::c_int = 0 as std::ffi::c_int;
static mut literal_char: lbool = LFALSE;
static mut ignoring_input: lbool = LFALSE;
static mut ignoring_input_time: time_t = 0;
static mut pipec: std::ffi::c_char = 0;
static mut ungot: *mut ungot = 0 as *const ungot as *mut ungot;
#[no_mangle]
pub unsafe extern "C" fn cmd_exec() {
    clear_attn();
    clear_bot();
    flush();
}
unsafe extern "C" fn set_mca(mut action: std::ffi::c_int) {
    mca = action;
    clear_bot();
    clear_cmd();
}
unsafe extern "C" fn clear_mca() {
    if mca == 0 as std::ffi::c_int {
        return;
    }
    mca = 0 as std::ffi::c_int;
}
unsafe extern "C" fn start_mca(
    mut action: std::ffi::c_int,
    mut prompt_0: *const std::ffi::c_char,
    mut mlist: *mut std::ffi::c_void,
    mut cmdflags: std::ffi::c_int,
) {
    set_mca(action);
    cmd_putstr(prompt_0);
    set_mlist(mlist, cmdflags);
}
#[no_mangle]
pub unsafe extern "C" fn in_mca() -> std::ffi::c_int {
    return (mca != 0 as std::ffi::c_int && mca != 105 as std::ffi::c_int)
        as std::ffi::c_int;
}
unsafe extern "C" fn mca_search1() {
    let mut i: std::ffi::c_int = 0;
    if search_type & (1 as std::ffi::c_int) << 13 as std::ffi::c_int != 0 {
        set_mca(55 as std::ffi::c_int);
    } else if search_type & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
        set_mca(15 as std::ffi::c_int);
    } else {
        set_mca(5 as std::ffi::c_int);
    }
    if search_type & (1 as std::ffi::c_int) << 8 as std::ffi::c_int != 0 {
        cmd_putstr(b"Non-match \0" as *const u8 as *const std::ffi::c_char);
    }
    if search_type & (1 as std::ffi::c_int) << 10 as std::ffi::c_int != 0 {
        cmd_putstr(b"First-file \0" as *const u8 as *const std::ffi::c_char);
    }
    if search_type & (1 as std::ffi::c_int) << 9 as std::ffi::c_int != 0 {
        cmd_putstr(b"EOF-ignore \0" as *const u8 as *const std::ffi::c_char);
    }
    if search_type & (1 as std::ffi::c_int) << 2 as std::ffi::c_int != 0 {
        cmd_putstr(b"Keep-pos \0" as *const u8 as *const std::ffi::c_char);
    }
    if search_type & (1 as std::ffi::c_int) << 12 as std::ffi::c_int != 0 {
        cmd_putstr(b"Regex-off \0" as *const u8 as *const std::ffi::c_char);
    }
    if search_type & (1 as std::ffi::c_int) << 15 as std::ffi::c_int != 0 {
        cmd_putstr(b"Wrap \0" as *const u8 as *const std::ffi::c_char);
    }
    i = 1 as std::ffi::c_int;
    while i <= 16 as std::ffi::c_int - 10 as std::ffi::c_int - 1 as std::ffi::c_int {
        if search_type & (1 as std::ffi::c_int) << 17 as std::ffi::c_int + i != 0 {
            let mut buf: [std::ffi::c_char; 19] = [0; 19];
            snprintf(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[std::ffi::c_char; 19]>() as std::ffi::c_ulong,
                b"Sub-%d \0" as *const u8 as *const std::ffi::c_char,
                i,
            );
            cmd_putstr(buf.as_mut_ptr());
        }
        i += 1;
        i;
    }
    if literal_char as u64 != 0 {
        cmd_putstr(b"Lit \0" as *const u8 as *const std::ffi::c_char);
    }
    if search_type & (1 as std::ffi::c_int) << 13 as std::ffi::c_int != 0 {
        cmd_putstr(b"&/\0" as *const u8 as *const std::ffi::c_char);
    } else if search_type & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
        cmd_putstr(b"/\0" as *const u8 as *const std::ffi::c_char);
    } else {
        cmd_putstr(b"?\0" as *const u8 as *const std::ffi::c_char);
    }
    forw_prompt = 0 as std::ffi::c_int;
}
unsafe extern "C" fn mca_search() {
    mca_search1();
    set_mlist(ml_search, 0 as std::ffi::c_int);
}
unsafe extern "C" fn mca_opt_toggle() {
    let mut no_prompt: std::ffi::c_int = optflag & 0o100 as std::ffi::c_int;
    let mut flag: std::ffi::c_int = optflag & !(0o100 as std::ffi::c_int);
    let mut dash: *const std::ffi::c_char = if flag == 0 as std::ffi::c_int {
        b"_\0" as *const u8 as *const std::ffi::c_char
    } else {
        b"-\0" as *const u8 as *const std::ffi::c_char
    };
    set_mca(47 as std::ffi::c_int);
    cmd_putstr(dash);
    if optgetname as u64 != 0 {
        cmd_putstr(dash);
    }
    if no_prompt != 0 {
        cmd_putstr(b"(P)\0" as *const u8 as *const std::ffi::c_char);
    }
    match flag {
        2 => {
            cmd_putstr(b"+\0" as *const u8 as *const std::ffi::c_char);
        }
        3 => {
            cmd_putstr(b"!\0" as *const u8 as *const std::ffi::c_char);
        }
        _ => {}
    }
    forw_prompt = 0 as std::ffi::c_int;
    set_mlist(
        0 as *mut std::ffi::c_void,
        (1 as std::ffi::c_int) << 1 as std::ffi::c_int,
    );
}
unsafe extern "C" fn exec_mca() {
    let mut cbuf: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    cmd_exec();
    cbuf = get_cmdbuf();
    if cbuf.is_null() {
        return;
    }
    match mca {
        15 | 5 => {
            multi_search(cbuf, number as std::ffi::c_int, 0 as std::ffi::c_int);
        }
        55 => {
            search_type ^= (1 as std::ffi::c_int) << 8 as std::ffi::c_int;
            set_filter_pattern(cbuf, search_type);
            soft_eof = -(1 as std::ffi::c_int) as POSITION;
        }
        10 => {
            while *cbuf as std::ffi::c_int == '+' as i32
                || *cbuf as std::ffi::c_int == ' ' as i32
            {
                cbuf = cbuf.offset(1);
                cbuf;
            }
            if !every_first_cmd.is_null() {
                free(every_first_cmd as *mut std::ffi::c_void);
            }
            if *cbuf as std::ffi::c_int == '\0' as i32 {
                every_first_cmd = 0 as *mut std::ffi::c_char;
            } else {
                every_first_cmd = save(cbuf);
            }
        }
        47 => {
            toggle_option(curropt, opt_lower, cbuf, optflag);
            curropt = 0 as *mut loption;
        }
        35 => {
            match_brac(
                *cbuf.offset(0 as std::ffi::c_int as isize),
                *cbuf.offset(1 as std::ffi::c_int as isize),
                1 as std::ffi::c_int,
                number as std::ffi::c_int,
            );
        }
        36 => {
            match_brac(
                *cbuf.offset(1 as std::ffi::c_int as isize),
                *cbuf.offset(0 as std::ffi::c_int as isize),
                0 as std::ffi::c_int,
                number as std::ffi::c_int,
            );
        }
        9 => {
            let mut p: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
            if !(secure_allow((1 as std::ffi::c_int) << 2 as std::ffi::c_int) == 0) {
                p = save(cbuf);
                edit_list(p);
                free(p as *mut std::ffi::c_void);
                cleantags();
            }
        }
        27 => {
            let mut done_msg: *const std::ffi::c_char = if *cbuf as std::ffi::c_int
                == 'P' as i32 & 0o37 as std::ffi::c_int
            {
                0 as *const std::ffi::c_char
            } else {
                b"!done\0" as *const u8 as *const std::ffi::c_char
            };
            if done_msg.is_null() {
                cbuf = cbuf.offset(1);
                cbuf;
            }
            if *cbuf as std::ffi::c_int != '!' as i32 {
                if !shellcmd.is_null() {
                    free(shellcmd as *mut std::ffi::c_void);
                }
                shellcmd = fexpand(cbuf);
            }
            if !(secure_allow((1 as std::ffi::c_int) << 9 as std::ffi::c_int) == 0) {
                if shellcmd.is_null() {
                    shellcmd = b"\0" as *const u8 as *const std::ffi::c_char
                        as *mut std::ffi::c_char;
                }
                lsystem(shellcmd, done_msg);
            }
        }
        69 => {
            let mut done_msg_0: *const std::ffi::c_char = if *cbuf as std::ffi::c_int
                == 'P' as i32 & 0o37 as std::ffi::c_int
            {
                0 as *const std::ffi::c_char
            } else {
                b"#done\0" as *const u8 as *const std::ffi::c_char
            };
            if done_msg_0.is_null() {
                cbuf = cbuf.offset(1);
                cbuf;
            }
            if !(secure_allow((1 as std::ffi::c_int) << 9 as std::ffi::c_int) == 0) {
                lsystem(pr_expand(cbuf), done_msg_0);
            }
        }
        37 => {
            let mut done_msg_1: *const std::ffi::c_char = if *cbuf as std::ffi::c_int
                == 'P' as i32 & 0o37 as std::ffi::c_int
            {
                0 as *const std::ffi::c_char
            } else {
                b"|done\0" as *const u8 as *const std::ffi::c_char
            };
            if done_msg_1.is_null() {
                cbuf = cbuf.offset(1);
                cbuf;
            }
            if !(secure_allow((1 as std::ffi::c_int) << 8 as std::ffi::c_int) == 0) {
                pipe_mark(pipec, cbuf);
                if !done_msg_1.is_null() {
                    error(done_msg_1, 0 as *mut std::ffi::c_void as *mut PARG);
                }
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn is_erase_char(mut c: std::ffi::c_char) -> lbool {
    return (c as std::ffi::c_int == erase_char || c as std::ffi::c_int == erase2_char
        || c as std::ffi::c_int == kill_char) as std::ffi::c_int as lbool;
}
unsafe extern "C" fn is_newline_char(mut c: std::ffi::c_char) -> lbool {
    return (c as std::ffi::c_int == '\n' as i32 || c as std::ffi::c_int == '\r' as i32)
        as std::ffi::c_int as lbool;
}
unsafe extern "C" fn mca_opt_first_char(mut c: std::ffi::c_char) -> std::ffi::c_int {
    let mut no_prompt: std::ffi::c_int = optflag & 0o100 as std::ffi::c_int;
    let mut flag: std::ffi::c_int = optflag & !(0o100 as std::ffi::c_int);
    if flag == 0 as std::ffi::c_int {
        match c as std::ffi::c_int {
            95 => {
                optgetname = LTRUE;
                mca_opt_toggle();
                return 2 as std::ffi::c_int;
            }
            _ => {}
        }
    } else {
        match c as std::ffi::c_int {
            43 => {
                optflag = no_prompt
                    | (if flag == 2 as std::ffi::c_int {
                        1 as std::ffi::c_int
                    } else {
                        2 as std::ffi::c_int
                    });
                mca_opt_toggle();
                return 2 as std::ffi::c_int;
            }
            33 => {
                optflag = no_prompt
                    | (if flag == 3 as std::ffi::c_int {
                        1 as std::ffi::c_int
                    } else {
                        3 as std::ffi::c_int
                    });
                mca_opt_toggle();
                return 2 as std::ffi::c_int;
            }
            16 => {
                optflag ^= 0o100 as std::ffi::c_int;
                mca_opt_toggle();
                return 2 as std::ffi::c_int;
            }
            45 => {
                optgetname = LTRUE;
                mca_opt_toggle();
                return 2 as std::ffi::c_int;
            }
            _ => {}
        }
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn mca_opt_nonfirst_char(mut c: std::ffi::c_char) -> std::ffi::c_int {
    let mut p: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut oname: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut ambig: lbool = LFALSE;
    let mut was_curropt: *mut loption = 0 as *mut loption;
    if !curropt.is_null() {
        if is_erase_char(c) as u64 != 0 {
            return 1 as std::ffi::c_int;
        }
        if c as std::ffi::c_int != '\t' as i32 {
            return 2 as std::ffi::c_int;
        }
    }
    if cmd_char(c) == 1 as std::ffi::c_int {
        return 1 as std::ffi::c_int;
    }
    p = get_cmdbuf();
    if p.is_null()
        || *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int == '\0' as i32
    {
        return 2 as std::ffi::c_int;
    }
    opt_lower = (*p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
        >= 'a' as i32
        && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int <= 'z' as i32)
        as std::ffi::c_int as lbool;
    was_curropt = curropt;
    curropt = findopt_name(&mut p, &mut oname, &mut ambig);
    if !curropt.is_null() {
        if was_curropt.is_null() {
            cmd_reset();
            mca_opt_toggle();
            cmd_setstring(oname, (opt_lower as u64 == 0) as std::ffi::c_int as lbool);
        }
    } else if ambig as u64 == 0 {
        bell();
    }
    return 2 as std::ffi::c_int;
}
unsafe extern "C" fn mca_opt_char(mut c: std::ffi::c_char) -> std::ffi::c_int {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    if curropt.is_null() && cmdbuf_empty() as std::ffi::c_uint != 0 {
        let mut ret: std::ffi::c_int = mca_opt_first_char(c);
        if ret != 0 as std::ffi::c_int {
            return ret;
        }
    }
    if optgetname as u64 != 0 {
        if is_newline_char(c) as u64 == 0 && c as std::ffi::c_int != '=' as i32 {
            return mca_opt_nonfirst_char(c);
        }
        if curropt.is_null() {
            parg.p_string = get_cmdbuf();
            if (parg.p_string).is_null() {
                return 2 as std::ffi::c_int;
            }
            error(
                b"There is no --%s option\0" as *const u8 as *const std::ffi::c_char,
                &mut parg,
            );
            return 1 as std::ffi::c_int;
        }
        optgetname = LFALSE;
        cmd_reset();
    } else {
        if is_erase_char(c) as u64 != 0 {
            return 0 as std::ffi::c_int;
        }
        if !curropt.is_null() {
            return 0 as std::ffi::c_int;
        }
        curropt = findopt(c as std::ffi::c_int);
        if curropt.is_null() {
            parg.p_string = propt(c);
            error(
                b"There is no %s option\0" as *const u8 as *const std::ffi::c_char,
                &mut parg,
            );
            return 1 as std::ffi::c_int;
        }
        opt_lower = (c as std::ffi::c_int >= 'a' as i32
            && c as std::ffi::c_int <= 'z' as i32) as std::ffi::c_int as lbool;
    }
    if optflag & !(0o100 as std::ffi::c_int) != 1 as std::ffi::c_int
        || opt_has_param(curropt) == 0
    {
        toggle_option(
            curropt,
            opt_lower,
            b"\0" as *const u8 as *const std::ffi::c_char,
            optflag,
        );
        return 1 as std::ffi::c_int;
    }
    start_mca(
        47 as std::ffi::c_int,
        opt_prompt(curropt),
        0 as *mut std::ffi::c_void,
        (1 as std::ffi::c_int) << 1 as std::ffi::c_int,
    );
    return 2 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn norm_search_type(mut st: std::ffi::c_int) -> std::ffi::c_int {
    if st
        & ((1 as std::ffi::c_int) << 9 as std::ffi::c_int
            | (1 as std::ffi::c_int) << 15 as std::ffi::c_int)
        == (1 as std::ffi::c_int) << 9 as std::ffi::c_int
            | (1 as std::ffi::c_int) << 15 as std::ffi::c_int
    {
        st ^= (1 as std::ffi::c_int) << 9 as std::ffi::c_int;
    }
    return st;
}
unsafe extern "C" fn mca_search_char(mut c: std::ffi::c_char) -> std::ffi::c_int {
    let mut flag: std::ffi::c_int = 0 as std::ffi::c_int;
    if cmdbuf_empty() as u64 == 0 || literal_char as std::ffi::c_uint != 0 {
        literal_char = LFALSE;
        return 0 as std::ffi::c_int;
    }
    match c as std::ffi::c_int {
        5 | 42 => {
            if mca != 55 as std::ffi::c_int {
                flag = (1 as std::ffi::c_int) << 9 as std::ffi::c_int;
            }
            search_type &= !((1 as std::ffi::c_int) << 15 as std::ffi::c_int);
        }
        6 | 64 => {
            if mca != 55 as std::ffi::c_int {
                flag = (1 as std::ffi::c_int) << 10 as std::ffi::c_int;
            }
        }
        11 => {
            if mca != 55 as std::ffi::c_int {
                flag = (1 as std::ffi::c_int) << 2 as std::ffi::c_int;
            }
        }
        19 => {
            let mut buf: [std::ffi::c_char; 35] = [0; 35];
            snprintf(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[std::ffi::c_char; 35]>() as std::ffi::c_ulong,
                b"Sub-pattern (1-%d):\0" as *const u8 as *const std::ffi::c_char,
                16 as std::ffi::c_int - 10 as std::ffi::c_int - 1 as std::ffi::c_int,
            );
            clear_bot();
            cmd_putstr(buf.as_mut_ptr());
            flush();
            c = getcc();
            if c as std::ffi::c_int >= '1' as i32
                && c as std::ffi::c_int
                    <= '0' as i32
                        + (16 as std::ffi::c_int - 10 as std::ffi::c_int
                            - 1 as std::ffi::c_int)
            {
                flag = (1 as std::ffi::c_int)
                    << 17 as std::ffi::c_int + (c as std::ffi::c_int - '0' as i32);
            } else {
                flag = -(1 as std::ffi::c_int);
            }
        }
        23 => {
            if mca != 55 as std::ffi::c_int {
                flag = (1 as std::ffi::c_int) << 15 as std::ffi::c_int;
            }
        }
        18 => {
            flag = (1 as std::ffi::c_int) << 12 as std::ffi::c_int;
        }
        14 | 33 => {
            flag = (1 as std::ffi::c_int) << 8 as std::ffi::c_int;
        }
        12 => {
            literal_char = LTRUE;
            flag = -(1 as std::ffi::c_int);
        }
        _ => {}
    }
    if flag != 0 as std::ffi::c_int {
        if flag != -(1 as std::ffi::c_int) {
            search_type = norm_search_type(search_type ^ flag);
        }
        mca_search();
        return 2 as std::ffi::c_int;
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn mca_char(mut c: std::ffi::c_char) -> std::ffi::c_int {
    let mut ret: std::ffi::c_int = 0;
    match mca {
        0 => return 0 as std::ffi::c_int,
        105 => return 0 as std::ffi::c_int,
        6 => {
            if !(c as std::ffi::c_int >= '0' as i32 && c as std::ffi::c_int <= '9' as i32
                || c as std::ffi::c_int == '.' as i32)
            {
                match editchar(
                    c,
                    0o1 as std::ffi::c_int | 0o2 as std::ffi::c_int
                        | 0o4 as std::ffi::c_int | 0o10 as std::ffi::c_int,
                ) {
                    101 => return 2 as std::ffi::c_int,
                    100 => {
                        number = cmd_int(&mut fraction);
                        clear_mca();
                        cmd_accept();
                        return 0 as std::ffi::c_int;
                    }
                    _ => {}
                }
            }
        }
        47 => {
            ret = mca_opt_char(c);
            if ret != 0 as std::ffi::c_int {
                return ret;
            }
        }
        15 | 5 | 55 => {
            ret = mca_search_char(c);
            if ret != 0 as std::ffi::c_int {
                return ret;
            }
        }
        _ => {}
    }
    if is_newline_char(c) as u64 != 0 {
        if pasting as std::ffi::c_uint != 0 && no_paste != 0 {
            start_ignoring_input();
            return 2 as std::ffi::c_int;
        }
        exec_mca();
        return 1 as std::ffi::c_int;
    }
    if cmd_char(c) == 1 as std::ffi::c_int {
        return 1 as std::ffi::c_int;
    }
    match mca {
        35 | 36 => {
            if len_cmdbuf() >= 2 as std::ffi::c_int {
                exec_mca();
                return 1 as std::ffi::c_int;
            }
        }
        15 | 5 => {
            if incr_search != 0 {
                let mut st: std::ffi::c_int = search_type
                    & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                        | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                        | (1 as std::ffi::c_int) << 8 as std::ffi::c_int
                        | (1 as std::ffi::c_int) << 12 as std::ffi::c_int
                        | (1 as std::ffi::c_int) << 2 as std::ffi::c_int
                        | (1 as std::ffi::c_int) << 15 as std::ffi::c_int
                        | ((1 as std::ffi::c_int)
                            << 17 as std::ffi::c_int + 1 as std::ffi::c_int
                            | (1 as std::ffi::c_int)
                                << 17 as std::ffi::c_int + 2 as std::ffi::c_int
                            | (1 as std::ffi::c_int)
                                << 17 as std::ffi::c_int + 3 as std::ffi::c_int
                            | (1 as std::ffi::c_int)
                                << 17 as std::ffi::c_int + 4 as std::ffi::c_int
                            | (1 as std::ffi::c_int)
                                << 17 as std::ffi::c_int + 5 as std::ffi::c_int));
                let mut save_updown: ssize_t = 0;
                let mut pattern: *const std::ffi::c_char = get_cmdbuf();
                if pattern.is_null() {
                    return 2 as std::ffi::c_int;
                }
                save_updown = save_updown_match();
                cmd_exec();
                if *pattern as std::ffi::c_int == '\0' as i32 {
                    undo_search(LTRUE);
                } else {
                    no_poll = LTRUE;
                    if search(
                        st | (1 as std::ffi::c_int) << 3 as std::ffi::c_int,
                        pattern,
                        1 as std::ffi::c_int,
                    ) != 0 as std::ffi::c_int
                    {
                        undo_search(LTRUE);
                    }
                    no_poll = LFALSE;
                }
                if is_screen_trashed() != 0 || full_screen == 0 {
                    clear();
                    repaint();
                }
                mca_search1();
                restore_updown_match(save_updown);
                cmd_repaint(0 as *const std::ffi::c_char);
            }
        }
        _ => {}
    }
    return 2 as std::ffi::c_int;
}
unsafe extern "C" fn clear_buffers() {
    if ch_getflags() & 0o1 as std::ffi::c_int == 0 {
        return;
    }
    ch_flush();
    clr_linenum();
    clr_hilite();
    set_line_contig_pos(-(1 as std::ffi::c_int) as POSITION);
}
#[no_mangle]
pub unsafe extern "C" fn screen_trashed_num(mut trashed: std::ffi::c_int) {
    screen_trashed_value = trashed;
}
#[no_mangle]
pub unsafe extern "C" fn screen_trashed() {
    screen_trashed_num(1 as std::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn is_screen_trashed() -> std::ffi::c_int {
    return screen_trashed_value;
}
unsafe extern "C" fn make_display() {
    if full_screen == 0 && !(quit_if_one_screen != 0 && one_screen != 0) {
        clear();
    }
    if empty_screen() != 0 {
        if initial_scrpos.pos == -(1 as std::ffi::c_int) as POSITION {
            jump_loc(0 as std::ffi::c_int as POSITION, 1 as std::ffi::c_int);
        } else {
            jump_loc(initial_scrpos.pos, initial_scrpos.ln);
        }
    } else if is_screen_trashed() != 0 || full_screen == 0 {
        let mut save_top_scroll: std::ffi::c_int = top_scroll;
        let mut save_ignore_eoi: std::ffi::c_int = ignore_eoi;
        top_scroll = 1 as std::ffi::c_int;
        ignore_eoi = 0 as std::ffi::c_int;
        if is_screen_trashed() == 2 as std::ffi::c_int {
            reopen_curr_ifile();
            jump_forw();
        }
        repaint();
        top_scroll = save_top_scroll;
        ignore_eoi = save_ignore_eoi;
    }
}
unsafe extern "C" fn prompt() {
    let mut p: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    if !ungot.is_null() && (*ungot).ug_end_command as u64 == 0 {
        return;
    }
    make_display();
    bottompos = position(-(2 as std::ffi::c_int));
    if get_quit_at_eof() == 2 as std::ffi::c_int
        && eof_displayed(LFALSE) as std::ffi::c_uint != 0
        && ch_getflags() & 0o10 as std::ffi::c_int == 0
        && next_ifile(curr_ifile) == 0 as *mut std::ffi::c_void
    {
        quit(0 as std::ffi::c_int);
    }
    if quit_if_one_screen != 0 && entire_file_displayed() as std::ffi::c_uint != 0
        && ch_getflags() & 0o10 as std::ffi::c_int == 0
        && next_ifile(curr_ifile) == 0 as *mut std::ffi::c_void
    {
        quit(0 as std::ffi::c_int);
    }
    quit_if_one_screen = LFALSE as std::ffi::c_int;
    if forw_prompt == 0 {
        clear_bot();
    }
    clear_cmd();
    forw_prompt = 0 as std::ffi::c_int;
    p = pr_string();
    if is_filtering() as u64 != 0 {
        putstr(b"& \0" as *const u8 as *const std::ffi::c_char);
    }
    if search_wrapped as u64 != 0 {
        if search_type & (1 as std::ffi::c_int) << 1 as std::ffi::c_int != 0 {
            error(
                b"Search hit top; continuing at bottom\0" as *const u8
                    as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
        } else {
            error(
                b"Search hit bottom; continuing at top\0" as *const u8
                    as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
        }
        search_wrapped = LFALSE;
    }
    if !osc8_uri.is_null() {
        let mut parg: PARG = parg {
            p_string: 0 as *const std::ffi::c_char,
        };
        parg.p_string = osc8_uri;
        error(b"Link: %s\0" as *const u8 as *const std::ffi::c_char, &mut parg);
        free(osc8_uri as *mut std::ffi::c_void);
        osc8_uri = 0 as *mut std::ffi::c_char;
    }
    if p.is_null() || *p as std::ffi::c_int == '\0' as i32 {
        at_enter(0 as std::ffi::c_int | (7 as std::ffi::c_int) << 8 as std::ffi::c_int);
        putchr(':' as i32);
        at_exit();
    } else {
        load_line(p);
        put_line(LFALSE);
    }
    clear_eol();
}
#[no_mangle]
pub unsafe extern "C" fn dispversion() {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    parg.p_string = version.as_mut_ptr();
    error(b"less %s\0" as *const u8 as *const std::ffi::c_char, &mut parg);
}
unsafe extern "C" fn getcc_end_command() -> std::ffi::c_char {
    let mut ch: std::ffi::c_int = 0;
    match mca {
        6 => return 'g' as i32 as std::ffi::c_char,
        15 | 5 | 55 => return '\n' as i32 as std::ffi::c_char,
        _ => {
            if !ungot.is_null() {
                return '\0' as i32 as std::ffi::c_char;
            }
            ch = getchr();
            if ch < 0 as std::ffi::c_int {
                ch = '\0' as i32;
            }
            return ch as std::ffi::c_char;
        }
    };
}
unsafe extern "C" fn get_ungot(mut p_end_command: *mut lbool) -> std::ffi::c_char {
    let mut ug: *mut ungot = ungot;
    let mut c: std::ffi::c_char = (*ug).ug_char;
    if !p_end_command.is_null() {
        *p_end_command = (*ug).ug_end_command;
    }
    ungot = (*ug).ug_next;
    free(ug as *mut std::ffi::c_void);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn getcc_clear() {
    while !ungot.is_null() {
        get_ungot(0 as *mut lbool);
    }
}
unsafe extern "C" fn getccu() -> std::ffi::c_char {
    let mut c: std::ffi::c_int = 0 as std::ffi::c_int;
    while c == 0 as std::ffi::c_int && sigs == 0 as std::ffi::c_int {
        if ungot.is_null() {
            c = getchr();
            if c < 0 as std::ffi::c_int {
                c = '\0' as i32;
            }
        } else {
            let mut end_command: lbool = LFALSE;
            c = get_ungot(&mut end_command) as std::ffi::c_int;
            if end_command as u64 != 0 {
                c = getcc_end_command() as std::ffi::c_int;
            }
        }
    }
    return c as std::ffi::c_char;
}
unsafe extern "C" fn getcc_repl(
    mut orig: *const std::ffi::c_char,
    mut repl: *const std::ffi::c_char,
    mut gr_getc: Option::<unsafe extern "C" fn() -> std::ffi::c_char>,
    mut gr_ungetc: Option::<unsafe extern "C" fn(std::ffi::c_char) -> ()>,
) -> std::ffi::c_char {
    let mut c: std::ffi::c_char = 0;
    let mut keys: [std::ffi::c_char; 16] = [0; 16];
    let mut ki: size_t = 0 as std::ffi::c_int as size_t;
    c = (Some(gr_getc.expect("non-null function pointer")))
        .expect("non-null function pointer")();
    if orig.is_null()
        || *orig.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int == '\0' as i32
    {
        return c;
    }
    loop {
        keys[ki as usize] = c;
        if c as std::ffi::c_int != *orig.offset(ki as isize) as std::ffi::c_int
            || ki
                >= (::core::mem::size_of::<[std::ffi::c_char; 16]>()
                    as std::ffi::c_ulong)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
        {
            while ki > 0 as std::ffi::c_int as size_t {
                let fresh0 = ki;
                ki = ki.wrapping_sub(1);
                (Some(gr_ungetc.expect("non-null function pointer")))
                    .expect("non-null function pointer")(keys[fresh0 as usize]);
            }
            return keys[0 as std::ffi::c_int as usize];
        }
        ki = ki.wrapping_add(1);
        if *orig.offset(ki as isize) as std::ffi::c_int == '\0' as i32 {
            ki = (strlen(repl)).wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong);
            while ki > 0 as std::ffi::c_int as size_t {
                let fresh1 = ki;
                ki = ki.wrapping_sub(1);
                (Some(gr_ungetc.expect("non-null function pointer")))
                    .expect("non-null function pointer")(*repl.offset(fresh1 as isize));
            }
            return *repl.offset(0 as std::ffi::c_int as isize);
        }
        c = (Some(gr_getc.expect("non-null function pointer")))
            .expect("non-null function pointer")();
    };
}
#[no_mangle]
pub unsafe extern "C" fn getcc() -> std::ffi::c_char {
    return getcc_repl(
        kent,
        b"\n\0" as *const u8 as *const std::ffi::c_char,
        Some(getccu as unsafe extern "C" fn() -> std::ffi::c_char),
        Some(ungetcc as unsafe extern "C" fn(std::ffi::c_char) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ungetcc(mut c: std::ffi::c_char) {
    let mut ug: *mut ungot = ecalloc(
        1 as std::ffi::c_int as size_t,
        ::core::mem::size_of::<ungot>() as std::ffi::c_ulong,
    ) as *mut ungot;
    (*ug).ug_char = c;
    (*ug).ug_next = ungot;
    ungot = ug;
}
unsafe extern "C" fn ungetcc_back1(mut c: std::ffi::c_char, mut end_command: lbool) {
    let mut ug: *mut ungot = ecalloc(
        1 as std::ffi::c_int as size_t,
        ::core::mem::size_of::<ungot>() as std::ffi::c_ulong,
    ) as *mut ungot;
    (*ug).ug_char = c;
    (*ug).ug_end_command = end_command;
    (*ug).ug_next = 0 as *mut ungot;
    if ungot.is_null() {
        ungot = ug;
    } else {
        let mut pu: *mut ungot = 0 as *mut ungot;
        pu = ungot;
        while !((*pu).ug_next).is_null() {
            pu = (*pu).ug_next;
        }
        (*pu).ug_next = ug;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ungetcc_back(mut c: std::ffi::c_char) {
    ungetcc_back1(c, LFALSE);
}
#[no_mangle]
pub unsafe extern "C" fn ungetcc_end_command() {
    ungetcc_back1('\0' as i32 as std::ffi::c_char, LTRUE);
}
#[no_mangle]
pub unsafe extern "C" fn ungetsc(mut s: *const std::ffi::c_char) {
    while *s as std::ffi::c_int != '\0' as i32 {
        let fresh2 = s;
        s = s.offset(1);
        ungetcc_back(*fresh2);
    }
}
#[no_mangle]
pub unsafe extern "C" fn peekcc() -> std::ffi::c_char {
    let mut c: std::ffi::c_char = getcc();
    ungetcc(c);
    return c;
}
unsafe extern "C" fn multi_search(
    mut pattern: *const std::ffi::c_char,
    mut n: std::ffi::c_int,
    mut silent: std::ffi::c_int,
) {
    let mut nomore: std::ffi::c_int = 0;
    let mut save_ifile: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    let mut changed_file: lbool = LFALSE;
    changed_file = LFALSE;
    save_ifile = save_curr_ifile();
    if search_type
        & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
            | (1 as std::ffi::c_int) << 1 as std::ffi::c_int) == 0 as std::ffi::c_int
    {
        search_type |= (1 as std::ffi::c_int) << 0 as std::ffi::c_int;
    }
    if search_type & (1 as std::ffi::c_int) << 10 as std::ffi::c_int != 0 {
        if search_type & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
            nomore = edit_first();
        } else {
            nomore = edit_last();
        }
        if nomore != 0 {
            unsave_ifile(save_ifile);
            return;
        }
        changed_file = LTRUE;
        search_type &= !((1 as std::ffi::c_int) << 10 as std::ffi::c_int);
    }
    loop {
        n = search(search_type, pattern, n);
        search_type &= !((1 as std::ffi::c_int) << 2 as std::ffi::c_int);
        last_search_type = search_type;
        if n == 0 as std::ffi::c_int {
            unsave_ifile(save_ifile);
            return;
        }
        if n < 0 as std::ffi::c_int {
            break;
        } else {
            if search_type & (1 as std::ffi::c_int) << 9 as std::ffi::c_int
                == 0 as std::ffi::c_int
            {
                break;
            }
            if search_type & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
                nomore = edit_next(1 as std::ffi::c_int);
            } else {
                nomore = edit_prev(1 as std::ffi::c_int);
            }
            if nomore != 0 {
                break;
            }
            changed_file = LTRUE;
        }
    }
    if n > 0 as std::ffi::c_int && silent == 0 {
        error(
            b"Pattern not found\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
    }
    if changed_file as u64 != 0 {
        reedit_ifile(save_ifile);
    } else {
        unsave_ifile(save_ifile);
    };
}
unsafe extern "C" fn forw_loop(mut until_hilite: std::ffi::c_int) -> std::ffi::c_int {
    let mut curr_len: POSITION = 0;
    if ch_getflags() & 0o10 as std::ffi::c_int != 0 {
        return 101 as std::ffi::c_int;
    }
    cmd_exec();
    jump_forw_buffered();
    curr_len = ch_length();
    highest_hilite = if until_hilite != 0 {
        curr_len
    } else {
        -(1 as std::ffi::c_int) as POSITION
    };
    ignore_eoi = 1 as std::ffi::c_int;
    while sigs == 0 {
        if until_hilite != 0 && highest_hilite > curr_len {
            bell();
            break;
        } else {
            make_display();
            forward(1 as std::ffi::c_int, LFALSE, LFALSE, LFALSE);
        }
    }
    ignore_eoi = 0 as std::ffi::c_int;
    ch_set_eof();
    if sigs != 0
        && sigs
            & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 2 as std::ffi::c_int) == 0
    {
        return if until_hilite != 0 {
            56 as std::ffi::c_int
        } else {
            50 as std::ffi::c_int
        };
    }
    return 101 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn start_ignoring_input() {
    ignoring_input = LTRUE;
    ignoring_input_time = get_time();
}
#[no_mangle]
pub unsafe extern "C" fn stop_ignoring_input() {
    ignoring_input = LFALSE;
    pasting = LFALSE;
}
#[no_mangle]
pub unsafe extern "C" fn is_ignoring_input(mut action: std::ffi::c_int) -> lbool {
    if ignoring_input as u64 == 0 {
        return LFALSE;
    }
    if action == 76 as std::ffi::c_int {
        stop_ignoring_input();
    }
    if get_time() >= ignoring_input_time + 5 as std::ffi::c_int as time_t {
        stop_ignoring_input();
    }
    return (action != 105 as std::ffi::c_int) as std::ffi::c_int as lbool;
}
#[no_mangle]
pub unsafe extern "C" fn commands() {
    let mut current_block: u64;
    let mut c: std::ffi::c_char = 0;
    let mut action: std::ffi::c_int = 0;
    let mut cbuf: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut msg: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut newaction: std::ffi::c_int = 0;
    let mut save_jump_sline: std::ffi::c_int = 0;
    let mut save_search_type: std::ffi::c_int = 0;
    let mut extra: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    let mut old_ifile: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    let mut new_ifile: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    let mut tagfile: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    search_type = (1 as std::ffi::c_int) << 0 as std::ffi::c_int;
    wscroll = (sc_height + 1 as std::ffi::c_int) / 2 as std::ffi::c_int;
    newaction = 101 as std::ffi::c_int;
    's_39: loop {
        clear_mca();
        cmd_accept();
        number = 0 as std::ffi::c_int as LINENUM;
        curropt = 0 as *mut loption;
        if sigs != 0 {
            psignals();
            if quitting as u64 != 0 {
                quit(-(1 as std::ffi::c_int));
            }
        }
        check_winch();
        cmd_reset();
        prompt();
        if sigs != 0 {
            continue;
        }
        if newaction == 101 as std::ffi::c_int {
            c = getcc();
        }
        loop {
            if sigs != 0 {
                continue 's_39;
            }
            if newaction != 101 as std::ffi::c_int {
                action = newaction;
                newaction = 101 as std::ffi::c_int;
            } else {
                if mca != 0 {
                    match mca_char(c) {
                        2 => {
                            c = getcc();
                            continue;
                        }
                        1 => {
                            continue 's_39;
                        }
                        0 | _ => {}
                    }
                }
                extra = 0 as *const std::ffi::c_char;
                if mca != 0 {
                    if cmd_char(c) == 1 as std::ffi::c_int
                        || cmdbuf_empty() as std::ffi::c_uint != 0
                    {
                        continue 's_39;
                    }
                    cbuf = get_cmdbuf();
                    if cbuf.is_null() {
                        c = getcc();
                        continue;
                    } else {
                        action = fcmd_decode(cbuf, &mut extra);
                    }
                } else {
                    let tbuf: [std::ffi::c_char; 2] = [
                        c,
                        '\0' as i32 as std::ffi::c_char,
                    ];
                    action = fcmd_decode(tbuf.as_ptr(), &mut extra);
                }
                if !extra.is_null() {
                    ungetsc(extra);
                }
            }
            if action != 105 as std::ffi::c_int {
                cmd_reset();
            }
            if is_ignoring_input(action) as u64 != 0 {
                continue 's_39;
            }
            match action {
                75 => {
                    if no_paste != 0 {
                        start_ignoring_input();
                    }
                    continue 's_39;
                }
                6 => {
                    start_mca(
                        6 as std::ffi::c_int,
                        b":\0" as *const u8 as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void,
                        (1 as std::ffi::c_int) << 0 as std::ffi::c_int,
                    );
                }
                33 => {
                    if number > 0 as std::ffi::c_int as LINENUM {
                        swindow = number as std::ffi::c_int;
                    }
                    current_block = 3507267478320338004;
                    break;
                }
                13 => {
                    current_block = 3507267478320338004;
                    break;
                }
                34 => {
                    if number > 0 as std::ffi::c_int as LINENUM {
                        swindow = number as std::ffi::c_int;
                    }
                    current_block = 2194593563755971021;
                    break;
                }
                3 => {
                    current_block = 2194593563755971021;
                    break;
                }
                12 | 60 => {
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    cmd_exec();
                    if show_attn == 2 as std::ffi::c_int
                        && number > 1 as std::ffi::c_int as LINENUM
                    {
                        set_attnpos(bottompos);
                    }
                    forward(
                        number as std::ffi::c_int,
                        LFALSE,
                        LFALSE,
                        (action == 60 as std::ffi::c_int && chopline == 0)
                            as std::ffi::c_int as lbool,
                    );
                    continue 's_39;
                }
                2 | 61 => {
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    cmd_exec();
                    backward(
                        number as std::ffi::c_int,
                        LFALSE,
                        LFALSE,
                        (action == 61 as std::ffi::c_int && chopline == 0)
                            as std::ffi::c_int as lbool,
                    );
                    continue 's_39;
                }
                66 => {
                    cmd_exec();
                    forward(wheel_lines, LFALSE, LFALSE, LFALSE);
                    continue 's_39;
                }
                67 => {
                    cmd_exec();
                    backward(wheel_lines, LFALSE, LFALSE, LFALSE);
                    continue 's_39;
                }
                29 => {
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    cmd_exec();
                    if show_attn == 2 as std::ffi::c_int
                        && number > 1 as std::ffi::c_int as LINENUM
                    {
                        set_attnpos(bottompos);
                    }
                    forward(number as std::ffi::c_int, LTRUE, LFALSE, LFALSE);
                    continue 's_39;
                }
                30 => {
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    cmd_exec();
                    backward(number as std::ffi::c_int, LTRUE, LFALSE, LFALSE);
                    continue 's_39;
                }
                40 => {
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = get_swindow() as LINENUM;
                    }
                    cmd_exec();
                    if show_attn == 2 as std::ffi::c_int {
                        set_attnpos(bottompos);
                    }
                    forward(number as std::ffi::c_int, LTRUE, LFALSE, LFALSE);
                    continue 's_39;
                }
                22 => {
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = get_swindow() as LINENUM;
                    }
                    cmd_exec();
                    backward(number as std::ffi::c_int, LTRUE, LFALSE, LFALSE);
                    continue 's_39;
                }
                50 => {
                    if !(get_altfilename(curr_ifile)).is_null() {
                        error(
                            b"Warning: command may not work correctly when file is viewed via LESSOPEN\0"
                                as *const u8 as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void as *mut PARG,
                        );
                    }
                    if show_attn != 0 {
                        set_attnpos(bottompos);
                    }
                    newaction = forw_loop(0 as std::ffi::c_int);
                    continue 's_39;
                }
                56 => {
                    newaction = forw_loop(1 as std::ffi::c_int);
                    continue 's_39;
                }
                14 => {
                    if number > 0 as std::ffi::c_int as LINENUM {
                        wscroll = number as std::ffi::c_int;
                    }
                    cmd_exec();
                    if show_attn == 2 as std::ffi::c_int {
                        set_attnpos(bottompos);
                    }
                    forward(wscroll, LFALSE, LFALSE, LFALSE);
                    continue 's_39;
                }
                4 => {
                    if number > 0 as std::ffi::c_int as LINENUM {
                        wscroll = number as std::ffi::c_int;
                    }
                    cmd_exec();
                    backward(wscroll, LFALSE, LFALSE, LFALSE);
                    continue 's_39;
                }
                11 => {
                    clear_buffers();
                    current_block = 12373568287479140350;
                    break;
                }
                25 => {
                    current_block = 12373568287479140350;
                    break;
                }
                17 => {
                    save_jump_sline = jump_sline;
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                        jump_sline = 0 as std::ffi::c_int;
                    }
                    cmd_exec();
                    jump_back(number);
                    jump_sline = save_jump_sline;
                    continue 's_39;
                }
                21 => {
                    if number < 0 as std::ffi::c_int as LINENUM {
                        number = 0 as std::ffi::c_int as LINENUM;
                        fraction = 0 as std::ffi::c_int as std::ffi::c_long;
                    }
                    if number > 100 as std::ffi::c_int as LINENUM
                        || number == 100 as std::ffi::c_int as LINENUM
                            && fraction != 0 as std::ffi::c_int as std::ffi::c_long
                    {
                        number = 100 as std::ffi::c_int as LINENUM;
                        fraction = 0 as std::ffi::c_int as std::ffi::c_long;
                    }
                    cmd_exec();
                    jump_percent(number as std::ffi::c_int, fraction);
                    continue 's_39;
                }
                16 => {
                    cmd_exec();
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        jump_forw();
                    } else {
                        jump_back(number);
                    }
                    continue 's_39;
                }
                57 => {
                    cmd_exec();
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        jump_forw_buffered();
                    } else {
                        jump_back(number);
                    }
                    continue 's_39;
                }
                51 => {
                    cmd_exec();
                    if number < 0 as std::ffi::c_int as LINENUM {
                        number = 0 as std::ffi::c_int as LINENUM;
                    }
                    jump_line_loc(number, jump_sline);
                    continue 's_39;
                }
                28 => {
                    if ch_getflags() & 0o10 as std::ffi::c_int != 0 {
                        continue 's_39;
                    }
                    cmd_exec();
                    parg.p_string = eq_message();
                    error(b"%s\0" as *const u8 as *const std::ffi::c_char, &mut parg);
                    continue 's_39;
                }
                31 => {
                    cmd_exec();
                    dispversion();
                    continue 's_39;
                }
                24 => {
                    if curr_ifile != 0 as *mut std::ffi::c_void
                        && ch_getflags() & 0o10 as std::ffi::c_int != 0
                    {
                        current_block = 5431927413890720344;
                        break;
                    } else {
                        current_block = 16974974966130203269;
                        break;
                    }
                }
                15 => {
                    search_type = (1 as std::ffi::c_int) << 0 as std::ffi::c_int
                        | def_search_type;
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    literal_char = LFALSE;
                    mca_search();
                    c = getcc();
                }
                5 => {
                    search_type = (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                        | def_search_type;
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    literal_char = LFALSE;
                    mca_search();
                    c = getcc();
                }
                71 => {
                    cmd_exec();
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    osc8_search(
                        (1 as std::ffi::c_int) << 0 as std::ffi::c_int,
                        0 as *const std::ffi::c_char,
                        number as std::ffi::c_int,
                    );
                    continue 's_39;
                }
                72 => {
                    cmd_exec();
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    osc8_search(
                        (1 as std::ffi::c_int) << 1 as std::ffi::c_int,
                        0 as *const std::ffi::c_char,
                        number as std::ffi::c_int,
                    );
                    continue 's_39;
                }
                73 => {
                    if secure_allow((1 as std::ffi::c_int) << 12 as std::ffi::c_int) != 0
                    {
                        current_block = 6662862405959679103;
                        break;
                    } else {
                        current_block = 2089914658669629659;
                        break;
                    }
                }
                74 => {
                    cmd_exec();
                    osc8_jump();
                    continue 's_39;
                }
                55 => {
                    search_type = (1 as std::ffi::c_int) << 0 as std::ffi::c_int
                        | (1 as std::ffi::c_int) << 13 as std::ffi::c_int;
                    literal_char = LFALSE;
                    mca_search();
                    c = getcc();
                }
                43 => {
                    search_type = last_search_type;
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    mca_search();
                    cmd_exec();
                    multi_search(
                        0 as *const std::ffi::c_char,
                        number as std::ffi::c_int,
                        0 as std::ffi::c_int,
                    );
                    continue 's_39;
                }
                44 => {
                    search_type = last_search_type
                        | (1 as std::ffi::c_int) << 9 as std::ffi::c_int;
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    mca_search();
                    cmd_exec();
                    multi_search(
                        0 as *const std::ffi::c_char,
                        number as std::ffi::c_int,
                        0 as std::ffi::c_int,
                    );
                    continue 's_39;
                }
                45 => {
                    search_type = last_search_type;
                    save_search_type = search_type;
                    search_type = if search_type
                        & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0
                    {
                        search_type & !((1 as std::ffi::c_int) << 0 as std::ffi::c_int)
                            | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                    } else {
                        search_type & !((1 as std::ffi::c_int) << 1 as std::ffi::c_int)
                            | (1 as std::ffi::c_int) << 0 as std::ffi::c_int
                    };
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    mca_search();
                    cmd_exec();
                    multi_search(
                        0 as *const std::ffi::c_char,
                        number as std::ffi::c_int,
                        0 as std::ffi::c_int,
                    );
                    last_search_type = save_search_type;
                    continue 's_39;
                }
                46 => {
                    search_type = last_search_type;
                    save_search_type = search_type;
                    search_type = (if search_type
                        & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0
                    {
                        search_type & !((1 as std::ffi::c_int) << 0 as std::ffi::c_int)
                            | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                    } else {
                        search_type & !((1 as std::ffi::c_int) << 1 as std::ffi::c_int)
                            | (1 as std::ffi::c_int) << 0 as std::ffi::c_int
                    }) | (1 as std::ffi::c_int) << 9 as std::ffi::c_int;
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    mca_search();
                    cmd_exec();
                    multi_search(
                        0 as *const std::ffi::c_char,
                        number as std::ffi::c_int,
                        0 as std::ffi::c_int,
                    );
                    last_search_type = save_search_type;
                    continue 's_39;
                }
                39 | 70 => {
                    undo_search(
                        (action == 70 as std::ffi::c_int) as std::ffi::c_int as lbool,
                    );
                    continue 's_39;
                }
                19 => {
                    if ch_getflags() & 0o10 as std::ffi::c_int != 0 {
                        continue 's_39;
                    }
                    cmd_exec();
                    save_hshift = hshift;
                    hshift = 0 as std::ffi::c_int;
                    save_bs_mode = bs_mode;
                    bs_mode = 0 as std::ffi::c_int;
                    save_proc_backspace = proc_backspace;
                    proc_backspace = 0 as std::ffi::c_int;
                    edit(
                        b"@/\\less/\\help/\\file/\\@\0" as *const u8
                            as *const std::ffi::c_char,
                    );
                    continue 's_39;
                }
                9 => {
                    if secure_allow((1 as std::ffi::c_int) << 2 as std::ffi::c_int) != 0
                    {
                        start_mca(
                            9 as std::ffi::c_int,
                            b"Examine: \0" as *const u8 as *const std::ffi::c_char,
                            ml_examine,
                            0 as std::ffi::c_int,
                        );
                        c = getcc();
                    } else {
                        error(
                            b"Command not available\0" as *const u8
                                as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void as *mut PARG,
                        );
                        continue 's_39;
                    }
                }
                32 => {
                    if secure_allow((1 as std::ffi::c_int) << 1 as std::ffi::c_int) != 0
                    {
                        current_block = 16718638665978159145;
                        break;
                    } else {
                        current_block = 8120009455218959897;
                        break;
                    }
                }
                20 => {
                    if ntags() != 0 {
                        current_block = 18001984906674336099;
                        break;
                    } else {
                        current_block = 14148461183130080616;
                        break;
                    }
                }
                23 => {
                    if ntags() != 0 {
                        current_block = 8193737063574930042;
                        break;
                    } else {
                        current_block = 3818209998506676277;
                        break;
                    }
                }
                53 => {
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    tagfile = nexttag(number as std::ffi::c_int);
                    if tagfile.is_null() {
                        current_block = 8075351136037156718;
                        break;
                    } else {
                        current_block = 5798072534372498777;
                        break;
                    }
                }
                54 => {
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    tagfile = prevtag(number as std::ffi::c_int);
                    if tagfile.is_null() {
                        current_block = 10029375464402185584;
                        break;
                    } else {
                        current_block = 13267105165099174640;
                        break;
                    }
                }
                38 => {
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    cmd_exec();
                    if edit_index(number as std::ffi::c_int) != 0 {
                        error(
                            b"No such file\0" as *const u8 as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void as *mut PARG,
                        );
                    }
                    continue 's_39;
                }
                52 => {
                    if ch_getflags() & 0o10 as std::ffi::c_int != 0 {
                        continue 's_39;
                    }
                    old_ifile = curr_ifile;
                    new_ifile = getoff_ifile(curr_ifile);
                    cmd_exec();
                    if new_ifile == 0 as *mut std::ffi::c_void {
                        current_block = 18330534242458572360;
                        break;
                    } else {
                        current_block = 18361011714114715771;
                        break;
                    }
                }
                47 => {
                    optflag = 1 as std::ffi::c_int;
                    optgetname = LFALSE;
                    mca_opt_toggle();
                    c = getcc();
                    msg = opt_toggle_disallowed(c as std::ffi::c_int);
                    if msg.is_null() {
                        continue;
                    }
                    error(msg, 0 as *mut std::ffi::c_void as *mut PARG);
                    continue 's_39;
                }
                7 => {
                    optflag = 0 as std::ffi::c_int;
                    optgetname = LFALSE;
                    mca_opt_toggle();
                    c = getcc();
                }
                10 => {
                    start_mca(
                        10 as std::ffi::c_int,
                        b"+\0" as *const u8 as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void,
                        0 as std::ffi::c_int,
                    );
                    c = getcc();
                }
                27 | 69 => {
                    if secure_allow((1 as std::ffi::c_int) << 9 as std::ffi::c_int) != 0
                    {
                        start_mca(
                            action,
                            if action == 27 as std::ffi::c_int {
                                b"!\0" as *const u8 as *const std::ffi::c_char
                            } else {
                                b"#\0" as *const u8 as *const std::ffi::c_char
                            },
                            ml_shell,
                            0 as std::ffi::c_int,
                        );
                        c = getcc();
                    } else {
                        error(
                            b"Command not available\0" as *const u8
                                as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void as *mut PARG,
                        );
                        continue 's_39;
                    }
                }
                26 | 63 => {
                    if ch_getflags() & 0o10 as std::ffi::c_int != 0 {
                        current_block = 7991679940794782184;
                        break;
                    } else {
                        current_block = 2595745308905254098;
                        break;
                    }
                }
                62 => {
                    start_mca(
                        62 as std::ffi::c_int,
                        b"clear mark: \0" as *const u8 as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void,
                        0 as std::ffi::c_int,
                    );
                    c = getcc();
                    if is_erase_char(c) as std::ffi::c_uint != 0
                        || is_newline_char(c) as std::ffi::c_uint != 0
                    {
                        continue 's_39;
                    }
                    clrmark(c);
                    repaint();
                    continue 's_39;
                }
                18 => {
                    start_mca(
                        18 as std::ffi::c_int,
                        b"goto mark: \0" as *const u8 as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void,
                        0 as std::ffi::c_int,
                    );
                    c = getcc();
                    if is_erase_char(c) as std::ffi::c_uint != 0
                        || is_newline_char(c) as std::ffi::c_uint != 0
                    {
                        continue 's_39;
                    }
                    cmd_exec();
                    gomark(c);
                    continue 's_39;
                }
                37 => {
                    if secure_allow((1 as std::ffi::c_int) << 8 as std::ffi::c_int) != 0
                    {
                        start_mca(
                            37 as std::ffi::c_int,
                            b"|mark: \0" as *const u8 as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void,
                            0 as std::ffi::c_int,
                        );
                        c = getcc();
                        if is_erase_char(c) as u64 != 0 {
                            continue 's_39;
                        }
                        if is_newline_char(c) as u64 != 0 {
                            c = '.' as i32 as std::ffi::c_char;
                        }
                        if badmark(c) != 0 {
                            continue 's_39;
                        }
                        pipec = c;
                        start_mca(
                            37 as std::ffi::c_int,
                            b"!\0" as *const u8 as *const std::ffi::c_char,
                            ml_shell,
                            0 as std::ffi::c_int,
                        );
                        c = getcc();
                    } else {
                        error(
                            b"Command not available\0" as *const u8
                                as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void as *mut PARG,
                        );
                        continue 's_39;
                    }
                }
                36 | 35 => {
                    start_mca(
                        action,
                        b"Brackets: \0" as *const u8 as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void,
                        0 as std::ffi::c_int,
                    );
                    c = getcc();
                }
                41 => {
                    if number > 0 as std::ffi::c_int as LINENUM {
                        shift_count = number as std::ffi::c_int;
                    } else {
                        number = (if shift_count > 0 as std::ffi::c_int {
                            shift_count
                        } else {
                            sc_width / 2 as std::ffi::c_int
                        }) as LINENUM;
                    }
                    if number > hshift as LINENUM {
                        number = hshift as LINENUM;
                    }
                    pos_rehead();
                    hshift -= number as std::ffi::c_int;
                    screen_trashed();
                    continue 's_39;
                }
                42 => {
                    if number > 0 as std::ffi::c_int as LINENUM {
                        shift_count = number as std::ffi::c_int;
                    } else {
                        number = (if shift_count > 0 as std::ffi::c_int {
                            shift_count
                        } else {
                            sc_width / 2 as std::ffi::c_int
                        }) as LINENUM;
                    }
                    pos_rehead();
                    hshift += number as std::ffi::c_int;
                    screen_trashed();
                    continue 's_39;
                }
                58 => {
                    pos_rehead();
                    hshift = 0 as std::ffi::c_int;
                    screen_trashed();
                    continue 's_39;
                }
                59 => {
                    pos_rehead();
                    hshift = rrshift();
                    screen_trashed();
                    continue 's_39;
                }
                105 => {
                    if mca != 105 as std::ffi::c_int {
                        cmd_reset();
                        start_mca(
                            105 as std::ffi::c_int,
                            b" \0" as *const u8 as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void,
                            (1 as std::ffi::c_int) << 0 as std::ffi::c_int,
                        );
                        cmd_char(c);
                    }
                    c = getcc();
                }
                101 => {
                    continue 's_39;
                }
                _ => {
                    bell();
                    continue 's_39;
                }
            }
        }
        match current_block {
            16718638665978159145 => {
                if ch_getflags() & 0o10 as std::ffi::c_int != 0 {
                    continue;
                }
                if strcmp(
                    get_filename(curr_ifile),
                    b"-\0" as *const u8 as *const std::ffi::c_char,
                ) == 0 as std::ffi::c_int
                {
                    error(
                        b"Cannot edit standard input\0" as *const u8
                            as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void as *mut PARG,
                    );
                    continue;
                } else {
                    if no_edit_warn == 0 && !(get_altfilename(curr_ifile)).is_null() {
                        error(
                            b"WARNING: This file was viewed via LESSOPEN\0" as *const u8
                                as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void as *mut PARG,
                        );
                    }
                    start_mca(
                        27 as std::ffi::c_int,
                        b"!\0" as *const u8 as *const std::ffi::c_char,
                        ml_shell,
                        0 as std::ffi::c_int,
                    );
                    make_display();
                    cmd_exec();
                    lsystem(pr_expand(editproto), 0 as *const std::ffi::c_char);
                    continue;
                }
            }
            18361011714114715771 => {
                if edit_ifile(new_ifile) != 0 as std::ffi::c_int {
                    reedit_ifile(old_ifile);
                    continue;
                } else {
                    del_ifile(old_ifile);
                    continue;
                }
            }
            2595745308905254098 => {
                start_mca(
                    26 as std::ffi::c_int,
                    b"set mark: \0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void,
                    0 as std::ffi::c_int,
                );
                c = getcc();
                if is_erase_char(c) as std::ffi::c_uint != 0
                    || is_newline_char(c) as std::ffi::c_uint != 0
                {
                    continue;
                }
                setmark(
                    c,
                    if action == 63 as std::ffi::c_int {
                        -(1 as std::ffi::c_int)
                    } else {
                        0 as std::ffi::c_int
                    },
                );
                repaint();
                continue;
            }
            14148461183130080616 => {
                if number <= 0 as std::ffi::c_int as LINENUM {
                    number = 1 as std::ffi::c_int as LINENUM;
                }
                cmd_exec();
                if edit_next(number as std::ffi::c_int) != 0 {
                    if get_quit_at_eof() != 0
                        && eof_displayed(LFALSE) as std::ffi::c_uint != 0
                        && ch_getflags() & 0o10 as std::ffi::c_int == 0
                    {
                        quit(0 as std::ffi::c_int);
                    }
                    parg
                        .p_string = if number > 1 as std::ffi::c_int as LINENUM {
                        b"(N-th) \0" as *const u8 as *const std::ffi::c_char
                    } else {
                        b"\0" as *const u8 as *const std::ffi::c_char
                    };
                    error(
                        b"No %snext file\0" as *const u8 as *const std::ffi::c_char,
                        &mut parg,
                    );
                }
                continue;
            }
            2194593563755971021 => {
                if number <= 0 as std::ffi::c_int as LINENUM {
                    number = get_swindow() as LINENUM;
                }
                cmd_exec();
                backward(number as std::ffi::c_int, LFALSE, LTRUE, LFALSE);
                continue;
            }
            6662862405959679103 => {
                cmd_exec();
                osc8_open();
                continue;
            }
            3507267478320338004 => {
                if number <= 0 as std::ffi::c_int as LINENUM {
                    number = get_swindow() as LINENUM;
                }
                cmd_exec();
                if show_attn != 0 {
                    set_attnpos(bottompos);
                }
                forward(number as std::ffi::c_int, LFALSE, LTRUE, LFALSE);
                continue;
            }
            3818209998506676277 => {
                if number <= 0 as std::ffi::c_int as LINENUM {
                    number = 1 as std::ffi::c_int as LINENUM;
                }
                cmd_exec();
                if edit_prev(number as std::ffi::c_int) != 0 {
                    parg
                        .p_string = if number > 1 as std::ffi::c_int as LINENUM {
                        b"(N-th) \0" as *const u8 as *const std::ffi::c_char
                    } else {
                        b"\0" as *const u8 as *const std::ffi::c_char
                    };
                    error(
                        b"No %sprevious file\0" as *const u8 as *const std::ffi::c_char,
                        &mut parg,
                    );
                }
                continue;
            }
            5431927413890720344 => {
                hshift = save_hshift;
                bs_mode = save_bs_mode;
                proc_backspace = save_proc_backspace;
                if edit_prev(1 as std::ffi::c_int) == 0 as std::ffi::c_int {
                    continue;
                }
            }
            12373568287479140350 => {
                cmd_exec();
                repaint();
                continue;
            }
            5798072534372498777 => {
                cmd_exec();
                if edit(tagfile) == 0 as std::ffi::c_int {
                    let mut pos: POSITION = tagsearch();
                    if pos != -(1 as std::ffi::c_int) as POSITION {
                        jump_loc(pos, jump_sline);
                    }
                }
                continue;
            }
            13267105165099174640 => {
                cmd_exec();
                if edit(tagfile) == 0 as std::ffi::c_int {
                    let mut pos_0: POSITION = tagsearch();
                    if pos_0 != -(1 as std::ffi::c_int) as POSITION {
                        jump_loc(pos_0, jump_sline);
                    }
                }
                continue;
            }
            2089914658669629659 => {
                error(
                    b"Command not available\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                continue;
            }
            18001984906674336099 => {
                error(
                    b"No next file\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                continue;
            }
            8193737063574930042 => {
                error(
                    b"No previous file\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                continue;
            }
            8075351136037156718 => {
                error(
                    b"No next tag\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                continue;
            }
            18330534242458572360 => {
                bell();
                continue;
            }
            7991679940794782184 => {
                if !ungot.is_null() {
                    getcc();
                }
                continue;
            }
            10029375464402185584 => {
                error(
                    b"No previous tag\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                continue;
            }
            8120009455218959897 => {
                error(
                    b"Command not available\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                continue;
            }
            _ => {}
        }
        if !extra.is_null() {
            quit(*extra as std::ffi::c_int);
        }
        quit(0 as std::ffi::c_int);
    };
}
