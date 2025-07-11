use crate::decode::lgetenv;
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
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn lstrtoic(
        _: *const std::ffi::c_char,
        _: *mut *const std::ffi::c_char,
        _: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn save(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
    fn skipspc(s: *const std::ffi::c_char) -> *const std::ffi::c_char;
    fn ch_getflags() -> std::ffi::c_int;
    fn prchar(c: LWCHAR) -> *const std::ffi::c_char;
    fn screen_trashed();
    fn ungetcc_end_command();
    fn ungetsc(s: *const std::ffi::c_char);
    fn isnullenv(s: *const std::ffi::c_char) -> lbool;
    fn findopt(c: std::ffi::c_int) -> *mut loption;
    fn findopt_name(
        p_optname: *mut *const std::ffi::c_char,
        p_oname: *mut *const std::ffi::c_char,
        p_ambig: *mut lbool,
    ) -> *mut loption;
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn repaint_hilite(on: lbool);
    fn chg_hilite();
    static mut less_is_more: std::ffi::c_int;
    static mut quit_at_eof: std::ffi::c_int;
    static mut every_first_cmd: *mut std::ffi::c_char;
    static mut opt_use_backslash: std::ffi::c_int;
    static mut ctldisp: std::ffi::c_int;
}
pub type __off_t = std::ffi::c_long;
pub type off_t = __off_t;
pub type size_t = std::ffi::c_ulong;
pub type lbool = std::ffi::c_uint;
pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;
pub type LWCHAR = std::ffi::c_ulong;
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
pub struct loption {
    pub oletter: std::ffi::c_char,
    pub onames: *mut optname,
    pub otype: std::ffi::c_int,
    pub odefault: std::ffi::c_int,
    pub ovar: *mut std::ffi::c_int,
    pub ofunc: Option<unsafe extern "C" fn(std::ffi::c_int, *const std::ffi::c_char) -> ()>,
    pub odesc: [*const std::ffi::c_char; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct optname {
    pub oname: *const std::ffi::c_char,
    pub onext: *mut optname,
}
static mut pendopt: *mut loption = 0 as *const loption as *mut loption;
#[no_mangle]
pub static mut plusoption: lbool = LFALSE;
unsafe extern "C" fn opt_desc(mut o: *mut loption) -> *const std::ffi::c_char {
    static mut buf: [std::ffi::c_char; 42] = [0; 42];
    if (*o).oletter as std::ffi::c_int == '\u{1}' as i32 {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[std::ffi::c_char; 42]>() as std::ffi::c_ulong,
            b"--%s\0" as *const u8 as *const std::ffi::c_char,
            (*(*o).onames).oname,
        );
    } else {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[std::ffi::c_char; 42]>() as std::ffi::c_ulong,
            b"-%c (--%s)\0" as *const u8 as *const std::ffi::c_char,
            (*o).oletter as std::ffi::c_int,
            (*(*o).onames).oname,
        );
    }
    return buf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn propt(mut c: std::ffi::c_char) -> *const std::ffi::c_char {
    static mut buf: [std::ffi::c_char; 33] = [0; 33];
    sprintf(
        buf.as_mut_ptr(),
        b"-%s\0" as *const u8 as *const std::ffi::c_char,
        prchar(c as LWCHAR),
    );
    return buf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn scan_option(mut s: *const std::ffi::c_char, mut is_env: lbool) {
    let mut o: *mut loption = 0 as *mut loption;
    let mut optc: std::ffi::c_char = 0;
    let mut optname: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut printopt: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut str: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut set_default: lbool = LFALSE;
    let mut lc: lbool = LFALSE;
    let mut ambig: lbool = LFALSE;
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    if s.is_null() {
        return;
    }
    if !pendopt.is_null() {
        if (*pendopt).otype & 0o2000 as std::ffi::c_int == 0 {
            match (*pendopt).otype
                & (0o1 as std::ffi::c_int
                    | 0o2 as std::ffi::c_int
                    | 0o4 as std::ffi::c_int
                    | 0o10 as std::ffi::c_int
                    | 0o20 as std::ffi::c_int)
            {
                8 => {
                    (Some(((*pendopt).ofunc).expect("non-null function pointer")))
                        .expect("non-null function pointer")(
                        0 as std::ffi::c_int, s
                    );
                }
                4 => {
                    printopt = opt_desc(pendopt);
                    *(*pendopt).ovar = getnumc(&mut s, printopt, 0 as *mut lbool);
                }
                _ => {}
            }
        }
        pendopt = 0 as *mut loption;
        return;
    }
    set_default = LFALSE;
    optname = 0 as *const std::ffi::c_char;
    while *s as std::ffi::c_int != '\0' as i32 {
        let fresh0 = s;
        s = s.offset(1);
        optc = *fresh0;
        match optc as std::ffi::c_int {
            32 | 9 | 36 => {
                continue;
            }
            45 => {
                if *s as std::ffi::c_int == '-' as i32 {
                    s = s.offset(1);
                    optname = s;
                }
                set_default = (*s as std::ffi::c_int == '+' as i32) as std::ffi::c_int as lbool;
                if set_default as u64 != 0 {
                    s = s.offset(1);
                }
                if optname.is_null() {
                    continue;
                }
                optname = s;
            }
            43 => {
                plusoption = LTRUE;
                s = optstring(
                    s,
                    &mut str,
                    propt('+' as i32 as std::ffi::c_char),
                    0 as *const std::ffi::c_char,
                );
                if s.is_null() {
                    return;
                }
                if *str as std::ffi::c_int == '+' as i32 {
                    if !every_first_cmd.is_null() {
                        free(every_first_cmd as *mut std::ffi::c_void);
                    }
                    every_first_cmd = save(str.offset(1 as std::ffi::c_int as isize));
                } else {
                    ungetsc(str);
                    ungetcc_end_command();
                }
                free(str as *mut std::ffi::c_void);
                continue;
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                s = s.offset(-1);
                optc = 'z' as i32 as std::ffi::c_char;
            }
            110 => {
                if less_is_more != 0 {
                    optc = 'z' as i32 as std::ffi::c_char;
                }
            }
            _ => {}
        }
        ambig = LFALSE;
        if optname.is_null() {
            printopt = propt(optc);
            lc = (optc as std::ffi::c_int >= 'a' as i32 && optc as std::ffi::c_int <= 'z' as i32)
                as std::ffi::c_int as lbool;
            o = findopt(optc as std::ffi::c_int);
        } else {
            printopt = optname;
            lc = (*optname.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int >= 'a' as i32
                && *optname.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int <= 'z' as i32)
                as std::ffi::c_int as lbool;
            o = findopt_name(&mut optname, 0 as *mut *const std::ffi::c_char, &mut ambig);
            s = optname;
            optname = 0 as *const std::ffi::c_char;
            if !(*s as std::ffi::c_int == '\0' as i32 || *s as std::ffi::c_int == ' ' as i32) {
                if *s as std::ffi::c_int == '=' as i32 {
                    if !o.is_null()
                        && (*o).otype
                            & (0o1 as std::ffi::c_int
                                | 0o2 as std::ffi::c_int
                                | 0o4 as std::ffi::c_int
                                | 0o10 as std::ffi::c_int
                                | 0o20 as std::ffi::c_int)
                            != 0o10 as std::ffi::c_int
                        && (*o).otype
                            & (0o1 as std::ffi::c_int
                                | 0o2 as std::ffi::c_int
                                | 0o4 as std::ffi::c_int
                                | 0o10 as std::ffi::c_int
                                | 0o20 as std::ffi::c_int)
                            != 0o4 as std::ffi::c_int
                    {
                        parg.p_string = printopt;
                        error(
                            b"The %s option should not be followed by =\0" as *const u8
                                as *const std::ffi::c_char,
                            &mut parg,
                        );
                        return;
                    }
                    s = s.offset(1);
                } else {
                    o = 0 as *mut loption;
                }
            }
        }
        if o.is_null() {
            parg.p_string = printopt;
            if ambig as u64 != 0 {
                error(
                    b"%s is an ambiguous abbreviation (\"less --help\" for help)\0" as *const u8
                        as *const std::ffi::c_char,
                    &mut parg,
                );
            } else {
                error(
                    b"There is no %s option (\"less --help\" for help)\0" as *const u8
                        as *const std::ffi::c_char,
                    &mut parg,
                );
            }
            return;
        }
        str = 0 as *mut std::ffi::c_char;
        match (*o).otype
            & (0o1 as std::ffi::c_int
                | 0o2 as std::ffi::c_int
                | 0o4 as std::ffi::c_int
                | 0o10 as std::ffi::c_int
                | 0o20 as std::ffi::c_int)
        {
            1 => {
                if !((*o).otype & 0o2000 as std::ffi::c_int != 0) {
                    if !((*o).ovar).is_null() {
                        if set_default as u64 != 0 {
                            *(*o).ovar = (*o).odefault;
                        } else {
                            *(*o).ovar = ((*o).odefault == 0) as std::ffi::c_int;
                        }
                    }
                }
            }
            2 => {
                if !((*o).otype & 0o2000 as std::ffi::c_int != 0) {
                    if !((*o).ovar).is_null() {
                        if set_default as u64 != 0 {
                            *(*o).ovar = (*o).odefault;
                        } else if is_env as std::ffi::c_uint != 0
                            && (*o).ovar == &mut ctldisp as *mut std::ffi::c_int
                        {
                            *(*o).ovar = 2 as std::ffi::c_int;
                        } else {
                            *(*o).ovar = flip_triple((*o).odefault, lc);
                        }
                    }
                }
            }
            8 => {
                if *s as std::ffi::c_int == '\0' as i32 {
                    pendopt = o;
                    return;
                }
                while *s as std::ffi::c_int == ' ' as i32 {
                    s = s.offset(1);
                }
                s = optstring(
                    s,
                    &mut str,
                    printopt,
                    (*o).odesc[1 as std::ffi::c_int as usize],
                );
                if s.is_null() {
                    return;
                }
            }
            4 => {
                if *s as std::ffi::c_int == '\0' as i32 {
                    pendopt = o;
                    return;
                }
                if !((*o).otype & 0o2000 as std::ffi::c_int != 0) {
                    *(*o).ovar = getnumc(&mut s, printopt, 0 as *mut lbool);
                }
            }
            _ => {}
        }
        if ((*o).ofunc).is_some() && (*o).otype & 0o2000 as std::ffi::c_int == 0 {
            (Some(((*o).ofunc).expect("non-null function pointer")))
                .expect("non-null function pointer")(0 as std::ffi::c_int, str);
        }
        if !str.is_null() {
            free(str as *mut std::ffi::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn toggle_option(
    mut o: *mut loption,
    mut lower: lbool,
    mut s: *const std::ffi::c_char,
    mut how_toggle: std::ffi::c_int,
) {
    let mut num: std::ffi::c_int = 0;
    let mut no_prompt: std::ffi::c_int = 0;
    let mut err: lbool = LFALSE;
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    no_prompt = how_toggle & 0o100 as std::ffi::c_int;
    how_toggle &= !(0o100 as std::ffi::c_int);
    if o.is_null() {
        error(
            b"No such option\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        return;
    }
    if how_toggle == 1 as std::ffi::c_int && (*o).otype & 0o100 as std::ffi::c_int != 0 {
        parg.p_string = opt_desc(o);
        error(
            b"Cannot change the %s option\0" as *const u8 as *const std::ffi::c_char,
            &mut parg,
        );
        return;
    }
    if how_toggle == 0 as std::ffi::c_int && (*o).otype & 0o400 as std::ffi::c_int != 0 {
        parg.p_string = opt_desc(o);
        error(
            b"Cannot query the %s option\0" as *const u8 as *const std::ffi::c_char,
            &mut parg,
        );
        return;
    }
    match (*o).otype
        & (0o1 as std::ffi::c_int
            | 0o2 as std::ffi::c_int
            | 0o4 as std::ffi::c_int
            | 0o10 as std::ffi::c_int
            | 0o20 as std::ffi::c_int)
    {
        8 | 4 => {
            if how_toggle == 1 as std::ffi::c_int && *s as std::ffi::c_int == '\0' as i32 {
                how_toggle = 0 as std::ffi::c_int;
            }
        }
        _ => {}
    }
    if how_toggle != 0 as std::ffi::c_int && (*o).otype & 0o200 as std::ffi::c_int != 0 {
        repaint_hilite(LFALSE);
    }
    if how_toggle != 0 as std::ffi::c_int {
        match (*o).otype
            & (0o1 as std::ffi::c_int
                | 0o2 as std::ffi::c_int
                | 0o4 as std::ffi::c_int
                | 0o10 as std::ffi::c_int
                | 0o20 as std::ffi::c_int)
        {
            1 => {
                if !((*o).ovar).is_null() {
                    match how_toggle {
                        1 => {
                            *(*o).ovar = (*(*o).ovar == 0) as std::ffi::c_int;
                        }
                        2 => {
                            *(*o).ovar = (*o).odefault;
                        }
                        3 => {
                            *(*o).ovar = ((*o).odefault == 0) as std::ffi::c_int;
                        }
                        _ => {}
                    }
                }
            }
            2 => {
                if !((*o).ovar).is_null() {
                    match how_toggle {
                        1 => {
                            *(*o).ovar = flip_triple(*(*o).ovar, lower);
                        }
                        2 => {
                            *(*o).ovar = (*o).odefault;
                        }
                        3 => {
                            *(*o).ovar = flip_triple((*o).odefault, lower);
                        }
                        _ => {}
                    }
                }
            }
            8 => match how_toggle {
                3 | 2 => {
                    error(
                        b"Cannot use \"-+\" or \"-!\" for a string option\0" as *const u8
                            as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void as *mut PARG,
                    );
                    return;
                }
                _ => {}
            },
            4 => match how_toggle {
                1 => {
                    num = getnumc(&mut s, 0 as *const std::ffi::c_char, &mut err);
                    if err as u64 == 0 {
                        *(*o).ovar = num;
                    }
                }
                2 => {
                    *(*o).ovar = (*o).odefault;
                }
                3 => {
                    error(
                        b"Can't use \"-!\" for a numeric option\0" as *const u8
                            as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void as *mut PARG,
                    );
                    return;
                }
                _ => {}
            },
            _ => {}
        }
    }
    if ((*o).ofunc).is_some() {
        (Some(((*o).ofunc).expect("non-null function pointer")))
            .expect("non-null function pointer")(
            if how_toggle == 0 as std::ffi::c_int {
                1 as std::ffi::c_int
            } else {
                2 as std::ffi::c_int
            },
            s,
        );
    }
    if how_toggle != 0 as std::ffi::c_int && (*o).otype & 0o200 as std::ffi::c_int != 0 {
        chg_hilite();
    }
    if no_prompt == 0 {
        match (*o).otype
            & (0o1 as std::ffi::c_int
                | 0o2 as std::ffi::c_int
                | 0o4 as std::ffi::c_int
                | 0o10 as std::ffi::c_int
                | 0o20 as std::ffi::c_int)
        {
            1 | 2 => {
                if !((*o).ovar).is_null() {
                    error(
                        (*o).odesc[*(*o).ovar as usize],
                        0 as *mut std::ffi::c_void as *mut PARG,
                    );
                }
            }
            4 => {
                parg.p_int = *(*o).ovar;
                error((*o).odesc[1 as std::ffi::c_int as usize], &mut parg);
            }
            8 | _ => {}
        }
    }
    if how_toggle != 0 as std::ffi::c_int && (*o).otype & 0o40 as std::ffi::c_int != 0 {
        screen_trashed();
    }
}
unsafe extern "C" fn flip_triple(mut val: std::ffi::c_int, mut lc: lbool) -> std::ffi::c_int {
    if lc as u64 != 0 {
        return if val == 1 as std::ffi::c_int {
            0 as std::ffi::c_int
        } else {
            1 as std::ffi::c_int
        };
    } else {
        return if val == 2 as std::ffi::c_int {
            0 as std::ffi::c_int
        } else {
            2 as std::ffi::c_int
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_has_param(mut o: *mut loption) -> std::ffi::c_int {
    if o.is_null() {
        return 0 as std::ffi::c_int;
    }
    if (*o).otype
        & (0o1 as std::ffi::c_int
            | 0o2 as std::ffi::c_int
            | 0o20 as std::ffi::c_int
            | 0o100 as std::ffi::c_int)
        != 0
    {
        return 0 as std::ffi::c_int;
    }
    return 1 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn opt_prompt(mut o: *mut loption) -> *const std::ffi::c_char {
    if o.is_null()
        || (*o).otype & (0o10 as std::ffi::c_int | 0o4 as std::ffi::c_int) == 0 as std::ffi::c_int
    {
        return b"?\0" as *const u8 as *const std::ffi::c_char;
    }
    return (*o).odesc[0 as std::ffi::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn opt_toggle_disallowed(mut c: std::ffi::c_int) -> *const std::ffi::c_char {
    match c {
        111 => {
            if ch_getflags() & 0o1 as std::ffi::c_int != 0 {
                return b"Input is not a pipe\0" as *const u8 as *const std::ffi::c_char;
            }
        }
        _ => {}
    }
    return 0 as *const std::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn isoptpending() -> lbool {
    return (pendopt != 0 as *mut std::ffi::c_void as *mut loption) as std::ffi::c_int as lbool;
}
unsafe extern "C" fn nostring(mut printopt: *const std::ffi::c_char) {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    parg.p_string = printopt;
    error(
        b"Value is required after %s\0" as *const u8 as *const std::ffi::c_char,
        &mut parg,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nopendopt() {
    nostring(opt_desc(pendopt));
}
unsafe extern "C" fn optstring(
    mut s: *const std::ffi::c_char,
    mut p_str: *mut *mut std::ffi::c_char,
    mut printopt: *const std::ffi::c_char,
    mut validchars: *const std::ffi::c_char,
) -> *const std::ffi::c_char {
    let mut p: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut out: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    if *s as std::ffi::c_int == '\0' as i32 {
        nostring(printopt);
        return 0 as *const std::ffi::c_char;
    }
    *p_str = ecalloc(
        (strlen(s)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong),
        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_char;
    out = *p_str;
    p = s;
    while *p as std::ffi::c_int != '\0' as i32 {
        if opt_use_backslash != 0
            && *p as std::ffi::c_int == '\\' as i32
            && *p.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int != '\0' as i32
        {
            p = p.offset(1);
        } else {
            if !validchars.is_null() {
                if *validchars.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                    == 's' as i32
                {
                    if *p as std::ffi::c_int == ' ' as i32 {
                        break;
                    }
                } else if *p as std::ffi::c_int == '-' as i32 {
                    if *validchars.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                        != '-' as i32
                    {
                        break;
                    }
                    validchars = validchars.offset(1);
                } else if *p as std::ffi::c_int == '.' as i32 {
                    if *validchars.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                        == '-' as i32
                    {
                        validchars = validchars.offset(1);
                    }
                    if *validchars.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                        != '.' as i32
                    {
                        break;
                    }
                    validchars = validchars.offset(1);
                } else if *p as std::ffi::c_int == ',' as i32 {
                    if *validchars.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                        == '\0' as i32
                        || *validchars.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                            != ',' as i32
                    {
                        break;
                    }
                } else {
                    if !(*p as std::ffi::c_int >= '0' as i32 && *p as std::ffi::c_int <= '9' as i32)
                    {
                        break;
                    }
                    while *validchars.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                        == '-' as i32
                        || *validchars.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                            == '.' as i32
                    {
                        validchars = validchars.offset(1);
                    }
                    if *validchars.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                        != 'd' as i32
                    {
                        break;
                    }
                }
            }
            if *p as std::ffi::c_int == '$' as i32 {
                break;
            }
        }
        let fresh1 = out;
        out = out.offset(1);
        *fresh1 = *p;
        p = p.offset(1);
    }
    *out = '\0' as i32 as std::ffi::c_char;
    return p;
}
unsafe extern "C" fn num_error(
    mut printopt: *const std::ffi::c_char,
    mut errp: *mut lbool,
    mut overflow: lbool,
) -> std::ffi::c_int {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    if !errp.is_null() {
        *errp = LTRUE;
        return -(1 as std::ffi::c_int);
    }
    if !printopt.is_null() {
        parg.p_string = printopt;
        error(
            if overflow as std::ffi::c_uint != 0 {
                b"Number too large in '%s'\0" as *const u8 as *const std::ffi::c_char
            } else {
                b"Number is required after %s\0" as *const u8 as *const std::ffi::c_char
            },
            &mut parg,
        );
    }
    return -(1 as std::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn getnumc(
    mut sp: *mut *const std::ffi::c_char,
    mut printopt: *const std::ffi::c_char,
    mut errp: *mut lbool,
) -> std::ffi::c_int {
    let mut s: *const std::ffi::c_char = *sp;
    let mut n: std::ffi::c_int = 0;
    let mut neg: lbool = LFALSE;
    s = skipspc(s);
    neg = LFALSE;
    if *s as std::ffi::c_int == '-' as i32 {
        neg = LTRUE;
        s = s.offset(1);
    }
    if (*s as std::ffi::c_int) < '0' as i32 || *s as std::ffi::c_int > '9' as i32 {
        return num_error(printopt, errp, LFALSE);
    }
    n = lstrtoic(s, sp, 10 as std::ffi::c_int);
    if n < 0 as std::ffi::c_int {
        return num_error(printopt, errp, LTRUE);
    }
    if !errp.is_null() {
        *errp = LFALSE;
    }
    if neg as u64 != 0 {
        n = -n;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn getnum(
    mut sp: *mut *mut std::ffi::c_char,
    mut printopt: *const std::ffi::c_char,
    mut errp: *mut lbool,
) -> std::ffi::c_int {
    let mut cs: *const std::ffi::c_char = *sp;
    let mut r: std::ffi::c_int = getnumc(&mut cs, printopt, errp);
    *sp = cs as *mut std::ffi::c_char;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn getfraction(
    mut sp: *mut *const std::ffi::c_char,
    mut printopt: *const std::ffi::c_char,
    mut errp: *mut lbool,
) -> std::ffi::c_long {
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut frac: std::ffi::c_long = 0 as std::ffi::c_int as std::ffi::c_long;
    let mut fraclen: std::ffi::c_int = 0 as std::ffi::c_int;
    s = skipspc(*sp);
    if (*s as std::ffi::c_int) < '0' as i32 || *s as std::ffi::c_int > '9' as i32 {
        return num_error(printopt, errp, LFALSE) as std::ffi::c_long;
    }
    while *s as std::ffi::c_int >= '0' as i32 && *s as std::ffi::c_int <= '9' as i32 {
        if !(6 as std::ffi::c_int <= fraclen) {
            frac = frac * 10 as std::ffi::c_int as std::ffi::c_long
                + (*s as std::ffi::c_int - '0' as i32) as std::ffi::c_long;
            fraclen += 1;
        }
        s = s.offset(1);
    }
    loop {
        let fresh2 = fraclen;
        fraclen = fraclen + 1;
        if !(fresh2 < 6 as std::ffi::c_int) {
            break;
        }
        frac *= 10 as std::ffi::c_int as std::ffi::c_long;
    }
    *sp = s;
    if !errp.is_null() {
        *errp = LFALSE;
    }
    return frac;
}
#[no_mangle]
pub unsafe extern "C" fn init_unsupport() {
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut ss = lgetenv("LESS_UNSUPPORT");
    if ss.is_err() {
        return;
    } else {
        s = ss.unwrap();
    }
    loop {
        let mut opt: *mut loption = 0 as *mut loption;
        s = skipspc(s);
        if *s as std::ffi::c_int == '\0' as i32 {
            break;
        }
        if *s as std::ffi::c_int == '-' as i32 && {
            s = s.offset(1);
            *s as std::ffi::c_int == '\0' as i32
        } {
            break;
        }
        if *s as std::ffi::c_int == '-' as i32 {
            s = s.offset(1);
            opt = findopt_name(&mut s, 0 as *mut *const std::ffi::c_char, 0 as *mut lbool);
        } else {
            opt = findopt(*s as std::ffi::c_int);
            if !opt.is_null() {
                s = s.offset(1);
            }
        }
        if !opt.is_null() {
            (*opt).otype |= 0o2000 as std::ffi::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_quit_at_eof() -> std::ffi::c_int {
    if less_is_more == 0 {
        return quit_at_eof;
    }
    return if quit_at_eof != 0 {
        2 as std::ffi::c_int
    } else {
        1 as std::ffi::c_int
    };
}
