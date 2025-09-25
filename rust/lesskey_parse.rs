use crate::defs::*;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn fclose(__stream: *mut FILE) -> std::ffi::c_int;
    fn fopen(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> *mut FILE;
    fn snprintf(
        _: *mut std::ffi::c_char,
        _: std::ffi::c_ulong,
        _: *const std::ffi::c_char,
        _: ...
    ) -> std::ffi::c_int;
    fn fgets(
        __s: *mut std::ffi::c_char,
        __n: std::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut std::ffi::c_char;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strncmp(
        _: *const std::ffi::c_char,
        _: *const std::ffi::c_char,
        _: std::ffi::c_ulong,
    ) -> std::ffi::c_int;
    fn strdup(_: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strchr(_: *const std::ffi::c_char, _: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn free(_: *mut std::ffi::c_void);
    fn xbuf_init(xbuf: *mut xbuffer);
    fn xbuf_add_byte(xbuf: *mut xbuffer, b: std::ffi::c_uchar);
    fn xbuf_pop(xbuf: *mut xbuffer) -> std::ffi::c_int;
    fn lesskey_parse_error(msg: *mut std::ffi::c_char);
    fn homefile(filename: *mut std::ffi::c_char) -> *mut std::ffi::c_char;
    fn lstrtoi(
        str: *mut std::ffi::c_char,
        end: *mut *mut std::ffi::c_char,
        radix: std::ffi::c_int,
    ) -> std::ffi::c_int;
    static mut version: [std::ffi::c_char; 0];
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
pub struct xbuffer {
    pub data: *mut std::ffi::c_uchar,
    pub end: size_t,
    pub size: size_t,
    pub init_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lesskey_cmdname {
    pub cn_name: *const std::ffi::c_char,
    pub cn_action: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lesskey_table {
    pub names: *const lesskey_cmdname,
    pub buf: xbuffer,
    pub is_var: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lesskey_tables {
    pub currtable: *mut lesskey_table,
    pub cmdtable: lesskey_table,
    pub edittable: lesskey_table,
    pub vartable: lesskey_table,
}
static mut linenum: std::ffi::c_int = 0;
static mut errors: std::ffi::c_int = 0;
static mut less_version: std::ffi::c_int = 0 as std::ffi::c_int;
static mut lesskey_file: *mut std::ffi::c_char =
    0 as *const std::ffi::c_char as *mut std::ffi::c_char;
static mut cmdnames: [lesskey_cmdname; 74] = [
    {
        let mut init = lesskey_cmdname {
            cn_name: b"back-bracket\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 36 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"back-line\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 2 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"back-line-force\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 30 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"back-newline\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 61 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"back-screen\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 3 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"back-screen-force\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 22 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"back-scroll\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 4 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"back-search\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 5 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"back-window\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 34 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"clear-mark\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 62 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"clear-search\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 70 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"debug\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 8 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"digit\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 6 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"display-flag\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 7 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"display-option\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 7 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"end\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 16 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"end-scroll\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 59 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"examine\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 9 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"filter\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 55 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"first-cmd\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 10 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"firstcmd\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 10 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"flush-repaint\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 11 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"forw-bracket\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 35 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"forw-forever\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 50 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"forw-line\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 12 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"forw-line-force\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 29 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"forw-newline\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 60 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"forw-screen\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 13 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"forw-screen-force\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 40 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"forw-scroll\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 14 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"forw-search\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 15 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"forw-until-hilite\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 56 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"forw-window\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 33 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"goto-end\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 16 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"goto-end-buffered\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 57 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"goto-line\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 17 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"goto-mark\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 18 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"help\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 19 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"index-file\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 38 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"invalid\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 102 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"left-scroll\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 41 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"mouse\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 64 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"mouse6\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 68 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"next-file\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 20 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"next-tag\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 53 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"no-scroll\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 58 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"noaction\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 101 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"osc8-forw-search\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 71 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"osc8-back-search\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 72 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"osc8-open\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 73 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"percent\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 21 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"pipe\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 37 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"prev-file\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 23 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"prev-tag\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 54 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"pshell\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 69 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"quit\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 24 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"remove-file\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 52 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"repaint\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 25 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"repaint-flush\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 11 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"repeat-search\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 43 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"repeat-search-all\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 44 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"reverse-search\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 45 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"reverse-search-all\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 46 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"right-scroll\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 42 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"set-mark\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 26 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"set-mark-bottom\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 63 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"shell\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 27 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"status\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 28 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"toggle-flag\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 47 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"toggle-option\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 47 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"undo-hilite\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 39 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"version\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 31 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"visual\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 32 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: 0 as *const std::ffi::c_char,
            cn_action: 0 as std::ffi::c_int,
        };
        init
    },
];
static mut editnames: [lesskey_cmdname; 23] = [
    {
        let mut init = lesskey_cmdname {
            cn_name: b"back-complete\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 18 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"backspace\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 1 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"delete\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 8 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"down\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 14 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"end\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 10 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"expand\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 15 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"forw-complete\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 17 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"home\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 9 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"insert\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 7 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"invalid\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 102 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"kill-line\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 2 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"abort\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 20 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"left\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 4 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"literal\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 19 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"mouse\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 21 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"mouse6\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 22 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"right\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 3 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"up\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 13 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"word-backspace\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 11 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"word-delete\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 12 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"word-left\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 5 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: b"word-right\0" as *const u8 as *const std::ffi::c_char,
            cn_action: 6 as std::ffi::c_int,
        };
        init
    },
    {
        let mut init = lesskey_cmdname {
            cn_name: 0 as *const std::ffi::c_char,
            cn_action: 0 as std::ffi::c_int,
        };
        init
    },
];
unsafe extern "C" fn parse_error(
    mut fmt: *const std::ffi::c_char,
    mut arg1: *const std::ffi::c_char,
) {
    let mut buf: [std::ffi::c_char; 1024] = [0; 1024];
    let mut n: std::ffi::c_int = snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[std::ffi::c_char; 1024]>() as std::ffi::c_ulong,
        b"%s: line %d: \0" as *const u8 as *const std::ffi::c_char,
        lesskey_file,
        linenum,
    );
    if n >= 0 as std::ffi::c_int {
        let mut len: size_t = n as size_t;
        if len < ::core::mem::size_of::<[std::ffi::c_char; 1024]>() as std::ffi::c_ulong {
            snprintf(
                buf.as_mut_ptr().offset(len as isize),
                (::core::mem::size_of::<[std::ffi::c_char; 1024]>() as std::ffi::c_ulong)
                    .wrapping_sub(len),
                fmt,
                arg1,
            );
        }
    }
    errors += 1;
    lesskey_parse_error(buf.as_mut_ptr());
}
unsafe extern "C" fn init_tables(mut tables: *mut lesskey_tables) {
    (*tables).currtable = &mut (*tables).cmdtable;
    (*tables).cmdtable.names = cmdnames.as_ptr();
    (*tables).cmdtable.is_var = 0 as std::ffi::c_int;
    xbuf_init(&mut (*tables).cmdtable.buf);
    (*tables).edittable.names = editnames.as_ptr();
    (*tables).edittable.is_var = 0 as std::ffi::c_int;
    xbuf_init(&mut (*tables).edittable.buf);
    (*tables).vartable.names = 0 as *const lesskey_cmdname;
    (*tables).vartable.is_var = 1 as std::ffi::c_int;
    xbuf_init(&mut (*tables).vartable.buf);
}
unsafe extern "C" fn char_string(
    mut buf: *mut std::ffi::c_char,
    mut ch: std::ffi::c_char,
    mut lit: std::ffi::c_int,
) -> *const std::ffi::c_char {
    if lit != 0
        || ch as std::ffi::c_int >= 0x20 as std::ffi::c_int
            && (ch as std::ffi::c_int) < 0x7f as std::ffi::c_int
    {
        *buf.offset(0 as std::ffi::c_int as isize) = ch;
        *buf.offset(1 as std::ffi::c_int as isize) = '\0' as i32 as std::ffi::c_char;
    } else {
        snprintf(
            buf,
            8 as std::ffi::c_int as std::ffi::c_ulong,
            b"\\x%02x\0" as *const u8 as *const std::ffi::c_char,
            ch as std::ffi::c_int,
        );
    }
    return buf;
}
unsafe extern "C" fn increment_pointer(mut p: *mut std::ffi::c_char) -> *mut std::ffi::c_char {
    if *p as std::ffi::c_int == '\0' as i32 {
        return p;
    }
    return p.offset(1 as std::ffi::c_int as isize);
}
unsafe extern "C" fn tstr(
    mut pp: *mut *mut std::ffi::c_char,
    mut xlate: std::ffi::c_int,
) -> *const std::ffi::c_char {
    let mut p: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut ch: std::ffi::c_char = 0;
    let mut i: std::ffi::c_int = 0;
    static mut buf: [std::ffi::c_char; 8] = [0; 8];
    static mut tstr_control_k: [std::ffi::c_char; 7] = [
        ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_char,
        40 as std::ffi::c_int as std::ffi::c_char,
        6 as std::ffi::c_int as std::ffi::c_char,
        1 as std::ffi::c_int as std::ffi::c_char,
        1 as std::ffi::c_int as std::ffi::c_char,
        1 as std::ffi::c_int as std::ffi::c_char,
        '\0' as i32 as std::ffi::c_char,
    ];
    p = *pp;
    match *p as std::ffi::c_int {
        92 => {
            p = p.offset(1);
            match *p as std::ffi::c_int {
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                    ch = 0 as std::ffi::c_int as std::ffi::c_char;
                    i = 0 as std::ffi::c_int;
                    loop {
                        ch = (8 as std::ffi::c_int * ch as std::ffi::c_int
                            + (*p as std::ffi::c_int - '0' as i32))
                            as std::ffi::c_char;
                        p = p.offset(1);
                        if !(*p as std::ffi::c_int >= '0' as i32
                            && *p as std::ffi::c_int <= '7' as i32
                            && {
                                i += 1;
                                i < 3 as std::ffi::c_int
                            })
                        {
                            break;
                        }
                    }
                    *pp = p;
                    if xlate != 0 && ch as std::ffi::c_int == 'K' as i32 & 0o37 as std::ffi::c_int {
                        return tstr_control_k.as_mut_ptr();
                    }
                    return char_string(buf.as_mut_ptr(), ch, 1 as std::ffi::c_int);
                }
                98 => {
                    *pp = p.offset(1 as std::ffi::c_int as isize);
                    return b"\x08\0" as *const u8 as *const std::ffi::c_char;
                }
                101 => {
                    *pp = p.offset(1 as std::ffi::c_int as isize);
                    return char_string(
                        buf.as_mut_ptr(),
                        ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_char,
                        1 as std::ffi::c_int,
                    );
                }
                110 => {
                    *pp = p.offset(1 as std::ffi::c_int as isize);
                    return b"\n\0" as *const u8 as *const std::ffi::c_char;
                }
                114 => {
                    *pp = p.offset(1 as std::ffi::c_int as isize);
                    return b"\r\0" as *const u8 as *const std::ffi::c_char;
                }
                116 => {
                    *pp = p.offset(1 as std::ffi::c_int as isize);
                    return b"\t\0" as *const u8 as *const std::ffi::c_char;
                }
                107 => {
                    if xlate != 0 {
                        p = p.offset(1);
                        match *p as std::ffi::c_int {
                            98 => {
                                ch = 17 as std::ffi::c_int as std::ffi::c_char;
                            }
                            66 => {
                                ch = 16 as std::ffi::c_int as std::ffi::c_char;
                            }
                            100 => {
                                ch = 4 as std::ffi::c_int as std::ffi::c_char;
                            }
                            68 => {
                                ch = 6 as std::ffi::c_int as std::ffi::c_char;
                            }
                            101 => {
                                ch = 8 as std::ffi::c_int as std::ffi::c_char;
                            }
                            104 => {
                                ch = 7 as std::ffi::c_int as std::ffi::c_char;
                            }
                            105 => {
                                ch = 10 as std::ffi::c_int as std::ffi::c_char;
                            }
                            108 => {
                                ch = 2 as std::ffi::c_int as std::ffi::c_char;
                            }
                            76 => {
                                ch = 11 as std::ffi::c_int as std::ffi::c_char;
                            }
                            114 => {
                                ch = 1 as std::ffi::c_int as std::ffi::c_char;
                            }
                            82 => {
                                ch = 12 as std::ffi::c_int as std::ffi::c_char;
                            }
                            116 => {
                                ch = 15 as std::ffi::c_int as std::ffi::c_char;
                            }
                            117 => {
                                ch = 3 as std::ffi::c_int as std::ffi::c_char;
                            }
                            85 => {
                                ch = 5 as std::ffi::c_int as std::ffi::c_char;
                            }
                            120 => {
                                ch = 9 as std::ffi::c_int as std::ffi::c_char;
                            }
                            88 => {
                                ch = 13 as std::ffi::c_int as std::ffi::c_char;
                            }
                            49 => {
                                ch = 14 as std::ffi::c_int as std::ffi::c_char;
                            }
                            _ => {
                                parse_error(
                                    b"invalid escape sequence \"\\k%s\"\0" as *const u8
                                        as *const std::ffi::c_char,
                                    char_string(buf.as_mut_ptr(), *p, 0 as std::ffi::c_int),
                                );
                                *pp = increment_pointer(p);
                                return b"\0" as *const u8 as *const std::ffi::c_char;
                            }
                        }
                        *pp = p.offset(1 as std::ffi::c_int as isize);
                        buf[0 as std::ffi::c_int as usize] =
                            ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_char;
                        buf[1 as std::ffi::c_int as usize] = ch;
                        buf[2 as std::ffi::c_int as usize] =
                            6 as std::ffi::c_int as std::ffi::c_char;
                        buf[3 as std::ffi::c_int as usize] =
                            1 as std::ffi::c_int as std::ffi::c_char;
                        buf[4 as std::ffi::c_int as usize] =
                            1 as std::ffi::c_int as std::ffi::c_char;
                        buf[5 as std::ffi::c_int as usize] =
                            1 as std::ffi::c_int as std::ffi::c_char;
                        buf[6 as std::ffi::c_int as usize] = '\0' as i32 as std::ffi::c_char;
                        return buf.as_mut_ptr();
                    }
                }
                _ => {}
            }
            *pp = increment_pointer(p);
            char_string(buf.as_mut_ptr(), *p, 1 as std::ffi::c_int);
            if xlate != 0
                && buf[0 as std::ffi::c_int as usize] as std::ffi::c_int
                    == 'K' as i32 & 0o37 as std::ffi::c_int
            {
                return tstr_control_k.as_mut_ptr();
            }
            return buf.as_mut_ptr();
        }
        94 => {
            *pp = increment_pointer(p.offset(1 as std::ffi::c_int as isize));
            char_string(
                buf.as_mut_ptr(),
                (*p.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                    & 0o37 as std::ffi::c_int) as std::ffi::c_char,
                1 as std::ffi::c_int,
            );
            if xlate != 0
                && buf[0 as std::ffi::c_int as usize] as std::ffi::c_int
                    == 'K' as i32 & 0o37 as std::ffi::c_int
            {
                return tstr_control_k.as_mut_ptr();
            }
            return buf.as_mut_ptr();
        }
        _ => {}
    }
    *pp = increment_pointer(p);
    char_string(buf.as_mut_ptr(), *p, 1 as std::ffi::c_int);
    if xlate != 0
        && buf[0 as std::ffi::c_int as usize] as std::ffi::c_int
            == 'K' as i32 & 0o37 as std::ffi::c_int
    {
        return tstr_control_k.as_mut_ptr();
    }
    return buf.as_mut_ptr();
}
unsafe extern "C" fn issp(mut ch: std::ffi::c_char) -> std::ffi::c_int {
    return (ch as std::ffi::c_int == ' ' as i32 || ch as std::ffi::c_int == '\t' as i32)
        as std::ffi::c_int;
}
unsafe extern "C" fn skipsp(mut s: *mut std::ffi::c_char) -> *mut std::ffi::c_char {
    while issp(*s) != 0 {
        s = s.offset(1);
    }
    return s;
}
unsafe extern "C" fn skipnsp(mut s: *mut std::ffi::c_char) -> *mut std::ffi::c_char {
    while *s as std::ffi::c_int != '\0' as i32 && issp(*s) == 0 {
        s = s.offset(1);
    }
    return s;
}
unsafe extern "C" fn clean_line(mut s: *mut std::ffi::c_char) -> *mut std::ffi::c_char {
    let mut i: std::ffi::c_int = 0;
    s = skipsp(s);
    i = 0 as std::ffi::c_int;
    while *s.offset(i as isize) as std::ffi::c_int != '\0' as i32
        && *s.offset(i as isize) as std::ffi::c_int != '\n' as i32
        && *s.offset(i as isize) as std::ffi::c_int != '\r' as i32
    {
        if *s.offset(i as isize) as std::ffi::c_int == '#' as i32
            && (i == 0 as std::ffi::c_int
                || *s.offset((i - 1 as std::ffi::c_int) as isize) as std::ffi::c_int != '\\' as i32)
        {
            break;
        }
        i += 1;
    }
    *s.offset(i as isize) = '\0' as i32 as std::ffi::c_char;
    return s;
}
unsafe extern "C" fn add_cmd_char(mut c: std::ffi::c_uchar, mut tables: *mut lesskey_tables) {
    xbuf_add_byte(&mut (*(*tables).currtable).buf, c);
}
unsafe extern "C" fn erase_cmd_char(mut tables: *mut lesskey_tables) {
    xbuf_pop(&mut (*(*tables).currtable).buf);
}
unsafe extern "C" fn add_cmd_str(mut s: *const std::ffi::c_char, mut tables: *mut lesskey_tables) {
    while *s as std::ffi::c_int != '\0' as i32 {
        add_cmd_char(*s as std::ffi::c_uchar, tables);
        s = s.offset(1);
    }
}
unsafe extern "C" fn match_version(
    mut op: std::ffi::c_char,
    mut ver: std::ffi::c_int,
) -> std::ffi::c_int {
    match op as std::ffi::c_int {
        62 => return (less_version > ver) as std::ffi::c_int,
        60 => return (less_version < ver) as std::ffi::c_int,
        43 => return (less_version >= ver) as std::ffi::c_int,
        45 => return (less_version <= ver) as std::ffi::c_int,
        61 => return (less_version == ver) as std::ffi::c_int,
        33 => return (less_version != ver) as std::ffi::c_int,
        _ => return 0 as std::ffi::c_int,
    };
}
unsafe extern "C" fn version_line(mut s: *mut std::ffi::c_char) -> *mut std::ffi::c_char {
    let mut op: std::ffi::c_char = 0;
    let mut ver: std::ffi::c_int = 0;
    let mut e: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut buf: [std::ffi::c_char; 8] = [0; 8];
    s = s.offset(strlen(b"#version\0" as *const u8 as *const std::ffi::c_char) as isize);
    s = skipsp(s);
    let fresh0 = s;
    s = s.offset(1);
    op = *fresh0;
    match op as std::ffi::c_int {
        60 => {
            if *s as std::ffi::c_int == '=' as i32 {
                s = s.offset(1);
                op = '-' as i32 as std::ffi::c_char;
            }
        }
        62 => {
            if *s as std::ffi::c_int == '=' as i32 {
                s = s.offset(1);
                op = '+' as i32 as std::ffi::c_char;
            }
        }
        61 => {
            if *s as std::ffi::c_int == '=' as i32 {
                s = s.offset(1);
            }
        }
        33 => {
            if *s as std::ffi::c_int == '=' as i32 {
                s = s.offset(1);
            }
        }
        _ => {
            parse_error(
                b"invalid operator '%s' in #version line\0" as *const u8 as *const std::ffi::c_char,
                char_string(buf.as_mut_ptr(), op, 0 as std::ffi::c_int),
            );
            return 0 as *mut std::ffi::c_char;
        }
    }
    s = skipsp(s);
    ver = lstrtoi(s, &mut e, 10 as std::ffi::c_int);
    if e == s {
        parse_error(
            b"non-numeric version number in #version line\0" as *const u8
                as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
        );
        return 0 as *mut std::ffi::c_char;
    }
    if match_version(op, ver) == 0 {
        return 0 as *mut std::ffi::c_char;
    }
    return e;
}
unsafe extern "C" fn control_line(
    mut s: *mut std::ffi::c_char,
    mut tables: *mut lesskey_tables,
) -> *mut std::ffi::c_char {
    if strncmp(
        s,
        b"#line-edit\0" as *const u8 as *const std::ffi::c_char,
        strlen(b"#line-edit\0" as *const u8 as *const std::ffi::c_char),
    ) == 0 as std::ffi::c_int
    {
        (*tables).currtable = &mut (*tables).edittable;
        return 0 as *mut std::ffi::c_char;
    }
    if strncmp(
        s,
        b"#command\0" as *const u8 as *const std::ffi::c_char,
        strlen(b"#command\0" as *const u8 as *const std::ffi::c_char),
    ) == 0 as std::ffi::c_int
    {
        (*tables).currtable = &mut (*tables).cmdtable;
        return 0 as *mut std::ffi::c_char;
    }
    if strncmp(
        s,
        b"#env\0" as *const u8 as *const std::ffi::c_char,
        strlen(b"#env\0" as *const u8 as *const std::ffi::c_char),
    ) == 0 as std::ffi::c_int
    {
        (*tables).currtable = &mut (*tables).vartable;
        return 0 as *mut std::ffi::c_char;
    }
    if strncmp(
        s,
        b"#stop\0" as *const u8 as *const std::ffi::c_char,
        strlen(b"#stop\0" as *const u8 as *const std::ffi::c_char),
    ) == 0 as std::ffi::c_int
    {
        add_cmd_char('\0' as i32 as std::ffi::c_uchar, tables);
        add_cmd_char(103 as std::ffi::c_int as std::ffi::c_uchar, tables);
        return 0 as *mut std::ffi::c_char;
    }
    if strncmp(
        s,
        b"#version\0" as *const u8 as *const std::ffi::c_char,
        strlen(b"#version\0" as *const u8 as *const std::ffi::c_char),
    ) == 0 as std::ffi::c_int
    {
        return version_line(s);
    }
    return s;
}
unsafe extern "C" fn findaction(
    mut actname: *mut std::ffi::c_char,
    mut tables: *mut lesskey_tables,
) -> std::ffi::c_int {
    let mut i: std::ffi::c_int = 0;
    i = 0 as std::ffi::c_int;
    while !((*((*(*tables).currtable).names).offset(i as isize)).cn_name).is_null() {
        if strcmp(
            (*((*(*tables).currtable).names).offset(i as isize)).cn_name,
            actname,
        ) == 0 as std::ffi::c_int
        {
            return (*((*(*tables).currtable).names).offset(i as isize)).cn_action;
        }
        i += 1;
    }
    parse_error(
        b"unknown action: \"%s\"\0" as *const u8 as *const std::ffi::c_char,
        actname,
    );
    return 100 as std::ffi::c_int;
}
unsafe extern "C" fn parse_cmdline(mut p: *mut std::ffi::c_char, mut tables: *mut lesskey_tables) {
    let mut actname: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut action: std::ffi::c_int = 0;
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut c: std::ffi::c_char = 0;
    loop {
        s = tstr(&mut p, 1 as std::ffi::c_int);
        add_cmd_str(s, tables);
        if !(*p as std::ffi::c_int != '\0' as i32 && issp(*p) == 0) {
            break;
        }
    }
    add_cmd_char('\0' as i32 as std::ffi::c_uchar, tables);
    p = skipsp(p);
    if *p as std::ffi::c_int == '\0' as i32 {
        parse_error(
            b"missing action\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
        );
        return;
    }
    actname = p;
    p = skipnsp(p);
    c = *p;
    *p = '\0' as i32 as std::ffi::c_char;
    action = findaction(actname, tables);
    *p = c;
    p = skipsp(p);
    if *p as std::ffi::c_int == '\0' as i32 {
        add_cmd_char(action as std::ffi::c_uchar, tables);
    } else {
        add_cmd_char(
            (action | 0o200 as std::ffi::c_int) as std::ffi::c_uchar,
            tables,
        );
        while *p as std::ffi::c_int != '\0' as i32 {
            add_cmd_str(tstr(&mut p, 0 as std::ffi::c_int), tables);
        }
        add_cmd_char('\0' as i32 as std::ffi::c_uchar, tables);
    };
}
unsafe extern "C" fn parse_varline(
    mut line: *mut std::ffi::c_char,
    mut tables: *mut lesskey_tables,
) {
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut p: *mut std::ffi::c_char = line;
    let mut eq: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    eq = strchr(line, '=' as i32);
    if !eq.is_null()
        && eq > line
        && *eq.offset(-(1 as std::ffi::c_int) as isize) as std::ffi::c_int == '+' as i32
    {
        erase_cmd_char(tables);
        p = eq.offset(1 as std::ffi::c_int as isize);
    } else {
        loop {
            s = tstr(&mut p, 0 as std::ffi::c_int);
            add_cmd_str(s, tables);
            if !(*p as std::ffi::c_int != '\0' as i32
                && issp(*p) == 0
                && *p as std::ffi::c_int != '=' as i32)
            {
                break;
            }
        }
        add_cmd_char('\0' as i32 as std::ffi::c_uchar, tables);
        p = skipsp(p);
        let fresh1 = p;
        p = p.offset(1);
        if *fresh1 as std::ffi::c_int != '=' as i32 {
            parse_error(
                b"missing = in variable definition\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
            );
            return;
        }
        add_cmd_char(
            (0o1 as std::ffi::c_int | 0o200 as std::ffi::c_int) as std::ffi::c_uchar,
            tables,
        );
    }
    p = skipsp(p);
    while *p as std::ffi::c_int != '\0' as i32 {
        s = tstr(&mut p, 0 as std::ffi::c_int);
        add_cmd_str(s, tables);
    }
    add_cmd_char('\0' as i32 as std::ffi::c_uchar, tables);
}
unsafe extern "C" fn parse_line(mut line: *mut std::ffi::c_char, mut tables: *mut lesskey_tables) {
    let mut p: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    p = control_line(line, tables);
    if p.is_null() {
        return;
    }
    p = clean_line(p);
    if *p as std::ffi::c_int == '\0' as i32 {
        return;
    }
    if (*(*tables).currtable).is_var != 0 {
        parse_varline(p, tables);
    } else {
        parse_cmdline(p, tables);
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_lesskey(
    mut infile: *const std::ffi::c_char,
    mut tables: *mut lesskey_tables,
) -> std::ffi::c_int {
    let mut desc: *mut FILE = 0 as *mut FILE;
    let mut line: [std::ffi::c_char; 1024] = [0; 1024];
    lesskey_file = if !infile.is_null() {
        strdup(infile)
    } else {
        homefile(b".lesskey\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char)
    };
    if lesskey_file.is_null() {
        return -(1 as std::ffi::c_int);
    }
    init_tables(tables);
    errors = 0 as std::ffi::c_int;
    linenum = 0 as std::ffi::c_int;
    if less_version == 0 as std::ffi::c_int {
        less_version = lstrtoi(
            version.as_mut_ptr(),
            0 as *mut *mut std::ffi::c_char,
            10 as std::ffi::c_int,
        );
    }
    if strcmp(lesskey_file, b"-\0" as *const u8 as *const std::ffi::c_char) == 0 as std::ffi::c_int
    {
        desc = stdin;
    } else {
        desc = fopen(lesskey_file, b"r\0" as *const u8 as *const std::ffi::c_char);
        if desc.is_null() {
            errors = -(1 as std::ffi::c_int);
        }
    }
    if !desc.is_null() {
        while !(fgets(
            line.as_mut_ptr(),
            ::core::mem::size_of::<[std::ffi::c_char; 1024]>() as std::ffi::c_ulong
                as std::ffi::c_int,
            desc,
        ))
        .is_null()
        {
            linenum += 1;
            parse_line(line.as_mut_ptr(), tables);
        }
        if desc != stdin {
            fclose(desc);
        }
    }
    free(lesskey_file as *mut std::ffi::c_void);
    lesskey_file = 0 as *mut std::ffi::c_char;
    return errors;
}
#[no_mangle]
pub unsafe extern "C" fn parse_lesskey_content(
    mut content: *const std::ffi::c_char,
    mut tables: *mut lesskey_tables,
) -> std::ffi::c_int {
    let mut cx: size_t = 0 as std::ffi::c_int as size_t;
    lesskey_file =
        b"lesskey-content\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char;
    init_tables(tables);
    errors = 0 as std::ffi::c_int;
    linenum = 0 as std::ffi::c_int;
    if less_version == 0 as std::ffi::c_int {
        less_version = lstrtoi(
            version.as_mut_ptr(),
            0 as *mut *mut std::ffi::c_char,
            10 as std::ffi::c_int,
        );
    }
    while *content.offset(cx as isize) as std::ffi::c_int != '\0' as i32 {
        let mut line: [std::ffi::c_char; 1024] = [0; 1024];
        let mut lx: size_t = 0 as std::ffi::c_int as size_t;
        while *content.offset(cx as isize) as std::ffi::c_int != '\0' as i32
            && *content.offset(cx as isize) as std::ffi::c_int != '\n' as i32
            && *content.offset(cx as isize) as std::ffi::c_int != ';' as i32
        {
            if lx
                >= (::core::mem::size_of::<[std::ffi::c_char; 1024]>() as std::ffi::c_ulong)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
            {
                break;
            }
            if *content.offset(cx as isize) as std::ffi::c_int == '\\' as i32
                && *content.offset(cx.wrapping_add(1 as std::ffi::c_int as size_t) as isize)
                    as std::ffi::c_int
                    == ';' as i32
            {
                cx = cx.wrapping_add(1);
            }
            let fresh2 = cx;
            cx = cx.wrapping_add(1);
            let fresh3 = lx;
            lx = lx.wrapping_add(1);
            line[fresh3 as usize] = *content.offset(fresh2 as isize);
        }
        line[lx as usize] = '\0' as i32 as std::ffi::c_char;
        linenum += 1;
        parse_line(line.as_mut_ptr(), tables);
        if *content.offset(cx as isize) as std::ffi::c_int != '\0' as i32 {
            cx = cx.wrapping_add(1);
        }
    }
    lesskey_file = 0 as *mut std::ffi::c_char;
    return errors;
}
