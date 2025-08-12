use crate::decode::{lgetenv, lgetenv_ext};
use crate::xbuf::XBuffer;
use ::c2rust_bitfields;
use std::ffi::CString;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ansi_state;
    fn sprintf(_: *mut std::ffi::c_char, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn snprintf(
        _: *mut std::ffi::c_char,
        _: std::ffi::c_ulong,
        _: *const std::ffi::c_char,
        _: ...
    ) -> std::ffi::c_int;
    fn getc(__stream: *mut FILE) -> std::ffi::c_int;
    fn fileno(__stream: *mut FILE) -> std::ffi::c_int;
    fn pclose(__stream: *mut FILE) -> std::ffi::c_int;
    fn popen(__command: *const std::ffi::c_char, __modes: *const std::ffi::c_char) -> *mut FILE;
    fn open(__file: *const std::ffi::c_char, __oflag: std::ffi::c_int, _: ...) -> std::ffi::c_int;
    fn lseek(__fd: std::ffi::c_int, __offset: __off_t, __whence: std::ffi::c_int) -> __off_t;
    fn close(__fd: std::ffi::c_int) -> std::ffi::c_int;
    fn read(__fd: std::ffi::c_int, __buf: *mut std::ffi::c_void, __nbytes: size_t) -> ssize_t;
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn realpath(
        __name: *const std::ffi::c_char,
        __resolved: *mut std::ffi::c_char,
    ) -> *mut std::ffi::c_char;
    fn strcpy(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strcat(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strncmp(
        _: *const std::ffi::c_char,
        _: *const std::ffi::c_char,
        _: std::ffi::c_ulong,
    ) -> std::ffi::c_int;
    fn strchr(_: *const std::ffi::c_char, _: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn save(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
    fn secure_allow(features: std::ffi::c_int) -> std::ffi::c_int;
    fn ch_ungetchar(c: std::ffi::c_int);
    fn ch_tell() -> POSITION;
    fn seekable(f: std::ffi::c_int) -> std::ffi::c_int;
    fn binary_char(c: LWCHAR) -> lbool;
    fn is_utf8_well_formed(ss: *const std::ffi::c_char, slen: std::ffi::c_int) -> lbool;
    fn utf_skip_to_lead(pp: *mut *const std::ffi::c_char, limit: *const std::ffi::c_char);
    fn step_charc(
        pp: *mut *const std::ffi::c_char,
        dir: std::ffi::c_int,
        limit: *const std::ffi::c_char,
    ) -> LWCHAR;
    fn isnullenv(s: *const std::ffi::c_char) -> lbool;
    fn get_filename(ifile: *mut std::ffi::c_void) -> *const std::ffi::c_char;
    fn skip_ansi(
        pansi: *mut ansi_state,
        ch: LWCHAR,
        pp: *mut *const std::ffi::c_char,
        limit: *const std::ffi::c_char,
    );
    fn ansi_start(ch: LWCHAR) -> *mut ansi_state;
    fn ansi_done(pansi: *mut ansi_state);
    fn errno_message(filename: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn stat(__file: *const std::ffi::c_char, __buf: *mut stat) -> std::ffi::c_int;
    fn fstat(__fd: std::ffi::c_int, __buf: *mut stat) -> std::ffi::c_int;
    static mut force_open: std::ffi::c_int;
    static mut use_lessopen: std::ffi::c_int;
    static mut ctldisp: std::ffi::c_int;
    static mut utf_mode: std::ffi::c_int;
    static mut curr_ifile: *mut std::ffi::c_void;
    static mut old_ifile: *mut std::ffi::c_void;
    static mut openquote: std::ffi::c_char;
    static mut closequote: std::ffi::c_char;
    static mut curr_ino: ino_t;
    static mut curr_dev: dev_t;
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
pub type LWCHAR = std::ffi::c_ulong;
pub type less_off_t = off_t;
pub type less_stat_t = stat;
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
pub struct xcpy {
    pub dest: *mut std::ffi::c_char,
    pub copied: size_t,
}
const DEF_METACHARS: *const std::ffi::c_char =
    b"; *?\t\n'\"()<>[]|&^`#\\$%=~{}," as *const u8 as *const std::ffi::c_char;

#[no_mangle]
pub unsafe extern "C" fn shell_unquote(mut str: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    let mut name: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut p: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    p = ecalloc(
        (strlen(str)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong),
        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_char;
    name = p;
    if *str as std::ffi::c_int == openquote as std::ffi::c_int {
        str = str.offset(1);
        while *str as std::ffi::c_int != '\0' as i32 {
            if *str as std::ffi::c_int == closequote as std::ffi::c_int {
                if *str.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                    != closequote as std::ffi::c_int
                {
                    break;
                }
                str = str.offset(1);
            }
            let fresh0 = str;
            str = str.offset(1);
            let fresh1 = p;
            p = p.offset(1);
            *fresh1 = *fresh0;
        }
    } else {
        let mut esc: *const std::ffi::c_char = get_meta_escape();
        let mut esclen: size_t = strlen(esc);
        while *str as std::ffi::c_int != '\0' as i32 {
            if esclen > 0 as std::ffi::c_int as size_t
                && strncmp(str, esc, esclen) == 0 as std::ffi::c_int
            {
                str = str.offset(esclen as isize);
            }
            let fresh2 = str;
            str = str.offset(1);
            let fresh3 = p;
            p = p.offset(1);
            *fresh3 = *fresh2;
        }
    }
    *p = '\0' as i32 as std::ffi::c_char;
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn get_meta_escape() -> *const std::ffi::c_char {
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let ss = lgetenv("LESSMETAESCAPE");
    if ss.is_err() {
        s = b"\\\0" as *const u8 as *const std::ffi::c_char;
    } else {
        s = CString::new(ss.unwrap()).unwrap().as_ptr();
    }
    return s;
}
unsafe extern "C" fn metachars() -> *const std::ffi::c_char {
    let mchars = lgetenv("LESSMETACHARS");
    if mchars.is_err() {
        return DEF_METACHARS;
    }
    0 as *const std::ffi::c_char
}
unsafe extern "C" fn metachar(mut c: std::ffi::c_char) -> lbool {
    return (strchr(metachars(), c as std::ffi::c_int)
        != 0 as *mut std::ffi::c_void as *mut std::ffi::c_char) as std::ffi::c_int
        as lbool;
}
unsafe extern "C" fn must_quote(mut c: std::ffi::c_char) -> lbool {
    return (c as std::ffi::c_int == '\n' as i32) as std::ffi::c_int as lbool;
}
#[no_mangle]
pub unsafe extern "C" fn shell_quoten(
    mut s: *const std::ffi::c_char,
    mut slen: size_t,
) -> *mut std::ffi::c_char {
    let mut p: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut np: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut newstr: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut len: size_t = 0;
    let mut esc: *const std::ffi::c_char = get_meta_escape();
    let mut esclen: size_t = strlen(esc);
    let mut use_quotes: lbool = LFALSE;
    let mut have_quotes: lbool = LFALSE;
    len = 1 as std::ffi::c_int as size_t;
    p = s;
    while p < s.offset(slen as isize) {
        len = len.wrapping_add(1);
        if *p as std::ffi::c_int == openquote as std::ffi::c_int
            || *p as std::ffi::c_int == closequote as std::ffi::c_int
        {
            have_quotes = LTRUE;
        }
        if metachar(*p) as u64 != 0 {
            if esclen == 0 as std::ffi::c_int as size_t {
                use_quotes = LTRUE;
            } else if must_quote(*p) as u64 != 0 {
                len = len.wrapping_add(3 as std::ffi::c_int as size_t);
            } else {
                len = len.wrapping_add(esclen);
            }
        }
        p = p.offset(1);
    }
    if use_quotes as u64 != 0 {
        if have_quotes as u64 != 0 {
            return 0 as *mut std::ffi::c_char;
        }
        len = slen.wrapping_add(3 as std::ffi::c_int as size_t);
    }
    np = ecalloc(
        len,
        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_char;
    newstr = np;
    if use_quotes as u64 != 0 {
        snprintf(
            newstr,
            len,
            b"%c%.*s%c\0" as *const u8 as *const std::ffi::c_char,
            openquote as std::ffi::c_int,
            slen as std::ffi::c_int,
            s,
            closequote as std::ffi::c_int,
        );
    } else {
        let mut es: *const std::ffi::c_char = s.offset(slen as isize);
        while s < es {
            if metachar(*s) as u64 == 0 {
                let fresh4 = s;
                s = s.offset(1);
                let fresh5 = np;
                np = np.offset(1);
                *fresh5 = *fresh4;
            } else if must_quote(*s) as u64 != 0 {
                let fresh6 = np;
                np = np.offset(1);
                *fresh6 = openquote;
                let fresh7 = s;
                s = s.offset(1);
                let fresh8 = np;
                np = np.offset(1);
                *fresh8 = *fresh7;
                let fresh9 = np;
                np = np.offset(1);
                *fresh9 = closequote;
            } else {
                strcpy(np, esc);
                np = np.offset(esclen as isize);
                let fresh10 = s;
                s = s.offset(1);
                let fresh11 = np;
                np = np.offset(1);
                *fresh11 = *fresh10;
            }
        }
        *np = '\0' as i32 as std::ffi::c_char;
    }
    return newstr;
}
#[no_mangle]
pub unsafe extern "C" fn shell_quote(mut s: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    return shell_quoten(s, strlen(s));
}
#[no_mangle]
pub unsafe extern "C" fn dirfile(
    mut dirname: *const std::ffi::c_char,
    mut filename: *const std::ffi::c_char,
    mut must_exist: std::ffi::c_int,
) -> *mut std::ffi::c_char {
    let mut pathname: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut len: size_t = 0;
    let mut f: std::ffi::c_int = 0;
    if dirname.is_null() || *dirname as std::ffi::c_int == '\0' as i32 {
        return 0 as *mut std::ffi::c_char;
    }
    len = (strlen(dirname))
        .wrapping_add(strlen(filename))
        .wrapping_add(2 as std::ffi::c_int as std::ffi::c_ulong);
    pathname = calloc(
        len,
        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_char;
    if pathname.is_null() {
        return 0 as *mut std::ffi::c_char;
    }
    snprintf(
        pathname,
        len,
        b"%s%s%s\0" as *const u8 as *const std::ffi::c_char,
        dirname,
        b"/\0" as *const u8 as *const std::ffi::c_char,
        filename,
    );
    if must_exist != 0 {
        f = open(pathname, 0 as std::ffi::c_int);
        if f < 0 as std::ffi::c_int {
            free(pathname as *mut std::ffi::c_void);
            pathname = 0 as *mut std::ffi::c_char;
        } else {
            close(f);
        }
    }
    return pathname;
}
#[no_mangle]
pub unsafe extern "C" fn homefile(mut filename: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    let mut pathname: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    pathname = dirfile(
        CString::new(lgetenv("HOME").unwrap()).unwrap().as_ptr(),
        filename,
        1 as std::ffi::c_int,
    );
    if !pathname.is_null() {
        return pathname;
    }
    return 0 as *mut std::ffi::c_char;
}
unsafe extern "C" fn xcpy_char(mut xp: *mut xcpy, mut ch: std::ffi::c_char) {
    if !((*xp).dest).is_null() {
        let fresh12 = (*xp).dest;
        (*xp).dest = ((*xp).dest).offset(1);
        *fresh12 = ch;
    }
    (*xp).copied = ((*xp).copied).wrapping_add(1);
    (*xp).copied;
}
unsafe extern "C" fn xcpy_filename(mut xp: *mut xcpy, mut str: *const std::ffi::c_char) {
    let mut quote: lbool = (strchr(str, ' ' as i32)
        != 0 as *mut std::ffi::c_void as *mut std::ffi::c_char)
        as std::ffi::c_int as lbool;
    if quote as u64 != 0 {
        xcpy_char(xp, openquote);
    }
    while *str as std::ffi::c_int != '\0' as i32 {
        xcpy_char(xp, *str);
        str = str.offset(1);
    }
    if quote as u64 != 0 {
        xcpy_char(xp, closequote);
    }
}
unsafe extern "C" fn fexpand_copy(
    mut fr: *const std::ffi::c_char,
    mut to: *mut std::ffi::c_char,
) -> size_t {
    let mut xp: xcpy = xcpy {
        dest: 0 as *mut std::ffi::c_char,
        copied: 0,
    };
    xp.copied = 0 as std::ffi::c_int as size_t;
    xp.dest = to;
    while *fr as std::ffi::c_int != '\0' as i32 {
        let mut expand: lbool = LFALSE;
        match *fr as std::ffi::c_int {
            37 | 35 => {
                if *fr.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                    == *fr as std::ffi::c_int
                {
                    fr = fr.offset(1 as std::ffi::c_int as isize);
                } else {
                    expand = LTRUE;
                }
            }
            _ => {}
        }
        if expand as u64 != 0 {
            let mut ifile: *mut std::ffi::c_void = if *fr as std::ffi::c_int == '%' as i32 {
                curr_ifile
            } else if *fr as std::ffi::c_int == '#' as i32 {
                old_ifile
            } else {
                0 as *mut std::ffi::c_void
            };
            if ifile == 0 as *mut std::ffi::c_void {
                xcpy_char(&mut xp, *fr);
            } else {
                xcpy_filename(&mut xp, get_filename(ifile));
            }
        } else {
            xcpy_char(&mut xp, *fr);
        }
        fr = fr.offset(1);
    }
    xcpy_char(&mut xp, '\0' as i32 as std::ffi::c_char);
    return xp.copied;
}
#[no_mangle]
pub unsafe extern "C" fn fexpand(mut s: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    let mut n: size_t = 0;
    let mut e: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    n = fexpand_copy(s, 0 as *mut std::ffi::c_char);
    e = ecalloc(
        n,
        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_char;
    fexpand_copy(s, e);
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn fcomplete(mut s: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    let mut fpat: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut qs: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut uqs: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    if secure_allow((1 as std::ffi::c_int) << 3 as std::ffi::c_int) == 0 {
        return 0 as *mut std::ffi::c_char;
    }
    let mut len: size_t = (strlen(s)).wrapping_add(2 as std::ffi::c_int as std::ffi::c_ulong);
    fpat = ecalloc(
        len,
        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_char;
    snprintf(
        fpat,
        len,
        b"%s*\0" as *const u8 as *const std::ffi::c_char,
        s,
    );
    qs = lglob(fpat);
    uqs = shell_unquote(qs);
    if strcmp(uqs, fpat) == 0 as std::ffi::c_int {
        free(qs as *mut std::ffi::c_void);
        qs = 0 as *mut std::ffi::c_char;
    }
    free(uqs as *mut std::ffi::c_void);
    free(fpat as *mut std::ffi::c_void);
    return qs;
}
#[no_mangle]
pub unsafe extern "C" fn bin_file(mut f: std::ffi::c_int, mut n: *mut ssize_t) -> std::ffi::c_int {
    let mut bin_count: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut data: [std::ffi::c_char; 256] = [0; 256];
    let mut p: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut edata: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    if seekable(f) == 0 {
        return 0 as std::ffi::c_int;
    }
    if lseek(f, 0 as std::ffi::c_int as less_off_t, 0 as std::ffi::c_int)
        == -(1 as std::ffi::c_int) as off_t
    {
        return 0 as std::ffi::c_int;
    }
    *n = read(
        f,
        data.as_mut_ptr() as *mut std::ffi::c_void,
        ::core::mem::size_of::<[std::ffi::c_char; 256]>() as std::ffi::c_ulong,
    );
    if *n <= 0 as std::ffi::c_int as ssize_t {
        return 0 as std::ffi::c_int;
    }
    edata = &mut *data.as_mut_ptr().offset(*n as isize) as *mut std::ffi::c_char;
    p = data.as_mut_ptr();
    while p < edata {
        if utf_mode != 0
            && is_utf8_well_formed(
                p,
                edata.offset_from(p) as std::ffi::c_long as size_t as std::ffi::c_int,
            ) as u64
                == 0
        {
            bin_count += 1;
            utf_skip_to_lead(&mut p, edata);
        } else {
            let mut c: LWCHAR = step_charc(&mut p, 1 as std::ffi::c_int, edata);
            let mut pansi: *mut ansi_state = 0 as *mut ansi_state;
            if ctldisp == 2 as std::ffi::c_int && {
                pansi = ansi_start(c);
                !pansi.is_null()
            } {
                skip_ansi(pansi, c, &mut p, edata);
                ansi_done(pansi);
            } else if binary_char(c) as u64 != 0 {
                bin_count += 1;
            }
        }
    }
    return (bin_count > 5 as std::ffi::c_int) as std::ffi::c_int;
}
unsafe extern "C" fn seek_filesize(mut f: std::ffi::c_int) -> POSITION {
    let mut spos: less_off_t = 0;
    spos = lseek(f, 0 as std::ffi::c_int as less_off_t, 2 as std::ffi::c_int);
    if spos == -(1 as std::ffi::c_int) as off_t {
        return -(1 as std::ffi::c_int) as POSITION;
    }
    return spos;
}
#[no_mangle]
pub unsafe extern "C" fn readfd(mut fd: *mut FILE) -> *mut std::ffi::c_char {
    let mut xbuf = XBuffer::new(16);
    loop {
        let mut ch: std::ffi::c_int = 0;
        ch = getc(fd);
        if ch == '\n' as i32 || ch == -(1 as std::ffi::c_int) {
            break;
        }
        xbuf.xbuf_add_char(ch as std::ffi::c_char);
    }
    xbuf.xbuf_add_char(b'\0' as i8);
    return std::ptr::from_ref(xbuf.char_data().first().unwrap()) as *mut i8;
}
unsafe extern "C" fn shellcmd(mut cmd: *const std::ffi::c_char) -> *mut FILE {
    let mut fd: *mut FILE = 0 as *mut FILE;
    if let Ok(shell) = lgetenv("SHELL") {
        let mut scmd: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
        let mut esccmd: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
        esccmd = shell_quote(cmd);
        if esccmd.is_null() {
            fd = popen(cmd, b"r\0" as *const u8 as *const std::ffi::c_char);
        } else {
            let mut len: size_t = (shell.len() as u64)
                .wrapping_add(strlen(esccmd))
                .wrapping_add(5 as std::ffi::c_int as std::ffi::c_ulong);
            scmd = ecalloc(
                len,
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            ) as *mut std::ffi::c_char;
            snprintf(
                scmd,
                len,
                b"%s %s %s\0" as *const u8 as *const std::ffi::c_char,
                shell,
                shell_coption(),
                esccmd,
            );
            free(esccmd as *mut std::ffi::c_void);
            fd = popen(scmd, b"r\0" as *const u8 as *const std::ffi::c_char);
            free(scmd as *mut std::ffi::c_void);
        }
    } else {
        fd = popen(cmd, b"r\0" as *const u8 as *const std::ffi::c_char);
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn lglob(mut afilename: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    let mut gfilename: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut filename: *mut std::ffi::c_char = fexpand(afilename);
    if secure_allow((1 as std::ffi::c_int) << 3 as std::ffi::c_int) == 0 {
        return filename;
    }
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut lessecho: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut cmd: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut esc: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut qesc: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut len: size_t = 0;
    esc = get_meta_escape();
    if strlen(esc) == 0 as std::ffi::c_int as std::ffi::c_ulong {
        esc = b"-\0" as *const u8 as *const std::ffi::c_char;
    }
    qesc = shell_quote(esc);
    if qesc.is_null() {
        return filename;
    }
    let lessecho_ = lgetenv("LESSECHO");
    if lessecho_.is_err() {
        lessecho = b"lessecho\0" as *const u8 as *const std::ffi::c_char;
    } else {
        lessecho = CString::new(lessecho_.unwrap()).unwrap().as_ptr();
    }
    len = (strlen(lessecho))
        .wrapping_add(strlen(filename))
        .wrapping_add((7 as std::ffi::c_int as std::ffi::c_ulong).wrapping_mul(strlen(metachars())))
        .wrapping_add(24 as std::ffi::c_int as std::ffi::c_ulong);
    cmd = ecalloc(
        len,
        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_char;
    snprintf(
        cmd,
        len,
        b"%s -p0x%x -d0x%x -e%s \0" as *const u8 as *const std::ffi::c_char,
        lessecho,
        openquote as std::ffi::c_uchar as std::ffi::c_int,
        closequote as std::ffi::c_uchar as std::ffi::c_int,
        qesc,
    );
    free(qesc as *mut std::ffi::c_void);
    s = metachars();
    while *s as std::ffi::c_int != '\0' as i32 {
        sprintf(
            cmd.offset(strlen(cmd) as isize),
            b"-n0x%x \0" as *const u8 as *const std::ffi::c_char,
            *s as std::ffi::c_uchar as std::ffi::c_int,
        );
        s = s.offset(1);
    }
    sprintf(
        cmd.offset(strlen(cmd) as isize),
        b"-- %s\0" as *const u8 as *const std::ffi::c_char,
        filename,
    );
    fd = shellcmd(cmd);
    free(cmd as *mut std::ffi::c_void);
    if fd.is_null() {
        return filename;
    }
    gfilename = readfd(fd);
    pclose(fd);
    if *gfilename as std::ffi::c_int == '\0' as i32 {
        free(gfilename as *mut std::ffi::c_void);
        return filename;
    }
    free(filename as *mut std::ffi::c_void);
    return gfilename;
}
#[no_mangle]
pub unsafe extern "C" fn is_fake_pathname(mut path: *const std::ffi::c_char) -> lbool {
    return (strcmp(path, b"-\0" as *const u8 as *const std::ffi::c_char) == 0 as std::ffi::c_int
        || strcmp(
            path,
            b"@/\\less/\\help/\\file/\\@\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        || strcmp(
            path,
            b"@/\\less/\\empty/\\file/\\@\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int) as std::ffi::c_int as lbool;
}
#[no_mangle]
pub unsafe extern "C" fn lrealpath(mut path: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    if is_fake_pathname(path) as u64 == 0 {
        let mut rpath: [std::ffi::c_char; 4096] = [0; 4096];
        if !(realpath(path, rpath.as_mut_ptr())).is_null() {
            return save(rpath.as_mut_ptr());
        }
    }
    return save(path);
}
unsafe extern "C" fn num_pct_s(mut lessopen: *const std::ffi::c_char) -> std::ffi::c_int {
    let mut num: std::ffi::c_int = 0 as std::ffi::c_int;
    while *lessopen as std::ffi::c_int != '\0' as i32 {
        if *lessopen as std::ffi::c_int == '%' as i32 {
            if *lessopen.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int == '%' as i32 {
                lessopen = lessopen.offset(1);
            } else if *lessopen.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                == 's' as i32
            {
                num += 1;
            } else {
                return 999 as std::ffi::c_int;
            }
        }
        lessopen = lessopen.offset(1);
    }
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn open_altfile(
    mut filename: *const std::ffi::c_char,
    mut pf: *mut std::ffi::c_int,
    mut pfd: *mut *mut std::ffi::c_void,
) -> *mut std::ffi::c_char {
    let mut lessopen: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut qfilename: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut cmd: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut len: size_t = 0;
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut returnfd: std::ffi::c_int = 0 as std::ffi::c_int;
    if secure_allow((1 as std::ffi::c_int) << 6 as std::ffi::c_int) == 0 {
        return 0 as *mut std::ffi::c_char;
    }
    if use_lessopen == 0 {
        return 0 as *mut std::ffi::c_char;
    }
    ch_ungetchar(-(1 as std::ffi::c_int));
    let lessopen_ = lgetenv("LESSOPEN");
    if lessopen_.is_err() {
        return 0 as *mut std::ffi::c_char;
    } else {
        lessopen = CString::new(lessopen_.unwrap()).unwrap().as_ptr();
    }
    while *lessopen as std::ffi::c_int == '|' as i32 {
        lessopen = lessopen.offset(1);
        returnfd += 1;
    }
    if *lessopen as std::ffi::c_int == '-' as i32 {
        lessopen = lessopen.offset(1);
    } else if strcmp(filename, b"-\0" as *const u8 as *const std::ffi::c_char)
        == 0 as std::ffi::c_int
    {
        return 0 as *mut std::ffi::c_char;
    }
    if num_pct_s(lessopen) != 1 as std::ffi::c_int {
        error(
            b"LESSOPEN ignored: must contain exactly one %%s\0" as *const u8
                as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        return 0 as *mut std::ffi::c_char;
    }
    qfilename = shell_quote(filename);
    len = (strlen(lessopen))
        .wrapping_add(strlen(qfilename))
        .wrapping_add(2 as std::ffi::c_int as std::ffi::c_ulong);
    cmd = ecalloc(
        len,
        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_char;
    snprintf(cmd, len, lessopen, qfilename);
    free(qfilename as *mut std::ffi::c_void);
    fd = shellcmd(cmd);
    free(cmd as *mut std::ffi::c_void);
    if fd.is_null() {
        return 0 as *mut std::ffi::c_char;
    }
    if returnfd != 0 {
        let mut c: std::ffi::c_uchar = 0;
        let mut f: std::ffi::c_int = 0;
        f = fileno(fd);
        if read(
            f,
            &mut c as *mut std::ffi::c_uchar as *mut std::ffi::c_void,
            1 as std::ffi::c_int as size_t,
        ) != 1 as std::ffi::c_int as ssize_t
        {
            let mut status: std::ffi::c_int = pclose(fd);
            if returnfd > 1 as std::ffi::c_int && status == 0 as std::ffi::c_int {
                *pfd = 0 as *mut std::ffi::c_void;
                *pf = -(1 as std::ffi::c_int);
                return save(
                    b"@/\\less/\\empty/\\file/\\@\0" as *const u8 as *const std::ffi::c_char,
                );
            }
            return 0 as *mut std::ffi::c_char;
        }
        ch_ungetchar(c as std::ffi::c_int);
        *pfd = fd as *mut std::ffi::c_void;
        *pf = f;
        return save(b"-\0" as *const u8 as *const std::ffi::c_char);
    }
    cmd = readfd(fd);
    pclose(fd);
    if *cmd as std::ffi::c_int == '\0' as i32 {
        free(cmd as *mut std::ffi::c_void);
        return 0 as *mut std::ffi::c_char;
    }
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn close_altfile(
    mut altfilename: *const std::ffi::c_char,
    mut filename: *const std::ffi::c_char,
) {
    let mut lessclose: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut qfilename: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut qaltfilename: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut cmd: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut len: size_t = 0;
    if secure_allow((1 as std::ffi::c_int) << 6 as std::ffi::c_int) == 0 {
        return;
    }
    let lessclose_ = lgetenv("LESSCLOSE");
    if lessclose_.is_err() {
        return;
    } else {
        lessclose = CString::new(lessclose_.unwrap()).unwrap().as_ptr();
    }
    if num_pct_s(lessclose) > 2 as std::ffi::c_int {
        error(
            b"LESSCLOSE ignored; must contain no more than 2 %%s\0" as *const u8
                as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        return;
    }
    qfilename = shell_quote(filename);
    qaltfilename = shell_quote(altfilename);
    len = (strlen(lessclose))
        .wrapping_add(strlen(qfilename))
        .wrapping_add(strlen(qaltfilename))
        .wrapping_add(2 as std::ffi::c_int as std::ffi::c_ulong);
    cmd = ecalloc(
        len,
        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_char;
    snprintf(cmd, len, lessclose, qfilename, qaltfilename);
    free(qaltfilename as *mut std::ffi::c_void);
    free(qfilename as *mut std::ffi::c_void);
    fd = shellcmd(cmd);
    free(cmd as *mut std::ffi::c_void);
    if !fd.is_null() {
        pclose(fd);
    }
}
#[no_mangle]
pub unsafe extern "C" fn is_dir(mut filename: *const std::ffi::c_char) -> lbool {
    let mut isdir: lbool = LFALSE;
    let mut r: std::ffi::c_int = 0;
    let mut statbuf: less_stat_t = stat {
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
    r = stat(filename, &mut statbuf);
    isdir = (r >= 0 as std::ffi::c_int
        && statbuf.st_mode & 0o170000 as std::ffi::c_int as __mode_t
            == 0o40000 as std::ffi::c_int as __mode_t) as std::ffi::c_int as lbool;
    return isdir;
}
#[no_mangle]
pub unsafe extern "C" fn bad_file(mut filename: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    let mut m: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    if force_open == 0 && is_dir(filename) as std::ffi::c_uint != 0 {
        static mut is_a_dir: [std::ffi::c_char; 16] = unsafe {
            *::core::mem::transmute::<&[u8; 16], &mut [std::ffi::c_char; 16]>(b" is a directory\0")
        };
        m =
            ecalloc(
                (strlen(filename)).wrapping_add(
                    ::core::mem::size_of::<[std::ffi::c_char; 16]>() as std::ffi::c_ulong
                ),
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            ) as *mut std::ffi::c_char;
        strcpy(m, filename);
        strcat(m, is_a_dir.as_mut_ptr());
    } else {
        let mut r: std::ffi::c_int = 0;
        let mut statbuf: less_stat_t = stat {
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
        r = stat(filename, &mut statbuf);
        if r < 0 as std::ffi::c_int {
            m = errno_message(filename);
        } else if force_open != 0 {
            m = 0 as *mut std::ffi::c_char;
        } else if !(statbuf.st_mode & 0o170000 as std::ffi::c_int as __mode_t
            == 0o100000 as std::ffi::c_int as __mode_t)
        {
            static mut not_reg: [std::ffi::c_char; 42] = unsafe {
                *::core::mem::transmute::<&[u8; 42], &mut [std::ffi::c_char; 42]>(
                    b" is not a regular file (use -f to see it)\0",
                )
            };
            m = ecalloc(
                (strlen(filename)).wrapping_add(
                    ::core::mem::size_of::<[std::ffi::c_char; 42]>() as std::ffi::c_ulong
                ),
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            ) as *mut std::ffi::c_char;
            strcpy(m, filename);
            strcat(m, not_reg.as_mut_ptr());
        }
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn filesize(mut f: std::ffi::c_int) -> POSITION {
    let mut statbuf: less_stat_t = stat {
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
    if fstat(f, &mut statbuf) >= 0 as std::ffi::c_int {
        return statbuf.st_size;
    }
    return seek_filesize(f);
}
#[no_mangle]
pub unsafe extern "C" fn curr_ifile_changed() -> lbool {
    let mut st: stat = stat {
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
    let mut curr_pos: POSITION = ch_tell();
    let mut r: std::ffi::c_int = stat(get_filename(curr_ifile), &mut st);
    if r == 0 as std::ffi::c_int
        && (st.st_ino != curr_ino
            || st.st_dev != curr_dev
            || curr_pos != -(1 as std::ffi::c_int) as POSITION && st.st_size < curr_pos)
    {
        return LTRUE;
    }
    return LFALSE;
}
#[no_mangle]
pub unsafe extern "C" fn shell_coption() -> *const std::ffi::c_char {
    return b"-c\0" as *const u8 as *const std::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn last_component(
    mut name: *const std::ffi::c_char,
) -> *const std::ffi::c_char {
    let mut slash: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    slash = name.offset(strlen(name) as isize);
    while slash > name {
        slash = slash.offset(-1);
        if *slash as std::ffi::c_int
            == *(b"/\0" as *const u8 as *const std::ffi::c_char) as std::ffi::c_int
            || *slash as std::ffi::c_int == '/' as i32
        {
            return slash.offset(1 as std::ffi::c_int as isize);
        }
    }
    return name;
}
