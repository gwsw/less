use crate::decode::lgetenv;
use ::c2rust_bitfields;
use ::libc;
use std::ffi::CString;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn snprintf(
        _: *mut std::ffi::c_char,
        _: std::ffi::c_ulong,
        _: *const std::ffi::c_char,
        _: ...
    ) -> std::ffi::c_int;
    fn putc(__c: std::ffi::c_int, __stream: *mut FILE) -> std::ffi::c_int;
    fn pclose(__stream: *mut FILE) -> std::ffi::c_int;
    fn popen(__command: *const std::ffi::c_char, __modes: *const std::ffi::c_char) -> *mut FILE;
    fn close(__fd: std::ffi::c_int) -> std::ffi::c_int;
    fn dup(__fd: std::ffi::c_int) -> std::ffi::c_int;
    fn free(_: *mut std::ffi::c_void);
    fn system(__command: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn save(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
    fn raw_mode(on: std::ffi::c_int);
    fn init();
    fn deinit();
    fn clear_bot();
    fn ch_seek(pos: POSITION) -> std::ffi::c_int;
    fn ch_forw_get() -> std::ffi::c_int;
    fn screen_trashed();
    fn edit_ifile(ifile: *mut std::ffi::c_void) -> std::ffi::c_int;
    fn save_curr_ifile() -> *mut std::ffi::c_void;
    fn reedit_ifile(save_ifile: *mut std::ffi::c_void);
    fn shell_quote(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn shell_coption() -> *const std::ffi::c_char;
    fn markpos(c: std::ffi::c_char) -> POSITION;
    fn flush();
    fn putchr(ch: std::ffi::c_int) -> std::ffi::c_int;
    fn putstr(s: *const std::ffi::c_char);
    fn get_return();
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn position(sindex: std::ffi::c_int) -> POSITION;
    fn winch(type_0: std::ffi::c_int);
    fn init_signals(on: std::ffi::c_int);
    fn open_tty() -> std::ffi::c_int;
    fn signal(__sig: std::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
}
pub type __off_t = std::ffi::c_long;
pub type __off64_t = std::ffi::c_long;
pub type off_t = __off_t;
pub type size_t = std::ffi::c_ulong;
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
pub type __sighandler_t = Option<unsafe extern "C" fn(std::ffi::c_int) -> ()>;
#[no_mangle]
pub unsafe extern "C" fn lsystem(
    mut cmd: *const std::ffi::c_char,
    mut donemsg: *const std::ffi::c_char,
) {
    let mut inp: std::ffi::c_int = 0;
    let mut shell: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut p: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut save_ifile: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    if *cmd.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int == '-' as i32 {
        cmd = cmd.offset(1);
    } else {
        clear_bot();
        putstr(b"!\0" as *const u8 as *const std::ffi::c_char);
        putstr(cmd);
        putstr(b"\n\0" as *const u8 as *const std::ffi::c_char);
    }
    save_ifile = save_curr_ifile();
    edit_ifile(0 as *mut std::ffi::c_void);
    deinit();
    flush();
    raw_mode(0 as std::ffi::c_int);
    init_signals(0 as std::ffi::c_int);
    inp = dup(0 as std::ffi::c_int);
    close(0 as std::ffi::c_int);
    if open_tty() < 0 as std::ffi::c_int {
        dup(inp);
    }
    p = 0 as *mut std::ffi::c_char;
    if let Ok(shell) = lgetenv("SHELL") {
        if *cmd as std::ffi::c_int == '\0' as i32 {
            p = save(CString::new(shell).unwrap().as_ptr());
        } else {
            let mut esccmd: *mut std::ffi::c_char = shell_quote(cmd);
            if !esccmd.is_null() {
                let mut len: size_t = (shell.len() as u64)
                    .wrapping_add(strlen(esccmd))
                    .wrapping_add(5 as std::ffi::c_int as std::ffi::c_ulong);
                p = ecalloc(
                    len,
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                ) as *mut std::ffi::c_char;
                snprintf(
                    p,
                    len,
                    b"%s %s %s\0" as *const u8 as *const std::ffi::c_char,
                    shell,
                    shell_coption(),
                    esccmd,
                );
                free(esccmd as *mut std::ffi::c_void);
            }
        }
    }
    if p.is_null() {
        if *cmd as std::ffi::c_int == '\0' as i32 {
            p = save(b"sh\0" as *const u8 as *const std::ffi::c_char);
        } else {
            p = save(cmd);
        }
    }
    system(p);
    free(p as *mut std::ffi::c_void);
    close(0 as std::ffi::c_int);
    dup(inp);
    close(inp);
    init_signals(1 as std::ffi::c_int);
    raw_mode(1 as std::ffi::c_int);
    if !donemsg.is_null() {
        putstr(donemsg);
        putstr(b"  (press RETURN)\0" as *const u8 as *const std::ffi::c_char);
        get_return();
        putchr('\n' as i32);
        flush();
    }
    init();
    screen_trashed();
    reedit_ifile(save_ifile);
    winch(0 as std::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn pipe_mark(
    mut c: std::ffi::c_char,
    mut cmd: *const std::ffi::c_char,
) -> std::ffi::c_int {
    let mut mpos: POSITION = 0;
    let mut tpos: POSITION = 0;
    let mut bpos: POSITION = 0;
    mpos = markpos(c);
    if mpos == -(1 as std::ffi::c_int) as POSITION {
        return -(1 as std::ffi::c_int);
    }
    tpos = position(0 as std::ffi::c_int);
    if tpos == -(1 as std::ffi::c_int) as POSITION {
        tpos = 0 as std::ffi::c_int as POSITION;
    }
    bpos = position(-(1 as std::ffi::c_int));
    if c as std::ffi::c_int == '.' as i32 {
        return pipe_data(cmd, tpos, bpos);
    } else if mpos <= tpos {
        return pipe_data(cmd, mpos, bpos);
    } else if bpos == -(1 as std::ffi::c_int) as POSITION {
        return pipe_data(cmd, tpos, bpos);
    } else {
        return pipe_data(cmd, tpos, mpos);
    };
}
#[no_mangle]
pub unsafe extern "C" fn pipe_data(
    mut cmd: *const std::ffi::c_char,
    mut spos: POSITION,
    mut epos: POSITION,
) -> std::ffi::c_int {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut c: std::ffi::c_int = 0;
    if ch_seek(spos) != 0 as std::ffi::c_int {
        error(
            b"Cannot seek to start position\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        return -(1 as std::ffi::c_int);
    }
    f = popen(cmd, b"w\0" as *const u8 as *const std::ffi::c_char);
    if f.is_null() {
        error(
            b"Cannot create pipe\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        return -(1 as std::ffi::c_int);
    }
    clear_bot();
    putstr(b"!\0" as *const u8 as *const std::ffi::c_char);
    putstr(cmd);
    putstr(b"\n\0" as *const u8 as *const std::ffi::c_char);
    deinit();
    flush();
    raw_mode(0 as std::ffi::c_int);
    init_signals(0 as std::ffi::c_int);
    signal(
        13 as std::ffi::c_int,
        ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
            1 as std::ffi::c_int as libc::intptr_t,
        ),
    );
    c = -(1 as std::ffi::c_int);
    while epos == -(1 as std::ffi::c_int) as POSITION || {
        let fresh0 = spos;
        spos = spos + 1;
        fresh0 <= epos
    } {
        c = ch_forw_get();
        if c == -(1 as std::ffi::c_int) {
            break;
        }
        if putc(c, f) == -(1 as std::ffi::c_int) {
            break;
        }
    }
    while c != '\n' as i32 && c != -(1 as std::ffi::c_int) {
        c = ch_forw_get();
        if c == -(1 as std::ffi::c_int) {
            break;
        }
        if putc(c, f) == -(1 as std::ffi::c_int) {
            break;
        }
    }
    pclose(f);
    signal(13 as std::ffi::c_int, None);
    init_signals(1 as std::ffi::c_int);
    raw_mode(1 as std::ffi::c_int);
    init();
    screen_trashed();
    winch(0 as std::ffi::c_int);
    return 0 as std::ffi::c_int;
}
