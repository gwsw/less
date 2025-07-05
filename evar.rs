use ::libc;
extern "C" {
    fn free(_: *mut std::ffi::c_void);
    fn xbuf_add_char(xbuf: *mut xbuffer, c: std::ffi::c_char);
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
    fn lgetenv_ext(
        var: *const std::ffi::c_char,
        env_buf: *mut std::ffi::c_uchar,
        env_buf_len: size_t,
    ) -> *const std::ffi::c_char;
}
pub type size_t = std::ffi::c_ulong;
pub type lbool = std::ffi::c_uint;
pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;
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
pub struct replace {
    pub r_next: *mut replace,
    pub r_fm: *mut std::ffi::c_char,
    pub r_to: *mut std::ffi::c_char,
}
unsafe extern "C" fn skipsl(
    mut buf: *const std::ffi::c_char,
    mut len: size_t,
    mut e: size_t,
) -> size_t {
    let mut esc: lbool = LFALSE;
    while e < len && *buf.offset(e as isize) as std::ffi::c_int != '\0' as i32
        && (esc as std::ffi::c_uint != 0
            || *buf.offset(e as isize) as std::ffi::c_int != '/' as i32
                && *buf.offset(e as isize) as std::ffi::c_int != '}' as i32)
    {
        esc = (esc as u64 == 0
            && *buf.offset(e as isize) as std::ffi::c_int == '\\' as i32
            && *buf.offset(e.wrapping_add(1 as std::ffi::c_int as size_t) as isize)
                as std::ffi::c_int != '\0' as i32) as std::ffi::c_int as lbool;
        e = e.wrapping_add(1);
        e;
    }
    return e;
}
unsafe extern "C" fn make_replaces(
    mut buf: *mut std::ffi::c_char,
    mut len: size_t,
    mut pe: *mut size_t,
    mut term: std::ffi::c_char,
) -> *mut replace {
    let mut e: size_t = *pe;
    let mut replaces: *mut replace = 0 as *mut replace;
    while term as std::ffi::c_int == '/' as i32 {
        let mut repl: *mut replace = 0 as *mut replace;
        let mut to: size_t = 0;
        let mut fm: size_t = e;
        e = skipsl(buf, len, e);
        if e >= len {
            break;
        }
        if e == fm {
            while e < len {
                let fresh0 = e;
                e = e.wrapping_add(1);
                if *buf.offset(fresh0 as isize) as std::ffi::c_int == '}' as i32 {
                    break;
                }
            }
            break;
        } else {
            term = *buf.offset(e as isize);
            let fresh1 = e;
            e = e.wrapping_add(1);
            *buf.offset(fresh1 as isize) = '\0' as i32 as std::ffi::c_char;
            if term as std::ffi::c_int != '/' as i32 {
                to = e.wrapping_sub(1 as std::ffi::c_int as size_t);
            } else {
                to = e;
                e = skipsl(buf, len, e);
                if e >= len {
                    break;
                }
                term = *buf.offset(e as isize);
                let fresh2 = e;
                e = e.wrapping_add(1);
                *buf.offset(fresh2 as isize) = '\0' as i32 as std::ffi::c_char;
            }
            repl = ecalloc(
                1 as std::ffi::c_int as size_t,
                ::core::mem::size_of::<replace>() as std::ffi::c_ulong,
            ) as *mut replace;
            (*repl).r_fm = &mut *buf.offset(fm as isize) as *mut std::ffi::c_char;
            (*repl).r_to = &mut *buf.offset(to as isize) as *mut std::ffi::c_char;
            (*repl).r_next = replaces;
            replaces = repl;
        }
    }
    *pe = e;
    return replaces;
}
unsafe extern "C" fn free_replaces(mut replaces: *mut replace) {
    while !replaces.is_null() {
        let mut r: *mut replace = replaces;
        replaces = (*r).r_next;
        free(r as *mut std::ffi::c_void);
    }
}
unsafe extern "C" fn evar_match(
    mut str: *const std::ffi::c_char,
    mut pat: *const std::ffi::c_char,
) -> size_t {
    let mut len: size_t = 0 as std::ffi::c_int as size_t;
    while *pat as std::ffi::c_int != '\0' as i32 {
        if *pat as std::ffi::c_int == '\\' as i32 {
            pat = pat.offset(1);
            pat;
        }
        let fresh3 = str;
        str = str.offset(1);
        let fresh4 = pat;
        pat = pat.offset(1);
        if *fresh3 as std::ffi::c_int != *fresh4 as std::ffi::c_int {
            return 0 as std::ffi::c_int as size_t;
        }
        len = len.wrapping_add(1);
        len;
    }
    return len;
}
unsafe extern "C" fn find_replace(
    mut repl: *const replace,
    mut evar: *const std::ffi::c_char,
    mut pv: *mut size_t,
) -> *const std::ffi::c_char {
    while !repl.is_null() {
        let mut len: size_t = evar_match(&*evar.offset(*pv as isize), (*repl).r_fm);
        if len > 0 as std::ffi::c_int as size_t {
            *pv = (*pv).wrapping_add(len);
            return (*repl).r_to;
        }
        repl = (*repl).r_next;
    }
    return 0 as *const std::ffi::c_char;
}
unsafe extern "C" fn add_evar(
    mut xbuf: *mut xbuffer,
    mut buf: *mut std::ffi::c_char,
    mut len: size_t,
    mut e: size_t,
    mut evar: *const std::ffi::c_char,
    mut term: std::ffi::c_char,
) -> size_t {
    let mut replaces: *mut replace = make_replaces(buf, len, &mut e, term);
    let mut v: size_t = 0;
    v = 0 as std::ffi::c_int as size_t;
    while *evar.offset(v as isize) as std::ffi::c_int != '\0' as i32 {
        let mut repl: *const std::ffi::c_char = find_replace(replaces, evar, &mut v);
        if repl.is_null() {
            let fresh5 = v;
            v = v.wrapping_add(1);
            xbuf_add_char(xbuf, *evar.offset(fresh5 as isize));
        } else {
            let mut r: size_t = 0;
            r = 0 as std::ffi::c_int as size_t;
            while *repl.offset(r as isize) as std::ffi::c_int != '\0' as i32 {
                if *repl.offset(r as isize) as std::ffi::c_int == '\\' as i32
                    && *repl
                        .offset(r.wrapping_add(1 as std::ffi::c_int as size_t) as isize)
                        as std::ffi::c_int != '\0' as i32
                {
                    r = r.wrapping_add(1);
                    r;
                }
                xbuf_add_char(xbuf, *repl.offset(r as isize));
                r = r.wrapping_add(1);
                r;
            }
        }
    }
    free_replaces(replaces);
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn expand_evars(
    mut buf: *mut std::ffi::c_char,
    mut len: size_t,
    mut xbuf: *mut xbuffer,
) {
    let mut i: size_t = 0;
    i = 0 as std::ffi::c_int as size_t;
    while i < len {
        if i.wrapping_add(1 as std::ffi::c_int as size_t) < len
            && *buf.offset(i as isize) as std::ffi::c_int == '$' as i32
            && *buf.offset(i.wrapping_add(1 as std::ffi::c_int as size_t) as isize)
                as std::ffi::c_int == '{' as i32
        {
            let mut evar: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
            let mut term: std::ffi::c_char = 0;
            let mut e: size_t = 0;
            i = i.wrapping_add(2 as std::ffi::c_int as size_t);
            e = i;
            while e < len {
                if *buf.offset(e as isize) as std::ffi::c_int == '\0' as i32
                    || *buf.offset(e as isize) as std::ffi::c_int == '}' as i32
                    || *buf.offset(e as isize) as std::ffi::c_int == '/' as i32
                {
                    break;
                }
                e = e.wrapping_add(1);
                e;
            }
            if e >= len || *buf.offset(e as isize) as std::ffi::c_int == '\0' as i32 {
                break;
            }
            term = *buf.offset(e as isize);
            let fresh6 = e;
            e = e.wrapping_add(1);
            *buf.offset(fresh6 as isize) = '\0' as i32 as std::ffi::c_char;
            evar = lgetenv_ext(&mut *buf.offset(i as isize), (*xbuf).data, (*xbuf).end);
            if evar.is_null() {
                evar = b"\0" as *const u8 as *const std::ffi::c_char;
            }
            i = add_evar(xbuf, buf, len, e, evar, term);
        } else {
            let fresh7 = i;
            i = i.wrapping_add(1);
            xbuf_add_char(xbuf, *buf.offset(fresh7 as isize));
        }
    }
}
