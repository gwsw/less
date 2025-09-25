use crate::defs::*;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn exit(_: std::ffi::c_int) -> !;
    fn strcpy(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strchr(_: *const std::ffi::c_char, _: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
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
static mut version: *mut std::ffi::c_char =
    b"$Revision: 1.15 $\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char;
static mut quote_all: std::ffi::c_int = 0 as std::ffi::c_int;
static mut openquote: std::ffi::c_char = '"' as i32 as std::ffi::c_char;
static mut closequote: std::ffi::c_char = '"' as i32 as std::ffi::c_char;
static mut meta_escape: *mut std::ffi::c_char =
    b"\\\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char;
static mut meta_escape_buf: [std::ffi::c_char; 2] = [0; 2];
static mut metachars: *mut std::ffi::c_char = 0 as *const std::ffi::c_char as *mut std::ffi::c_char;
static mut num_metachars: std::ffi::c_int = 0 as std::ffi::c_int;
static mut size_metachars: std::ffi::c_int = 0 as std::ffi::c_int;
unsafe extern "C" fn pr_usage() {
    fprintf(
        stderr,
        b"usage: lessecho [-ox] [-cx] [-pn] [-dn] [-mx] [-nn] [-ex] [-fn] [-a] file ...\n\0"
            as *const u8 as *const std::ffi::c_char,
    );
}
unsafe extern "C" fn pr_version() {
    let mut p: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut buf: [std::ffi::c_char; 10] = [0; 10];
    let mut pbuf: *mut std::ffi::c_char = buf.as_mut_ptr();
    p = version;
    while *p as std::ffi::c_int != ' ' as i32 {
        if *p as std::ffi::c_int == '\0' as i32 {
            return;
        }
        p = p.offset(1);
    }
    p = p.offset(1);
    while *p as std::ffi::c_int != '$' as i32
        && *p as std::ffi::c_int != ' ' as i32
        && *p as std::ffi::c_int != '\0' as i32
    {
        let fresh0 = pbuf;
        pbuf = pbuf.offset(1);
        *fresh0 = *p;
        p = p.offset(1);
    }
    *pbuf = '\0' as i32 as std::ffi::c_char;
    printf(
        b"%s\n\0" as *const u8 as *const std::ffi::c_char,
        buf.as_mut_ptr(),
    );
}
unsafe extern "C" fn pr_error(mut s: *mut std::ffi::c_char) {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const std::ffi::c_char, s);
    exit(1 as std::ffi::c_int);
}
unsafe extern "C" fn lstrtol(
    mut s: *mut std::ffi::c_char,
    mut pend: *mut *mut std::ffi::c_char,
    mut radix: std::ffi::c_int,
) -> std::ffi::c_long {
    let mut v: std::ffi::c_int = 0;
    let mut neg: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut n: std::ffi::c_long = 0 as std::ffi::c_int as std::ffi::c_long;
    while *s as std::ffi::c_int == ' ' as i32 || *s as std::ffi::c_int == '\t' as i32 {
        s = s.offset(1);
    }
    if *s as std::ffi::c_int == '-' as i32 {
        neg = 1 as std::ffi::c_int;
        s = s.offset(1);
    } else if *s as std::ffi::c_int == '+' as i32 {
        s = s.offset(1);
    }
    if radix == 0 as std::ffi::c_int {
        radix = 10 as std::ffi::c_int;
        if *s as std::ffi::c_int == '0' as i32 {
            s = s.offset(1);
            match *s as std::ffi::c_int {
                120 => {
                    radix = 16 as std::ffi::c_int;
                    s = s.offset(1);
                }
                _ => {
                    radix = 8 as std::ffi::c_int;
                }
            }
        }
    }
    loop {
        if *s as std::ffi::c_int >= '0' as i32 && *s as std::ffi::c_int <= '9' as i32 {
            v = *s as std::ffi::c_int - '0' as i32;
        } else if *s as std::ffi::c_int >= 'a' as i32 && *s as std::ffi::c_int <= 'f' as i32 {
            v = *s as std::ffi::c_int - 'a' as i32 + 10 as std::ffi::c_int;
        } else {
            if !(*s as std::ffi::c_int >= 'A' as i32 && *s as std::ffi::c_int <= 'F' as i32) {
                break;
            }
            v = *s as std::ffi::c_int - 'A' as i32 + 10 as std::ffi::c_int;
        }
        if v >= radix {
            break;
        }
        n = n * radix as std::ffi::c_long + v as std::ffi::c_long;
        s = s.offset(1);
    }
    if !pend.is_null() {
        while *s as std::ffi::c_int == ' ' as i32 || *s as std::ffi::c_int == '\t' as i32 {
            s = s.offset(1);
        }
        *pend = s;
    }
    if neg != 0 {
        return -n;
    }
    return n;
}
unsafe extern "C" fn add_metachar(mut ch: std::ffi::c_char) {
    if num_metachars + 1 as std::ffi::c_int >= size_metachars {
        let mut p: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
        size_metachars = if size_metachars > 0 as std::ffi::c_int {
            size_metachars * 2 as std::ffi::c_int
        } else {
            16 as std::ffi::c_int
        };
        p = malloc(size_metachars as size_t) as *mut std::ffi::c_char;
        if p.is_null() {
            pr_error(
                b"Cannot allocate memory\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
            );
        }
        if !metachars.is_null() {
            strcpy(p, metachars);
            free(metachars as *mut std::ffi::c_void);
        }
        metachars = p;
    }
    let fresh1 = num_metachars;
    num_metachars = num_metachars + 1;
    *metachars.offset(fresh1 as isize) = ch;
    *metachars.offset(num_metachars as isize) = '\0' as i32 as std::ffi::c_char;
}
unsafe extern "C" fn is_metachar(mut ch: std::ffi::c_int) -> std::ffi::c_int {
    return (!metachars.is_null() && !(strchr(metachars, ch)).is_null()) as std::ffi::c_int;
}
unsafe fn main_0(
    mut argc: std::ffi::c_int,
    mut argv: *mut *mut std::ffi::c_char,
) -> std::ffi::c_int {
    let mut arg: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut s: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut no_more_options: std::ffi::c_int = 0;
    no_more_options = 0 as std::ffi::c_int;
    loop {
        argc -= 1;
        if !(argc > 0 as std::ffi::c_int) {
            break;
        }
        argv = argv.offset(1);
        arg = *argv;
        if *arg as std::ffi::c_int != '-' as i32 || no_more_options != 0 {
            break;
        }
        arg = arg.offset(1);
        match *arg as std::ffi::c_int {
            97 => {
                quote_all = 1 as std::ffi::c_int;
            }
            99 => {
                arg = arg.offset(1);
                closequote = *arg;
            }
            100 => {
                arg = arg.offset(1);
                closequote = lstrtol(arg, &mut s, 0 as std::ffi::c_int) as std::ffi::c_char;
                if s == arg {
                    pr_error(
                        b"Missing number after -d\0" as *const u8 as *const std::ffi::c_char
                            as *mut std::ffi::c_char,
                    );
                }
            }
            101 => {
                arg = arg.offset(1);
                if strcmp(arg, b"-\0" as *const u8 as *const std::ffi::c_char)
                    == 0 as std::ffi::c_int
                {
                    meta_escape =
                        b"\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char;
                } else {
                    meta_escape = arg;
                }
            }
            102 => {
                arg = arg.offset(1);
                meta_escape_buf[0 as std::ffi::c_int as usize] =
                    lstrtol(arg, &mut s, 0 as std::ffi::c_int) as std::ffi::c_char;
                meta_escape_buf[1 as std::ffi::c_int as usize] = '\0' as i32 as std::ffi::c_char;
                meta_escape = meta_escape_buf.as_mut_ptr();
                if s == arg {
                    pr_error(
                        b"Missing number after -f\0" as *const u8 as *const std::ffi::c_char
                            as *mut std::ffi::c_char,
                    );
                }
            }
            111 => {
                arg = arg.offset(1);
                openquote = *arg;
            }
            112 => {
                arg = arg.offset(1);
                openquote = lstrtol(arg, &mut s, 0 as std::ffi::c_int) as std::ffi::c_char;
                if s == arg {
                    pr_error(
                        b"Missing number after -p\0" as *const u8 as *const std::ffi::c_char
                            as *mut std::ffi::c_char,
                    );
                }
            }
            109 => {
                arg = arg.offset(1);
                add_metachar(*arg);
            }
            110 => {
                arg = arg.offset(1);
                add_metachar(lstrtol(arg, &mut s, 0 as std::ffi::c_int) as std::ffi::c_char);
                if s == arg {
                    pr_error(
                        b"Missing number after -n\0" as *const u8 as *const std::ffi::c_char
                            as *mut std::ffi::c_char,
                    );
                }
            }
            63 => {
                pr_usage();
                return 0 as std::ffi::c_int;
            }
            45 => {
                arg = arg.offset(1);
                if *arg as std::ffi::c_int == '\0' as i32 {
                    no_more_options = 1 as std::ffi::c_int;
                } else {
                    if strcmp(arg, b"version\0" as *const u8 as *const std::ffi::c_char)
                        == 0 as std::ffi::c_int
                    {
                        pr_version();
                        return 0 as std::ffi::c_int;
                    }
                    if strcmp(arg, b"help\0" as *const u8 as *const std::ffi::c_char)
                        == 0 as std::ffi::c_int
                    {
                        pr_usage();
                        return 0 as std::ffi::c_int;
                    }
                    pr_error(
                        b"Invalid option after --\0" as *const u8 as *const std::ffi::c_char
                            as *mut std::ffi::c_char,
                    );
                    return 0 as std::ffi::c_int;
                }
            }
            _ => {
                pr_error(
                    b"Invalid option letter\0" as *const u8 as *const std::ffi::c_char
                        as *mut std::ffi::c_char,
                );
            }
        }
    }
    loop {
        let fresh2 = argc;
        argc = argc - 1;
        if !(fresh2 > 0 as std::ffi::c_int) {
            break;
        }
        let mut has_meta: std::ffi::c_int = 0 as std::ffi::c_int;
        let fresh3 = argv;
        argv = argv.offset(1);
        arg = *fresh3;
        s = arg;
        while *s as std::ffi::c_int != '\0' as i32 {
            if is_metachar(*s as std::ffi::c_int) != 0 {
                has_meta = 1 as std::ffi::c_int;
                break;
            } else {
                s = s.offset(1);
            }
        }
        if quote_all != 0
            || has_meta != 0 && strlen(meta_escape) == 0 as std::ffi::c_int as std::ffi::c_ulong
        {
            printf(
                b"%c%s%c\0" as *const u8 as *const std::ffi::c_char,
                openquote as std::ffi::c_int,
                arg,
                closequote as std::ffi::c_int,
            );
        } else {
            s = arg;
            while *s as std::ffi::c_int != '\0' as i32 {
                if is_metachar(*s as std::ffi::c_int) != 0 {
                    printf(b"%s\0" as *const u8 as *const std::ffi::c_char, meta_escape);
                }
                printf(
                    b"%c\0" as *const u8 as *const std::ffi::c_char,
                    *s as std::ffi::c_int,
                );
                s = s.offset(1);
            }
        }
        if argc > 0 as std::ffi::c_int {
            printf(b" \0" as *const u8 as *const std::ffi::c_char);
        } else {
            printf(b"\n\0" as *const u8 as *const std::ffi::c_char);
        }
    }
    return 0 as std::ffi::c_int;
}
pub fn main() {
    let mut args: Vec<*mut std::ffi::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as std::ffi::c_int,
            args.as_mut_ptr() as *mut *mut std::ffi::c_char,
        ) as i32)
    }
}
