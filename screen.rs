use ::libc;
extern "C" {
    fn sprintf(
        _: *mut std::ffi::c_char,
        _: *const std::ffi::c_char,
        _: ...
    ) -> std::ffi::c_int;
    fn snprintf(
        _: *mut std::ffi::c_char,
        _: std::ffi::c_ulong,
        _: *const std::ffi::c_char,
        _: ...
    ) -> std::ffi::c_int;
    fn fsync(__fd: std::ffi::c_int) -> std::ffi::c_int;
    fn strtol(
        _: *const std::ffi::c_char,
        _: *mut *mut std::ffi::c_char,
        _: std::ffi::c_int,
    ) -> std::ffi::c_long;
    fn memcpy(
        _: *mut std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn strcpy(
        _: *mut std::ffi::c_char,
        _: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_char;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strchr(_: *const std::ffi::c_char, _: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn strstr(
        _: *const std::ffi::c_char,
        _: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_char;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn lstrtoic(
        _: *const std::ffi::c_char,
        _: *mut *const std::ffi::c_char,
        _: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn save(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
    fn lgetenv(var: *const std::ffi::c_char) -> *const std::ffi::c_char;
    fn isnullenv(s: *const std::ffi::c_char) -> lbool;
    fn get_color_map(attr: std::ffi::c_int) -> *const std::ffi::c_char;
    fn calc_jump_sline();
    fn calc_shift_count();
    fn calc_match_shift();
    fn sleep_ms(ms: std::ffi::c_int);
    fn flush();
    fn putchr(ch: std::ffi::c_int) -> std::ffi::c_int;
    fn clear_bot_if_needed();
    fn putstr(s: *const std::ffi::c_char);
    fn pos_init();
    fn ioctl(
        __fd: std::ffi::c_int,
        __request: std::ffi::c_ulong,
        _: ...
    ) -> std::ffi::c_int;
    fn cfgetospeed(__termios_p: *const termios) -> speed_t;
    fn tcgetattr(__fd: std::ffi::c_int, __termios_p: *mut termios) -> std::ffi::c_int;
    fn tcsetattr(
        __fd: std::ffi::c_int,
        __optional_actions: std::ffi::c_int,
        __termios_p: *const termios,
    ) -> std::ffi::c_int;
    static mut PC: std::ffi::c_char;
    static mut ospeed: std::ffi::c_short;
    fn tgetstr(
        _: *const std::ffi::c_char,
        _: *mut *mut std::ffi::c_char,
    ) -> *mut std::ffi::c_char;
    fn tgoto(
        _: *const std::ffi::c_char,
        _: std::ffi::c_int,
        _: std::ffi::c_int,
    ) -> *mut std::ffi::c_char;
    fn tgetent(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn tgetflag(_: *const std::ffi::c_char) -> std::ffi::c_int;
    fn tgetnum(_: *const std::ffi::c_char) -> std::ffi::c_int;
    fn tputs(
        _: *const std::ffi::c_char,
        _: std::ffi::c_int,
        _: Option::<unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int>,
    ) -> std::ffi::c_int;
    static mut binattr: std::ffi::c_int;
    static mut one_screen: std::ffi::c_int;
    static mut shell_lines: std::ffi::c_int;
    static mut quiet: std::ffi::c_int;
    static mut no_vbell: std::ffi::c_int;
    static mut no_back_scroll: std::ffi::c_int;
    static mut no_init: std::ffi::c_int;
    static mut no_keypad: std::ffi::c_int;
    static mut top_scroll: std::ffi::c_int;
    static mut quit_if_one_screen: std::ffi::c_int;
    static mut oldbot: std::ffi::c_int;
    static mut mousecap: std::ffi::c_int;
    static mut is_tty: std::ffi::c_int;
    static mut use_color: std::ffi::c_int;
    static mut no_paste: std::ffi::c_int;
    static mut hilite_search: std::ffi::c_int;
    static mut tty: std::ffi::c_int;
}
pub type size_t = std::ffi::c_ulong;
pub type lbool = std::ffi::c_uint;
pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;
pub type COLOR_TYPE = std::ffi::c_uint;
pub const CT_6BIT: COLOR_TYPE = 2;
pub const CT_4BIT: COLOR_TYPE = 1;
pub const CT_NULL: COLOR_TYPE = 0;
pub type C2RustUnnamed = std::ffi::c_int;
pub const CV_ERROR: C2RustUnnamed = -1;
pub const CV_NOCHANGE: C2RustUnnamed = -2;
pub const CV_BRIGHT: C2RustUnnamed = 8;
pub const CV_RED: C2RustUnnamed = 4;
pub const CV_GREEN: C2RustUnnamed = 2;
pub const CV_BLUE: C2RustUnnamed = 1;
pub type CHAR_ATTR = std::ffi::c_uint;
pub const CATTR_BLINK: CHAR_ATTR = 8;
pub const CATTR_UNDERLINE: CHAR_ATTR = 4;
pub const CATTR_BOLD: CHAR_ATTR = 2;
pub const CATTR_STANDOUT: CHAR_ATTR = 1;
pub const CATTR_NULL: CHAR_ATTR = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
pub type speed_t = std::ffi::c_uint;
pub type cc_t = std::ffi::c_uchar;
pub type tcflag_t = std::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct env {
    pub next: *mut env,
    pub name: *mut std::ffi::c_char,
    pub value: *mut std::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: std::ffi::c_ushort,
    pub ws_col: std::ffi::c_ushort,
    pub ws_xpixel: std::ffi::c_ushort,
    pub ws_ypixel: std::ffi::c_ushort,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const std::ffi::c_char) -> std::ffi::c_int {
    return strtol(
        __nptr,
        0 as *mut std::ffi::c_void as *mut *mut std::ffi::c_char,
        10 as std::ffi::c_int,
    ) as std::ffi::c_int;
}
static mut sc_pad: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_home: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_addline: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_lower_left: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_return: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_move: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_clear: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_eol_clear: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_eos_clear: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_s_in: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_s_out: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_u_in: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_u_out: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_b_in: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_b_out: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_bl_in: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_bl_out: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_visual_bell: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_backspace: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_s_keypad: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_e_keypad: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_s_mousecap: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_e_mousecap: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_s_bracketed_paste: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_e_bracketed_paste: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_init: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut sc_deinit: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut attrcolor: std::ffi::c_int = -(1 as std::ffi::c_int);
static mut init_done: std::ffi::c_int = 0 as std::ffi::c_int;
#[no_mangle]
pub static mut auto_wrap: std::ffi::c_int = 0;
#[no_mangle]
pub static mut ignaw: std::ffi::c_int = 0;
#[no_mangle]
pub static mut erase_char: std::ffi::c_int = 0;
#[no_mangle]
pub static mut erase2_char: std::ffi::c_int = 0;
#[no_mangle]
pub static mut kill_char: std::ffi::c_int = 0;
#[no_mangle]
pub static mut werase_char: std::ffi::c_int = 0;
#[no_mangle]
pub static mut sc_width: std::ffi::c_int = 0;
#[no_mangle]
pub static mut sc_height: std::ffi::c_int = 0;
#[no_mangle]
pub static mut bo_s_width: std::ffi::c_int = 0;
#[no_mangle]
pub static mut bo_e_width: std::ffi::c_int = 0;
#[no_mangle]
pub static mut ul_s_width: std::ffi::c_int = 0;
#[no_mangle]
pub static mut ul_e_width: std::ffi::c_int = 0;
#[no_mangle]
pub static mut so_s_width: std::ffi::c_int = 0;
#[no_mangle]
pub static mut so_e_width: std::ffi::c_int = 0;
#[no_mangle]
pub static mut bl_s_width: std::ffi::c_int = 0;
#[no_mangle]
pub static mut bl_e_width: std::ffi::c_int = 0;
#[no_mangle]
pub static mut above_mem: std::ffi::c_int = 0;
#[no_mangle]
pub static mut below_mem: std::ffi::c_int = 0;
#[no_mangle]
pub static mut can_goto_line: std::ffi::c_int = 0;
#[no_mangle]
pub static mut clear_bg: std::ffi::c_int = 0;
#[no_mangle]
pub static mut missing_cap: lbool = LFALSE;
#[no_mangle]
pub static mut kent: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
#[no_mangle]
pub static mut term_init_done: lbool = LFALSE;
#[no_mangle]
pub static mut full_screen: lbool = LTRUE;
static mut attrmode: std::ffi::c_int = 0 as std::ffi::c_int;
static mut termcap_debug: std::ffi::c_int = -(1 as std::ffi::c_int);
static mut no_alt_screen: std::ffi::c_int = 0;
unsafe extern "C" fn set_termio_flags(mut s: *mut termios) {
    (*s).c_lflag
        &= !(0 as std::ffi::c_int | 0o2 as std::ffi::c_int | 0o10 as std::ffi::c_int
            | 0o20 as std::ffi::c_int | 0o40 as std::ffi::c_int
            | 0o100 as std::ffi::c_int) as tcflag_t;
    (*s).c_oflag
        |= (0 as std::ffi::c_int | 0o1 as std::ffi::c_int | 0o4 as std::ffi::c_int)
            as tcflag_t;
    (*s).c_oflag
        &= !(0 as std::ffi::c_int | 0o10 as std::ffi::c_int | 0o20 as std::ffi::c_int
            | 0o40 as std::ffi::c_int) as tcflag_t;
}
#[no_mangle]
pub unsafe extern "C" fn raw_mode(mut on: std::ffi::c_int) {
    static mut curr_on: std::ffi::c_int = 0 as std::ffi::c_int;
    if on == curr_on {
        return;
    }
    erase2_char = '\u{8}' as i32;
    let mut s: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    static mut save_term: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    static mut saved_term: std::ffi::c_int = 0 as std::ffi::c_int;
    if on != 0 {
        if tcgetattr(tty, &mut s) < 0 as std::ffi::c_int {
            erase_char = '\u{8}' as i32;
            kill_char = 'U' as i32 & 0o37 as std::ffi::c_int;
            werase_char = 'W' as i32 & 0o37 as std::ffi::c_int;
        } else {
            if saved_term == 0 {
                save_term = s;
                saved_term = 1 as std::ffi::c_int;
            }
            match cfgetospeed(&mut s) {
                0 => {
                    ospeed = 0 as std::ffi::c_int as std::ffi::c_short;
                }
                1 => {
                    ospeed = 1 as std::ffi::c_int as std::ffi::c_short;
                }
                2 => {
                    ospeed = 2 as std::ffi::c_int as std::ffi::c_short;
                }
                3 => {
                    ospeed = 3 as std::ffi::c_int as std::ffi::c_short;
                }
                4 => {
                    ospeed = 4 as std::ffi::c_int as std::ffi::c_short;
                }
                5 => {
                    ospeed = 5 as std::ffi::c_int as std::ffi::c_short;
                }
                6 => {
                    ospeed = 6 as std::ffi::c_int as std::ffi::c_short;
                }
                7 => {
                    ospeed = 7 as std::ffi::c_int as std::ffi::c_short;
                }
                8 => {
                    ospeed = 8 as std::ffi::c_int as std::ffi::c_short;
                }
                9 => {
                    ospeed = 9 as std::ffi::c_int as std::ffi::c_short;
                }
                10 => {
                    ospeed = 10 as std::ffi::c_int as std::ffi::c_short;
                }
                11 => {
                    ospeed = 11 as std::ffi::c_int as std::ffi::c_short;
                }
                12 => {
                    ospeed = 12 as std::ffi::c_int as std::ffi::c_short;
                }
                13 => {
                    ospeed = 13 as std::ffi::c_int as std::ffi::c_short;
                }
                14 => {
                    ospeed = 14 as std::ffi::c_int as std::ffi::c_short;
                }
                15 => {
                    ospeed = 15 as std::ffi::c_int as std::ffi::c_short;
                }
                4097 => {
                    ospeed = 16 as std::ffi::c_int as std::ffi::c_short;
                }
                4098 => {
                    ospeed = 17 as std::ffi::c_int as std::ffi::c_short;
                }
                _ => {}
            }
            erase_char = s.c_cc[2 as std::ffi::c_int as usize] as std::ffi::c_int;
            kill_char = s.c_cc[3 as std::ffi::c_int as usize] as std::ffi::c_int;
            werase_char = s.c_cc[14 as std::ffi::c_int as usize] as std::ffi::c_int;
            set_termio_flags(&mut s);
            s.c_cc[6 as std::ffi::c_int as usize] = 1 as std::ffi::c_int as cc_t;
            s.c_cc[5 as std::ffi::c_int as usize] = 0 as std::ffi::c_int as cc_t;
            s.c_cc[15 as std::ffi::c_int as usize] = 0 as std::ffi::c_int as cc_t;
            s.c_cc[9 as std::ffi::c_int as usize] = 0 as std::ffi::c_int as cc_t;
            s.c_cc[8 as std::ffi::c_int as usize] = 0 as std::ffi::c_int as cc_t;
            s.c_cc[13 as std::ffi::c_int as usize] = 0 as std::ffi::c_int as cc_t;
        }
    } else {
        s = save_term;
    }
    fsync(tty);
    tcsetattr(tty, 1 as std::ffi::c_int, &mut s);
    curr_on = on;
}
static mut hardcopy: std::ffi::c_int = 0;
unsafe extern "C" fn ltget_env(
    mut capname: *const std::ffi::c_char,
) -> *const std::ffi::c_char {
    let mut name: [std::ffi::c_char; 64] = [0; 64];
    if termcap_debug != 0 {
        static mut envs: *mut env = 0 as *const env as *mut env;
        let mut p: *mut env = 0 as *mut env;
        p = envs;
        while !p.is_null() {
            if strcmp((*p).name, capname) == 0 as std::ffi::c_int {
                return (*p).value;
            }
            p = (*p).next;
        }
        p = ecalloc(
            1 as std::ffi::c_int as size_t,
            ::core::mem::size_of::<env>() as std::ffi::c_ulong,
        ) as *mut env;
        (*p).name = save(capname);
        (*p)
            .value = ecalloc(
            (strlen(capname)).wrapping_add(3 as std::ffi::c_int as std::ffi::c_ulong),
            ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
        ) as *mut std::ffi::c_char;
        sprintf((*p).value, b"<%s>\0" as *const u8 as *const std::ffi::c_char, capname);
        (*p).next = envs;
        envs = p;
        return (*p).value;
    }
    snprintf(
        name.as_mut_ptr(),
        ::core::mem::size_of::<[std::ffi::c_char; 64]>() as std::ffi::c_ulong,
        b"LESS_TERMCAP_%s\0" as *const u8 as *const std::ffi::c_char,
        capname,
    );
    return lgetenv(name.as_mut_ptr());
}
unsafe extern "C" fn ltgetflag(mut capname: *const std::ffi::c_char) -> std::ffi::c_int {
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    s = ltget_env(capname);
    if !s.is_null() {
        return (*s as std::ffi::c_int != '\0' as i32
            && *s as std::ffi::c_int != '0' as i32) as std::ffi::c_int;
    }
    if hardcopy != 0 {
        return 0 as std::ffi::c_int;
    }
    return tgetflag(capname);
}
unsafe extern "C" fn ltgetnum(mut capname: *const std::ffi::c_char) -> std::ffi::c_int {
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    s = ltget_env(capname);
    if !s.is_null() {
        return atoi(s);
    }
    if hardcopy != 0 {
        return -(1 as std::ffi::c_int);
    }
    return tgetnum(capname);
}
unsafe extern "C" fn ltgetstr(
    mut capname: *const std::ffi::c_char,
    mut pp: *mut *mut std::ffi::c_char,
) -> *const std::ffi::c_char {
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    s = ltget_env(capname);
    if !s.is_null() {
        return s;
    }
    if hardcopy != 0 {
        return 0 as *const std::ffi::c_char;
    }
    return tgetstr(capname, pp);
}
unsafe extern "C" fn scrsize() {
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut sys_height: std::ffi::c_int = 0;
    let mut sys_width: std::ffi::c_int = 0;
    let mut n: std::ffi::c_int = 0;
    sys_height = 0 as std::ffi::c_int;
    sys_width = sys_height;
    let mut w: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    if ioctl(
        2 as std::ffi::c_int,
        0x5413 as std::ffi::c_int as std::ffi::c_ulong,
        &mut w as *mut winsize,
    ) == 0 as std::ffi::c_int
    {
        if w.ws_row as std::ffi::c_int > 0 as std::ffi::c_int {
            sys_height = w.ws_row as std::ffi::c_int;
        }
        if w.ws_col as std::ffi::c_int > 0 as std::ffi::c_int {
            sys_width = w.ws_col as std::ffi::c_int;
        }
    }
    if sys_height > 0 as std::ffi::c_int {
        sc_height = sys_height;
    } else {
        s = lgetenv(b"LINES\0" as *const u8 as *const std::ffi::c_char);
        if !s.is_null() {
            sc_height = atoi(s);
        } else {
            n = ltgetnum(b"li\0" as *const u8 as *const std::ffi::c_char);
            if n > 0 as std::ffi::c_int {
                sc_height = n;
            }
        }
    }
    s = lgetenv(b"LESS_LINES\0" as *const u8 as *const std::ffi::c_char);
    if !s.is_null() {
        let mut height: std::ffi::c_int = atoi(s);
        sc_height = if height < 0 as std::ffi::c_int {
            sc_height + height
        } else {
            height
        };
        full_screen = LFALSE;
    }
    if sc_height <= 0 as std::ffi::c_int {
        sc_height = 24 as std::ffi::c_int;
    }
    if sys_width > 0 as std::ffi::c_int {
        sc_width = sys_width;
    } else {
        s = lgetenv(b"COLUMNS\0" as *const u8 as *const std::ffi::c_char);
        if !s.is_null() {
            sc_width = atoi(s);
        } else {
            n = ltgetnum(b"co\0" as *const u8 as *const std::ffi::c_char);
            if n > 0 as std::ffi::c_int {
                sc_width = n;
            }
        }
    }
    s = lgetenv(b"LESS_COLUMNS\0" as *const u8 as *const std::ffi::c_char);
    if !s.is_null() {
        let mut width: std::ffi::c_int = atoi(s);
        sc_width = if width < 0 as std::ffi::c_int { sc_width + width } else { width };
    }
    if sc_width <= 0 as std::ffi::c_int {
        sc_width = 80 as std::ffi::c_int;
    }
    screen_size_changed();
}
#[no_mangle]
pub unsafe extern "C" fn screen_size_changed() {
    calc_jump_sline();
    calc_shift_count();
    calc_match_shift();
}
#[no_mangle]
pub unsafe extern "C" fn special_key_str(
    mut key: std::ffi::c_int,
) -> *const std::ffi::c_char {
    static mut tbuf: [std::ffi::c_char; 40] = [0; 40];
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut sp: *mut std::ffi::c_char = tbuf.as_mut_ptr();
    match key {
        1 => {
            s = ltgetstr(b"kr\0" as *const u8 as *const std::ffi::c_char, &mut sp);
        }
        2 => {
            s = ltgetstr(b"kl\0" as *const u8 as *const std::ffi::c_char, &mut sp);
        }
        3 => {
            s = ltgetstr(b"ku\0" as *const u8 as *const std::ffi::c_char, &mut sp);
        }
        4 => {
            s = ltgetstr(b"kd\0" as *const u8 as *const std::ffi::c_char, &mut sp);
        }
        5 => {
            s = ltgetstr(b"kP\0" as *const u8 as *const std::ffi::c_char, &mut sp);
        }
        6 => {
            s = ltgetstr(b"kN\0" as *const u8 as *const std::ffi::c_char, &mut sp);
        }
        7 => {
            s = ltgetstr(b"kh\0" as *const u8 as *const std::ffi::c_char, &mut sp);
        }
        8 => {
            s = ltgetstr(b"@7\0" as *const u8 as *const std::ffi::c_char, &mut sp);
        }
        9 => {
            s = ltgetstr(b"kD\0" as *const u8 as *const std::ffi::c_char, &mut sp);
            if s.is_null() {
                tbuf[0 as std::ffi::c_int
                    as usize] = '\u{7f}' as i32 as std::ffi::c_char;
                tbuf[1 as std::ffi::c_int as usize] = '\0' as i32 as std::ffi::c_char;
                s = tbuf.as_mut_ptr();
            }
        }
        17 => {
            s = ltgetstr(b"kb\0" as *const u8 as *const std::ffi::c_char, &mut sp);
            if s.is_null() {
                tbuf[0 as std::ffi::c_int as usize] = '\u{8}' as i32 as std::ffi::c_char;
                tbuf[1 as std::ffi::c_int as usize] = '\0' as i32 as std::ffi::c_char;
                s = tbuf.as_mut_ptr();
            }
        }
        40 => {
            tbuf[0 as std::ffi::c_int
                as usize] = ('K' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_char;
            tbuf[1 as std::ffi::c_int as usize] = '\0' as i32 as std::ffi::c_char;
            s = tbuf.as_mut_ptr();
        }
        _ => return 0 as *const std::ffi::c_char,
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn get_term() {
    termcap_debug = (isnullenv(
        lgetenv(b"LESS_TERMCAP_DEBUG\0" as *const u8 as *const std::ffi::c_char),
    ) as u64 == 0) as std::ffi::c_int;
    let mut sp: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut t1: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut t2: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut term: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    static mut termbuf: [std::ffi::c_char; 2048] = [0; 2048];
    static mut sbuf: [std::ffi::c_char; 1024] = [0; 1024];
    term = lgetenv(b"TERM\0" as *const u8 as *const std::ffi::c_char);
    if term.is_null() {
        term = b"unknown\0" as *const u8 as *const std::ffi::c_char;
    }
    hardcopy = 0 as std::ffi::c_int;
    if tgetent(termbuf.as_mut_ptr(), term) != 1 as std::ffi::c_int {
        hardcopy = 1 as std::ffi::c_int;
    }
    if ltgetflag(b"hc\0" as *const u8 as *const std::ffi::c_char) != 0 {
        hardcopy = 1 as std::ffi::c_int;
    }
    scrsize();
    pos_init();
    auto_wrap = ltgetflag(b"am\0" as *const u8 as *const std::ffi::c_char);
    ignaw = ltgetflag(b"xn\0" as *const u8 as *const std::ffi::c_char);
    above_mem = ltgetflag(b"da\0" as *const u8 as *const std::ffi::c_char);
    below_mem = ltgetflag(b"db\0" as *const u8 as *const std::ffi::c_char);
    clear_bg = ltgetflag(b"ut\0" as *const u8 as *const std::ffi::c_char);
    no_alt_screen = ltgetflag(b"NR\0" as *const u8 as *const std::ffi::c_char);
    so_s_width = ltgetnum(b"sg\0" as *const u8 as *const std::ffi::c_char);
    if so_s_width < 0 as std::ffi::c_int {
        so_s_width = 0 as std::ffi::c_int;
    }
    so_e_width = so_s_width;
    bo_e_width = so_s_width;
    bo_s_width = bo_e_width;
    ul_e_width = so_s_width;
    ul_s_width = ul_e_width;
    bl_e_width = so_s_width;
    bl_s_width = bl_e_width;
    if so_s_width > 0 as std::ffi::c_int || so_e_width > 0 as std::ffi::c_int {
        hilite_search = 0 as std::ffi::c_int;
    }
    sp = sbuf.as_mut_ptr();
    sc_pad = ltgetstr(b"pc\0" as *const u8 as *const std::ffi::c_char, &mut sp);
    if !sc_pad.is_null() {
        PC = *sc_pad;
    }
    sc_s_keypad = ltgetstr(b"ks\0" as *const u8 as *const std::ffi::c_char, &mut sp);
    if sc_s_keypad.is_null() {
        sc_s_keypad = b"\0" as *const u8 as *const std::ffi::c_char;
    }
    sc_e_keypad = ltgetstr(b"ke\0" as *const u8 as *const std::ffi::c_char, &mut sp);
    if sc_e_keypad.is_null() {
        sc_e_keypad = b"\0" as *const u8 as *const std::ffi::c_char;
    }
    kent = ltgetstr(b"@8\0" as *const u8 as *const std::ffi::c_char, &mut sp);
    sc_s_mousecap = ltgetstr(
        b"MOUSE_START\0" as *const u8 as *const std::ffi::c_char,
        &mut sp,
    );
    if sc_s_mousecap.is_null() {
        sc_s_mousecap = b"\x1B[?1000h\x1B[?1002h\x1B[?1006h\0" as *const u8
            as *const std::ffi::c_char;
    }
    sc_e_mousecap = ltgetstr(
        b"MOUSE_END\0" as *const u8 as *const std::ffi::c_char,
        &mut sp,
    );
    if sc_e_mousecap.is_null() {
        sc_e_mousecap = b"\x1B[?1006l\x1B[?1002l\x1B[?1000l\0" as *const u8
            as *const std::ffi::c_char;
    }
    sc_s_bracketed_paste = ltgetstr(
        b"BRACKETED_PASTE_START\0" as *const u8 as *const std::ffi::c_char,
        &mut sp,
    );
    if sc_s_bracketed_paste.is_null() {
        sc_s_bracketed_paste = b"\x1B[?2004h\0" as *const u8 as *const std::ffi::c_char;
    }
    sc_e_bracketed_paste = ltgetstr(
        b"BRACKETED_PASTE_END\0" as *const u8 as *const std::ffi::c_char,
        &mut sp,
    );
    if sc_e_bracketed_paste.is_null() {
        sc_e_bracketed_paste = b"\x1B[?2004l\0" as *const u8 as *const std::ffi::c_char;
    }
    sc_init = ltgetstr(b"ti\0" as *const u8 as *const std::ffi::c_char, &mut sp);
    if sc_init.is_null() {
        sc_init = b"\0" as *const u8 as *const std::ffi::c_char;
    }
    sc_deinit = ltgetstr(b"te\0" as *const u8 as *const std::ffi::c_char, &mut sp);
    if sc_deinit.is_null() {
        sc_deinit = b"\0" as *const u8 as *const std::ffi::c_char;
    }
    sc_eol_clear = ltgetstr(b"ce\0" as *const u8 as *const std::ffi::c_char, &mut sp);
    if sc_eol_clear.is_null() || *sc_eol_clear as std::ffi::c_int == '\0' as i32 {
        missing_cap = LTRUE;
        sc_eol_clear = b"\0" as *const u8 as *const std::ffi::c_char;
    }
    sc_eos_clear = ltgetstr(b"cd\0" as *const u8 as *const std::ffi::c_char, &mut sp);
    if below_mem != 0
        && (sc_eos_clear.is_null() || *sc_eos_clear as std::ffi::c_int == '\0' as i32)
    {
        missing_cap = LTRUE;
        sc_eos_clear = b"\0" as *const u8 as *const std::ffi::c_char;
    }
    sc_clear = ltgetstr(b"cl\0" as *const u8 as *const std::ffi::c_char, &mut sp);
    if sc_clear.is_null() || *sc_clear as std::ffi::c_int == '\0' as i32 {
        missing_cap = LTRUE;
        sc_clear = b"\n\n\0" as *const u8 as *const std::ffi::c_char;
    }
    sc_move = ltgetstr(b"cm\0" as *const u8 as *const std::ffi::c_char, &mut sp);
    if sc_move.is_null() || *sc_move as std::ffi::c_int == '\0' as i32 {
        sc_move = b"\0" as *const u8 as *const std::ffi::c_char;
        can_goto_line = 0 as std::ffi::c_int;
    } else {
        can_goto_line = 1 as std::ffi::c_int;
    }
    tmodes(
        b"so\0" as *const u8 as *const std::ffi::c_char,
        b"se\0" as *const u8 as *const std::ffi::c_char,
        &mut sc_s_in,
        &mut sc_s_out,
        b"\0" as *const u8 as *const std::ffi::c_char,
        b"\0" as *const u8 as *const std::ffi::c_char,
        &mut sp,
    );
    tmodes(
        b"us\0" as *const u8 as *const std::ffi::c_char,
        b"ue\0" as *const u8 as *const std::ffi::c_char,
        &mut sc_u_in,
        &mut sc_u_out,
        sc_s_in,
        sc_s_out,
        &mut sp,
    );
    tmodes(
        b"md\0" as *const u8 as *const std::ffi::c_char,
        b"me\0" as *const u8 as *const std::ffi::c_char,
        &mut sc_b_in,
        &mut sc_b_out,
        sc_s_in,
        sc_s_out,
        &mut sp,
    );
    tmodes(
        b"mb\0" as *const u8 as *const std::ffi::c_char,
        b"me\0" as *const u8 as *const std::ffi::c_char,
        &mut sc_bl_in,
        &mut sc_bl_out,
        sc_s_in,
        sc_s_out,
        &mut sp,
    );
    sc_visual_bell = ltgetstr(b"vb\0" as *const u8 as *const std::ffi::c_char, &mut sp);
    if sc_visual_bell.is_null() {
        sc_visual_bell = b"\0" as *const u8 as *const std::ffi::c_char;
    }
    if ltgetflag(b"bs\0" as *const u8 as *const std::ffi::c_char) != 0 {
        sc_backspace = b"\x08\0" as *const u8 as *const std::ffi::c_char;
    } else {
        sc_backspace = ltgetstr(
            b"bc\0" as *const u8 as *const std::ffi::c_char,
            &mut sp,
        );
        if sc_backspace.is_null() || *sc_backspace as std::ffi::c_int == '\0' as i32 {
            sc_backspace = b"\x08\0" as *const u8 as *const std::ffi::c_char;
        }
    }
    t1 = ltgetstr(b"ho\0" as *const u8 as *const std::ffi::c_char, &mut sp);
    if t1.is_null() {
        t1 = b"\0" as *const u8 as *const std::ffi::c_char;
    }
    if *sc_move as std::ffi::c_int == '\0' as i32 {
        t2 = b"\0" as *const u8 as *const std::ffi::c_char;
    } else {
        strcpy(sp, tgoto(sc_move, 0 as std::ffi::c_int, 0 as std::ffi::c_int));
        t2 = sp;
        sp = sp
            .offset(
                (strlen(sp)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong)
                    as isize,
            );
    }
    sc_home = cheaper(t1, t2, b"|\x08^\0" as *const u8 as *const std::ffi::c_char);
    t1 = ltgetstr(b"ll\0" as *const u8 as *const std::ffi::c_char, &mut sp);
    if t1.is_null() || full_screen as u64 == 0 {
        t1 = b"\0" as *const u8 as *const std::ffi::c_char;
    }
    if *sc_move as std::ffi::c_int == '\0' as i32 {
        t2 = b"\0" as *const u8 as *const std::ffi::c_char;
    } else {
        strcpy(
            sp,
            tgoto(sc_move, 0 as std::ffi::c_int, sc_height - 1 as std::ffi::c_int),
        );
        t2 = sp;
        sp = sp
            .offset(
                (strlen(sp)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong)
                    as isize,
            );
    }
    sc_lower_left = cheaper(t1, t2, b"\r\0" as *const u8 as *const std::ffi::c_char);
    sc_return = ltgetstr(b"cr\0" as *const u8 as *const std::ffi::c_char, &mut sp);
    if sc_return.is_null() {
        sc_return = b"\r\0" as *const u8 as *const std::ffi::c_char;
    }
    t1 = ltgetstr(b"al\0" as *const u8 as *const std::ffi::c_char, &mut sp);
    if t1.is_null() {
        t1 = b"\0" as *const u8 as *const std::ffi::c_char;
    }
    t2 = ltgetstr(b"sr\0" as *const u8 as *const std::ffi::c_char, &mut sp);
    if t2.is_null() {
        t2 = b"\0" as *const u8 as *const std::ffi::c_char;
    }
    if *t1 as std::ffi::c_int == '\0' as i32 && *t2 as std::ffi::c_int == '\0' as i32 {
        sc_addline = b"\0" as *const u8 as *const std::ffi::c_char;
    } else if above_mem != 0 {
        sc_addline = t1;
    } else {
        sc_addline = cheaper(t1, t2, b"\0" as *const u8 as *const std::ffi::c_char);
    }
    if *sc_addline as std::ffi::c_int == '\0' as i32 {
        no_back_scroll = 1 as std::ffi::c_int;
    }
    let mut env: *const std::ffi::c_char = lgetenv(
        b"LESS_SHELL_LINES\0" as *const u8 as *const std::ffi::c_char,
    );
    shell_lines = if isnullenv(env) as std::ffi::c_uint != 0 {
        1 as std::ffi::c_int
    } else {
        atoi(env)
    };
    if shell_lines >= sc_height {
        shell_lines = sc_height - 1 as std::ffi::c_int;
    }
}
static mut costcount: std::ffi::c_int = 0;
unsafe extern "C" fn inc_costcount(mut c: std::ffi::c_int) -> std::ffi::c_int {
    costcount += 1;
    costcount;
    return c;
}
unsafe extern "C" fn cost(mut t: *const std::ffi::c_char) -> std::ffi::c_int {
    costcount = 0 as std::ffi::c_int;
    tputs(
        t,
        sc_height,
        Some(inc_costcount as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
    return costcount;
}
unsafe extern "C" fn cheaper(
    mut t1: *const std::ffi::c_char,
    mut t2: *const std::ffi::c_char,
    mut def: *const std::ffi::c_char,
) -> *const std::ffi::c_char {
    if *t1 as std::ffi::c_int == '\0' as i32 && *t2 as std::ffi::c_int == '\0' as i32 {
        missing_cap = LTRUE;
        return def;
    }
    if *t1 as std::ffi::c_int == '\0' as i32 {
        return t2;
    }
    if *t2 as std::ffi::c_int == '\0' as i32 {
        return t1;
    }
    if cost(t1) < cost(t2) {
        return t1;
    }
    return t2;
}
unsafe extern "C" fn tmodes(
    mut incap: *const std::ffi::c_char,
    mut outcap: *const std::ffi::c_char,
    mut instr: *mut *const std::ffi::c_char,
    mut outstr: *mut *const std::ffi::c_char,
    mut def_instr: *const std::ffi::c_char,
    mut def_outstr: *const std::ffi::c_char,
    mut spp: *mut *mut std::ffi::c_char,
) {
    *instr = ltgetstr(incap, spp);
    if (*instr).is_null() {
        *instr = def_instr;
        *outstr = def_outstr;
        return;
    }
    *outstr = ltgetstr(outcap, spp);
    if (*outstr).is_null() {
        *outstr = ltgetstr(b"me\0" as *const u8 as *const std::ffi::c_char, spp);
    }
    if (*outstr).is_null() {
        *outstr = b"\0" as *const u8 as *const std::ffi::c_char;
    }
}
unsafe extern "C" fn do_tputs(
    mut str: *const std::ffi::c_char,
    mut affcnt: std::ffi::c_int,
    mut f_putc: Option::<unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int>,
) {
    tputs(str, affcnt, f_putc);
}
unsafe extern "C" fn ltputs(
    mut str: *const std::ffi::c_char,
    mut affcnt: std::ffi::c_int,
    mut f_putc: Option::<unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int>,
) {
    while !str.is_null() && *str as std::ffi::c_int != '\0' as i32 {
        let mut obrac: *const std::ffi::c_char = strstr(
            str,
            b"$<\0" as *const u8 as *const std::ffi::c_char,
        );
        if !obrac.is_null() {
            let mut str2: [std::ffi::c_char; 64] = [0; 64];
            let mut slen: size_t = obrac.offset_from(str) as std::ffi::c_long as size_t;
            if slen
                < ::core::mem::size_of::<[std::ffi::c_char; 64]>() as std::ffi::c_ulong
            {
                let mut delay: std::ffi::c_int = 0;
                memcpy(
                    str2.as_mut_ptr() as *mut std::ffi::c_void,
                    str as *const std::ffi::c_void,
                    slen,
                );
                str2[slen as usize] = '\0' as i32 as std::ffi::c_char;
                do_tputs(str2.as_mut_ptr(), affcnt, f_putc);
                str = str
                    .offset(slen.wrapping_add(2 as std::ffi::c_int as size_t) as isize);
                delay = lstrtoic(str, &mut str, 10 as std::ffi::c_int);
                if *str as std::ffi::c_int == '*' as i32 {
                    let (fresh0, fresh1) = delay.overflowing_mul(affcnt);
                    *(&mut delay as *mut std::ffi::c_int) = fresh0;
                    if fresh1 {
                        delay = 2147483647 as std::ffi::c_int;
                    }
                }
                flush();
                sleep_ms(delay);
                str = strstr(str, b">\0" as *const u8 as *const std::ffi::c_char);
                if !str.is_null() {
                    str = str.offset(1);
                    str;
                }
                continue;
            }
        }
        do_tputs(str, affcnt, f_putc);
        break;
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_mouse() {
    ltputs(
        sc_s_mousecap,
        sc_height,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn deinit_mouse() {
    ltputs(
        sc_e_mousecap,
        sc_height,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn init() {
    clear_bot_if_needed();
    if !(quit_if_one_screen != 0 && one_screen != 0) {
        if no_init == 0 {
            ltputs(
                sc_init,
                sc_height,
                Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
            );
            if *sc_init as std::ffi::c_int != '\0' as i32
                && *sc_deinit as std::ffi::c_int != '\0' as i32 && no_alt_screen == 0
            {
                lower_left();
            }
            term_init_done = LTRUE;
        }
        if no_keypad == 0 {
            ltputs(
                sc_s_keypad,
                sc_height,
                Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
            );
        }
        if mousecap != 0 {
            init_mouse();
        }
        if no_paste != 0 {
            init_bracketed_paste();
        }
    }
    init_done = 1 as std::ffi::c_int;
    if top_scroll != 0 {
        let mut i: std::ffi::c_int = 0;
        i = 1 as std::ffi::c_int;
        while i < sc_height {
            putchr('\n' as i32);
            i += 1;
            i;
        }
    } else {
        line_left();
    };
}
#[no_mangle]
pub unsafe extern "C" fn deinit() {
    if init_done == 0 {
        return;
    }
    if !(quit_if_one_screen != 0 && one_screen != 0) {
        if mousecap != 0 {
            deinit_mouse();
        }
        if no_paste != 0 {
            deinit_bracketed_paste();
        }
        if no_keypad == 0 {
            ltputs(
                sc_e_keypad,
                sc_height,
                Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
            );
        }
        if no_init == 0 {
            ltputs(
                sc_deinit,
                sc_height,
                Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
            );
        }
    }
    init_done = 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn interactive() -> std::ffi::c_int {
    return (is_tty != 0 && init_done != 0) as std::ffi::c_int;
}
unsafe extern "C" fn assert_interactive() {
    if interactive() != 0 {
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn home() {
    assert_interactive();
    ltputs(
        sc_home,
        1 as std::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn add_line() {
    assert_interactive();
    ltputs(
        sc_addline,
        sc_height,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn lower_left() {
    assert_interactive();
    ltputs(
        sc_lower_left,
        1 as std::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn line_left() {
    assert_interactive();
    ltputs(
        sc_return,
        1 as std::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn check_winch() {}
#[no_mangle]
pub unsafe extern "C" fn goto_line(mut sindex: std::ffi::c_int) {
    assert_interactive();
    ltputs(
        tgoto(sc_move, 0 as std::ffi::c_int, sindex),
        1 as std::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn vbell() {
    if no_vbell != 0 {
        return;
    }
    if *sc_visual_bell as std::ffi::c_int == '\0' as i32 {
        return;
    }
    ltputs(
        sc_visual_bell,
        sc_height,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
}
unsafe extern "C" fn beep() {
    putchr('G' as i32 & 0o37 as std::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn bell() {
    if quiet == 2 as std::ffi::c_int {
        vbell();
    } else {
        beep();
    };
}
#[no_mangle]
pub unsafe extern "C" fn clear() {
    assert_interactive();
    ltputs(
        sc_clear,
        sc_height,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn clear_eol() {
    ltputs(
        sc_eol_clear,
        1 as std::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
}
unsafe extern "C" fn clear_eol_bot() {
    assert_interactive();
    if below_mem != 0 {
        ltputs(
            sc_eos_clear,
            1 as std::ffi::c_int,
            Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
        );
    } else {
        ltputs(
            sc_eol_clear,
            1 as std::ffi::c_int,
            Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn clear_bot() {
    if oldbot != 0 {
        lower_left();
    } else {
        line_left();
    }
    if attrmode == 0 as std::ffi::c_int {
        clear_eol_bot();
    } else {
        let mut saved_attrmode: std::ffi::c_int = attrmode;
        at_exit();
        clear_eol_bot();
        at_enter(saved_attrmode);
    };
}
#[no_mangle]
pub unsafe extern "C" fn init_bracketed_paste() {
    ltputs(
        sc_s_bracketed_paste,
        1 as std::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn deinit_bracketed_paste() {
    ltputs(
        sc_e_bracketed_paste,
        1 as std::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
}
unsafe extern "C" fn parse_color4(mut ch: std::ffi::c_char) -> std::ffi::c_int {
    match ch as std::ffi::c_int {
        107 => return 0 as std::ffi::c_int,
        114 => return CV_RED as std::ffi::c_int,
        103 => return CV_GREEN as std::ffi::c_int,
        121 => return CV_RED as std::ffi::c_int | CV_GREEN as std::ffi::c_int,
        98 => return CV_BLUE as std::ffi::c_int,
        109 => return CV_RED as std::ffi::c_int | CV_BLUE as std::ffi::c_int,
        99 => return CV_GREEN as std::ffi::c_int | CV_BLUE as std::ffi::c_int,
        119 => {
            return CV_RED as std::ffi::c_int | CV_GREEN as std::ffi::c_int
                | CV_BLUE as std::ffi::c_int;
        }
        75 => return 0 as std::ffi::c_int | CV_BRIGHT as std::ffi::c_int,
        82 => return CV_RED as std::ffi::c_int | CV_BRIGHT as std::ffi::c_int,
        71 => return CV_GREEN as std::ffi::c_int | CV_BRIGHT as std::ffi::c_int,
        89 => {
            return CV_RED as std::ffi::c_int | CV_GREEN as std::ffi::c_int
                | CV_BRIGHT as std::ffi::c_int;
        }
        66 => return CV_BLUE as std::ffi::c_int | CV_BRIGHT as std::ffi::c_int,
        77 => {
            return CV_RED as std::ffi::c_int | CV_BLUE as std::ffi::c_int
                | CV_BRIGHT as std::ffi::c_int;
        }
        67 => {
            return CV_GREEN as std::ffi::c_int | CV_BLUE as std::ffi::c_int
                | CV_BRIGHT as std::ffi::c_int;
        }
        87 => {
            return CV_RED as std::ffi::c_int | CV_GREEN as std::ffi::c_int
                | CV_BLUE as std::ffi::c_int | CV_BRIGHT as std::ffi::c_int;
        }
        45 => return CV_NOCHANGE as std::ffi::c_int,
        _ => return CV_ERROR as std::ffi::c_int,
    };
}
unsafe extern "C" fn parse_color6(
    mut ps: *mut *const std::ffi::c_char,
) -> std::ffi::c_int {
    if **ps as std::ffi::c_int == '-' as i32 {
        *ps = (*ps).offset(1);
        *ps;
        return CV_NOCHANGE as std::ffi::c_int;
    } else {
        let mut os: *const std::ffi::c_char = *ps;
        let mut color: std::ffi::c_int = lstrtoic(os, ps, 10 as std::ffi::c_int);
        if color < 0 as std::ffi::c_int || *ps == os {
            return CV_ERROR as std::ffi::c_int;
        }
        return color;
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_color(
    mut str: *const std::ffi::c_char,
    mut p_fg: *mut std::ffi::c_int,
    mut p_bg: *mut std::ffi::c_int,
    mut p_cattr: *mut CHAR_ATTR,
) -> COLOR_TYPE {
    let mut fg: std::ffi::c_int = 0;
    let mut bg: std::ffi::c_int = CV_ERROR as std::ffi::c_int;
    let mut cattr: CHAR_ATTR = CATTR_NULL;
    let mut type_0: COLOR_TYPE = CT_NULL;
    if str.is_null() || *str as std::ffi::c_int == '\0' as i32 {
        return CT_NULL;
    }
    if *str as std::ffi::c_int == '+' as i32 {
        str = str.offset(1);
        str;
    }
    fg = parse_color4(*str);
    if fg != CV_ERROR as std::ffi::c_int {
        if *str.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int == '\0' as i32
            || !(strchr(
                b"*~_&dsul\0" as *const u8 as *const std::ffi::c_char,
                *str.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int,
            ))
                .is_null()
        {
            bg = CV_NOCHANGE as std::ffi::c_int;
            str = str.offset(1);
            str;
        } else {
            bg = parse_color4(*str.offset(1 as std::ffi::c_int as isize));
            if bg != CV_ERROR as std::ffi::c_int {
                str = str.offset(2 as std::ffi::c_int as isize);
            }
        }
    }
    if fg != CV_ERROR as std::ffi::c_int && bg != CV_ERROR as std::ffi::c_int {
        type_0 = CT_4BIT;
    } else {
        fg = if *str as std::ffi::c_int == '.' as i32 {
            CV_NOCHANGE as std::ffi::c_int
        } else {
            parse_color6(&mut str)
        };
        if fg != CV_ERROR as std::ffi::c_int {
            if *str as std::ffi::c_int != '.' as i32 {
                bg = CV_NOCHANGE as std::ffi::c_int;
            } else {
                str = str.offset(1);
                str;
                bg = parse_color6(&mut str);
            }
        }
        if fg != CV_ERROR as std::ffi::c_int && bg != CV_ERROR as std::ffi::c_int {
            type_0 = CT_6BIT;
        }
    }
    if type_0 as std::ffi::c_uint != CT_NULL as std::ffi::c_int as std::ffi::c_uint {
        loop {
            if *str as std::ffi::c_int == '*' as i32
                || *str as std::ffi::c_int == 'd' as i32
            {
                cattr = ::core::mem::transmute::<
                    std::ffi::c_uint,
                    CHAR_ATTR,
                >(
                    cattr as std::ffi::c_uint
                        | CATTR_BOLD as std::ffi::c_int as std::ffi::c_uint,
                );
            } else if *str as std::ffi::c_int == '~' as i32
                || *str as std::ffi::c_int == 's' as i32
            {
                cattr = ::core::mem::transmute::<
                    std::ffi::c_uint,
                    CHAR_ATTR,
                >(
                    cattr as std::ffi::c_uint
                        | CATTR_STANDOUT as std::ffi::c_int as std::ffi::c_uint,
                );
            } else if *str as std::ffi::c_int == '_' as i32
                || *str as std::ffi::c_int == 'u' as i32
            {
                cattr = ::core::mem::transmute::<
                    std::ffi::c_uint,
                    CHAR_ATTR,
                >(
                    cattr as std::ffi::c_uint
                        | CATTR_UNDERLINE as std::ffi::c_int as std::ffi::c_uint,
                );
            } else {
                if !(*str as std::ffi::c_int == '&' as i32
                    || *str as std::ffi::c_int == 'l' as i32)
                {
                    break;
                }
                cattr = ::core::mem::transmute::<
                    std::ffi::c_uint,
                    CHAR_ATTR,
                >(
                    cattr as std::ffi::c_uint
                        | CATTR_BLINK as std::ffi::c_int as std::ffi::c_uint,
                );
            }
            str = str.offset(1);
            str;
        }
        if !p_fg.is_null() {
            *p_fg = fg;
        }
        if !p_bg.is_null() {
            *p_bg = bg;
        }
        if !p_cattr.is_null() {
            *p_cattr = cattr;
        }
    }
    return type_0;
}
unsafe extern "C" fn sgr_color(mut color: std::ffi::c_int) -> std::ffi::c_int {
    match color {
        0 => return 30 as std::ffi::c_int,
        4 => return 31 as std::ffi::c_int,
        2 => return 32 as std::ffi::c_int,
        6 => return 33 as std::ffi::c_int,
        1 => return 34 as std::ffi::c_int,
        5 => return 35 as std::ffi::c_int,
        3 => return 36 as std::ffi::c_int,
        7 => return 37 as std::ffi::c_int,
        8 => return 90 as std::ffi::c_int,
        12 => return 91 as std::ffi::c_int,
        10 => return 92 as std::ffi::c_int,
        14 => return 93 as std::ffi::c_int,
        9 => return 94 as std::ffi::c_int,
        13 => return 95 as std::ffi::c_int,
        11 => return 96 as std::ffi::c_int,
        15 => return 97 as std::ffi::c_int,
        _ => return color,
    };
}
unsafe extern "C" fn tput_fmt(
    mut fmt: *const std::ffi::c_char,
    mut color: std::ffi::c_int,
    mut f_putc: Option::<unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int>,
) {
    let mut buf: [std::ffi::c_char; 27] = [0; 27];
    if color == attrcolor {
        return;
    }
    snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[std::ffi::c_char; 27]>() as std::ffi::c_ulong,
        fmt,
        color,
    );
    ltputs(buf.as_mut_ptr(), 1 as std::ffi::c_int, f_putc);
    attrcolor = color;
}
unsafe extern "C" fn tput_char_cattr(
    mut cattr: CHAR_ATTR,
    mut f_putc: Option::<unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int>,
) {
    if cattr as std::ffi::c_uint & CATTR_UNDERLINE as std::ffi::c_int as std::ffi::c_uint
        != 0
    {
        ltputs(sc_u_in, 1 as std::ffi::c_int, f_putc);
    }
    if cattr as std::ffi::c_uint & CATTR_BOLD as std::ffi::c_int as std::ffi::c_uint != 0
    {
        ltputs(sc_b_in, 1 as std::ffi::c_int, f_putc);
    }
    if cattr as std::ffi::c_uint & CATTR_BLINK as std::ffi::c_int as std::ffi::c_uint
        != 0
    {
        ltputs(sc_bl_in, 1 as std::ffi::c_int, f_putc);
    }
    if cattr as std::ffi::c_uint & CATTR_STANDOUT as std::ffi::c_int as std::ffi::c_uint
        != 0
    {
        ltputs(sc_s_in, 1 as std::ffi::c_int, f_putc);
    }
}
unsafe extern "C" fn tput_color(
    mut str: *const std::ffi::c_char,
    mut f_putc: Option::<unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int>,
) {
    let mut fg: std::ffi::c_int = 0;
    let mut bg: std::ffi::c_int = 0;
    let mut cattr: CHAR_ATTR = CATTR_NULL;
    if !str.is_null()
        && strcmp(str, b"*\0" as *const u8 as *const std::ffi::c_char)
            == 0 as std::ffi::c_int
    {
        tput_fmt(
            b"\x1B[m\0" as *const u8 as *const std::ffi::c_char,
            -(1 as std::ffi::c_int),
            f_putc,
        );
        return;
    }
    match parse_color(str, &mut fg, &mut bg, &mut cattr) as std::ffi::c_uint {
        1 => {
            if fg >= 0 as std::ffi::c_int {
                tput_fmt(
                    b"\x1B[%dm\0" as *const u8 as *const std::ffi::c_char,
                    sgr_color(fg),
                    f_putc,
                );
            }
            if bg >= 0 as std::ffi::c_int {
                tput_fmt(
                    b"\x1B[%dm\0" as *const u8 as *const std::ffi::c_char,
                    sgr_color(bg) + 10 as std::ffi::c_int,
                    f_putc,
                );
            }
            tput_char_cattr(cattr, f_putc);
        }
        2 => {
            if fg >= 0 as std::ffi::c_int {
                tput_fmt(
                    b"\x1B[38;5;%dm\0" as *const u8 as *const std::ffi::c_char,
                    fg,
                    f_putc,
                );
            }
            if bg >= 0 as std::ffi::c_int {
                tput_fmt(
                    b"\x1B[48;5;%dm\0" as *const u8 as *const std::ffi::c_char,
                    bg,
                    f_putc,
                );
            }
            tput_char_cattr(cattr, f_putc);
        }
        _ => {}
    };
}
unsafe extern "C" fn tput_inmode(
    mut mode_str: *const std::ffi::c_char,
    mut attr: std::ffi::c_int,
    mut attr_bit: std::ffi::c_int,
    mut f_putc: Option::<unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int>,
) {
    let mut color_str: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    if attr & attr_bit == 0 as std::ffi::c_int {
        return;
    }
    color_str = get_color_map(attr_bit);
    if color_str.is_null() || *color_str as std::ffi::c_int == '\0' as i32
        || *color_str as std::ffi::c_int == '+' as i32
    {
        ltputs(mode_str, 1 as std::ffi::c_int, f_putc);
        if color_str.is_null()
            || {
                let fresh2 = color_str;
                color_str = color_str.offset(1);
                *fresh2 as std::ffi::c_int != '+' as i32
            }
        {
            return;
        }
    }
    tput_color(color_str, f_putc);
}
unsafe extern "C" fn tput_outmode(
    mut mode_str: *const std::ffi::c_char,
    mut attr_bit: std::ffi::c_int,
    mut f_putc: Option::<unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int>,
) {
    if attrmode & attr_bit == 0 as std::ffi::c_int {
        return;
    }
    ltputs(mode_str, 1 as std::ffi::c_int, f_putc);
}
#[no_mangle]
pub unsafe extern "C" fn at_enter(mut attr: std::ffi::c_int) {
    attr = apply_at_specials(attr);
    tput_inmode(
        sc_u_in,
        attr,
        (1 as std::ffi::c_int) << 0 as std::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
    tput_inmode(
        sc_b_in,
        attr,
        (1 as std::ffi::c_int) << 1 as std::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
    tput_inmode(
        sc_bl_in,
        attr,
        (1 as std::ffi::c_int) << 2 as std::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
    if use_color != 0
        && attr & (16 as std::ffi::c_int - 1 as std::ffi::c_int) << 8 as std::ffi::c_int
            != 0
    {
        tput_color(
            get_color_map(attr),
            Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
        );
    } else {
        tput_inmode(
            sc_s_in,
            attr,
            (1 as std::ffi::c_int) << 3 as std::ffi::c_int,
            Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
        );
    }
    attrmode = attr;
}
#[no_mangle]
pub unsafe extern "C" fn at_exit() {
    tput_color(
        b"*\0" as *const u8 as *const std::ffi::c_char,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
    tput_outmode(
        sc_s_out,
        (1 as std::ffi::c_int) << 3 as std::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
    tput_outmode(
        sc_bl_out,
        (1 as std::ffi::c_int) << 2 as std::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
    tput_outmode(
        sc_b_out,
        (1 as std::ffi::c_int) << 1 as std::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
    tput_outmode(
        sc_u_out,
        (1 as std::ffi::c_int) << 0 as std::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
    );
    attrmode = 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn at_switch(mut attr: std::ffi::c_int) {
    let mut new_attrmode: std::ffi::c_int = apply_at_specials(attr);
    let mut ignore_modes: std::ffi::c_int = (1 as std::ffi::c_int)
        << 4 as std::ffi::c_int;
    if new_attrmode & !ignore_modes != attrmode & !ignore_modes {
        at_exit();
        at_enter(attr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn is_at_equiv(
    mut attr1: std::ffi::c_int,
    mut attr2: std::ffi::c_int,
) -> lbool {
    attr1 = apply_at_specials(attr1);
    attr2 = apply_at_specials(attr2);
    return (attr1 == attr2) as std::ffi::c_int as lbool;
}
#[no_mangle]
pub unsafe extern "C" fn apply_at_specials(
    mut attr: std::ffi::c_int,
) -> std::ffi::c_int {
    if attr & (1 as std::ffi::c_int) << 5 as std::ffi::c_int != 0 {
        attr |= binattr;
    }
    if attr & (1 as std::ffi::c_int) << 6 as std::ffi::c_int != 0 {
        attr |= (1 as std::ffi::c_int) << 3 as std::ffi::c_int;
    }
    attr
        &= !((1 as std::ffi::c_int) << 5 as std::ffi::c_int
            | (1 as std::ffi::c_int) << 6 as std::ffi::c_int);
    return attr;
}
#[no_mangle]
pub unsafe extern "C" fn putbs() {
    if termcap_debug != 0 {
        putstr(b"<bs>\0" as *const u8 as *const std::ffi::c_char);
    } else {
        ltputs(
            sc_backspace,
            1 as std::ffi::c_int,
            Some(putchr as unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int),
        );
    };
}
