use ::c2rust_bitfields;
use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type mlist;
    fn pclose(__stream: *mut FILE) -> std::ffi::c_int;
    fn open(__file: *const std::ffi::c_char, __oflag: std::ffi::c_int, _: ...) -> std::ffi::c_int;
    fn creat(__file: *const std::ffi::c_char, __mode: mode_t) -> std::ffi::c_int;
    fn lseek(__fd: std::ffi::c_int, __offset: __off_t, __whence: std::ffi::c_int) -> __off_t;
    fn close(__fd: std::ffi::c_int) -> std::ffi::c_int;
    fn isatty(__fd: std::ffi::c_int) -> std::ffi::c_int;
    fn free(_: *mut std::ffi::c_void);
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strncmp(
        _: *const std::ffi::c_char,
        _: *const std::ffi::c_char,
        _: std::ffi::c_ulong,
    ) -> std::ffi::c_int;
    fn strstr(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn skipsp(s: *mut std::ffi::c_char) -> *mut std::ffi::c_char;
    fn skipspc(s: *const std::ffi::c_char) -> *const std::ffi::c_char;
    fn quit(status: std::ffi::c_int);
    fn end_logfile();
    fn ch_length() -> POSITION;
    fn ch_forw_get() -> std::ffi::c_int;
    fn ch_init(f: std::ffi::c_int, flags: std::ffi::c_int, nread: ssize_t);
    fn ch_close();
    fn ch_getflags() -> std::ffi::c_int;
    fn cmd_addhist(mlist: *mut mlist, cmd: *const std::ffi::c_char, modified: lbool);
    fn ungetcc_end_command();
    fn ungetsc(s: *const std::ffi::c_char);
    fn shell_unquote(str: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn get_meta_escape() -> *const std::ffi::c_char;
    fn shell_quote(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn bin_file(f: std::ffi::c_int, n: *mut ssize_t) -> std::ffi::c_int;
    fn lglob(afilename: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn open_altfile(
        filename: *const std::ffi::c_char,
        pf: *mut std::ffi::c_int,
        pfd: *mut *mut std::ffi::c_void,
    ) -> *mut std::ffi::c_char;
    fn close_altfile(altfilename: *const std::ffi::c_char, filename: *const std::ffi::c_char);
    fn bad_file(filename: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn del_ifile(h: *mut std::ffi::c_void);
    fn next_ifile(h: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn prev_ifile(h: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn nifile() -> std::ffi::c_int;
    fn get_ifile(
        filename: *const std::ffi::c_char,
        prev: *mut std::ffi::c_void,
    ) -> *mut std::ffi::c_void;
    fn get_filename(ifile: *mut std::ffi::c_void) -> *const std::ffi::c_char;
    fn get_index(ifile: *mut std::ffi::c_void) -> std::ffi::c_int;
    fn store_pos(ifile: *mut std::ffi::c_void, scrpos: *mut scrpos);
    fn get_pos(ifile: *mut std::ffi::c_void, scrpos: *mut scrpos);
    fn set_open(ifile: *mut std::ffi::c_void);
    fn opened(ifile: *mut std::ffi::c_void) -> std::ffi::c_int;
    fn hold_ifile(ifile: *mut std::ffi::c_void, incr: std::ffi::c_int);
    fn held_ifile(ifile: *mut std::ffi::c_void) -> std::ffi::c_int;
    fn set_altpipe(ifile: *mut std::ffi::c_void, p: *mut std::ffi::c_void);
    fn get_altpipe(ifile: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn set_altfilename(ifile: *mut std::ffi::c_void, altfilename: *mut std::ffi::c_char);
    fn get_altfilename(ifile: *mut std::ffi::c_void) -> *mut std::ffi::c_char;
    fn forw_raw_line(
        curr_pos: POSITION,
        linep: *mut *const std::ffi::c_char,
        line_lenp: *mut size_t,
    ) -> POSITION;
    fn clr_linenum();
    fn scan_eof();
    fn lastmark();
    fn set_tabs(s: *const std::ffi::c_char, len: size_t);
    fn iopen(filename: *const std::ffi::c_char, flags: std::ffi::c_int) -> std::ffi::c_int;
    fn errno_message(filename: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn signal_message(sig: std::ffi::c_int) -> *const std::ffi::c_char;
    fn flush();
    fn putchr(ch: std::ffi::c_int) -> std::ffi::c_int;
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn query(fmt: *const std::ffi::c_char, parg: *mut PARG) -> std::ffi::c_int;
    fn pos_clear();
    fn get_scrpos(scrpos: *mut scrpos, where_0: std::ffi::c_int);
    fn undo_osc8();
    fn clr_hilite();
    fn set_header(pos: POSITION);
    fn stat(__file: *const std::ffi::c_char, __buf: *mut stat) -> std::ffi::c_int;
    static mut new_file: lbool;
    static mut every_first_cmd: *mut std::ffi::c_char;
    static mut force_open: std::ffi::c_int;
    static mut is_tty: std::ffi::c_int;
    static mut sigs: std::ffi::c_int;
    static mut hshift: std::ffi::c_int;
    static mut want_filesize: std::ffi::c_int;
    static mut consecutive_nulls: std::ffi::c_int;
    static mut modelines: std::ffi::c_int;
    static mut show_preproc_error: std::ffi::c_int;
    static mut curr_ifile: *mut std::ffi::c_void;
    static mut old_ifile: *mut std::ffi::c_void;
    static mut initial_scrpos: scrpos;
    static mut ml_examine: *mut std::ffi::c_void;
    static mut soft_eof: POSITION;
    static mut openquote: std::ffi::c_char;
    static mut closequote: std::ffi::c_char;
    static mut logfile: std::ffi::c_int;
    static mut force_logfile: std::ffi::c_int;
    static mut namelogfile: *mut std::ffi::c_char;
}
pub type __dev_t = std::ffi::c_ulong;
pub type __uid_t = std::ffi::c_uint;
pub type __gid_t = std::ffi::c_uint;
pub type __ino_t = std::ffi::c_ulong;
pub type __mode_t = std::ffi::c_uint;
pub type __nlink_t = std::ffi::c_ulong;
pub type __off_t = std::ffi::c_long;
pub type __off64_t = std::ffi::c_long;
pub type __time_t = std::ffi::c_long;
pub type __blksize_t = std::ffi::c_long;
pub type __blkcnt_t = std::ffi::c_long;
pub type __ssize_t = std::ffi::c_long;
pub type __syscall_slong_t = std::ffi::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = std::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::ffi::c_int,
    pub _IO_read_ptr: *mut std::ffi::c_char,
    pub _IO_read_end: *mut std::ffi::c_char,
    pub _IO_read_base: *mut std::ffi::c_char,
    pub _IO_write_base: *mut std::ffi::c_char,
    pub _IO_write_ptr: *mut std::ffi::c_char,
    pub _IO_write_end: *mut std::ffi::c_char,
    pub _IO_buf_base: *mut std::ffi::c_char,
    pub _IO_buf_end: *mut std::ffi::c_char,
    pub _IO_save_base: *mut std::ffi::c_char,
    pub _IO_backup_base: *mut std::ffi::c_char,
    pub _IO_save_end: *mut std::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "std::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [std::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: std::ffi::c_ushort,
    pub _vtable_offset: std::ffi::c_schar,
    pub _shortbuf: [std::ffi::c_char; 1],
    pub _lock: *mut std::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut std::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: std::ffi::c_int,
    pub _unused2: [std::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: std::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub struct textlist {
    pub string: *mut std::ffi::c_char,
    pub endstring: *mut std::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mloption {
    pub opt_name: *const std::ffi::c_char,
    pub opt_func: Option<unsafe extern "C" fn(*const std::ffi::c_char, size_t) -> ()>,
}
#[no_mangle]
pub static mut fd0: std::ffi::c_int = 0 as std::ffi::c_int;
#[no_mangle]
pub static mut curr_dev: dev_t = 0;
#[no_mangle]
pub static mut curr_ino: ino_t = 0;
#[no_mangle]
pub unsafe extern "C" fn init_textlist(mut tlist: *mut textlist, mut str: *mut std::ffi::c_char) {
    let mut s: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut meta_quoted: lbool = LFALSE;
    let mut delim_quoted: lbool = LFALSE;
    let mut esc: *const std::ffi::c_char = get_meta_escape();
    let mut esclen: size_t = strlen(esc);
    (*tlist).string = skipsp(str);
    (*tlist).endstring = ((*tlist).string).offset(strlen((*tlist).string) as isize);
    s = str;
    while s < (*tlist).endstring {
        if meta_quoted as u64 != 0 {
            meta_quoted = LFALSE;
        } else if esclen > 0 as std::ffi::c_int as size_t
            && s.offset(esclen as isize) < (*tlist).endstring
            && strncmp(s, esc, esclen) == 0 as std::ffi::c_int
        {
            meta_quoted = LTRUE;
            s = s.offset(esclen.wrapping_sub(1 as std::ffi::c_int as size_t) as isize);
        } else if delim_quoted as u64 != 0 {
            if *s as std::ffi::c_int == closequote as std::ffi::c_int {
                delim_quoted = LFALSE;
            }
        } else if *s as std::ffi::c_int == openquote as std::ffi::c_int {
            delim_quoted = LTRUE;
        } else if *s as std::ffi::c_int == ' ' as i32 {
            *s = '\0' as i32 as std::ffi::c_char;
        }
        s = s.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn forw_textlist(
    mut tlist: *mut textlist,
    mut prev: *const std::ffi::c_char,
) -> *const std::ffi::c_char {
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    if prev.is_null() {
        s = (*tlist).string;
    } else {
        s = prev.offset(strlen(prev) as isize);
    }
    if s >= (*tlist).endstring as *const std::ffi::c_char {
        return 0 as *const std::ffi::c_char;
    }
    while *s as std::ffi::c_int == '\0' as i32 {
        s = s.offset(1);
    }
    if s >= (*tlist).endstring as *const std::ffi::c_char {
        return 0 as *const std::ffi::c_char;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn back_textlist(
    mut tlist: *mut textlist,
    mut prev: *const std::ffi::c_char,
) -> *const std::ffi::c_char {
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    if prev.is_null() {
        s = (*tlist).endstring;
    } else if prev <= (*tlist).string as *const std::ffi::c_char {
        return 0 as *const std::ffi::c_char;
    } else {
        s = prev.offset(-(1 as std::ffi::c_int as isize));
    }
    while *s as std::ffi::c_int == '\0' as i32 {
        s = s.offset(-1);
    }
    if s <= (*tlist).string as *const std::ffi::c_char {
        return 0 as *const std::ffi::c_char;
    }
    while *s.offset(-(1 as std::ffi::c_int) as isize) as std::ffi::c_int != '\0' as i32
        && s > (*tlist).string as *const std::ffi::c_char
    {
        s = s.offset(-1);
    }
    return s;
}
unsafe extern "C" fn modeline_option(mut str: *const std::ffi::c_char, mut opt_len: size_t) {
    let mut options: [mloption; 3] = [
        {
            let mut init = mloption {
                opt_name: b"ts=\0" as *const u8 as *const std::ffi::c_char,
                opt_func: Some(
                    set_tabs as unsafe extern "C" fn(*const std::ffi::c_char, size_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mloption {
                opt_name: b"tabstop=\0" as *const u8 as *const std::ffi::c_char,
                opt_func: Some(
                    set_tabs as unsafe extern "C" fn(*const std::ffi::c_char, size_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mloption {
                opt_name: 0 as *const std::ffi::c_char,
                opt_func: None,
            };
            init
        },
    ];
    let mut opt: *mut mloption = 0 as *mut mloption;
    opt = options.as_mut_ptr();
    while !((*opt).opt_name).is_null() {
        let mut name_len: size_t = strlen((*opt).opt_name);
        if opt_len > name_len && strncmp(str, (*opt).opt_name, name_len) == 0 as std::ffi::c_int {
            (Some(((*opt).opt_func).expect("non-null function pointer")))
                .expect("non-null function pointer")(
                str.offset(name_len as isize),
                opt_len.wrapping_sub(name_len),
            );
            break;
        } else {
            opt = opt.offset(1);
        }
    }
}
unsafe extern "C" fn modeline_option_len(mut str: *const std::ffi::c_char) -> size_t {
    let mut esc: lbool = LFALSE;
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    s = str;
    while *s as std::ffi::c_int != '\0' as i32 {
        if esc as u64 != 0 {
            esc = LFALSE;
        } else if *s as std::ffi::c_int == '\\' as i32 {
            esc = LTRUE;
        } else if *s as std::ffi::c_int == ' ' as i32 || *s as std::ffi::c_int == ':' as i32 {
            break;
        }
        s = s.offset(1);
    }
    return s.offset_from(str) as std::ffi::c_long as size_t;
}
unsafe extern "C" fn modeline_options(
    mut str: *const std::ffi::c_char,
    mut end_char: std::ffi::c_char,
) {
    loop {
        let mut opt_len: size_t = 0;
        str = skipspc(str);
        if *str as std::ffi::c_int == '\0' as i32
            || *str as std::ffi::c_int == end_char as std::ffi::c_int
        {
            break;
        }
        opt_len = modeline_option_len(str);
        modeline_option(str, opt_len);
        str = str.offset(opt_len as isize);
        if *str as std::ffi::c_int != '\0' as i32 {
            str = str.offset(1 as std::ffi::c_int as isize);
        }
    }
}
unsafe extern "C" fn check_modeline(mut line: *const std::ffi::c_char) {
    static mut pgms: [*const std::ffi::c_char; 5] = [
        b"less:\0" as *const u8 as *const std::ffi::c_char,
        b"vim:\0" as *const u8 as *const std::ffi::c_char,
        b"vi:\0" as *const u8 as *const std::ffi::c_char,
        b"ex:\0" as *const u8 as *const std::ffi::c_char,
        0 as *const std::ffi::c_char,
    ];
    let mut pgm: *mut *const std::ffi::c_char = 0 as *mut *const std::ffi::c_char;
    pgm = pgms.as_mut_ptr();
    while !(*pgm).is_null() {
        let mut pline: *const std::ffi::c_char = line;
        loop {
            let mut str: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
            pline = strstr(pline, *pgm);
            if pline.is_null() {
                break;
            }
            str = skipspc(pline.offset(strlen(*pgm) as isize));
            if pline == line
                || *pline.offset(-(1 as std::ffi::c_int) as isize) as std::ffi::c_int == ' ' as i32
            {
                if strncmp(
                    str,
                    b"set \0" as *const u8 as *const std::ffi::c_char,
                    4 as std::ffi::c_int as std::ffi::c_ulong,
                ) == 0 as std::ffi::c_int
                {
                    modeline_options(
                        str.offset(4 as std::ffi::c_int as isize),
                        ':' as i32 as std::ffi::c_char,
                    );
                } else if pgm
                    != &mut *pgms.as_mut_ptr().offset(0 as std::ffi::c_int as isize)
                        as *mut *const std::ffi::c_char
                {
                    modeline_options(str, '\0' as i32 as std::ffi::c_char);
                }
                break;
            } else {
                pline = str;
            }
        }
        pgm = pgm.offset(1);
    }
}
unsafe extern "C" fn check_modelines() {
    let mut pos: POSITION = 0 as std::ffi::c_int as POSITION;
    let mut i: std::ffi::c_int = 0;
    i = 0 as std::ffi::c_int;
    while i < modelines {
        let mut line: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
        let mut line_len: size_t = 0;
        if sigs
            & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
            != 0
        {
            return;
        }
        pos = forw_raw_line(pos, &mut line, &mut line_len);
        if pos == -(1 as std::ffi::c_int) as POSITION {
            break;
        }
        check_modeline(line);
        i += 1;
    }
}
unsafe extern "C" fn close_pipe(mut pipefd: *mut FILE) {
    let mut status: std::ffi::c_int = 0;
    let mut p: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    if pipefd.is_null() {
        return;
    }
    status = pclose(pipefd);
    if status == -(1 as std::ffi::c_int) {
        p = errno_message(b"pclose\0" as *const u8 as *const std::ffi::c_char);
        parg.p_string = p;
        error(b"%s\0" as *const u8 as *const std::ffi::c_char, &mut parg);
        free(p as *mut std::ffi::c_void);
        return;
    }
    if show_preproc_error == 0 {
        return;
    }
    if status & 0x7f as std::ffi::c_int == 0 as std::ffi::c_int {
        let mut s: std::ffi::c_int = (status & 0xff00 as std::ffi::c_int) >> 8 as std::ffi::c_int;
        if s != 0 as std::ffi::c_int {
            parg.p_int = s;
            error(
                b"Input preprocessor failed (status %d)\0" as *const u8 as *const std::ffi::c_char,
                &mut parg,
            );
        }
        return;
    }
    if ((status & 0x7f as std::ffi::c_int) + 1 as std::ffi::c_int) as std::ffi::c_schar
        as std::ffi::c_int
        >> 1 as std::ffi::c_int
        > 0 as std::ffi::c_int
    {
        let mut sig: std::ffi::c_int = status & 0x7f as std::ffi::c_int;
        if sig != 13 as std::ffi::c_int || ch_length() != -(1 as std::ffi::c_int) as POSITION {
            parg.p_string = signal_message(sig);
            error(
                b"Input preprocessor terminated: %s\0" as *const u8 as *const std::ffi::c_char,
                &mut parg,
            );
        }
        return;
    }
    if status != 0 as std::ffi::c_int {
        parg.p_int = status;
        error(
            b"Input preprocessor exited with status %x\0" as *const u8 as *const std::ffi::c_char,
            &mut parg,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn close_altpipe(mut ifile: *mut std::ffi::c_void) {
    let mut altpipe: *mut FILE = get_altpipe(ifile) as *mut FILE;
    if !altpipe.is_null() && ch_getflags() & 0o2 as std::ffi::c_int == 0 {
        close_pipe(altpipe);
        set_altpipe(ifile, 0 as *mut std::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn check_altpipe_error() {
    if show_preproc_error == 0 {
        return;
    }
    if curr_ifile != 0 as *mut std::ffi::c_void && !(get_altfilename(curr_ifile)).is_null() {
        close_altpipe(curr_ifile);
    }
}
unsafe extern "C" fn close_file() {
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    let mut altfilename: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    if curr_ifile == 0 as *mut std::ffi::c_void {
        return;
    }
    get_scrpos(&mut scrpos, 0 as std::ffi::c_int);
    if scrpos.pos != -(1 as std::ffi::c_int) as POSITION {
        store_pos(curr_ifile, &mut scrpos);
        lastmark();
    }
    ch_close();
    altfilename = get_altfilename(curr_ifile);
    if !altfilename.is_null() {
        close_altpipe(curr_ifile);
        close_altfile(altfilename, get_filename(curr_ifile));
        set_altfilename(curr_ifile, 0 as *mut std::ffi::c_char);
    }
    curr_ifile = 0 as *mut std::ffi::c_void;
    curr_dev = 0 as std::ffi::c_int as dev_t;
    curr_ino = curr_dev;
}
#[no_mangle]
pub unsafe extern "C" fn edit(mut filename: *const std::ffi::c_char) -> std::ffi::c_int {
    if filename.is_null() {
        return edit_ifile(0 as *mut std::ffi::c_void);
    }
    return edit_ifile(get_ifile(filename, curr_ifile));
}
unsafe extern "C" fn edit_error(
    mut filename: *const std::ffi::c_char,
    mut alt_filename: *const std::ffi::c_char,
    mut altpipe: *mut std::ffi::c_void,
    mut ifile: *mut std::ffi::c_void,
) -> std::ffi::c_int {
    if !alt_filename.is_null() {
        close_pipe(altpipe as *mut FILE);
        close_altfile(alt_filename, filename);
        free(alt_filename as *mut std::ffi::c_char as *mut std::ffi::c_void);
    }
    del_ifile(ifile);
    if curr_ifile == ifile {
        quit(1 as std::ffi::c_int);
    }
    return 1 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn edit_ifile(mut ifile: *mut std::ffi::c_void) -> std::ffi::c_int {
    let mut f: std::ffi::c_int = 0;
    let mut answer: std::ffi::c_int = 0;
    let mut chflags: std::ffi::c_int = 0;
    let mut filename: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut open_filename: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut alt_filename: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut altpipe: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    let mut was_curr_ifile: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    let mut p: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    let mut nread: ssize_t = 0 as std::ffi::c_int as ssize_t;
    if ifile == curr_ifile {
        return 0 as std::ffi::c_int;
    }
    new_file = LTRUE;
    if ifile != 0 as *mut std::ffi::c_void {
        filename = get_filename(ifile);
        altpipe = get_altpipe(ifile);
        if !altpipe.is_null() {
            chflags = 0 as std::ffi::c_int;
            f = -(1 as std::ffi::c_int);
            alt_filename = get_altfilename(ifile);
            open_filename = if !alt_filename.is_null() {
                alt_filename as *const std::ffi::c_char
            } else {
                filename
            };
        } else {
            if strcmp(
                filename,
                b"@/\\less/\\help/\\file/\\@\0" as *const u8 as *const std::ffi::c_char,
            ) == 0 as std::ffi::c_int
                || strcmp(
                    filename,
                    b"@/\\less/\\empty/\\file/\\@\0" as *const u8 as *const std::ffi::c_char,
                ) == 0 as std::ffi::c_int
            {
                alt_filename = 0 as *mut std::ffi::c_char;
            } else {
                alt_filename = open_altfile(filename, &mut f, &mut altpipe);
            }
            open_filename = if !alt_filename.is_null() {
                alt_filename as *const std::ffi::c_char
            } else {
                filename
            };
            chflags = 0 as std::ffi::c_int;
            if !altpipe.is_null() {
                chflags |= 0o4 as std::ffi::c_int;
                if strcmp(filename, b"-\0" as *const u8 as *const std::ffi::c_char)
                    == 0 as std::ffi::c_int
                {
                    chflags |= 0o2 as std::ffi::c_int;
                }
            } else if strcmp(filename, b"-\0" as *const u8 as *const std::ffi::c_char)
                == 0 as std::ffi::c_int
            {
                f = fd0;
                chflags |= 0o2 as std::ffi::c_int;
            } else if strcmp(
                open_filename,
                b"@/\\less/\\empty/\\file/\\@\0" as *const u8 as *const std::ffi::c_char,
            ) == 0 as std::ffi::c_int
            {
                f = -(1 as std::ffi::c_int);
                chflags |= 0o20 as std::ffi::c_int;
            } else if strcmp(
                open_filename,
                b"@/\\less/\\help/\\file/\\@\0" as *const u8 as *const std::ffi::c_char,
            ) == 0 as std::ffi::c_int
            {
                f = -(1 as std::ffi::c_int);
                chflags |= 0o10 as std::ffi::c_int;
            } else {
                p = bad_file(open_filename);
                if !p.is_null() {
                    parg.p_string = p;
                    error(b"%s\0" as *const u8 as *const std::ffi::c_char, &mut parg);
                    free(p as *mut std::ffi::c_void);
                    return edit_error(filename, alt_filename, altpipe, ifile);
                } else {
                    f = iopen(open_filename, 0 as std::ffi::c_int);
                    if f < 0 as std::ffi::c_int {
                        let mut p_0: *mut std::ffi::c_char = errno_message(filename);
                        parg.p_string = p_0;
                        error(b"%s\0" as *const u8 as *const std::ffi::c_char, &mut parg);
                        free(p_0 as *mut std::ffi::c_void);
                        return edit_error(filename, alt_filename, altpipe, ifile);
                    } else {
                        chflags |= 0o1 as std::ffi::c_int;
                        if bin_file(f, &mut nread) != 0 && force_open == 0 && opened(ifile) == 0 {
                            parg.p_string = filename;
                            answer = query(
                                b"\"%s\" may be a binary file.  See it anyway? \0" as *const u8
                                    as *const std::ffi::c_char,
                                &mut parg,
                            );
                            if answer != 'y' as i32 && answer != 'Y' as i32 {
                                close(f);
                                return edit_error(filename, alt_filename, altpipe, ifile);
                            }
                        }
                    }
                }
            }
        }
        if force_open == 0 && f >= 0 as std::ffi::c_int && isatty(f) != 0 {
            let mut parg_0: PARG = parg {
                p_string: 0 as *const std::ffi::c_char,
            };
            parg_0.p_string = filename;
            error(
                b"%s is a terminal (use -f to open it)\0" as *const u8 as *const std::ffi::c_char,
                &mut parg_0,
            );
            return edit_error(filename, alt_filename, altpipe, ifile);
        }
    }
    end_logfile();
    was_curr_ifile = save_curr_ifile();
    if curr_ifile != 0 as *mut std::ffi::c_void {
        let mut was_helpfile: std::ffi::c_int = ch_getflags() & 0o10 as std::ffi::c_int;
        close_file();
        if was_helpfile != 0 && held_ifile(was_curr_ifile) <= 1 as std::ffi::c_int {
            del_ifile(was_curr_ifile);
            was_curr_ifile = 0 as *mut std::ffi::c_void;
        }
    }
    unsave_ifile(was_curr_ifile);
    if ifile == 0 as *mut std::ffi::c_void {
        return 0 as std::ffi::c_int;
    }
    curr_ifile = ifile;
    soft_eof = -(1 as std::ffi::c_int) as POSITION;
    set_altfilename(curr_ifile, alt_filename);
    set_altpipe(curr_ifile, altpipe);
    set_open(curr_ifile);
    get_pos(curr_ifile, &mut initial_scrpos);
    ch_init(f, chflags, nread);
    consecutive_nulls = 0 as std::ffi::c_int;
    check_modelines();
    if chflags & 0o10 as std::ffi::c_int == 0 {
        if was_curr_ifile != 0 as *mut std::ffi::c_void {
            old_ifile = was_curr_ifile;
        }
        if !namelogfile.is_null() && is_tty != 0 {
            use_logfile(namelogfile);
        }
        if strcmp(
            open_filename,
            b"-\0" as *const u8 as *const std::ffi::c_char,
        ) != 0 as std::ffi::c_int
        {
            let mut statbuf: stat = stat {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_mtim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_ctim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                __glibc_reserved: [0; 3],
            };
            let mut r: std::ffi::c_int = stat(open_filename, &mut statbuf);
            if r == 0 as std::ffi::c_int {
                curr_ino = statbuf.st_ino;
                curr_dev = statbuf.st_dev;
            }
        }
        if !every_first_cmd.is_null() {
            ungetsc(every_first_cmd);
            ungetcc_end_command();
        }
    }
    flush();
    if is_tty != 0 {
        pos_clear();
        clr_linenum();
        clr_hilite();
        undo_osc8();
        hshift = 0 as std::ffi::c_int;
        if strcmp(
            filename,
            b"@/\\less/\\help/\\file/\\@\0" as *const u8 as *const std::ffi::c_char,
        ) != 0
            && strcmp(
                filename,
                b"@/\\less/\\empty/\\file/\\@\0" as *const u8 as *const std::ffi::c_char,
            ) != 0
        {
            let mut qfilename: *mut std::ffi::c_char = shell_quote(filename);
            cmd_addhist(ml_examine as *mut mlist, qfilename, LTRUE);
            free(qfilename as *mut std::ffi::c_void);
        }
        if want_filesize != 0 {
            scan_eof();
        }
        set_header(0 as std::ffi::c_int as POSITION);
    }
    return 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn edit_list(mut filelist: *mut std::ffi::c_char) -> std::ffi::c_int {
    let mut save_ifile: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    let mut good_filename: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut filename: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut gfilelist: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut gfilename: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut qfilename: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut tl_files: textlist = textlist {
        string: 0 as *mut std::ffi::c_char,
        endstring: 0 as *mut std::ffi::c_char,
    };
    let mut tl_gfiles: textlist = textlist {
        string: 0 as *mut std::ffi::c_char,
        endstring: 0 as *mut std::ffi::c_char,
    };
    save_ifile = save_curr_ifile();
    good_filename = 0 as *const std::ffi::c_char;
    init_textlist(&mut tl_files, filelist);
    filename = 0 as *const std::ffi::c_char;
    loop {
        filename = forw_textlist(&mut tl_files, filename);
        if filename.is_null() {
            break;
        }
        gfilelist = lglob(filename);
        init_textlist(&mut tl_gfiles, gfilelist);
        gfilename = 0 as *const std::ffi::c_char;
        loop {
            gfilename = forw_textlist(&mut tl_gfiles, gfilename);
            if gfilename.is_null() {
                break;
            }
            qfilename = shell_unquote(gfilename);
            if edit(qfilename) == 0 as std::ffi::c_int && good_filename.is_null() {
                good_filename = get_filename(curr_ifile);
            }
            free(qfilename as *mut std::ffi::c_void);
        }
        free(gfilelist as *mut std::ffi::c_void);
    }
    if good_filename.is_null() {
        unsave_ifile(save_ifile);
        return 1 as std::ffi::c_int;
    }
    if get_ifile(good_filename, curr_ifile) == curr_ifile {
        unsave_ifile(save_ifile);
        return 0 as std::ffi::c_int;
    }
    reedit_ifile(save_ifile);
    return edit(good_filename);
}
#[no_mangle]
pub unsafe extern "C" fn edit_first() -> std::ffi::c_int {
    if nifile() == 0 as std::ffi::c_int {
        return edit_stdin();
    }
    curr_ifile = 0 as *mut std::ffi::c_void;
    return edit_next(1 as std::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn edit_last() -> std::ffi::c_int {
    curr_ifile = 0 as *mut std::ffi::c_void;
    return edit_prev(1 as std::ffi::c_int);
}
unsafe extern "C" fn edit_istep(
    mut h: *mut std::ffi::c_void,
    mut n: std::ffi::c_int,
    mut dir: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut next: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    loop {
        next = if dir > 0 as std::ffi::c_int {
            next_ifile(h)
        } else {
            prev_ifile(h)
        };
        n -= 1;
        if n < 0 as std::ffi::c_int {
            if edit_ifile(h) == 0 as std::ffi::c_int {
                break;
            }
        }
        if next == 0 as *mut std::ffi::c_void {
            return 1 as std::ffi::c_int;
        }
        if sigs
            & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
            != 0
        {
            return 1 as std::ffi::c_int;
        }
        h = next;
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn edit_inext(
    mut h: *mut std::ffi::c_void,
    mut n: std::ffi::c_int,
) -> std::ffi::c_int {
    return edit_istep(h, n, 1 as std::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn edit_next(mut n: std::ffi::c_int) -> std::ffi::c_int {
    return edit_istep(curr_ifile, n, 1 as std::ffi::c_int);
}
unsafe extern "C" fn edit_iprev(
    mut h: *mut std::ffi::c_void,
    mut n: std::ffi::c_int,
) -> std::ffi::c_int {
    return edit_istep(h, n, -(1 as std::ffi::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn edit_prev(mut n: std::ffi::c_int) -> std::ffi::c_int {
    return edit_istep(curr_ifile, n, -(1 as std::ffi::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn edit_index(mut n: std::ffi::c_int) -> std::ffi::c_int {
    let mut h: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    h = 0 as *mut std::ffi::c_void;
    loop {
        h = next_ifile(h);
        if h == 0 as *mut std::ffi::c_void {
            return 1 as std::ffi::c_int;
        }
        if !(get_index(h) != n) {
            break;
        }
    }
    return edit_ifile(h);
}
#[no_mangle]
pub unsafe extern "C" fn save_curr_ifile() -> *mut std::ffi::c_void {
    if curr_ifile != 0 as *mut std::ffi::c_void {
        hold_ifile(curr_ifile, 1 as std::ffi::c_int);
    }
    return curr_ifile;
}
#[no_mangle]
pub unsafe extern "C" fn unsave_ifile(mut save_ifile: *mut std::ffi::c_void) {
    if save_ifile != 0 as *mut std::ffi::c_void {
        hold_ifile(save_ifile, -(1 as std::ffi::c_int));
    }
}
#[no_mangle]
pub unsafe extern "C" fn reedit_ifile(mut save_ifile: *mut std::ffi::c_void) {
    let mut next: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    let mut prev: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    unsave_ifile(save_ifile);
    next = next_ifile(save_ifile);
    prev = prev_ifile(save_ifile);
    if edit_ifile(save_ifile) == 0 as std::ffi::c_int {
        return;
    }
    if next != 0 as *mut std::ffi::c_void
        && edit_inext(next, 0 as std::ffi::c_int) == 0 as std::ffi::c_int
    {
        return;
    }
    if prev != 0 as *mut std::ffi::c_void
        && edit_iprev(prev, 0 as std::ffi::c_int) == 0 as std::ffi::c_int
    {
        return;
    }
    quit(1 as std::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn reopen_curr_ifile() {
    let mut save_ifile: *mut std::ffi::c_void = save_curr_ifile();
    close_file();
    reedit_ifile(save_ifile);
}
#[no_mangle]
pub unsafe extern "C" fn edit_stdin() -> std::ffi::c_int {
    if isatty(fd0) != 0 {
        error(
            b"Missing filename (\"less --help\" for help)\0" as *const u8
                as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        quit(0 as std::ffi::c_int);
    }
    return edit(b"-\0" as *const u8 as *const std::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn cat_file() {
    let mut c: std::ffi::c_int = 0;
    loop {
        c = ch_forw_get();
        if !(c != -(1 as std::ffi::c_int)) {
            break;
        }
        putchr(c);
    }
    flush();
}
#[no_mangle]
pub unsafe extern "C" fn use_logfile(mut filename: *const std::ffi::c_char) {
    let mut exists: std::ffi::c_int = 0;
    let mut answer: std::ffi::c_int = 0;
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    if ch_getflags() & 0o1 as std::ffi::c_int != 0 {
        return;
    }
    exists = open(filename, 0 as std::ffi::c_int);
    if exists >= 0 as std::ffi::c_int {
        close(exists);
    }
    exists = (exists >= 0 as std::ffi::c_int) as std::ffi::c_int;
    if exists == 0 || force_logfile != 0 {
        answer = 'O' as i32;
    } else {
        parg.p_string = filename;
        answer = query(
            b"Warning: \"%s\" exists; Overwrite, Append, Don't log, or Quit? \0" as *const u8
                as *const std::ffi::c_char,
            &mut parg,
        );
    }
    loop {
        match answer {
            79 | 111 => {
                logfile = creat(filename, 0o644 as std::ffi::c_int as mode_t);
                break;
            }
            65 | 97 => {
                logfile = open(filename, 0o2000 as std::ffi::c_int | 0o1 as std::ffi::c_int);
                if lseek(
                    logfile,
                    0 as std::ffi::c_int as less_off_t,
                    2 as std::ffi::c_int,
                ) == -(1 as std::ffi::c_int) as off_t
                {
                    close(logfile);
                    logfile = -(1 as std::ffi::c_int);
                }
                break;
            }
            68 | 100 => return,
            _ => {
                answer = query(
                    b"Overwrite, Append, Don't log, or Quit? (Type \"O\", \"A\", \"D\" or \"Q\") \0"
                        as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
            }
        }
    }
    if logfile < 0 as std::ffi::c_int {
        parg.p_string = filename;
        error(
            b"Cannot write to \"%s\"\0" as *const u8 as *const std::ffi::c_char,
            &mut parg,
        );
        return;
    }
}
