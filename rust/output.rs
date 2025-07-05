use ::libc;
extern "C" {
    fn write(__fd: std::ffi::c_int, __buf: *const std::ffi::c_void, __n: size_t) -> ssize_t;
    fn strcpy(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn quit(status: std::ffi::c_int);
    fn interactive() -> std::ffi::c_int;
    fn lower_left();
    fn clear_eol();
    fn clear_bot();
    fn at_enter(attr: std::ffi::c_int);
    fn at_exit();
    fn at_switch(attr: std::ffi::c_int);
    fn putbs();
    fn prchar(c: LWCHAR) -> *const std::ffi::c_char;
    fn prutfchar(ch: LWCHAR) -> *const std::ffi::c_char;
    fn step_charc(
        pp: *mut *const std::ffi::c_char,
        dir: std::ffi::c_int,
        limit: *const std::ffi::c_char,
    ) -> LWCHAR;
    fn screen_trashed();
    fn ungetcc(c: std::ffi::c_char);
    fn squish_check();
    fn gline(i: size_t, ap: *mut std::ffi::c_int) -> std::ffi::c_int;
    fn should_clear_after_line() -> lbool;
    fn supports_ctrl_x() -> std::ffi::c_int;
    fn getchr() -> std::ffi::c_int;
    static mut sigs: std::ffi::c_int;
    static mut sc_width: std::ffi::c_int;
    static mut so_s_width: std::ffi::c_int;
    static mut so_e_width: std::ffi::c_int;
    static mut oldbot: std::ffi::c_int;
    static mut utf_mode: std::ffi::c_int;
    static mut intr_char: std::ffi::c_char;
}
pub type __off_t = std::ffi::c_long;
pub type __ssize_t = std::ffi::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = std::ffi::c_ulong;
pub type lbool = std::ffi::c_uint;
pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;
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
pub static mut errmsgs: std::ffi::c_int = 0;
#[no_mangle]
pub static mut need_clr: std::ffi::c_int = 0;
#[no_mangle]
pub static mut final_attr: std::ffi::c_int = 0;
#[no_mangle]
pub static mut at_prompt: std::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn put_line(mut forw_scroll: lbool) {
    let mut c: std::ffi::c_int = 0;
    let mut i: size_t = 0;
    let mut a: std::ffi::c_int = 0;
    if sigs
        & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
            | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
            | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
        != 0
    {
        screen_trashed();
        return;
    }
    final_attr = 0 as std::ffi::c_int;
    i = 0 as std::ffi::c_int as size_t;
    loop {
        c = gline(i, &mut a);
        if !(c != '\0' as i32) {
            break;
        }
        at_switch(a);
        final_attr = a;
        if c == '\u{8}' as i32 {
            putbs();
        } else {
            putchr(c);
        }
        i = i.wrapping_add(1);
        i;
    }
    at_exit();
    if forw_scroll as std::ffi::c_uint != 0 && should_clear_after_line() as std::ffi::c_uint != 0 {
        clear_eol();
    }
}
static mut obuf: [std::ffi::c_char; 1024] = [0; 1024];
static mut ob: *mut std::ffi::c_char = unsafe { obuf.as_ptr() as *mut _ };
static mut outfd: std::ffi::c_int = 2 as std::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn flush() {
    let mut n: size_t = 0;
    n = ob.offset_from(obuf.as_mut_ptr()) as std::ffi::c_long as size_t;
    if n == 0 as std::ffi::c_int as size_t {
        return;
    }
    ob = obuf.as_mut_ptr();
    if write(outfd, obuf.as_mut_ptr() as *const std::ffi::c_void, n) as size_t != n {
        screen_trashed();
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_output(mut fd: std::ffi::c_int) {
    flush();
    outfd = fd;
}
#[no_mangle]
pub unsafe extern "C" fn putchr(mut ch: std::ffi::c_int) -> std::ffi::c_int {
    let mut c: std::ffi::c_char = ch as std::ffi::c_char;
    clear_bot_if_needed();
    if ob
        >= &mut *obuf.as_mut_ptr().offset(
            (::core::mem::size_of::<[std::ffi::c_char; 1024]>() as std::ffi::c_ulong)
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong) as isize,
        ) as *mut std::ffi::c_char
    {
        flush();
    }
    let fresh0 = ob;
    ob = ob.offset(1);
    *fresh0 = c;
    at_prompt = 0 as std::ffi::c_int;
    return c as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clear_bot_if_needed() {
    if need_clr == 0 {
        return;
    }
    need_clr = 0 as std::ffi::c_int;
    clear_bot();
}
#[no_mangle]
pub unsafe extern "C" fn putstr(mut s: *const std::ffi::c_char) {
    while *s as std::ffi::c_int != '\0' as i32 {
        let fresh1 = s;
        s = s.offset(1);
        putchr(*fresh1 as std::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn postoa(
    mut num: POSITION,
    mut buf: *mut std::ffi::c_char,
    mut radix: std::ffi::c_int,
) {
    let mut neg: std::ffi::c_int = (num < 0 as std::ffi::c_int as POSITION) as std::ffi::c_int;
    let mut tbuf: [std::ffi::c_char; 23] = [0; 23];
    let mut s: *mut std::ffi::c_char = tbuf
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[std::ffi::c_char; 23]>() as std::ffi::c_ulong as isize);
    if neg != 0 {
        num = -num;
    }
    s = s.offset(-1);
    *s = '\0' as i32 as std::ffi::c_char;
    loop {
        s = s.offset(-1);
        *s = (*::core::mem::transmute::<&[u8; 17], &[std::ffi::c_char; 17]>(b"0123456789ABCDEF\0"))
            [(num % radix as POSITION) as usize];
        num /= radix as POSITION;
        if !(num != 0 as std::ffi::c_int as POSITION) {
            break;
        }
    }
    if neg != 0 {
        s = s.offset(-1);
        *s = '-' as i32 as std::ffi::c_char;
    }
    strcpy(buf, s);
}
#[no_mangle]
pub unsafe extern "C" fn linenumtoa(
    mut num: LINENUM,
    mut buf: *mut std::ffi::c_char,
    mut radix: std::ffi::c_int,
) {
    let mut neg: std::ffi::c_int = (num < 0 as std::ffi::c_int as LINENUM) as std::ffi::c_int;
    let mut tbuf: [std::ffi::c_char; 23] = [0; 23];
    let mut s: *mut std::ffi::c_char = tbuf
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[std::ffi::c_char; 23]>() as std::ffi::c_ulong as isize);
    if neg != 0 {
        num = -num;
    }
    s = s.offset(-1);
    *s = '\0' as i32 as std::ffi::c_char;
    loop {
        s = s.offset(-1);
        *s = (*::core::mem::transmute::<&[u8; 17], &[std::ffi::c_char; 17]>(b"0123456789ABCDEF\0"))
            [(num % radix as LINENUM) as usize];
        num /= radix as LINENUM;
        if !(num != 0 as std::ffi::c_int as LINENUM) {
            break;
        }
    }
    if neg != 0 {
        s = s.offset(-1);
        *s = '-' as i32 as std::ffi::c_char;
    }
    strcpy(buf, s);
}
#[no_mangle]
pub unsafe extern "C" fn inttoa(
    mut num: std::ffi::c_int,
    mut buf: *mut std::ffi::c_char,
    mut radix: std::ffi::c_int,
) {
    let mut neg: std::ffi::c_int = (num < 0 as std::ffi::c_int) as std::ffi::c_int;
    let mut tbuf: [std::ffi::c_char; 13] = [0; 13];
    let mut s: *mut std::ffi::c_char = tbuf
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[std::ffi::c_char; 13]>() as std::ffi::c_ulong as isize);
    if neg != 0 {
        num = -num;
    }
    s = s.offset(-1);
    *s = '\0' as i32 as std::ffi::c_char;
    loop {
        s = s.offset(-1);
        *s = (*::core::mem::transmute::<&[u8; 17], &[std::ffi::c_char; 17]>(b"0123456789ABCDEF\0"))
            [(num % radix) as usize];
        num /= radix;
        if !(num != 0 as std::ffi::c_int) {
            break;
        }
    }
    if neg != 0 {
        s = s.offset(-1);
        *s = '-' as i32 as std::ffi::c_char;
    }
    strcpy(buf, s);
}
#[no_mangle]
pub unsafe extern "C" fn lstrtopos(
    mut buf: *mut std::ffi::c_char,
    mut ebuf: *mut *mut std::ffi::c_char,
    mut radix: std::ffi::c_int,
) -> POSITION {
    let mut cbuf: *const std::ffi::c_char = buf;
    let mut r: POSITION = lstrtoposc(cbuf, &mut cbuf, radix);
    if !ebuf.is_null() {
        *ebuf = cbuf as *mut std::ffi::c_char;
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn lstrtoposc(
    mut buf: *const std::ffi::c_char,
    mut ebuf: *mut *const std::ffi::c_char,
    mut radix: std::ffi::c_int,
) -> POSITION {
    let mut val: POSITION = 0 as std::ffi::c_int as POSITION;
    let mut v: lbool = LFALSE;
    loop {
        let mut c: std::ffi::c_char = *buf;
        let mut digit: std::ffi::c_int =
            if c as std::ffi::c_int >= '0' as i32 && c as std::ffi::c_int <= '9' as i32 {
                c as std::ffi::c_int - '0' as i32
            } else if c as std::ffi::c_int >= 'a' as i32 && c as std::ffi::c_int <= 'f' as i32 {
                c as std::ffi::c_int - 'a' as i32 + 10 as std::ffi::c_int
            } else if c as std::ffi::c_int >= 'A' as i32 && c as std::ffi::c_int <= 'F' as i32 {
                c as std::ffi::c_int - 'A' as i32 + 10 as std::ffi::c_int
            } else {
                -(1 as std::ffi::c_int)
            };
        if digit < 0 as std::ffi::c_int || digit >= radix {
            break;
        }
        v = (v as std::ffi::c_uint != 0 || {
            let (fresh2, fresh3) = val.overflowing_mul(radix as i64);
            *(&mut val as *mut POSITION) = fresh2;
            fresh3 as std::ffi::c_int != 0
        }) as std::ffi::c_int as lbool;
        v = (v as std::ffi::c_uint != 0 || {
            let (fresh4, fresh5) = val.overflowing_add(digit as i64);
            *(&mut val as *mut POSITION) = fresh4;
            fresh5 as std::ffi::c_int != 0
        }) as std::ffi::c_int as lbool;
        buf = buf.offset(1);
        buf;
    }
    if !ebuf.is_null() {
        *ebuf = buf;
    }
    return if v as std::ffi::c_uint != 0 {
        -(1 as std::ffi::c_int) as POSITION
    } else {
        val
    };
}
#[no_mangle]
pub unsafe extern "C" fn lstrtoi(
    mut buf: *mut std::ffi::c_char,
    mut ebuf: *mut *mut std::ffi::c_char,
    mut radix: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut cbuf: *const std::ffi::c_char = buf;
    let mut r: std::ffi::c_int = lstrtoic(cbuf, &mut cbuf, radix);
    if !ebuf.is_null() {
        *ebuf = cbuf as *mut std::ffi::c_char;
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn lstrtoic(
    mut buf: *const std::ffi::c_char,
    mut ebuf: *mut *const std::ffi::c_char,
    mut radix: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut val: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut v: lbool = LFALSE;
    loop {
        let mut c: std::ffi::c_char = *buf;
        let mut digit: std::ffi::c_int =
            if c as std::ffi::c_int >= '0' as i32 && c as std::ffi::c_int <= '9' as i32 {
                c as std::ffi::c_int - '0' as i32
            } else if c as std::ffi::c_int >= 'a' as i32 && c as std::ffi::c_int <= 'f' as i32 {
                c as std::ffi::c_int - 'a' as i32 + 10 as std::ffi::c_int
            } else if c as std::ffi::c_int >= 'A' as i32 && c as std::ffi::c_int <= 'F' as i32 {
                c as std::ffi::c_int - 'A' as i32 + 10 as std::ffi::c_int
            } else {
                -(1 as std::ffi::c_int)
            };
        if digit < 0 as std::ffi::c_int || digit >= radix {
            break;
        }
        v = (v as std::ffi::c_uint != 0 || {
            let (fresh6, fresh7) = val.overflowing_mul(radix);
            *(&mut val as *mut std::ffi::c_int) = fresh6;
            fresh7 as std::ffi::c_int != 0
        }) as std::ffi::c_int as lbool;
        v = (v as std::ffi::c_uint != 0 || {
            let (fresh8, fresh9) = val.overflowing_add(digit);
            *(&mut val as *mut std::ffi::c_int) = fresh8;
            fresh9 as std::ffi::c_int != 0
        }) as std::ffi::c_int as lbool;
        buf = buf.offset(1);
        buf;
    }
    if !ebuf.is_null() {
        *ebuf = buf;
    }
    return if v as std::ffi::c_uint != 0 {
        -(1 as std::ffi::c_int)
    } else {
        val
    };
}
#[no_mangle]
pub unsafe extern "C" fn lstrtoul(
    mut buf: *mut std::ffi::c_char,
    mut ebuf: *mut *mut std::ffi::c_char,
    mut radix: std::ffi::c_int,
) -> std::ffi::c_ulong {
    let mut cbuf: *const std::ffi::c_char = buf;
    let mut r: std::ffi::c_ulong = lstrtoulc(cbuf, &mut cbuf, radix);
    if !ebuf.is_null() {
        *ebuf = cbuf as *mut std::ffi::c_char;
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn lstrtoulc(
    mut buf: *const std::ffi::c_char,
    mut ebuf: *mut *const std::ffi::c_char,
    mut radix: std::ffi::c_int,
) -> std::ffi::c_ulong {
    let mut val: std::ffi::c_ulong = 0 as std::ffi::c_int as std::ffi::c_ulong;
    let mut v: lbool = LFALSE;
    loop {
        let mut c: std::ffi::c_char = *buf;
        let mut digit: std::ffi::c_int =
            if c as std::ffi::c_int >= '0' as i32 && c as std::ffi::c_int <= '9' as i32 {
                c as std::ffi::c_int - '0' as i32
            } else if c as std::ffi::c_int >= 'a' as i32 && c as std::ffi::c_int <= 'f' as i32 {
                c as std::ffi::c_int - 'a' as i32 + 10 as std::ffi::c_int
            } else if c as std::ffi::c_int >= 'A' as i32 && c as std::ffi::c_int <= 'F' as i32 {
                c as std::ffi::c_int - 'A' as i32 + 10 as std::ffi::c_int
            } else {
                -(1 as std::ffi::c_int)
            };
        if digit < 0 as std::ffi::c_int || digit >= radix {
            break;
        }
        v = (v as std::ffi::c_uint != 0 || {
            let (fresh10, fresh11) = val.overflowing_mul(radix as u64);
            *(&mut val as *mut std::ffi::c_ulong) = fresh10;
            fresh11 as std::ffi::c_int != 0
        }) as std::ffi::c_int as lbool;
        v = (v as std::ffi::c_uint != 0 || {
            let (fresh12, fresh13) = val.overflowing_add(digit as u64);
            *(&mut val as *mut std::ffi::c_ulong) = fresh12;
            fresh13 as std::ffi::c_int != 0
        }) as std::ffi::c_int as lbool;
        buf = buf.offset(1);
        buf;
    }
    if !ebuf.is_null() {
        *ebuf = buf;
    }
    return if v as std::ffi::c_uint != 0 {
        -(1 as std::ffi::c_int) as std::ffi::c_ulong
    } else {
        val
    };
}
unsafe extern "C" fn iprint_int(
    mut num: std::ffi::c_int,
    mut radix: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut buf: [std::ffi::c_char; 11] = [0; 11];
    inttoa(num, buf.as_mut_ptr(), radix);
    putstr(buf.as_mut_ptr());
    return strlen(buf.as_mut_ptr()) as std::ffi::c_int;
}
unsafe extern "C" fn iprint_linenum(
    mut num: LINENUM,
    mut radix: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut buf: [std::ffi::c_char; 21] = [0; 21];
    linenumtoa(num, buf.as_mut_ptr(), radix);
    putstr(buf.as_mut_ptr());
    return strlen(buf.as_mut_ptr()) as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn less_printf(
    mut fmt: *const std::ffi::c_char,
    mut parg: *mut PARG,
) -> std::ffi::c_int {
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut es: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut col: std::ffi::c_int = 0;
    col = 0 as std::ffi::c_int;
    while *fmt as std::ffi::c_int != '\0' as i32 {
        if *fmt as std::ffi::c_int != '%' as i32 {
            let fresh14 = fmt;
            fmt = fmt.offset(1);
            putchr(*fresh14 as std::ffi::c_int);
            col += 1;
            col;
        } else {
            fmt = fmt.offset(1);
            fmt;
            let fresh15 = fmt;
            fmt = fmt.offset(1);
            match *fresh15 as std::ffi::c_int {
                115 => {
                    s = (*parg).p_string;
                    es = s.offset(strlen(s) as isize);
                    parg = parg.offset(1);
                    parg;
                    while *s as std::ffi::c_int != '\0' as i32 {
                        let mut ch: LWCHAR = step_charc(&mut s, 1 as std::ffi::c_int, es);
                        let mut ps: *const std::ffi::c_char = if utf_mode != 0 {
                            prutfchar(ch)
                        } else {
                            prchar(ch)
                        };
                        while *ps as std::ffi::c_int != '\0' as i32 {
                            let fresh16 = ps;
                            ps = ps.offset(1);
                            putchr(*fresh16 as std::ffi::c_int);
                            col += 1;
                            col;
                        }
                    }
                }
                100 => {
                    col += iprint_int((*parg).p_int, 10 as std::ffi::c_int);
                    parg = parg.offset(1);
                    parg;
                }
                120 => {
                    col += iprint_int((*parg).p_int, 16 as std::ffi::c_int);
                    parg = parg.offset(1);
                    parg;
                }
                110 => {
                    col += iprint_linenum((*parg).p_linenum, 10 as std::ffi::c_int);
                    parg = parg.offset(1);
                    parg;
                }
                99 => {
                    s = prchar((*parg).p_char as LWCHAR);
                    parg = parg.offset(1);
                    parg;
                    while *s as std::ffi::c_int != '\0' as i32 {
                        let fresh17 = s;
                        s = s.offset(1);
                        putchr(*fresh17 as std::ffi::c_int);
                        col += 1;
                        col;
                    }
                }
                37 => {
                    putchr('%' as i32);
                }
                _ => {}
            }
        }
    }
    return col;
}
#[no_mangle]
pub unsafe extern "C" fn get_return() {
    let mut c: std::ffi::c_int = 0;
    c = getchr();
    if c != '\n' as i32 && c != '\r' as i32 && c != ' ' as i32 && c != -(2 as std::ffi::c_int) {
        ungetcc(c as std::ffi::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn error(mut fmt: *const std::ffi::c_char, mut parg: *mut PARG) {
    let mut col: std::ffi::c_int = 0 as std::ffi::c_int;
    static mut return_to_continue: [std::ffi::c_char; 17] = unsafe {
        *::core::mem::transmute::<&[u8; 17], &mut [std::ffi::c_char; 17]>(b"  (press RETURN)\0")
    };
    errmsgs += 1;
    errmsgs;
    if interactive() == 0 {
        less_printf(fmt, parg);
        putchr('\n' as i32);
        return;
    }
    if oldbot == 0 {
        squish_check();
    }
    at_exit();
    clear_bot();
    at_enter(
        (1 as std::ffi::c_int) << 3 as std::ffi::c_int
            | (4 as std::ffi::c_int) << 8 as std::ffi::c_int,
    );
    col += so_s_width;
    col += less_printf(fmt, parg);
    putstr(return_to_continue.as_mut_ptr());
    at_exit();
    col += ::core::mem::size_of::<[std::ffi::c_char; 17]>() as std::ffi::c_ulong as std::ffi::c_int
        + so_e_width;
    get_return();
    lower_left();
    clear_eol();
    if col >= sc_width {
        screen_trashed();
    }
    flush();
}
unsafe extern "C" fn ierror_suffix(
    mut fmt: *const std::ffi::c_char,
    mut parg: *mut PARG,
    mut suffix1: *const std::ffi::c_char,
    mut suffix2: *const std::ffi::c_char,
    mut suffix3: *const std::ffi::c_char,
) {
    at_exit();
    clear_bot();
    at_enter(
        (1 as std::ffi::c_int) << 3 as std::ffi::c_int
            | (4 as std::ffi::c_int) << 8 as std::ffi::c_int,
    );
    less_printf(fmt, parg);
    putstr(suffix1);
    putstr(suffix2);
    putstr(suffix3);
    at_exit();
    flush();
    need_clr = 1 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ierror(mut fmt: *const std::ffi::c_char, mut parg: *mut PARG) {
    ierror_suffix(
        fmt,
        parg,
        b"... (interrupt to abort)\0" as *const u8 as *const std::ffi::c_char,
        b"\0" as *const u8 as *const std::ffi::c_char,
        b"\0" as *const u8 as *const std::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ixerror(mut fmt: *const std::ffi::c_char, mut parg: *mut PARG) {
    if supports_ctrl_x() == 0 {
        ierror(fmt, parg);
    } else {
        let mut ichar: [std::ffi::c_char; 32] = [0; 32];
        strcpy(ichar.as_mut_ptr(), prchar(intr_char as LWCHAR));
        ierror_suffix(
            fmt,
            parg,
            b"... (\0" as *const u8 as *const std::ffi::c_char,
            ichar.as_mut_ptr(),
            b" or interrupt to abort)\0" as *const u8 as *const std::ffi::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn query(
    mut fmt: *const std::ffi::c_char,
    mut parg: *mut PARG,
) -> std::ffi::c_int {
    let mut c: std::ffi::c_int = 0;
    let mut col: std::ffi::c_int = 0 as std::ffi::c_int;
    if interactive() != 0 {
        clear_bot();
    }
    less_printf(fmt, parg);
    c = getchr();
    if interactive() != 0 {
        lower_left();
        if col >= sc_width {
            screen_trashed();
        }
        flush();
    } else {
        putchr('\n' as i32);
    }
    if c == 'Q' as i32 {
        quit(0 as std::ffi::c_int);
    }
    return c;
}
