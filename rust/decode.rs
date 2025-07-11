use ::libc;
use std::env;
use std::env::VarError;
use std::ffi::CString;
extern "C" {
    fn open(__file: *const std::ffi::c_char, __oflag: std::ffi::c_int, _: ...) -> std::ffi::c_int;
    fn lseek(__fd: std::ffi::c_int, __offset: __off_t, __whence: std::ffi::c_int) -> __off_t;
    fn close(__fd: std::ffi::c_int) -> std::ffi::c_int;
    fn read(__fd: std::ffi::c_int, __buf: *mut std::ffi::c_void, __nbytes: size_t) -> ssize_t;
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn getenv(__name: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strncmp(
        _: *const std::ffi::c_char,
        _: *const std::ffi::c_char,
        _: std::ffi::c_ulong,
    ) -> std::ffi::c_int;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn xbuf_init(xbuf: *mut xbuffer);
    fn save(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn secure_allow(features: std::ffi::c_int) -> std::ffi::c_int;
    fn special_key_str(key: std::ffi::c_int) -> *const std::ffi::c_char;
    fn cmd_exec();
    fn screen_trashed();
    fn getcc() -> std::ffi::c_char;
    fn ungetcc(c: std::ffi::c_char);
    fn ungetsc(s: *const std::ffi::c_char);
    fn expand_evars(buf: *mut std::ffi::c_char, len: size_t, xbuf: *mut xbuffer);
    fn dirfile(
        dirname: *const std::ffi::c_char,
        filename: *const std::ffi::c_char,
        must_exist: std::ffi::c_int,
    ) -> *mut std::ffi::c_char;
    fn homefile(filename: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn filesize(f: std::ffi::c_int) -> POSITION;
    fn forward(n: std::ffi::c_int, force: lbool, only_last: lbool, to_newline: lbool);
    fn backward(n: std::ffi::c_int, force: lbool, only_last: lbool, to_newline: lbool);
    fn setmark(c: std::ffi::c_char, where_0: std::ffi::c_int);
    fn gomark(c: std::ffi::c_char);
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn osc8_click(sindex: std::ffi::c_int, col: std::ffi::c_int) -> lbool;
    fn parse_lesskey(
        infile: *const std::ffi::c_char,
        tables: *mut lesskey_tables,
    ) -> std::ffi::c_int;
    fn parse_lesskey_content(
        content: *const std::ffi::c_char,
        tables: *mut lesskey_tables,
    ) -> std::ffi::c_int;
    static mut erase_char: std::ffi::c_int;
    static mut erase2_char: std::ffi::c_int;
    static mut kill_char: std::ffi::c_int;
    static mut mousecap: std::ffi::c_int;
    static mut sc_height: std::ffi::c_int;
}
pub type __off_t = std::ffi::c_long;
pub type __ssize_t = std::ffi::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
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
pub struct xbuffer {
    pub data: *mut std::ffi::c_uchar,
    pub end: size_t,
    pub size: size_t,
    pub init_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tablelist {
    pub t_next: *mut tablelist,
    pub t_start: *mut std::ffi::c_uchar,
    pub t_end: *mut std::ffi::c_uchar,
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
pub struct lesskey_cmdname {
    pub cn_name: *const std::ffi::c_char,
    pub cn_action: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lesskey_tables {
    pub currtable: *mut lesskey_table,
    pub cmdtable: lesskey_table,
    pub edittable: lesskey_table,
    pub vartable: lesskey_table,
}
static mut allow_drag: lbool = LTRUE;
static mut cmdtable: [std::ffi::c_uchar; 556] = [
    '\r' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    12 as std::ffi::c_int as std::ffi::c_uchar,
    '\n' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    12 as std::ffi::c_int as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    12 as std::ffi::c_int as std::ffi::c_uchar,
    'j' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    12 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    4 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    12 as std::ffi::c_int as std::ffi::c_uchar,
    ('E' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    12 as std::ffi::c_int as std::ffi::c_uchar,
    ('N' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    12 as std::ffi::c_int as std::ffi::c_uchar,
    'k' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    2 as std::ffi::c_int as std::ffi::c_uchar,
    'y' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    2 as std::ffi::c_int as std::ffi::c_uchar,
    ('Y' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    2 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    40 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    2 as std::ffi::c_int as std::ffi::c_uchar,
    ('P' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    2 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    3 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    2 as std::ffi::c_int as std::ffi::c_uchar,
    'J' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    29 as std::ffi::c_int as std::ffi::c_uchar,
    'K' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    30 as std::ffi::c_int as std::ffi::c_uchar,
    'Y' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    30 as std::ffi::c_int as std::ffi::c_uchar,
    'd' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    14 as std::ffi::c_int as std::ffi::c_uchar,
    ('D' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    14 as std::ffi::c_int as std::ffi::c_uchar,
    'u' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    4 as std::ffi::c_int as std::ffi::c_uchar,
    ('U' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    4 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    '[' as i32 as std::ffi::c_uchar,
    'M' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    64 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    '[' as i32 as std::ffi::c_uchar,
    '<' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    68 as std::ffi::c_int as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    13 as std::ffi::c_int as std::ffi::c_uchar,
    'f' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    13 as std::ffi::c_int as std::ffi::c_uchar,
    ('F' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    13 as std::ffi::c_int as std::ffi::c_uchar,
    ('V' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    13 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    13 as std::ffi::c_int as std::ffi::c_uchar,
    'b' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    3 as std::ffi::c_int as std::ffi::c_uchar,
    ('B' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    3 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'v' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    3 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    5 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    3 as std::ffi::c_int as std::ffi::c_uchar,
    'z' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    33 as std::ffi::c_int as std::ffi::c_uchar,
    'w' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    34 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    40 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'b' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    22 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'j' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    60 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'k' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    61 as std::ffi::c_int as std::ffi::c_uchar,
    'F' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    50 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'F' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    56 as std::ffi::c_int as std::ffi::c_uchar,
    'R' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    11 as std::ffi::c_int as std::ffi::c_uchar,
    'r' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    25 as std::ffi::c_int as std::ffi::c_uchar,
    ('R' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    25 as std::ffi::c_int as std::ffi::c_uchar,
    ('L' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    25 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'u' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    39 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'U' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    70 as std::ffi::c_int as std::ffi::c_uchar,
    'g' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    17 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    7 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    17 as std::ffi::c_int as std::ffi::c_uchar,
    '<' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    17 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    '<' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    17 as std::ffi::c_int as std::ffi::c_uchar,
    'p' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    21 as std::ffi::c_int as std::ffi::c_uchar,
    '%' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    21 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    '(' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    41 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    ')' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    42 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    '{' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    58 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    '}' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    59 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    42 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    2 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    41 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    12 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    59 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    11 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    58 as std::ffi::c_int as std::ffi::c_uchar,
    '{' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    (35 as std::ffi::c_int | 0o200 as std::ffi::c_int) as std::ffi::c_uchar,
    '{' as i32 as std::ffi::c_uchar,
    '}' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    '}' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    (36 as std::ffi::c_int | 0o200 as std::ffi::c_int) as std::ffi::c_uchar,
    '{' as i32 as std::ffi::c_uchar,
    '}' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    '(' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    (35 as std::ffi::c_int | 0o200 as std::ffi::c_int) as std::ffi::c_uchar,
    '(' as i32 as std::ffi::c_uchar,
    ')' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    ')' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    (36 as std::ffi::c_int | 0o200 as std::ffi::c_int) as std::ffi::c_uchar,
    '(' as i32 as std::ffi::c_uchar,
    ')' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    '[' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    (35 as std::ffi::c_int | 0o200 as std::ffi::c_int) as std::ffi::c_uchar,
    '[' as i32 as std::ffi::c_uchar,
    ']' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    ']' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    (36 as std::ffi::c_int | 0o200 as std::ffi::c_int) as std::ffi::c_uchar,
    '[' as i32 as std::ffi::c_uchar,
    ']' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    ('F' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    35 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    ('B' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    36 as std::ffi::c_int as std::ffi::c_uchar,
    'G' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    16 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'G' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    57 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    '>' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    16 as std::ffi::c_int as std::ffi::c_uchar,
    '>' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    16 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    8 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    16 as std::ffi::c_int as std::ffi::c_uchar,
    'P' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    51 as std::ffi::c_int as std::ffi::c_uchar,
    '0' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    '1' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    '2' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    '3' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    '4' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    '5' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    '6' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    '7' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    '8' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    '9' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    '.' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    '=' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    28 as std::ffi::c_int as std::ffi::c_uchar,
    ('G' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    28 as std::ffi::c_int as std::ffi::c_uchar,
    ':' as i32 as std::ffi::c_uchar,
    'f' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    28 as std::ffi::c_int as std::ffi::c_uchar,
    '/' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    15 as std::ffi::c_int as std::ffi::c_uchar,
    '?' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    5 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    '/' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    (15 as std::ffi::c_int | 0o200 as std::ffi::c_int) as std::ffi::c_uchar,
    '*' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    '?' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    (5 as std::ffi::c_int | 0o200 as std::ffi::c_int) as std::ffi::c_uchar,
    '*' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    'n' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    43 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'n' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    44 as std::ffi::c_int as std::ffi::c_uchar,
    'N' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    45 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'N' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    46 as std::ffi::c_int as std::ffi::c_uchar,
    '&' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    55 as std::ffi::c_int as std::ffi::c_uchar,
    'm' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    26 as std::ffi::c_int as std::ffi::c_uchar,
    'M' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    63 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'm' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    62 as std::ffi::c_int as std::ffi::c_uchar,
    '\'' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    18 as std::ffi::c_int as std::ffi::c_uchar,
    ('X' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    ('X' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    18 as std::ffi::c_int as std::ffi::c_uchar,
    'E' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    9 as std::ffi::c_int as std::ffi::c_uchar,
    ':' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    9 as std::ffi::c_int as std::ffi::c_uchar,
    ('X' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    ('V' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    9 as std::ffi::c_int as std::ffi::c_uchar,
    ':' as i32 as std::ffi::c_uchar,
    'n' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    20 as std::ffi::c_int as std::ffi::c_uchar,
    ':' as i32 as std::ffi::c_uchar,
    'p' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    23 as std::ffi::c_int as std::ffi::c_uchar,
    ('O' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    ('N' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    71 as std::ffi::c_int as std::ffi::c_uchar,
    ('O' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'n' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    71 as std::ffi::c_int as std::ffi::c_uchar,
    ('O' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    ('P' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    72 as std::ffi::c_int as std::ffi::c_uchar,
    ('O' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'p' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    72 as std::ffi::c_int as std::ffi::c_uchar,
    ('O' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    ('O' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    73 as std::ffi::c_int as std::ffi::c_uchar,
    ('O' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    73 as std::ffi::c_int as std::ffi::c_uchar,
    ('O' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    ('L' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    74 as std::ffi::c_int as std::ffi::c_uchar,
    ('O' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'l' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    74 as std::ffi::c_int as std::ffi::c_uchar,
    't' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    53 as std::ffi::c_int as std::ffi::c_uchar,
    'T' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    54 as std::ffi::c_int as std::ffi::c_uchar,
    ':' as i32 as std::ffi::c_uchar,
    'x' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    38 as std::ffi::c_int as std::ffi::c_uchar,
    ':' as i32 as std::ffi::c_uchar,
    'd' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    52 as std::ffi::c_int as std::ffi::c_uchar,
    '-' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    47 as std::ffi::c_int as std::ffi::c_uchar,
    ':' as i32 as std::ffi::c_uchar,
    't' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    (47 as std::ffi::c_int | 0o200 as std::ffi::c_int) as std::ffi::c_uchar,
    't' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    's' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    (47 as std::ffi::c_int | 0o200 as std::ffi::c_int) as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    '_' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    7 as std::ffi::c_int as std::ffi::c_uchar,
    '|' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    37 as std::ffi::c_int as std::ffi::c_uchar,
    'v' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    32 as std::ffi::c_int as std::ffi::c_uchar,
    '!' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    27 as std::ffi::c_int as std::ffi::c_uchar,
    '#' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    69 as std::ffi::c_int as std::ffi::c_uchar,
    '+' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    10 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    '[' as i32 as std::ffi::c_uchar,
    '2' as i32 as std::ffi::c_uchar,
    '0' as i32 as std::ffi::c_uchar,
    '0' as i32 as std::ffi::c_uchar,
    '~' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    75 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    '[' as i32 as std::ffi::c_uchar,
    '2' as i32 as std::ffi::c_uchar,
    '0' as i32 as std::ffi::c_uchar,
    '1' as i32 as std::ffi::c_uchar,
    '~' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    76 as std::ffi::c_int as std::ffi::c_uchar,
    'H' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    19 as std::ffi::c_int as std::ffi::c_uchar,
    'h' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    19 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    14 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    19 as std::ffi::c_int as std::ffi::c_uchar,
    'V' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    31 as std::ffi::c_int as std::ffi::c_uchar,
    'q' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    24 as std::ffi::c_int as std::ffi::c_uchar,
    'Q' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    24 as std::ffi::c_int as std::ffi::c_uchar,
    ':' as i32 as std::ffi::c_uchar,
    'q' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    24 as std::ffi::c_int as std::ffi::c_uchar,
    ':' as i32 as std::ffi::c_uchar,
    'Q' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    24 as std::ffi::c_int as std::ffi::c_uchar,
    'Z' as i32 as std::ffi::c_uchar,
    'Z' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    24 as std::ffi::c_int as std::ffi::c_uchar,
];
static mut edittable: [std::ffi::c_uchar; 232] = [
    '\t' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    17 as std::ffi::c_int as std::ffi::c_uchar,
    '\u{f}' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    18 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    15 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    18 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    '\t' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    18 as std::ffi::c_int as std::ffi::c_uchar,
    ('L' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    15 as std::ffi::c_int as std::ffi::c_uchar,
    ('V' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    19 as std::ffi::c_int as std::ffi::c_uchar,
    ('A' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    19 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'l' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    3 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    3 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'h' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    4 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    2 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    4 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'b' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    5 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    2 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    5 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    11 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    5 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'w' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    12 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'i' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    7 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    10 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    7 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'x' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    8 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    9 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    8 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'X' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    12 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    9 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    12 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    13 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    12 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    16 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    11 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    17 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    11 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    '0' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    9 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    7 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    9 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    '$' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    10 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    8 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    10 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'k' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    13 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    3 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    13 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    'j' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    14 as std::ffi::c_int as std::ffi::c_uchar,
    ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    4 as std::ffi::c_int as std::ffi::c_uchar,
    6 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    1 as std::ffi::c_int as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    14 as std::ffi::c_int as std::ffi::c_uchar,
    ('G' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    20 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    '[' as i32 as std::ffi::c_uchar,
    'M' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    21 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    '[' as i32 as std::ffi::c_uchar,
    '<' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    22 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    '[' as i32 as std::ffi::c_uchar,
    '2' as i32 as std::ffi::c_uchar,
    '0' as i32 as std::ffi::c_uchar,
    '0' as i32 as std::ffi::c_uchar,
    '~' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    75 as std::ffi::c_int as std::ffi::c_uchar,
    ('[' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_uchar,
    '[' as i32 as std::ffi::c_uchar,
    '2' as i32 as std::ffi::c_uchar,
    '0' as i32 as std::ffi::c_uchar,
    '1' as i32 as std::ffi::c_uchar,
    '~' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    76 as std::ffi::c_int as std::ffi::c_uchar,
];
static mut dflt_vartable: [std::ffi::c_uchar; 460] = [
    'L' as i32 as std::ffi::c_uchar,
    'E' as i32 as std::ffi::c_uchar,
    'S' as i32 as std::ffi::c_uchar,
    'S' as i32 as std::ffi::c_uchar,
    '_' as i32 as std::ffi::c_uchar,
    'O' as i32 as std::ffi::c_uchar,
    'S' as i32 as std::ffi::c_uchar,
    'C' as i32 as std::ffi::c_uchar,
    '8' as i32 as std::ffi::c_uchar,
    '_' as i32 as std::ffi::c_uchar,
    'm' as i32 as std::ffi::c_uchar,
    'a' as i32 as std::ffi::c_uchar,
    'n' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    (0o1 as std::ffi::c_int | 0o200 as std::ffi::c_int) as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    'c' as i32 as std::ffi::c_uchar,
    'h' as i32 as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '\'' as i32 as std::ffi::c_uchar,
    '%' as i32 as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    '\'' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '|' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    's' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    'd' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '-' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    's' as i32 as std::ffi::c_uchar,
    ',' as i32 as std::ffi::c_uchar,
    '^' as i32 as std::ffi::c_uchar,
    'm' as i32 as std::ffi::c_uchar,
    'a' as i32 as std::ffi::c_uchar,
    'n' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    ':' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '(' as i32 as std::ffi::c_uchar,
    '[' as i32 as std::ffi::c_uchar,
    '^' as i32 as std::ffi::c_uchar,
    '(' as i32 as std::ffi::c_uchar,
    ']' as i32 as std::ffi::c_uchar,
    '*' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    ')' as i32 as std::ffi::c_uchar,
    '(' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '*' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '(' as i32 as std::ffi::c_uchar,
    '[' as i32 as std::ffi::c_uchar,
    '^' as i32 as std::ffi::c_uchar,
    ')' as i32 as std::ffi::c_uchar,
    ']' as i32 as std::ffi::c_uchar,
    '*' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    ')' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '.' as i32 as std::ffi::c_uchar,
    '*' as i32 as std::ffi::c_uchar,
    ',' as i32 as std::ffi::c_uchar,
    '-' as i32 as std::ffi::c_uchar,
    'm' as i32 as std::ffi::c_uchar,
    'a' as i32 as std::ffi::c_uchar,
    'n' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '\'' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '2' as i32 as std::ffi::c_uchar,
    '\'' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '\'' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '1' as i32 as std::ffi::c_uchar,
    '\'' as i32 as std::ffi::c_uchar,
    ',' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '-' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    't' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'X' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '-' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    's' as i32 as std::ffi::c_uchar,
    ',' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '.' as i32 as std::ffi::c_uchar,
    '*' as i32 as std::ffi::c_uchar,
    ',' as i32 as std::ffi::c_uchar,
    '-' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    'c' as i32 as std::ffi::c_uchar,
    'h' as i32 as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'I' as i32 as std::ffi::c_uchar,
    'n' as i32 as std::ffi::c_uchar,
    'v' as i32 as std::ffi::c_uchar,
    'a' as i32 as std::ffi::c_uchar,
    'l' as i32 as std::ffi::c_uchar,
    'i' as i32 as std::ffi::c_uchar,
    'd' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'm' as i32 as std::ffi::c_uchar,
    'a' as i32 as std::ffi::c_uchar,
    'n' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'l' as i32 as std::ffi::c_uchar,
    'i' as i32 as std::ffi::c_uchar,
    'n' as i32 as std::ffi::c_uchar,
    'k' as i32 as std::ffi::c_uchar,
    ',' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '-' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    ':' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'X' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    'L' as i32 as std::ffi::c_uchar,
    'E' as i32 as std::ffi::c_uchar,
    'S' as i32 as std::ffi::c_uchar,
    'S' as i32 as std::ffi::c_uchar,
    '_' as i32 as std::ffi::c_uchar,
    'O' as i32 as std::ffi::c_uchar,
    'S' as i32 as std::ffi::c_uchar,
    'C' as i32 as std::ffi::c_uchar,
    '8' as i32 as std::ffi::c_uchar,
    '_' as i32 as std::ffi::c_uchar,
    'f' as i32 as std::ffi::c_uchar,
    'i' as i32 as std::ffi::c_uchar,
    'l' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
    (0o1 as std::ffi::c_int | 0o200 as std::ffi::c_int) as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    'v' as i32 as std::ffi::c_uchar,
    'a' as i32 as std::ffi::c_uchar,
    'l' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '`' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    'c' as i32 as std::ffi::c_uchar,
    'h' as i32 as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '\'' as i32 as std::ffi::c_uchar,
    '%' as i32 as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    '\'' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '|' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    's' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    'd' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '-' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    's' as i32 as std::ffi::c_uchar,
    ',' as i32 as std::ffi::c_uchar,
    '^' as i32 as std::ffi::c_uchar,
    'f' as i32 as std::ffi::c_uchar,
    'i' as i32 as std::ffi::c_uchar,
    'l' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    ':' as i32 as std::ffi::c_uchar,
    '/' as i32 as std::ffi::c_uchar,
    '/' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '(' as i32 as std::ffi::c_uchar,
    '[' as i32 as std::ffi::c_uchar,
    '^' as i32 as std::ffi::c_uchar,
    '/' as i32 as std::ffi::c_uchar,
    ']' as i32 as std::ffi::c_uchar,
    '*' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    ')' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '(' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '.' as i32 as std::ffi::c_uchar,
    '*' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    ')' as i32 as std::ffi::c_uchar,
    ',' as i32 as std::ffi::c_uchar,
    '_' as i32 as std::ffi::c_uchar,
    'H' as i32 as std::ffi::c_uchar,
    '=' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '1' as i32 as std::ffi::c_uchar,
    ';' as i32 as std::ffi::c_uchar,
    '_' as i32 as std::ffi::c_uchar,
    'P' as i32 as std::ffi::c_uchar,
    '=' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '2' as i32 as std::ffi::c_uchar,
    ';' as i32 as std::ffi::c_uchar,
    '_' as i32 as std::ffi::c_uchar,
    'E' as i32 as std::ffi::c_uchar,
    '=' as i32 as std::ffi::c_uchar,
    '0' as i32 as std::ffi::c_uchar,
    ',' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '-' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    't' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'X' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '-' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    's' as i32 as std::ffi::c_uchar,
    ',' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    '.' as i32 as std::ffi::c_uchar,
    '*' as i32 as std::ffi::c_uchar,
    ',' as i32 as std::ffi::c_uchar,
    '_' as i32 as std::ffi::c_uchar,
    'E' as i32 as std::ffi::c_uchar,
    '=' as i32 as std::ffi::c_uchar,
    '1' as i32 as std::ffi::c_uchar,
    ',' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '-' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    ':' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'X' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    '`' as i32 as std::ffi::c_uchar,
    ';' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'i' as i32 as std::ffi::c_uchar,
    'f' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '[' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    '$' as i32 as std::ffi::c_uchar,
    '_' as i32 as std::ffi::c_uchar,
    'E' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '=' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '1' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    ']' as i32 as std::ffi::c_uchar,
    ';' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    't' as i32 as std::ffi::c_uchar,
    'h' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    'n' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    'c' as i32 as std::ffi::c_uchar,
    'h' as i32 as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '-' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    'c' as i32 as std::ffi::c_uchar,
    'h' as i32 as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'I' as i32 as std::ffi::c_uchar,
    'n' as i32 as std::ffi::c_uchar,
    'v' as i32 as std::ffi::c_uchar,
    'a' as i32 as std::ffi::c_uchar,
    'l' as i32 as std::ffi::c_uchar,
    'i' as i32 as std::ffi::c_uchar,
    'd' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'f' as i32 as std::ffi::c_uchar,
    'i' as i32 as std::ffi::c_uchar,
    'l' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'l' as i32 as std::ffi::c_uchar,
    'i' as i32 as std::ffi::c_uchar,
    'n' as i32 as std::ffi::c_uchar,
    'k' as i32 as std::ffi::c_uchar,
    ';' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    'l' as i32 as std::ffi::c_uchar,
    'i' as i32 as std::ffi::c_uchar,
    'f' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '[' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '-' as i32 as std::ffi::c_uchar,
    'z' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    '$' as i32 as std::ffi::c_uchar,
    '_' as i32 as std::ffi::c_uchar,
    'H' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '-' as i32 as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    '$' as i32 as std::ffi::c_uchar,
    '_' as i32 as std::ffi::c_uchar,
    'H' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '=' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'l' as i32 as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    'c' as i32 as std::ffi::c_uchar,
    'a' as i32 as std::ffi::c_uchar,
    'l' as i32 as std::ffi::c_uchar,
    'h' as i32 as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    's' as i32 as std::ffi::c_uchar,
    't' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '-' as i32 as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    '$' as i32 as std::ffi::c_uchar,
    '_' as i32 as std::ffi::c_uchar,
    'H' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '=' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '$' as i32 as std::ffi::c_uchar,
    'H' as i32 as std::ffi::c_uchar,
    'O' as i32 as std::ffi::c_uchar,
    'S' as i32 as std::ffi::c_uchar,
    'T' as i32 as std::ffi::c_uchar,
    'N' as i32 as std::ffi::c_uchar,
    'A' as i32 as std::ffi::c_uchar,
    'M' as i32 as std::ffi::c_uchar,
    'E' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    ']' as i32 as std::ffi::c_uchar,
    ';' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    't' as i32 as std::ffi::c_uchar,
    'h' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    'n' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    'c' as i32 as std::ffi::c_uchar,
    'h' as i32 as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    '\\' as i32 as std::ffi::c_uchar,
    ':' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '$' as i32 as std::ffi::c_uchar,
    '_' as i32 as std::ffi::c_uchar,
    'P' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    ';' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    'l' as i32 as std::ffi::c_uchar,
    's' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    'c' as i32 as std::ffi::c_uchar,
    'h' as i32 as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '-' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    'c' as i32 as std::ffi::c_uchar,
    'h' as i32 as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'C' as i32 as std::ffi::c_uchar,
    'a' as i32 as std::ffi::c_uchar,
    'n' as i32 as std::ffi::c_uchar,
    'n' as i32 as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    't' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    'p' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    'n' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'r' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    'm' as i32 as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    't' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'f' as i32 as std::ffi::c_uchar,
    'i' as i32 as std::ffi::c_uchar,
    'l' as i32 as std::ffi::c_uchar,
    'e' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'o' as i32 as std::ffi::c_uchar,
    'n' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    '$' as i32 as std::ffi::c_uchar,
    '_' as i32 as std::ffi::c_uchar,
    'H' as i32 as std::ffi::c_uchar,
    '"' as i32 as std::ffi::c_uchar,
    ';' as i32 as std::ffi::c_uchar,
    ' ' as i32 as std::ffi::c_uchar,
    'f' as i32 as std::ffi::c_uchar,
    'i' as i32 as std::ffi::c_uchar,
    0 as std::ffi::c_int as std::ffi::c_uchar,
];
static mut list_fcmd_tables: *mut tablelist = 0 as *const tablelist as *mut tablelist;
static mut list_ecmd_tables: *mut tablelist = 0 as *const tablelist as *mut tablelist;
static mut list_var_tables: *mut tablelist = 0 as *const tablelist as *mut tablelist;
static mut list_sysvar_tables: *mut tablelist = 0 as *const tablelist as *mut tablelist;
unsafe extern "C" fn expand_special_keys(mut table: *mut std::ffi::c_uchar, mut len: size_t) {
    let mut fm: *mut std::ffi::c_uchar = 0 as *mut std::ffi::c_uchar;
    let mut to: *mut std::ffi::c_uchar = 0 as *mut std::ffi::c_uchar;
    let mut a: std::ffi::c_int = 0;
    let mut repl: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut klen: size_t = 0;
    fm = table;
    while fm < table.offset(len as isize) {
        to = fm;
        while *fm as std::ffi::c_int != '\0' as i32 {
            if *fm as std::ffi::c_int != 'K' as i32 & 0o37 as std::ffi::c_int {
                let fresh0 = fm;
                fm = fm.offset(1);
                let fresh1 = to;
                to = to.offset(1);
                *fresh1 = *fresh0;
            } else {
                repl =
                    special_key_str(*fm.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int);
                klen = (*fm.offset(2 as std::ffi::c_int as isize) as std::ffi::c_int
                    & 0o377 as std::ffi::c_int) as size_t;
                fm = fm.offset(klen as isize);
                if repl.is_null() || strlen(repl) > klen {
                    repl = b"\xFF\0" as *const u8 as *const std::ffi::c_char;
                }
                while *repl as std::ffi::c_int != '\0' as i32 {
                    let fresh2 = repl;
                    repl = repl.offset(1);
                    let fresh3 = to;
                    to = to.offset(1);
                    *fresh3 = *fresh2 as std::ffi::c_uchar;
                }
            }
        }
        let fresh4 = to;
        to = to.offset(1);
        *fresh4 = '\0' as i32 as std::ffi::c_uchar;
        while to <= fm {
            let fresh5 = to;
            to = to.offset(1);
            *fresh5 = 127 as std::ffi::c_int as std::ffi::c_uchar;
        }
        fm = fm.offset(1);
        let fresh6 = fm;
        fm = fm.offset(1);
        a = *fresh6 as std::ffi::c_int & 0o377 as std::ffi::c_int;
        if a & 0o200 as std::ffi::c_int != 0 {
            loop {
                let fresh7 = fm;
                fm = fm.offset(1);
                if !(*fresh7 as std::ffi::c_int != '\0' as i32) {
                    break;
                }
            }
        }
    }
}
unsafe extern "C" fn expand_cmd_table(mut tlist: *mut tablelist) {
    let mut t: *mut tablelist = 0 as *mut tablelist;
    t = tlist;
    while !t.is_null() {
        expand_special_keys(
            (*t).t_start,
            ((*t).t_end).offset_from((*t).t_start) as std::ffi::c_long as size_t,
        );
        t = (*t).t_next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn expand_cmd_tables() {
    expand_cmd_table(list_fcmd_tables);
    expand_cmd_table(list_ecmd_tables);
    expand_cmd_table(list_var_tables);
    expand_cmd_table(list_sysvar_tables);
}
#[no_mangle]
pub unsafe extern "C" fn init_cmds() {
    add_fcmd_table(
        cmdtable.as_mut_ptr(),
        ::core::mem::size_of::<[std::ffi::c_uchar; 556]>() as std::ffi::c_ulong,
    );
    add_ecmd_table(
        edittable.as_mut_ptr(),
        ::core::mem::size_of::<[std::ffi::c_uchar; 232]>() as std::ffi::c_ulong,
    );
    add_sysvar_table(
        dflt_vartable.as_mut_ptr(),
        ::core::mem::size_of::<[std::ffi::c_uchar; 460]>() as std::ffi::c_ulong,
    );
    add_hometable(
        Some(lesskey as unsafe extern "C" fn(*const std::ffi::c_char, lbool) -> std::ffi::c_int),
        None,
        b"/usr/local/bin/.sysless\0" as *const u8 as *const std::ffi::c_char,
        LTRUE,
    );
    if add_hometable(
        Some(
            lesskey_src as unsafe extern "C" fn(*const std::ffi::c_char, lbool) -> std::ffi::c_int,
        ),
        Some("LESSKEYIN_SYSTEM"),
        b"/usr/local/etc/syslesskey\0" as *const u8 as *const std::ffi::c_char,
        LTRUE,
    ) != 0 as std::ffi::c_int
    {
        add_hometable(
            Some(
                lesskey as unsafe extern "C" fn(*const std::ffi::c_char, lbool) -> std::ffi::c_int,
            ),
            Some("LESSKEY_SYSTEM"),
            b"/usr/local/etc/sysless\0" as *const u8 as *const std::ffi::c_char,
            LTRUE,
        );
    }
    if add_hometable(
        Some(
            lesskey_src as unsafe extern "C" fn(*const std::ffi::c_char, lbool) -> std::ffi::c_int,
        ),
        Some("LESSKEYIN"),
        b".lesskey\0" as *const u8 as *const std::ffi::c_char,
        LFALSE,
    ) != 0 as std::ffi::c_int
    {
        add_hometable(
            Some(
                lesskey as unsafe extern "C" fn(*const std::ffi::c_char, lbool) -> std::ffi::c_int,
            ),
            Some("LESSKEY"),
            b".less\0" as *const u8 as *const std::ffi::c_char,
            LFALSE,
        );
    }
    add_content_table(
        Some(
            lesskey_content
                as unsafe extern "C" fn(*const std::ffi::c_char, lbool) -> std::ffi::c_int,
        ),
        "LESSKEY_CONTENT_SYSTEM",
        LTRUE,
    );
    add_content_table(
        Some(
            lesskey_content
                as unsafe extern "C" fn(*const std::ffi::c_char, lbool) -> std::ffi::c_int,
        ),
        "LESSKEY_CONTENT",
        LFALSE,
    );
}
unsafe extern "C" fn add_cmd_table(
    mut tlist: *mut *mut tablelist,
    mut buf: *mut std::ffi::c_uchar,
    mut len: size_t,
) -> std::ffi::c_int {
    let mut t: *mut tablelist = 0 as *mut tablelist;
    if len == 0 as std::ffi::c_int as size_t {
        return 0 as std::ffi::c_int;
    }
    t = calloc(
        1 as std::ffi::c_int as std::ffi::c_ulong,
        ::core::mem::size_of::<tablelist>() as std::ffi::c_ulong,
    ) as *mut tablelist;
    if t.is_null() {
        return -(1 as std::ffi::c_int);
    }
    (*t).t_start = buf;
    (*t).t_end = buf.offset(len as isize);
    (*t).t_next = 0 as *mut tablelist;
    if (*tlist).is_null() {
        *tlist = t;
    } else {
        let mut e: *mut tablelist = 0 as *mut tablelist;
        e = *tlist;
        while !((*e).t_next).is_null() {
            e = (*e).t_next;
        }
        (*e).t_next = t;
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn pop_cmd_table(mut tlist: *mut *mut tablelist) {
    let mut t: *mut tablelist = 0 as *mut tablelist;
    if (*tlist).is_null() {
        return;
    }
    if ((**tlist).t_next).is_null() {
        t = *tlist;
        *tlist = 0 as *mut tablelist;
    } else {
        let mut e: *mut tablelist = 0 as *mut tablelist;
        e = *tlist;
        while !((*(*e).t_next).t_next).is_null() {
            e = (*e).t_next;
        }
        t = (*e).t_next;
        (*e).t_next = 0 as *mut tablelist;
    }
    free(t as *mut std::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn add_fcmd_table(mut buf: *mut std::ffi::c_uchar, mut len: size_t) {
    if add_cmd_table(&mut list_fcmd_tables, buf, len) < 0 as std::ffi::c_int {
        error(
            b"Warning: some commands disabled\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn add_ecmd_table(mut buf: *mut std::ffi::c_uchar, mut len: size_t) {
    if add_cmd_table(&mut list_ecmd_tables, buf, len) < 0 as std::ffi::c_int {
        error(
            b"Warning: some edit commands disabled\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
    }
}
unsafe extern "C" fn add_var_table(
    mut tlist: *mut *mut tablelist,
    mut buf: *mut std::ffi::c_uchar,
    mut len: size_t,
) {
    let mut xbuf: xbuffer = xbuffer {
        data: 0 as *mut std::ffi::c_uchar,
        end: 0,
        size: 0,
        init_size: 0,
    };
    xbuf_init(&mut xbuf);
    expand_evars(buf as *mut std::ffi::c_char, len, &mut xbuf);
    if add_cmd_table(tlist, xbuf.data, xbuf.end) < 0 as std::ffi::c_int {
        error(
            b"Warning: environment variables from lesskey file unavailable\0" as *const u8
                as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn add_uvar_table(mut buf: *mut std::ffi::c_uchar, mut len: size_t) {
    add_var_table(&mut list_var_tables, buf, len);
}
#[no_mangle]
pub unsafe extern "C" fn add_sysvar_table(mut buf: *mut std::ffi::c_uchar, mut len: size_t) {
    add_var_table(&mut list_sysvar_tables, buf, len);
}
unsafe extern "C" fn mouse_wheel_down() -> std::ffi::c_int {
    return if mousecap == 2 as std::ffi::c_int {
        67 as std::ffi::c_int
    } else {
        66 as std::ffi::c_int
    };
}
unsafe extern "C" fn mouse_wheel_up() -> std::ffi::c_int {
    return if mousecap == 2 as std::ffi::c_int {
        66 as std::ffi::c_int
    } else {
        67 as std::ffi::c_int
    };
}
unsafe extern "C" fn mouse_button_left(
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut down: lbool,
    mut drag: lbool,
) -> std::ffi::c_int {
    static mut last_drag_y: std::ffi::c_int = -(1 as std::ffi::c_int);
    static mut last_click_y: std::ffi::c_int = -(1 as std::ffi::c_int);
    if down as std::ffi::c_uint != 0 && drag as u64 == 0 {
        last_click_y = y;
        last_drag_y = last_click_y;
    }
    if allow_drag as std::ffi::c_uint != 0
        && drag as std::ffi::c_uint != 0
        && last_drag_y >= 0 as std::ffi::c_int
    {
        if y > last_drag_y {
            cmd_exec();
            backward(y - last_drag_y, LFALSE, LFALSE, LFALSE);
            last_drag_y = y;
        } else if y < last_drag_y {
            cmd_exec();
            forward(last_drag_y - y, LFALSE, LFALSE, LFALSE);
            last_drag_y = y;
        }
    } else if down as u64 == 0 {
        if osc8_click(y, x) as u64 != 0 {
            return 101 as std::ffi::c_int;
        }
        if y < sc_height - 1 as std::ffi::c_int && y == last_click_y {
            setmark('#' as i32 as std::ffi::c_char, y);
            screen_trashed();
        }
    }
    return 101 as std::ffi::c_int;
}
unsafe extern "C" fn mouse_button_right(
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut down: lbool,
    mut drag: lbool,
) -> std::ffi::c_int {
    if down as u64 == 0 && y < sc_height - 1 as std::ffi::c_int {
        gomark('#' as i32 as std::ffi::c_char);
        screen_trashed();
    }
    return 101 as std::ffi::c_int;
}
unsafe extern "C" fn getcc_int(mut pterm: *mut std::ffi::c_char) -> std::ffi::c_int {
    let mut num: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut digits: std::ffi::c_int = 0 as std::ffi::c_int;
    loop {
        let mut ch: std::ffi::c_char = getcc();
        if (ch as std::ffi::c_int) < '0' as i32 || ch as std::ffi::c_int > '9' as i32 {
            if !pterm.is_null() {
                *pterm = ch;
            }
            if digits == 0 as std::ffi::c_int {
                return -(1 as std::ffi::c_int);
            }
            return num;
        }
        let (fresh8, fresh9) = num.overflowing_mul(10 as std::ffi::c_int);
        *(&mut num as *mut std::ffi::c_int) = fresh8;
        if fresh9 as std::ffi::c_int != 0 || {
            let (fresh10, fresh11) = num.overflowing_add(ch as std::ffi::c_int - '0' as i32);
            *(&mut num as *mut std::ffi::c_int) = fresh10;
            fresh11 as std::ffi::c_int != 0
        } {
            return -(1 as std::ffi::c_int);
        }
        digits += 1;
    }
}
unsafe extern "C" fn x11mouse_button(
    mut btn: std::ffi::c_int,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut down: lbool,
    mut drag: lbool,
) -> std::ffi::c_int {
    match btn {
        0 => return mouse_button_left(x, y, down, drag),
        1 | 2 => return mouse_button_right(x, y, down, drag),
        _ => {}
    }
    return 101 as std::ffi::c_int;
}
unsafe extern "C" fn x11mouse_action(mut skip: lbool) -> std::ffi::c_int {
    static mut prev_b: std::ffi::c_int = 3 as std::ffi::c_int;
    let mut x: std::ffi::c_int = 0;
    let mut y: std::ffi::c_int = 0;
    let mut b: std::ffi::c_int = getcc() as std::ffi::c_int - 0x20 as std::ffi::c_int;
    let mut drag: lbool =
        (b & 0x20 as std::ffi::c_int != 0 as std::ffi::c_int) as std::ffi::c_int as lbool;
    b &= !(0x20 as std::ffi::c_int);
    x = getcc() as std::ffi::c_int - 0x20 as std::ffi::c_int - 1 as std::ffi::c_int;
    y = getcc() as std::ffi::c_int - 0x20 as std::ffi::c_int - 1 as std::ffi::c_int;
    if skip as u64 != 0 {
        return 101 as std::ffi::c_int;
    }
    match b {
        65 => return mouse_wheel_down(),
        64 => return mouse_wheel_up(),
        0 | 1 | 2 => {
            prev_b = b;
            return x11mouse_button(b, x, y, LTRUE, drag);
        }
        3 => return x11mouse_button(prev_b, x, y, LFALSE, drag),
        _ => {}
    }
    return 101 as std::ffi::c_int;
}
unsafe extern "C" fn x116mouse_action(mut skip: lbool) -> std::ffi::c_int {
    let mut ch: std::ffi::c_char = 0;
    let mut x: std::ffi::c_int = 0;
    let mut y: std::ffi::c_int = 0;
    let mut b: std::ffi::c_int = getcc_int(&mut ch);
    let mut drag: lbool =
        (b & 0x20 as std::ffi::c_int != 0 as std::ffi::c_int) as std::ffi::c_int as lbool;
    b &= !(0x20 as std::ffi::c_int);
    if b < 0 as std::ffi::c_int || ch as std::ffi::c_int != ';' as i32 {
        return 101 as std::ffi::c_int;
    }
    x = getcc_int(&mut ch) - 1 as std::ffi::c_int;
    if x < 0 as std::ffi::c_int || ch as std::ffi::c_int != ';' as i32 {
        return 101 as std::ffi::c_int;
    }
    y = getcc_int(&mut ch) - 1 as std::ffi::c_int;
    if y < 0 as std::ffi::c_int {
        return 101 as std::ffi::c_int;
    }
    if skip as u64 != 0 {
        return 101 as std::ffi::c_int;
    }
    match b {
        65 => return mouse_wheel_down(),
        64 => return mouse_wheel_up(),
        0 | 1 | 2 => {
            let mut down: lbool = (ch as std::ffi::c_int == 'M' as i32) as std::ffi::c_int as lbool;
            let mut up: lbool = (ch as std::ffi::c_int == 'm' as i32) as std::ffi::c_int as lbool;
            if up as std::ffi::c_uint != 0 || down as std::ffi::c_uint != 0 {
                return x11mouse_button(b, x, y, down, drag);
            }
        }
        _ => {}
    }
    return 101 as std::ffi::c_int;
}
unsafe extern "C" fn cmd_match(
    mut goal: *const std::ffi::c_char,
    mut str: *const std::ffi::c_char,
) -> size_t {
    let mut slen: size_t = strlen(str);
    let mut len: size_t = 0;
    len = slen;
    while len > 0 as std::ffi::c_int as size_t {
        if strncmp(str.offset(slen as isize).offset(-(len as isize)), goal, len)
            == 0 as std::ffi::c_int
        {
            break;
        }
        len = len.wrapping_sub(1);
    }
    return len;
}
unsafe extern "C" fn cmd_next_entry(
    mut entry: *const std::ffi::c_uchar,
    mut action: *mut std::ffi::c_int,
    mut extra: *mut *const std::ffi::c_uchar,
    mut cmdlen: *mut size_t,
) -> *const std::ffi::c_uchar {
    let mut a: std::ffi::c_int = 0;
    let mut oentry: *const std::ffi::c_uchar = entry;
    while *entry as std::ffi::c_int != '\0' as i32 {
        entry = entry.offset(1);
    }
    if !cmdlen.is_null() {
        *cmdlen = entry.offset_from(oentry) as std::ffi::c_long as size_t;
    }
    loop {
        entry = entry.offset(1);
        a = *entry as std::ffi::c_int;
        if !(a == 127 as std::ffi::c_int) {
            break;
        }
    }
    entry = entry.offset(1);
    if !extra.is_null() {
        *extra = if a & 0o200 as std::ffi::c_int != 0 {
            entry
        } else {
            0 as *const std::ffi::c_uchar
        };
    }
    if a & 0o200 as std::ffi::c_int != 0 {
        loop {
            let fresh12 = entry;
            entry = entry.offset(1);
            if !(*fresh12 as std::ffi::c_int != '\0' as i32) {
                break;
            }
        }
        a &= !(0o200 as std::ffi::c_int);
    }
    if !action.is_null() {
        *action = a;
    }
    return entry;
}
unsafe extern "C" fn cmd_search(
    mut cmd: *const std::ffi::c_char,
    mut table: *const std::ffi::c_uchar,
    mut endtable: *const std::ffi::c_uchar,
    mut extra: *mut *const std::ffi::c_uchar,
    mut mlen: *mut size_t,
) -> std::ffi::c_int {
    let mut action: std::ffi::c_int = 100 as std::ffi::c_int;
    let mut match_len: size_t = 0 as std::ffi::c_int as size_t;
    if !extra.is_null() {
        *extra = 0 as *const std::ffi::c_uchar;
    }
    while table < endtable {
        let mut taction: std::ffi::c_int = 0;
        let mut textra: *const std::ffi::c_uchar = 0 as *const std::ffi::c_uchar;
        let mut cmdlen: size_t = 0;
        let mut match_0: size_t = cmd_match(table as *const std::ffi::c_char, cmd);
        table = cmd_next_entry(table, &mut taction, &mut textra, &mut cmdlen);
        if taction == 103 as std::ffi::c_int {
            return -action;
        }
        if match_0 >= match_len {
            if match_0 == cmdlen {
                action = taction;
                if !extra.is_null() {
                    *extra = textra;
                }
            } else if match_0 > 0 as std::ffi::c_int as size_t && action == 100 as std::ffi::c_int {
                action = 105 as std::ffi::c_int;
            }
            match_len = match_0;
        }
    }
    if !mlen.is_null() {
        *mlen = match_len;
    }
    return action;
}
unsafe extern "C" fn cmd_decode(
    mut tlist: *mut tablelist,
    mut cmd: *const std::ffi::c_char,
    mut sp: *mut *const std::ffi::c_char,
) -> std::ffi::c_int {
    let mut t: *mut tablelist = 0 as *mut tablelist;
    let mut action: std::ffi::c_int = 100 as std::ffi::c_int;
    let mut match_len: size_t = 0 as std::ffi::c_int as size_t;
    *sp = 0 as *const std::ffi::c_char;
    t = tlist;
    while !t.is_null() {
        let mut tsp: *const std::ffi::c_uchar = 0 as *const std::ffi::c_uchar;
        let mut mlen: size_t = 0;
        let mut taction: std::ffi::c_int =
            cmd_search(cmd, (*t).t_start, (*t).t_end, &mut tsp, &mut mlen);
        if mlen >= match_len {
            match_len = mlen;
            if taction == 102 as std::ffi::c_int {
                taction = 100 as std::ffi::c_int;
            }
            if taction != 100 as std::ffi::c_int {
                *sp = tsp as *const std::ffi::c_char;
                if taction < 0 as std::ffi::c_int {
                    action = -taction;
                    break;
                } else {
                    action = taction;
                }
            }
        }
        t = (*t).t_next;
    }
    if action == 64 as std::ffi::c_int {
        action = x11mouse_action(LFALSE);
    } else if action == 68 as std::ffi::c_int {
        action = x116mouse_action(LFALSE);
    }
    return action;
}
#[no_mangle]
pub unsafe extern "C" fn fcmd_decode(
    mut cmd: *const std::ffi::c_char,
    mut sp: *mut *const std::ffi::c_char,
) -> std::ffi::c_int {
    return cmd_decode(list_fcmd_tables, cmd, sp);
}
#[no_mangle]
pub unsafe extern "C" fn ecmd_decode(
    mut cmd: *const std::ffi::c_char,
    mut sp: *mut *const std::ffi::c_char,
) -> std::ffi::c_int {
    return cmd_decode(list_ecmd_tables, cmd, sp);
}

/*
 * Get the value of an environment variable.
 * Looks first in the lesskey file, then in the real environment.
 */
#[no_mangle]
pub fn lgetenv(key: &str) -> Result<*const std::ffi::c_char, VarError> {
    // FIXME add the lesskey file lookup
    /*
    let mut a: i32;
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    a = cmd_decode(list_var_tables, var, &mut s);
    if a == 0o1 as std::ffi::c_int {
        return s;
    }
    */
    let s = env::var(key);
    let ret = match s {
        Ok(val) => Ok(CString::new(val.as_str()).unwrap().as_ptr()),
        Err(e) => Err(e),
    };
    /*
    a = cmd_decode(list_sysvar_tables, var, &mut s);
    if a == 0o1 as std::ffi::c_int {
        return s;
    }
    */
    ret
}
#[no_mangle]
pub unsafe extern "C" fn lgetenv_ext(
    var: &str,
    mut env_buf: *mut std::ffi::c_uchar,
    mut env_buf_len: size_t,
) -> *const std::ffi::c_char {
    let mut r: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut e: size_t = 0;
    let mut env_end: size_t = 0 as std::ffi::c_int as size_t;
    e = 0 as std::ffi::c_int as size_t;
    loop {
        while e < env_buf_len {
            if *env_buf.offset(e as isize) as std::ffi::c_int == '\0' as i32 {
                break;
            }
            e = e.wrapping_add(1);
        }
        if e >= env_buf_len {
            break;
        }
        e = e.wrapping_add(1);
        if *env_buf.offset(e as isize) as std::ffi::c_int & 0o200 as std::ffi::c_int != 0 {
            e = e.wrapping_add(1 as std::ffi::c_int as size_t);
            while e < env_buf_len {
                if *env_buf.offset(e as isize) as std::ffi::c_int == '\0' as i32 {
                    break;
                }
                e = e.wrapping_add(1);
            }
        }
        e = e.wrapping_add(1);
        if e >= env_buf_len {
            break;
        }
        env_end = e;
    }
    add_uvar_table(env_buf, env_end);
    r = lgetenv(var).unwrap();
    pop_cmd_table(&mut list_var_tables);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn isnullenv(mut s: *const std::ffi::c_char) -> lbool {
    return (s.is_null() || *s as std::ffi::c_int == '\0' as i32) as std::ffi::c_int as lbool;
}
unsafe extern "C" fn gint(mut sp: *mut *mut std::ffi::c_uchar) -> size_t {
    let mut n: size_t = 0;
    let fresh13 = *sp;
    *sp = (*sp).offset(1);
    n = *fresh13 as size_t;
    let fresh14 = *sp;
    *sp = (*sp).offset(1);
    n = n.wrapping_add((*fresh14 as std::ffi::c_int * 64 as std::ffi::c_int) as size_t);
    return n;
}
unsafe extern "C" fn old_lesskey(
    mut buf: *mut std::ffi::c_uchar,
    mut len: size_t,
) -> std::ffi::c_int {
    if *buf.offset(len.wrapping_sub(1 as std::ffi::c_int as size_t) as isize) as std::ffi::c_int
        != '\0' as i32
        && *buf.offset(len.wrapping_sub(2 as std::ffi::c_int as size_t) as isize) as std::ffi::c_int
            != '\0' as i32
    {
        return -(1 as std::ffi::c_int);
    }
    add_fcmd_table(buf, len);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn new_lesskey(
    mut buf: *mut std::ffi::c_uchar,
    mut len: size_t,
    mut sysvar: lbool,
) -> std::ffi::c_int {
    let mut p: *mut std::ffi::c_uchar = 0 as *mut std::ffi::c_uchar;
    let mut end: *mut std::ffi::c_uchar = 0 as *mut std::ffi::c_uchar;
    let mut c: std::ffi::c_int = 0;
    let mut n: size_t = 0;
    if *buf.offset(len.wrapping_sub(3 as std::ffi::c_int as size_t) as isize) as std::ffi::c_int
        != 'E' as i32
        || *buf.offset(len.wrapping_sub(2 as std::ffi::c_int as size_t) as isize) as std::ffi::c_int
            != 'n' as i32
        || *buf.offset(len.wrapping_sub(1 as std::ffi::c_int as size_t) as isize) as std::ffi::c_int
            != 'd' as i32
    {
        return -(1 as std::ffi::c_int);
    }
    p = buf.offset(4 as std::ffi::c_int as isize);
    end = buf.offset(len as isize);
    loop {
        let fresh15 = p;
        p = p.offset(1);
        c = *fresh15 as std::ffi::c_int;
        match c {
            99 => {
                n = gint(&mut p);
                if p.offset(n as isize) >= end {
                    return -(1 as std::ffi::c_int);
                }
                add_fcmd_table(p, n);
                p = p.offset(n as isize);
            }
            101 => {
                n = gint(&mut p);
                if p.offset(n as isize) >= end {
                    return -(1 as std::ffi::c_int);
                }
                add_ecmd_table(p, n);
                p = p.offset(n as isize);
            }
            118 => {
                n = gint(&mut p);
                if p.offset(n as isize) >= end {
                    return -(1 as std::ffi::c_int);
                }
                if sysvar as u64 != 0 {
                    add_sysvar_table(p, n);
                } else {
                    add_uvar_table(p, n);
                }
                p = p.offset(n as isize);
            }
            120 => return 0 as std::ffi::c_int,
            _ => return -(1 as std::ffi::c_int),
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lesskey(
    mut filename: *const std::ffi::c_char,
    mut sysvar: lbool,
) -> std::ffi::c_int {
    let mut buf: *mut std::ffi::c_uchar = 0 as *mut std::ffi::c_uchar;
    let mut len: POSITION = 0;
    let mut n: ssize_t = 0;
    let mut f: std::ffi::c_int = 0;
    if secure_allow((1 as std::ffi::c_int) << 5 as std::ffi::c_int) == 0 {
        return 1 as std::ffi::c_int;
    }
    f = open(filename, 0 as std::ffi::c_int);
    if f < 0 as std::ffi::c_int {
        return 1 as std::ffi::c_int;
    }
    len = filesize(f);
    if len == -(1 as std::ffi::c_int) as POSITION || len < 3 as std::ffi::c_int as POSITION {
        close(f);
        return -(1 as std::ffi::c_int);
    }
    buf = calloc(
        len as size_t,
        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_uchar;
    if buf.is_null() {
        close(f);
        return -(1 as std::ffi::c_int);
    }
    if lseek(f, 0 as std::ffi::c_int as less_off_t, 0 as std::ffi::c_int)
        == -(1 as std::ffi::c_int) as off_t
    {
        free(buf as *mut std::ffi::c_void);
        close(f);
        return -(1 as std::ffi::c_int);
    }
    n = read(f, buf as *mut std::ffi::c_void, len as size_t);
    close(f);
    if n != len {
        free(buf as *mut std::ffi::c_void);
        return -(1 as std::ffi::c_int);
    }
    if len < 4 as std::ffi::c_int as POSITION
        || *buf.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != '\0' as i32
        || *buf.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int != 'M' as i32
        || *buf.offset(2 as std::ffi::c_int as isize) as std::ffi::c_int != '+' as i32
        || *buf.offset(3 as std::ffi::c_int as isize) as std::ffi::c_int != 'G' as i32
    {
        return old_lesskey(buf, len as size_t);
    }
    return new_lesskey(buf, len as size_t, sysvar);
}
unsafe extern "C" fn lesskey_text(
    mut filename: *const std::ffi::c_char,
    mut sysvar: lbool,
    mut content: lbool,
) -> std::ffi::c_int {
    let mut r: std::ffi::c_int = 0;
    static mut tables: lesskey_tables = lesskey_tables {
        currtable: 0 as *const lesskey_table as *mut lesskey_table,
        cmdtable: lesskey_table {
            names: 0 as *const lesskey_cmdname,
            buf: xbuffer {
                data: 0 as *mut std::ffi::c_uchar,
                end: 0,
                size: 0,
                init_size: 0,
            },
            is_var: 0,
        },
        edittable: lesskey_table {
            names: 0 as *const lesskey_cmdname,
            buf: xbuffer {
                data: 0 as *mut std::ffi::c_uchar,
                end: 0,
                size: 0,
                init_size: 0,
            },
            is_var: 0,
        },
        vartable: lesskey_table {
            names: 0 as *const lesskey_cmdname,
            buf: xbuffer {
                data: 0 as *mut std::ffi::c_uchar,
                end: 0,
                size: 0,
                init_size: 0,
            },
            is_var: 0,
        },
    };
    if secure_allow((1 as std::ffi::c_int) << 5 as std::ffi::c_int) == 0 {
        return 1 as std::ffi::c_int;
    }
    r = if content as std::ffi::c_uint != 0 {
        parse_lesskey_content(filename, &mut tables)
    } else {
        parse_lesskey(filename, &mut tables)
    };
    if r != 0 as std::ffi::c_int {
        return r;
    }
    add_fcmd_table(tables.cmdtable.buf.data, tables.cmdtable.buf.end);
    add_ecmd_table(tables.edittable.buf.data, tables.edittable.buf.end);
    if sysvar as u64 != 0 {
        add_sysvar_table(tables.vartable.buf.data, tables.vartable.buf.end);
    } else {
        add_uvar_table(tables.vartable.buf.data, tables.vartable.buf.end);
    }
    return 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lesskey_src(
    mut filename: *const std::ffi::c_char,
    mut sysvar: lbool,
) -> std::ffi::c_int {
    return lesskey_text(filename, sysvar, LFALSE);
}
#[no_mangle]
pub unsafe extern "C" fn lesskey_content(
    mut content: *const std::ffi::c_char,
    mut sysvar: lbool,
) -> std::ffi::c_int {
    return lesskey_text(content, sysvar, LTRUE);
}
#[no_mangle]
pub unsafe extern "C" fn lesskey_parse_error(mut s: *mut std::ffi::c_char) {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    parg.p_string = s;
    error(b"%s\0" as *const u8 as *const std::ffi::c_char, &mut parg);
}
unsafe extern "C" fn add_hometable(
    mut call_lesskey: Option<
        unsafe extern "C" fn(*const std::ffi::c_char, lbool) -> std::ffi::c_int,
    >,
    envname: Option<&str>,
    mut def_filename: *const std::ffi::c_char,
    mut sysvar: lbool,
) -> std::ffi::c_int {
    let mut filename: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut efilename: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut r: std::ffi::c_int = 0;
    if let Some(name) = envname {
        if let Ok(efilename) = lgetenv(name) {
            filename = save(efilename);
        }
    } else if sysvar as u64 != 0 {
        filename = save(def_filename);
    } else {
        if let Ok(xdg) = lgetenv("XDG_CONFIG_HOME") {
            filename = dirfile(
                xdg,
                &*def_filename.offset(1 as std::ffi::c_int as isize),
                1 as std::ffi::c_int,
            );
        }
        if filename.is_null() {
            if let Ok(home) = lgetenv("HOME") {
                let mut cfg_dir: *mut std::ffi::c_char = dirfile(
                    home,
                    b".config\0" as *const u8 as *const std::ffi::c_char,
                    0 as std::ffi::c_int,
                );
                filename = dirfile(
                    cfg_dir,
                    &*def_filename.offset(1 as std::ffi::c_int as isize),
                    1 as std::ffi::c_int,
                );
                free(cfg_dir as *mut std::ffi::c_void);
            }
        }
        if filename.is_null() {
            filename = homefile(def_filename);
        }
    }
    if filename.is_null() {
        return -(1 as std::ffi::c_int);
    }
    r = (Some(call_lesskey.expect("non-null function pointer")))
        .expect("non-null function pointer")(filename, sysvar);
    free(filename as *mut std::ffi::c_void);
    return r;
}
unsafe extern "C" fn add_content_table(
    mut call_lesskey: Option<
        unsafe extern "C" fn(*const std::ffi::c_char, lbool) -> std::ffi::c_int,
    >,
    envname: &str,
    mut sysvar: lbool,
) {
    if let Ok(content) = lgetenv(envname) {
        lesskey_content(content, sysvar);
    }
}

#[no_mangle]
pub unsafe extern "C" fn editchar(
    mut c: std::ffi::c_char,
    mut flags: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut action: std::ffi::c_int = 0;
    let mut nch: std::ffi::c_int = 0;
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut usercmd: [std::ffi::c_char; 17] = [0; 17];
    if c as std::ffi::c_int == erase_char || c as std::ffi::c_int == erase2_char {
        return 1 as std::ffi::c_int;
    }
    if c as std::ffi::c_int == kill_char {
        return 2 as std::ffi::c_int;
    }
    nch = 0 as std::ffi::c_int;
    loop {
        if nch > 0 as std::ffi::c_int {
            c = getcc();
        }
        usercmd[nch as usize] = c;
        usercmd[(nch + 1 as std::ffi::c_int) as usize] = '\0' as i32 as std::ffi::c_char;
        nch += 1;
        action = ecmd_decode(usercmd.as_mut_ptr(), &mut s);
        if !(action == 105 as std::ffi::c_int && nch < 16 as std::ffi::c_int) {
            break;
        }
    }
    if action == 21 as std::ffi::c_int {
        return x11mouse_action(LTRUE);
    }
    if action == 22 as std::ffi::c_int {
        return x116mouse_action(LTRUE);
    }
    if flags & 0o10 as std::ffi::c_int != 0 {
        match action {
            3 | 4 => {
                action = 100 as std::ffi::c_int;
            }
            _ => {}
        }
    }
    if flags & 0o2 as std::ffi::c_int != 0 {
        match action {
            13 | 14 => {
                action = 100 as std::ffi::c_int;
            }
            _ => {}
        }
    }
    if flags & 0o4 as std::ffi::c_int != 0 {
        match action {
            17 | 18 | 15 => {
                action = 100 as std::ffi::c_int;
            }
            _ => {}
        }
    }
    if flags & 0o1 as std::ffi::c_int != 0 || action == 100 as std::ffi::c_int {
        while nch > 1 as std::ffi::c_int {
            nch -= 1;
            ungetcc(usercmd[nch as usize]);
        }
    } else if !s.is_null() {
        ungetsc(s);
    }
    return action;
}
