use crate::decode::lgetenv;
use crate::xbuf::XBuffer;
use std::sync::LazyLock;
extern "C" {
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn memcpy(
        _: *mut std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn strcpy(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strchr(_: *const std::ffi::c_char, _: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn linenumtoa(_: LINENUM, _: *mut std::ffi::c_char, _: std::ffi::c_int);
    fn lstrtoulc(
        _: *const std::ffi::c_char,
        _: *mut *const std::ffi::c_char,
        _: std::ffi::c_int,
    ) -> std::ffi::c_ulong;
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
    fn skipspc(s: *const std::ffi::c_char) -> *const std::ffi::c_char;
    fn parse_color(
        str: *const std::ffi::c_char,
        p_fg: *mut std::ffi::c_int,
        p_bg: *mut std::ffi::c_int,
        p_cattr: *mut CHAR_ATTR,
    ) -> COLOR_TYPE;
    fn is_at_equiv(attr1: std::ffi::c_int, attr2: std::ffi::c_int) -> lbool;
    fn apply_at_specials(attr: std::ffi::c_int) -> std::ffi::c_int;
    fn ch_seek(pos: POSITION) -> std::ffi::c_int;
    fn ch_tell() -> POSITION;
    fn ch_forw_get() -> std::ffi::c_int;
    fn ch_back_get() -> std::ffi::c_int;
    fn control_char(c: LWCHAR) -> lbool;
    fn prchar(c: LWCHAR) -> *const std::ffi::c_char;
    fn prutfchar(ch: LWCHAR) -> *const std::ffi::c_char;
    fn utf_len(ch: std::ffi::c_char) -> std::ffi::c_int;
    fn is_utf8_well_formed(ss: *const std::ffi::c_char, slen: std::ffi::c_int) -> lbool;
    fn get_wchar(sp: *const std::ffi::c_char) -> LWCHAR;
    fn put_wchar(pp: *mut *mut std::ffi::c_char, ch: LWCHAR);
    fn step_charc(
        pp: *mut *const std::ffi::c_char,
        dir: std::ffi::c_int,
        limit: *const std::ffi::c_char,
    ) -> LWCHAR;
    fn step_char(
        pp: *mut *mut std::ffi::c_char,
        dir: std::ffi::c_int,
        limit: *const std::ffi::c_char,
    ) -> LWCHAR;
    fn is_composing_char(ch: LWCHAR) -> lbool;
    fn is_ubin_char(ch: LWCHAR) -> lbool;
    fn is_wide_char(ch: LWCHAR) -> lbool;
    fn is_combining_char(ch1: LWCHAR, ch2: LWCHAR) -> lbool;
    fn isnullenv(s: *const std::ffi::c_char) -> lbool;
    fn forw_line(curr_pos: POSITION, p_linepos: *mut POSITION, p_newline: *mut lbool) -> POSITION;
    fn find_linenum(pos: POSITION) -> LINENUM;
    fn vlinenum(linenum: LINENUM) -> LINENUM;
    fn posmark(pos: POSITION) -> std::ffi::c_char;
    fn position(sindex: std::ffi::c_int) -> POSITION;
    fn is_hilited_attr(
        pos: POSITION,
        epos: POSITION,
        nohide: std::ffi::c_int,
        p_matches: *mut std::ffi::c_int,
    ) -> std::ffi::c_int;
    static mut sigs: std::ffi::c_int;
    static mut bs_mode: std::ffi::c_int;
    static mut proc_backspace: std::ffi::c_int;
    static mut proc_tab: std::ffi::c_int;
    static mut proc_return: std::ffi::c_int;
    static mut linenums: std::ffi::c_int;
    static mut ctldisp: std::ffi::c_int;
    static mut twiddle: std::ffi::c_int;
    static mut status_col: std::ffi::c_int;
    static mut status_col_width: std::ffi::c_int;
    static mut linenum_width: std::ffi::c_int;
    static mut auto_wrap: std::ffi::c_int;
    static mut ignaw: std::ffi::c_int;
    static mut bo_s_width: std::ffi::c_int;
    static mut bo_e_width: std::ffi::c_int;
    static mut ul_s_width: std::ffi::c_int;
    static mut ul_e_width: std::ffi::c_int;
    static mut bl_s_width: std::ffi::c_int;
    static mut bl_e_width: std::ffi::c_int;
    static mut so_s_width: std::ffi::c_int;
    static mut so_e_width: std::ffi::c_int;
    static mut sc_width: std::ffi::c_int;
    static mut sc_height: std::ffi::c_int;
    static mut utf_mode: std::ffi::c_int;
    static mut start_attnpos: POSITION;
    static mut end_attnpos: POSITION;
    static mut rscroll_char: LWCHAR;
    static mut rscroll_attr: std::ffi::c_int;
    static mut use_color: std::ffi::c_int;
    static mut status_line: std::ffi::c_int;
}
pub type __off_t = std::ffi::c_long;
pub type off_t = __off_t;
pub type size_t = std::ffi::c_ulong;
pub type lbool = std::ffi::c_uint;
pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;
pub type LWCHAR = std::ffi::c_ulong;
pub type less_off_t = off_t;
pub type POSITION = less_off_t;
pub type LINENUM = off_t;
pub type osc8_state = std::ffi::c_uint;
pub const OSC8_NOT: osc8_state = 8;
pub const OSC8_URI: osc8_state = 7;
pub const OSC8_PARAMS: osc8_state = 6;
pub const OSC_END: osc8_state = 5;
pub const OSC_END_CSI: osc8_state = 4;
pub const OSC_STRING: osc8_state = 3;
pub const OSC_TYPENUM: osc8_state = 2;
pub const OSC_INTRO: osc8_state = 1;
pub const OSC_START: osc8_state = 0;
pub type COLOR_TYPE = std::ffi::c_uint;
pub const CT_6BIT: COLOR_TYPE = 2;
pub const CT_4BIT: COLOR_TYPE = 1;
pub const CT_NULL: COLOR_TYPE = 0;
pub type CHAR_ATTR = std::ffi::c_uint;
pub const CATTR_BLINK: CHAR_ATTR = 8;
pub const CATTR_UNDERLINE: CHAR_ATTR = 4;
pub const CATTR_BOLD: CHAR_ATTR = 2;
pub const CATTR_STANDOUT: CHAR_ATTR = 1;
pub const CATTR_NULL: CHAR_ATTR = 0;
pub type ansi_state = std::ffi::c_uint;
pub const ANSI_END: ansi_state = 3;
pub const ANSI_ERR: ansi_state = 2;
pub const ANSI_MID: ansi_state = 1;
pub const ANSI_NULL: ansi_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ansi_state_0 {
    pub ostate: osc8_state,
    pub otype: std::ffi::c_uint,
    pub escs_in_seq: std::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub buf: *mut std::ffi::c_char,
    pub attr: *mut std::ffi::c_int,
    pub print: size_t,
    pub end: size_t,
    pub prev_end: size_t,
    pub pfx: [std::ffi::c_char; 21],
    pub pfx_attr: [std::ffi::c_int; 21],
    pub pfx_end: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct col_pos {
    pub col: std::ffi::c_int,
    pub pos: POSITION,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color_map {
    pub attr: std::ffi::c_int,
    pub color: [std::ffi::c_char; 12],
}
static mut linebuf: C2RustUnnamed = C2RustUnnamed {
    buf: 0 as *const std::ffi::c_char as *mut std::ffi::c_char,
    attr: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
    print: 0,
    end: 0,
    prev_end: 0,
    pfx: [0; 21],
    pfx_attr: [0; 21],
    pfx_end: 0,
};
const NUM_LAST_ANSIS: usize = 3;
/*
 * Buffer of ansi sequences which have been shifted off the left edge
 * of the screen.
 */
static mut shifted_ansi: LazyLock<XBuffer> = LazyLock::new(|| XBuffer::new(16));
/*
 * Ring buffer of last ansi sequences sent.
 * While sending a line, these will be resent at the end
 * of any highlighted string, to restore text modes.
 * {{ Not ideal, since we don't really know how many to resend. }}
 */
static mut last_ansi: LazyLock<XBuffer> = LazyLock::new(|| XBuffer::new(16));
static mut last_ansis: LazyLock<[XBuffer; NUM_LAST_ANSIS]> =
    LazyLock::new(|| core::array::from_fn(|_| XBuffer::new(16)));

static mut curr_last_ansi: std::ffi::c_int = 0;
static mut size_linebuf: size_t = 0 as std::ffi::c_int as size_t;
static mut line_ansi: *mut ansi_state_0 = 0 as *const ansi_state_0 as *mut ansi_state_0;
static mut ansi_in_line: lbool = LFALSE;
static mut ff_starts_line: std::ffi::c_int = 0;
static mut hlink_in_line: lbool = LFALSE;
static mut line_mark_attr: std::ffi::c_int = 0;
static mut cshift: std::ffi::c_int = 0;
#[no_mangle]
pub static mut hshift: std::ffi::c_int = 0;
#[no_mangle]
pub static mut tabstops: [std::ffi::c_int; 128] = [
    0 as std::ffi::c_int,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
#[no_mangle]
pub static mut ntabstops: std::ffi::c_int = 1 as std::ffi::c_int;
#[no_mangle]
pub static mut tabdefault: std::ffi::c_int = 8 as std::ffi::c_int;
#[no_mangle]
pub static mut highest_hilite: POSITION = 0;
static mut line_pos: POSITION = 0;
static mut line_contig_pos: POSITION = -(1 as std::ffi::c_int) as POSITION;
static mut end_column: std::ffi::c_int = 0;
static mut right_curr: std::ffi::c_int = 0;
static mut right_column: std::ffi::c_int = 0;
static mut overstrike: std::ffi::c_int = 0;
static mut last_overstrike: std::ffi::c_int = 0 as std::ffi::c_int;
static mut is_null_line: lbool = LFALSE;
static mut pendc: LWCHAR = 0;
static mut pendpos: POSITION = 0;
static mut end_ansi_chars: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut mid_ansi_chars: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut osc_ansi_chars: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut osc_ansi_allow_count: std::ffi::c_int = 0;
static mut osc_ansi_allow: *mut std::ffi::c_long =
    0 as *const std::ffi::c_long as *mut std::ffi::c_long;
static mut in_hilite: lbool = LFALSE;
static mut clear_after_line: lbool = LFALSE;
static mut mbc_buf: [std::ffi::c_char; 6] = [0; 6];
static mut mbc_buf_len: std::ffi::c_int = 0 as std::ffi::c_int;
static mut mbc_buf_index: std::ffi::c_int = 0 as std::ffi::c_int;
static mut mbc_pos: POSITION = 0;
static mut saved_line_end: size_t = 0;
static mut saved_end_column: std::ffi::c_int = 0;
static mut color_map: [color_map; 19] = unsafe {
    [
        {
            let mut init = color_map {
                attr: (1 as std::ffi::c_int) << 0 as std::ffi::c_int,
                color: *::core::mem::transmute::<&[u8; 12], &mut [std::ffi::c_char; 12]>(
                    b"\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (1 as std::ffi::c_int) << 1 as std::ffi::c_int,
                color: *::core::mem::transmute::<&[u8; 12], &mut [std::ffi::c_char; 12]>(
                    b"\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (1 as std::ffi::c_int) << 2 as std::ffi::c_int,
                color: *::core::mem::transmute::<&[u8; 12], &mut [std::ffi::c_char; 12]>(
                    b"\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (1 as std::ffi::c_int) << 3 as std::ffi::c_int,
                color: *::core::mem::transmute::<&[u8; 12], &mut [std::ffi::c_char; 12]>(
                    b"\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (1 as std::ffi::c_int) << 8 as std::ffi::c_int,
                color: *::core::mem::transmute::<&[u8; 12], &mut [std::ffi::c_char; 12]>(
                    b"Wm\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (2 as std::ffi::c_int) << 8 as std::ffi::c_int,
                color: *::core::mem::transmute::<&[u8; 12], &mut [std::ffi::c_char; 12]>(
                    b"kR\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (3 as std::ffi::c_int) << 8 as std::ffi::c_int,
                color: *::core::mem::transmute::<&[u8; 12], &mut [std::ffi::c_char; 12]>(
                    b"kR\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (4 as std::ffi::c_int) << 8 as std::ffi::c_int,
                color: *::core::mem::transmute::<&[u8; 12], &mut [std::ffi::c_char; 12]>(
                    b"kY\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (5 as std::ffi::c_int) << 8 as std::ffi::c_int,
                color: *::core::mem::transmute::<&[u8; 12], &mut [std::ffi::c_char; 12]>(
                    b"c\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (6 as std::ffi::c_int) << 8 as std::ffi::c_int,
                color: *::core::mem::transmute::<&[u8; 12], &mut [std::ffi::c_char; 12]>(
                    b"Wb\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (7 as std::ffi::c_int) << 8 as std::ffi::c_int,
                color: *::core::mem::transmute::<&[u8; 12], &mut [std::ffi::c_char; 12]>(
                    b"kC\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (8 as std::ffi::c_int) << 8 as std::ffi::c_int,
                color: *::core::mem::transmute::<&[u8; 12], &mut [std::ffi::c_char; 12]>(
                    b"kc\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (9 as std::ffi::c_int) << 8 as std::ffi::c_int,
                color: *::core::mem::transmute::<&[u8; 12], &mut [std::ffi::c_char; 12]>(
                    b"\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (10 as std::ffi::c_int) << 8 as std::ffi::c_int,
                color: *::core::mem::transmute::<&[u8; 12], &mut [std::ffi::c_char; 12]>(
                    b"kG\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (10 as std::ffi::c_int + 1 as std::ffi::c_int) << 8 as std::ffi::c_int,
                color: *::core::mem::transmute::<&[u8; 12], &mut [std::ffi::c_char; 12]>(
                    b"ky\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (10 as std::ffi::c_int + 2 as std::ffi::c_int) << 8 as std::ffi::c_int,
                color: *::core::mem::transmute::<&[u8; 12], &mut [std::ffi::c_char; 12]>(
                    b"wb\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (10 as std::ffi::c_int + 3 as std::ffi::c_int) << 8 as std::ffi::c_int,
                color: *::core::mem::transmute::<&[u8; 12], &mut [std::ffi::c_char; 12]>(
                    b"YM\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (10 as std::ffi::c_int + 4 as std::ffi::c_int) << 8 as std::ffi::c_int,
                color: *::core::mem::transmute::<&[u8; 12], &mut [std::ffi::c_char; 12]>(
                    b"Yr\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (10 as std::ffi::c_int + 5 as std::ffi::c_int) << 8 as std::ffi::c_int,
                color: *::core::mem::transmute::<&[u8; 12], &mut [std::ffi::c_char; 12]>(
                    b"Wc\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn init_line() {
    let mut ax: std::ffi::c_int = 0;
    let end_ansi_chars_ = lgetenv("LESSANSIENDCHARS");
    if end_ansi_chars_.is_err() {
        end_ansi_chars = b"m\0" as *const u8 as *const std::ffi::c_char;
    } else {
        end_ansi_chars = end_ansi_chars_.unwrap();
    }
    let mid_ansi_chars_ = lgetenv("LESSANSIMIDCHARS");
    if mid_ansi_chars_.is_err() {
        mid_ansi_chars = b"0123456789:;[?!\"'#%()*+ \0" as *const u8 as *const std::ffi::c_char;
    } else {
        mid_ansi_chars = mid_ansi_chars_.unwrap();
    }
    let osc_ansi_chars_ = lgetenv("LESSANSIOSCCHARS");
    if osc_ansi_chars_.is_err() {
        osc_ansi_chars = b"\0" as *const u8 as *const std::ffi::c_char;
    } else {
        osc_ansi_chars = osc_ansi_chars_.unwrap();
    }
    osc_ansi_allow_count = 0 as std::ffi::c_int;
    if let Ok(mut s) = lgetenv("LESSANSIOSCALLOW") {
        let mut xbuf = XBuffer::new(16);
        loop {
            let mut num: std::ffi::c_long = 0;
            s = skipspc(s);
            if *s as std::ffi::c_int == '\0' as i32 {
                break;
            }
            num = lstrtoulc(s, &mut s, 10 as std::ffi::c_int) as std::ffi::c_long;
            s = skipspc(s);
            if *s as std::ffi::c_int == ',' as i32 {
                s = s.offset(1);
            }
            xbuf.add_data(
                std::slice::from_raw_parts(
                    &mut num as *mut std::ffi::c_long as *const std::ffi::c_void
                        as *const std::ffi::c_uchar,
                    ::core::mem::size_of::<std::ffi::c_long>() as usize,
                ),
                ::core::mem::size_of::<std::ffi::c_long>() as usize,
            );
            osc_ansi_allow_count += 1;
        }
        osc_ansi_allow = xbuf.data.as_ptr() as *mut std::ffi::c_long;
    }
    linebuf.buf = ecalloc(
        1024 as std::ffi::c_int as size_t,
        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_char;
    linebuf.attr = ecalloc(
        1024 as std::ffi::c_int as size_t,
        ::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_int;
    size_linebuf = 1024 as std::ffi::c_int as size_t;
    ax = 0 as std::ffi::c_int;
    curr_last_ansi = 0 as std::ffi::c_int;
}
unsafe extern "C" fn expand_linebuf() -> std::ffi::c_int {
    let mut new_size: size_t = size_linebuf * 2 as std::ffi::c_int as size_t;
    let mut new_buf: *mut std::ffi::c_char = calloc(
        new_size,
        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_char;
    let mut new_attr: *mut std::ffi::c_int = calloc(
        new_size,
        ::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_int;
    if new_buf.is_null() || new_attr.is_null() {
        if !new_attr.is_null() {
            free(new_attr as *mut std::ffi::c_void);
        }
        if !new_buf.is_null() {
            free(new_buf as *mut std::ffi::c_void);
        }
        return 1 as std::ffi::c_int;
    }
    memcpy(
        new_buf as *mut std::ffi::c_void,
        linebuf.buf as *const std::ffi::c_void,
        size_linebuf.wrapping_mul(::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong),
    );
    memcpy(
        new_attr as *mut std::ffi::c_void,
        linebuf.attr as *const std::ffi::c_void,
        size_linebuf.wrapping_mul(::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong),
    );
    free(linebuf.attr as *mut std::ffi::c_void);
    free(linebuf.buf as *mut std::ffi::c_void);
    linebuf.buf = new_buf;
    linebuf.attr = new_attr;
    size_linebuf = new_size;
    return 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn is_ascii_char(mut ch: LWCHAR) -> lbool {
    return (ch <= 0x7f as std::ffi::c_int as LWCHAR) as std::ffi::c_int as lbool;
}
unsafe extern "C" fn inc_end_column(mut w: std::ffi::c_int) {
    if end_column > right_column && w > 0 as std::ffi::c_int {
        right_column = end_column;
        right_curr = linebuf.end as std::ffi::c_int;
    }
    end_column += w;
}
#[no_mangle]
pub unsafe extern "C" fn line_position() -> POSITION {
    return line_pos;
}
#[no_mangle]
pub unsafe extern "C" fn is_line_contig_pos(mut pos: POSITION) -> lbool {
    return (pos == line_contig_pos) as std::ffi::c_int as lbool;
}
#[no_mangle]
pub unsafe extern "C" fn set_line_contig_pos(mut pos: POSITION) {
    line_contig_pos = pos;
}
unsafe extern "C" fn pshift(mut end: size_t) {
    let mut i: size_t = 0;
    i = linebuf.print;
    while i < end {
        if *(linebuf.attr).offset(i as isize) == (1 as std::ffi::c_int) << 4 as std::ffi::c_int {
            shifted_ansi.xbuf_add_char(*(linebuf.buf).offset(i as isize));
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn prewind(mut contig: lbool) {
    let mut ax: std::ffi::c_int = 0;
    shifted_ansi.reset();
    if contig as std::ffi::c_uint != 0 && linebuf.prev_end != 0 as std::ffi::c_int as size_t {
        pshift(linebuf.prev_end);
    }
    linebuf.print = 6 as std::ffi::c_int as size_t;
    linebuf.pfx_end = 0 as std::ffi::c_int as size_t;
    linebuf.end = 0 as std::ffi::c_int as size_t;
    while linebuf.end < linebuf.print {
        *(linebuf.buf).offset(linebuf.end as isize) = '\0' as i32 as std::ffi::c_char;
        *(linebuf.attr).offset(linebuf.end as isize) = 0 as std::ffi::c_int;
        linebuf.end = (linebuf.end).wrapping_add(1);
        linebuf.end;
    }
    end_column = 0 as std::ffi::c_int;
    right_curr = 0 as std::ffi::c_int;
    right_column = 0 as std::ffi::c_int;
    cshift = 0 as std::ffi::c_int;
    overstrike = 0 as std::ffi::c_int;
    last_overstrike = 0 as std::ffi::c_int;
    mbc_buf_len = 0 as std::ffi::c_int;
    is_null_line = LFALSE;
    pendc = '\0' as i32 as LWCHAR;
    in_hilite = LFALSE;
    ansi_in_line = LFALSE;
    ff_starts_line = -(1 as std::ffi::c_int);
    hlink_in_line = LFALSE;
    clear_after_line = LFALSE;
    line_mark_attr = 0 as std::ffi::c_int;
    line_pos = -(1 as std::ffi::c_int) as POSITION;
    last_ansi.reset();
    ax = 0 as std::ffi::c_int;
    while ax < 3 as std::ffi::c_int {
        last_ansis[ax as usize].reset();
        ax += 1;
    }
    curr_last_ansi = 0 as std::ffi::c_int;
}
unsafe extern "C" fn set_linebuf(
    mut n: size_t,
    mut ch: std::ffi::c_char,
    mut attr: std::ffi::c_int,
) {
    if n >= size_linebuf {
        if expand_linebuf() != 0 {
            return;
        }
    }
    *(linebuf.buf).offset(n as isize) = ch;
    *(linebuf.attr).offset(n as isize) = attr;
}
unsafe extern "C" fn add_linebuf(
    mut ch: std::ffi::c_char,
    mut attr: std::ffi::c_int,
    mut w: std::ffi::c_int,
) {
    let fresh0 = linebuf.end;
    linebuf.end = (linebuf.end).wrapping_add(1);
    set_linebuf(fresh0, ch, attr);
    inc_end_column(w);
}
unsafe extern "C" fn addstr_linebuf(
    mut s: *const std::ffi::c_char,
    mut attr: std::ffi::c_int,
    mut cw: std::ffi::c_int,
) {
    while *s as std::ffi::c_int != '\0' as i32 {
        add_linebuf(*s, attr, cw);
        s = s.offset(1);
    }
}
unsafe extern "C" fn set_pfx(mut n: size_t, mut ch: std::ffi::c_char, mut attr: std::ffi::c_int) {
    linebuf.pfx[n as usize] = ch;
    linebuf.pfx_attr[n as usize] = attr;
}
unsafe extern "C" fn add_pfx(mut ch: std::ffi::c_char, mut attr: std::ffi::c_int) {
    let fresh1 = linebuf.pfx_end;
    linebuf.pfx_end = (linebuf.pfx_end).wrapping_add(1);
    set_pfx(fresh1, ch, attr);
}
#[no_mangle]
pub unsafe extern "C" fn plinestart(mut pos: POSITION) {
    let mut linenum: LINENUM = 0 as std::ffi::c_int as LINENUM;
    if linenums == 2 as std::ffi::c_int {
        linenum = find_linenum(pos);
    }
    if status_col != 0 || status_line != 0 {
        let mut c: std::ffi::c_char = posmark(pos);
        if c as std::ffi::c_int != 0 as std::ffi::c_int {
            line_mark_attr = (1 as std::ffi::c_int) << 6 as std::ffi::c_int
                | (6 as std::ffi::c_int) << 8 as std::ffi::c_int;
        } else if start_attnpos != -(1 as std::ffi::c_int) as POSITION
            && pos >= start_attnpos
            && pos <= end_attnpos
        {
            line_mark_attr = (1 as std::ffi::c_int) << 6 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 8 as std::ffi::c_int;
        }
        if status_col != 0 {
            add_pfx(
                (if c as std::ffi::c_int != 0 {
                    c as std::ffi::c_int
                } else {
                    ' ' as i32
                }) as std::ffi::c_char,
                line_mark_attr,
            );
            while linebuf.pfx_end < status_col_width as size_t {
                add_pfx(' ' as i32 as std::ffi::c_char, 0 as std::ffi::c_int);
            }
        }
    }
    if linenums == 2 as std::ffi::c_int {
        let mut buf: [std::ffi::c_char; 23] = [0; 23];
        let mut len: size_t = 0;
        let mut i: size_t = 0;
        linenum = vlinenum(linenum);
        if linenum == 0 as std::ffi::c_int as LINENUM {
            len = 0 as std::ffi::c_int as size_t;
        } else {
            linenumtoa(linenum, buf.as_mut_ptr(), 10 as std::ffi::c_int);
            len = strlen(buf.as_mut_ptr());
        }
        i = 0 as std::ffi::c_int as size_t;
        while i.wrapping_add(len) < linenum_width as size_t {
            add_pfx(' ' as i32 as std::ffi::c_char, 0 as std::ffi::c_int);
            i = i.wrapping_add(1);
        }
        i = 0 as std::ffi::c_int as size_t;
        while i < len {
            add_pfx(
                buf[i as usize],
                (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                    | (5 as std::ffi::c_int) << 8 as std::ffi::c_int,
            );
            i = i.wrapping_add(1);
        }
        add_pfx(' ' as i32 as std::ffi::c_char, 0 as std::ffi::c_int);
    }
    end_column = linebuf.pfx_end as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn line_pfx_width() -> std::ffi::c_int {
    let mut width: std::ffi::c_int = 0 as std::ffi::c_int;
    if status_col != 0 {
        width += status_col_width;
    }
    if linenums == 2 as std::ffi::c_int {
        width += linenum_width + 1 as std::ffi::c_int;
    }
    return width;
}
#[no_mangle]
pub unsafe extern "C" fn pshift_all() {
    pshift(linebuf.end);
    linebuf.end = linebuf.print;
    end_column = linebuf.pfx_end as std::ffi::c_int;
    line_pos = -(1 as std::ffi::c_int) as POSITION;
}
unsafe extern "C" fn attr_swidth(mut a: std::ffi::c_int) -> std::ffi::c_int {
    let mut w: std::ffi::c_int = 0 as std::ffi::c_int;
    a = apply_at_specials(a);
    if a & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
        w += ul_s_width;
    }
    if a & (1 as std::ffi::c_int) << 1 as std::ffi::c_int != 0 {
        w += bo_s_width;
    }
    if a & (1 as std::ffi::c_int) << 2 as std::ffi::c_int != 0 {
        w += bl_s_width;
    }
    if a & (1 as std::ffi::c_int) << 3 as std::ffi::c_int != 0 {
        w += so_s_width;
    }
    return w;
}
unsafe extern "C" fn attr_ewidth(mut a: std::ffi::c_int) -> std::ffi::c_int {
    let mut w: std::ffi::c_int = 0 as std::ffi::c_int;
    a = apply_at_specials(a);
    if a & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
        w += ul_e_width;
    }
    if a & (1 as std::ffi::c_int) << 1 as std::ffi::c_int != 0 {
        w += bo_e_width;
    }
    if a & (1 as std::ffi::c_int) << 2 as std::ffi::c_int != 0 {
        w += bl_e_width;
    }
    if a & (1 as std::ffi::c_int) << 3 as std::ffi::c_int != 0 {
        w += so_e_width;
    }
    return w;
}
#[no_mangle]
pub unsafe extern "C" fn pwidth(
    mut ch: LWCHAR,
    mut a: std::ffi::c_int,
    mut prev_ch: LWCHAR,
    mut prev_a: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut w: std::ffi::c_int = 0;
    if ch == '\u{8}' as i32 as LWCHAR {
        if prev_a
            & ((1 as std::ffi::c_int) << 4 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 5 as std::ffi::c_int)
            != 0
        {
            return strlen(prchar('\u{8}' as i32 as LWCHAR)) as std::ffi::c_int;
        }
        return if utf_mode != 0 && is_wide_char(prev_ch) as std::ffi::c_uint != 0 {
            -(2 as std::ffi::c_int)
        } else {
            -(1 as std::ffi::c_int)
        };
    }
    if utf_mode == 0 || is_ascii_char(ch) as std::ffi::c_uint != 0 {
        if control_char(ch) as u64 != 0 {
            return 0 as std::ffi::c_int;
        }
    } else if is_composing_char(ch) as std::ffi::c_uint != 0
        || is_combining_char(prev_ch, ch) as std::ffi::c_uint != 0
    {
        return 0 as std::ffi::c_int;
    }
    w = 1 as std::ffi::c_int;
    if is_wide_char(ch) as u64 != 0 {
        w += 1;
    }
    if linebuf.end > 0 as std::ffi::c_int as size_t
        && is_at_equiv(
            *(linebuf.attr)
                .offset((linebuf.end).wrapping_sub(1 as std::ffi::c_int as size_t) as isize),
            a,
        ) as u64
            == 0
    {
        w += attr_ewidth(
            *(linebuf.attr)
                .offset((linebuf.end).wrapping_sub(1 as std::ffi::c_int as size_t) as isize),
        );
    }
    if apply_at_specials(a) != 0 as std::ffi::c_int
        && (linebuf.end == 0 as std::ffi::c_int as size_t
            || is_at_equiv(
                *(linebuf.attr)
                    .offset((linebuf.end).wrapping_sub(1 as std::ffi::c_int as size_t) as isize),
                a,
            ) as u64
                == 0)
    {
        w += attr_swidth(a);
    }
    return w;
}
unsafe extern "C" fn backc() -> std::ffi::c_int {
    let mut ch: LWCHAR = 0;
    let mut p: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    if linebuf.end == 0 as std::ffi::c_int as size_t {
        return 0 as std::ffi::c_int;
    }
    p = &mut *(linebuf.buf).offset(linebuf.end as isize) as *mut std::ffi::c_char;
    ch = step_char(&mut p, -(1 as std::ffi::c_int), linebuf.buf);
    while p > linebuf.buf {
        let mut prev_ch: LWCHAR = 0;
        let mut width: std::ffi::c_int = 0;
        linebuf.end = p.offset_from(linebuf.buf) as std::ffi::c_long as size_t;
        prev_ch = step_char(&mut p, -(1 as std::ffi::c_int), linebuf.buf);
        width = pwidth(
            ch,
            *(linebuf.attr).offset(linebuf.end as isize),
            prev_ch,
            *(linebuf.attr)
                .offset((linebuf.end).wrapping_sub(1 as std::ffi::c_int as size_t) as isize),
        );
        end_column -= width;
        if width > 0 as std::ffi::c_int {
            break;
        }
        ch = prev_ch;
    }
    return 1 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn savec() {
    saved_line_end = linebuf.end;
    saved_end_column = end_column;
}
#[no_mangle]
pub unsafe extern "C" fn loadc() {
    linebuf.end = saved_line_end;
    end_column = saved_end_column;
}
#[no_mangle]
pub unsafe extern "C" fn is_ansi_end(mut ch: LWCHAR) -> lbool {
    if is_ascii_char(ch) as u64 == 0 {
        return LFALSE;
    }
    return (ch != 0 as std::ffi::c_int as LWCHAR
        && !(strchr(end_ansi_chars, ch as std::ffi::c_char as std::ffi::c_int)).is_null())
        as std::ffi::c_int as lbool;
}
#[no_mangle]
pub unsafe extern "C" fn is_ansi_middle(mut ch: LWCHAR) -> lbool {
    if is_ascii_char(ch) as u64 == 0 {
        return LFALSE;
    }
    if is_ansi_end(ch) as u64 != 0 {
        return LFALSE;
    }
    return (ch != 0 as std::ffi::c_int as LWCHAR
        && !(strchr(mid_ansi_chars, ch as std::ffi::c_char as std::ffi::c_int)).is_null())
        as std::ffi::c_int as lbool;
}
#[no_mangle]
pub unsafe extern "C" fn skip_ansi(
    mut pansi: *mut ansi_state_0,
    mut ch: LWCHAR,
    mut pp: *mut *const std::ffi::c_char,
    mut limit: *const std::ffi::c_char,
) {
    ansi_step(pansi, ch);
    loop {
        ch = step_charc(pp, 1 as std::ffi::c_int, limit);
        if !(*pp < limit
            && ansi_step(pansi, ch) as std::ffi::c_uint
                == ANSI_MID as std::ffi::c_int as std::ffi::c_uint)
        {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ansi_start(mut ch: LWCHAR) -> *mut ansi_state_0 {
    let mut pansi: *mut ansi_state_0 = 0 as *mut ansi_state_0;
    if !(control_char(ch) as std::ffi::c_uint != 0
        && (ch == ('[' as i32 & 0o37 as std::ffi::c_int) as LWCHAR
            || ch == -101i32 as std::ffi::c_uchar as LWCHAR))
    {
        return 0 as *mut ansi_state_0;
    }
    pansi = ecalloc(
        1 as std::ffi::c_int as size_t,
        ::core::mem::size_of::<ansi_state_0>() as std::ffi::c_ulong,
    ) as *mut ansi_state_0;
    (*pansi).ostate = OSC_START;
    (*pansi).otype = 0 as std::ffi::c_int as std::ffi::c_uint;
    (*pansi).escs_in_seq = 0 as std::ffi::c_int as std::ffi::c_uint;
    return pansi;
}
unsafe extern "C" fn valid_osc_intro(mut ch: std::ffi::c_char, mut content: lbool) -> lbool {
    let mut p: *const std::ffi::c_char = strchr(osc_ansi_chars, ch as std::ffi::c_int);
    if p.is_null() {
        return LFALSE;
    }
    return (content as u64 == 0
        || *p.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int == '*' as i32)
        as std::ffi::c_int as lbool;
}
unsafe extern "C" fn valid_osc_type(mut otype: std::ffi::c_int, mut content: lbool) -> lbool {
    let mut i: std::ffi::c_int = 0;
    if content as u64 == 0 {
        return LTRUE;
    }
    if otype == 8 as std::ffi::c_int {
        return LTRUE;
    }
    i = 0 as std::ffi::c_int;
    while i < osc_ansi_allow_count {
        if *osc_ansi_allow.offset(i as isize) == otype as std::ffi::c_long {
            return LTRUE;
        }
        i += 1;
    }
    return LFALSE;
}
unsafe extern "C" fn osc_return(
    mut pansi: *mut ansi_state_0,
    mut ostate: osc8_state,
    mut astate: ansi_state,
) -> ansi_state {
    (*pansi).ostate = ostate;
    return astate;
}
unsafe extern "C" fn ansi_step2(
    mut pansi: *mut ansi_state_0,
    mut ch: LWCHAR,
    mut content: lbool,
) -> ansi_state {
    let mut current_block_33: u64;
    match (*pansi).ostate as std::ffi::c_uint {
        0 => {
            if control_char(ch) as std::ffi::c_uint != 0
                && (ch == ('[' as i32 & 0o37 as std::ffi::c_int) as LWCHAR
                    || ch == -101i32 as std::ffi::c_uchar as LWCHAR)
            {
                return osc_return(pansi, OSC_INTRO, ANSI_MID);
            }
            current_block_33 = 15345278821338558188;
        }
        1 => {
            if ch == ']' as i32 as LWCHAR {
                return osc_return(pansi, OSC_TYPENUM, ANSI_MID);
            }
            if is_ascii_char(ch) as std::ffi::c_uint != 0
                && valid_osc_intro(ch as std::ffi::c_char, content) as std::ffi::c_uint != 0
            {
                return osc_return(pansi, OSC_STRING, ANSI_MID);
            }
            if control_char(ch) as std::ffi::c_uint != 0
                && (ch == ('[' as i32 & 0o37 as std::ffi::c_int) as LWCHAR
                    || ch == -101i32 as std::ffi::c_uchar as LWCHAR)
            {
                return osc_return(pansi, OSC_INTRO, ANSI_MID);
            }
            (*pansi).ostate = OSC_START;
            current_block_33 = 15345278821338558188;
        }
        2 => {
            if ch >= '0' as i32 as LWCHAR && ch <= '9' as i32 as LWCHAR {
                let (fresh2, fresh3) =
                    ((*pansi).otype).overflowing_mul(10 as std::ffi::c_int as u32);
                *(&mut (*pansi).otype as *mut std::ffi::c_uint) = fresh2;
                if fresh3 as std::ffi::c_int != 0 || {
                    let (fresh4, fresh5) = ((*pansi).otype)
                        .overflowing_add(ch.wrapping_sub('0' as i32 as LWCHAR) as u32);
                    *(&mut (*pansi).otype as *mut std::ffi::c_uint) = fresh4;
                    fresh5 as std::ffi::c_int != 0
                } {
                    return osc_return(pansi, OSC_STRING, ANSI_MID);
                }
                return osc_return(pansi, OSC_TYPENUM, ANSI_MID);
            }
            if ch == ';' as i32 as LWCHAR {
                return osc_return(
                    pansi,
                    (if (*pansi).otype == 8 as std::ffi::c_int as std::ffi::c_uint {
                        OSC8_PARAMS as std::ffi::c_int
                    } else {
                        OSC_STRING as std::ffi::c_int
                    }) as osc8_state,
                    ANSI_MID,
                );
            }
            if control_char(ch) as std::ffi::c_uint != 0
                && (ch == ('[' as i32 & 0o37 as std::ffi::c_int) as LWCHAR
                    || ch == -101i32 as std::ffi::c_uchar as LWCHAR)
            {
                return osc_return(pansi, OSC_END_CSI, ANSI_MID);
            }
            if ch == '\u{7}' as i32 as LWCHAR {
                return osc_return(pansi, OSC_END, ANSI_END);
            }
            return osc_return(pansi, OSC_STRING, ANSI_MID);
        }
        6 => {
            if ch == ';' as i32 as LWCHAR {
                return osc_return(pansi, OSC8_URI, ANSI_MID);
            }
            current_block_33 = 4417942540951631311;
        }
        7 | 3 => {
            current_block_33 = 4417942540951631311;
        }
        4 => {
            if ch == '\\' as i32 as LWCHAR {
                return osc_return(
                    pansi,
                    OSC_END,
                    (if valid_osc_type((*pansi).otype as std::ffi::c_int, content)
                        as std::ffi::c_uint
                        != 0
                    {
                        ANSI_END as std::ffi::c_int
                    } else {
                        ANSI_ERR as std::ffi::c_int
                    }) as ansi_state,
                );
            }
            return osc_return(pansi, OSC_STRING, ANSI_MID);
        }
        5 => return ANSI_END,
        8 | _ => {
            current_block_33 = 15345278821338558188;
        }
    }
    match current_block_33 {
        15345278821338558188 => {}
        _ => {
            if ch == '\u{7}' as i32 as LWCHAR {
                return osc_return(
                    pansi,
                    OSC_END,
                    (if valid_osc_type((*pansi).otype as std::ffi::c_int, content)
                        as std::ffi::c_uint
                        != 0
                    {
                        ANSI_END as std::ffi::c_int
                    } else {
                        ANSI_ERR as std::ffi::c_int
                    }) as ansi_state,
                );
            }
            if control_char(ch) as std::ffi::c_uint != 0
                && (ch == ('[' as i32 & 0o37 as std::ffi::c_int) as LWCHAR
                    || ch == -101i32 as std::ffi::c_uchar as LWCHAR)
            {
                (*pansi).escs_in_seq = ((*pansi).escs_in_seq).wrapping_add(1);
                (*pansi).escs_in_seq;
                return osc_return(pansi, OSC_END_CSI, ANSI_MID);
            }
            return ANSI_MID;
        }
    }
    if is_ansi_middle(ch) as u64 != 0 {
        return ANSI_MID;
    }
    if is_ansi_end(ch) as u64 != 0 {
        return ANSI_END;
    }
    return ANSI_ERR;
}
#[no_mangle]
pub unsafe extern "C" fn ansi_step(mut pansi: *mut ansi_state_0, mut ch: LWCHAR) -> ansi_state {
    return ansi_step2(pansi, ch, LTRUE);
}
#[no_mangle]
pub unsafe extern "C" fn ansi_osc8_state(mut pansi: *mut ansi_state_0) -> osc8_state {
    return (*pansi).ostate;
}
#[no_mangle]
pub unsafe extern "C" fn ansi_done(mut pansi: *mut ansi_state_0) {
    free(pansi as *mut std::ffi::c_void);
}
unsafe extern "C" fn fits_on_screen(mut w: std::ffi::c_int, mut a: std::ffi::c_int) -> lbool {
    if ctldisp == 1 as std::ffi::c_int {
        return LTRUE;
    }
    return (end_column - cshift + w + attr_ewidth(a) <= sc_width) as std::ffi::c_int as lbool;
}
unsafe extern "C" fn store_char(
    mut ch: LWCHAR,
    mut a: std::ffi::c_int,
    mut rep: *const std::ffi::c_char,
    mut pos: POSITION,
) -> std::ffi::c_int {
    let mut w: std::ffi::c_int = 0;
    let mut i: size_t = 0;
    let mut replen: size_t = 0;
    let mut cs: std::ffi::c_char = 0;
    let mut ov: std::ffi::c_int = 0;
    ov = a
        & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
            | (1 as std::ffi::c_int) << 1 as std::ffi::c_int);
    if ov != 0 as std::ffi::c_int {
        last_overstrike = ov;
    }
    let mut matches: std::ffi::c_int = 0;
    let mut resend_last: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut hl_attr: std::ffi::c_int = 0 as std::ffi::c_int;
    if pos != -(1 as std::ffi::c_int) as POSITION
        && a != (1 as std::ffi::c_int) << 4 as std::ffi::c_int
    {
        hl_attr = is_hilited_attr(
            pos,
            pos + 1 as std::ffi::c_int as POSITION,
            0 as std::ffi::c_int,
            &mut matches,
        );
        if hl_attr == 0 as std::ffi::c_int && status_line != 0 {
            hl_attr = line_mark_attr;
        }
    }
    if hl_attr != 0 {
        a |= hl_attr;
        if highest_hilite != -(1 as std::ffi::c_int) as POSITION
            && pos != -(1 as std::ffi::c_int) as POSITION
            && pos > highest_hilite
        {
            highest_hilite = pos;
        }
        in_hilite = LTRUE;
    } else {
        if in_hilite as u64 != 0 {
            resend_last = 1 as std::ffi::c_int;
        }
        in_hilite = LFALSE;
    }
    if resend_last != 0 {
        let mut ai: std::ffi::c_int = 0;
        ai = 0 as std::ffi::c_int;
        while ai < 3 as std::ffi::c_int {
            let mut ax: std::ffi::c_int = (curr_last_ansi + ai) % 3 as std::ffi::c_int;
            i = 0 as std::ffi::c_int as size_t;
            while i < last_ansis[ax as usize].data.len() as u64 {
                if store_char(
                    ((&*last_ansis)[ax as usize].data)[i as usize] as LWCHAR,
                    (1 as std::ffi::c_int) << 4 as std::ffi::c_int,
                    0 as *const std::ffi::c_char,
                    pos,
                ) != 0
                {
                    return 1 as std::ffi::c_int;
                }
                i = i.wrapping_add(1);
            }
            ai += 1;
        }
    }
    if a == (1 as std::ffi::c_int) << 4 as std::ffi::c_int {
        w = 0 as std::ffi::c_int;
    } else {
        let mut p: *mut std::ffi::c_char =
            &mut *(linebuf.buf).offset(linebuf.end as isize) as *mut std::ffi::c_char;
        let mut prev_ch: LWCHAR = if linebuf.end > 0 as std::ffi::c_int as size_t {
            step_char(&mut p, -(1 as std::ffi::c_int), linebuf.buf)
        } else {
            0 as std::ffi::c_int as LWCHAR
        };
        let mut prev_a: std::ffi::c_int = if linebuf.end > 0 as std::ffi::c_int as size_t {
            *(linebuf.attr)
                .offset((linebuf.end).wrapping_sub(1 as std::ffi::c_int as size_t) as isize)
        } else {
            0 as std::ffi::c_int
        };
        w = pwidth(ch, a, prev_ch, prev_a);
    }
    if fits_on_screen(w, a) as u64 == 0 {
        return 1 as std::ffi::c_int;
    }
    if rep.is_null() {
        cs = ch as std::ffi::c_char;
        rep = &mut cs;
        replen = 1 as std::ffi::c_int as size_t;
    } else {
        replen = utf_len(*rep.offset(0 as std::ffi::c_int as isize)) as size_t;
    }
    if cshift == hshift {
        if line_pos == -(1 as std::ffi::c_int) as POSITION {
            line_pos = pos;
        }
        if shifted_ansi.data.len() > 0 {
            i = 0 as std::ffi::c_int as size_t;
            while i < shifted_ansi.data.len() as u64 {
                add_linebuf(
                    ((&*shifted_ansi).data)[i as usize] as std::ffi::c_char,
                    (1 as std::ffi::c_int) << 4 as std::ffi::c_int,
                    0 as std::ffi::c_int,
                );
                i = i.wrapping_add(1);
            }
            shifted_ansi.reset();
        }
    }
    inc_end_column(w);
    i = 0 as std::ffi::c_int as size_t;
    while i < replen {
        let fresh6 = rep;
        rep = rep.offset(1);
        add_linebuf(*fresh6, a, 0 as std::ffi::c_int);
        i = i.wrapping_add(1);
    }
    if cshift < hshift {
        if a == (1 as std::ffi::c_int) << 4 as std::ffi::c_int {
            shifted_ansi.xbuf_add_char(ch as std::ffi::c_char);
        }
        if linebuf.end > linebuf.print {
            let mut i_0: size_t = 0;
            i_0 = 0 as std::ffi::c_int as size_t;
            while i_0 < linebuf.print {
                *(linebuf.buf).offset(i_0 as isize) =
                    *(linebuf.buf).offset(i_0.wrapping_add(replen) as isize);
                *(linebuf.attr).offset(i_0 as isize) =
                    *(linebuf.attr).offset(i_0.wrapping_add(replen) as isize);
                i_0 = i_0.wrapping_add(1);
            }
            linebuf.end = (linebuf.end).wrapping_sub(replen);
            cshift += w;
            while cshift > hshift {
                add_linebuf(
                    ' ' as i32 as std::ffi::c_char,
                    rscroll_attr,
                    0 as std::ffi::c_int,
                );
                cshift -= 1;
            }
        }
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn store_string(
    mut s: *const std::ffi::c_char,
    mut a: std::ffi::c_int,
    mut pos: POSITION,
) -> std::ffi::c_int {
    if fits_on_screen(strlen(s) as std::ffi::c_int, a) as u64 == 0 {
        return 1 as std::ffi::c_int;
    }
    while *s as std::ffi::c_int != 0 as std::ffi::c_int {
        if store_char(*s as LWCHAR, a, 0 as *const std::ffi::c_char, pos) != 0 {
            return 1 as std::ffi::c_int;
        }
        s = s.offset(1);
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn tab_spaces(mut col: std::ffi::c_int) -> std::ffi::c_int {
    let mut to_tab: std::ffi::c_int = col - linebuf.pfx_end as std::ffi::c_int;
    if ntabstops < 2 as std::ffi::c_int
        || to_tab >= tabstops[(ntabstops - 1 as std::ffi::c_int) as usize]
    {
        to_tab = tabdefault
            - (to_tab - tabstops[(ntabstops - 1 as std::ffi::c_int) as usize]) % tabdefault;
    } else {
        let mut i: std::ffi::c_int = 0;
        i = ntabstops - 2 as std::ffi::c_int;
        while i >= 0 as std::ffi::c_int {
            if to_tab >= tabstops[i as usize] {
                break;
            }
            i -= 1;
        }
        to_tab = tabstops[(i + 1 as std::ffi::c_int) as usize] - to_tab;
    }
    return to_tab;
}
unsafe extern "C" fn store_tab(mut attr: std::ffi::c_int, mut pos: POSITION) -> std::ffi::c_int {
    let mut to_tab: std::ffi::c_int = tab_spaces(end_column);
    loop {
        if store_char(
            ' ' as i32 as LWCHAR,
            attr,
            b" \0" as *const u8 as *const std::ffi::c_char,
            pos,
        ) != 0
        {
            return 1 as std::ffi::c_int;
        }
        to_tab -= 1;
        if !(to_tab > 0 as std::ffi::c_int) {
            break;
        }
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn store_prchar(mut c: LWCHAR, mut pos: POSITION) -> std::ffi::c_int {
    if store_string(
        prchar(c),
        (1 as std::ffi::c_int) << 5 as std::ffi::c_int
            | (3 as std::ffi::c_int) << 8 as std::ffi::c_int,
        pos,
    ) != 0
    {
        return 1 as std::ffi::c_int;
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn flush_mbc_buf(mut pos: POSITION) -> std::ffi::c_int {
    let mut i: std::ffi::c_int = 0;
    i = 0 as std::ffi::c_int;
    while i < mbc_buf_index {
        if store_prchar(mbc_buf[i as usize] as LWCHAR, pos) != 0 {
            return mbc_buf_index - i;
        }
        i += 1;
    }
    return 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pappend_b(
    mut c: std::ffi::c_char,
    mut pos: POSITION,
    mut before_pendc: lbool,
) -> std::ffi::c_int {
    let mut ch: LWCHAR = (c as std::ffi::c_int & 0o377 as std::ffi::c_int) as LWCHAR;
    let mut r: std::ffi::c_int = 0;
    if pendc != 0 && before_pendc as u64 == 0 {
        if ch == '\r' as i32 as LWCHAR && pendc == '\r' as i32 as LWCHAR {
            return 0 as std::ffi::c_int;
        }
        if do_append(pendc, 0 as *const std::ffi::c_char, pendpos) != 0 {
            return 1 as std::ffi::c_int;
        }
        pendc = '\0' as i32 as LWCHAR;
    }
    if ch == '\r' as i32 as LWCHAR
        && (proc_return == 1 as std::ffi::c_int
            || bs_mode == 0 as std::ffi::c_int && proc_return == 0 as std::ffi::c_int)
    {
        if mbc_buf_len > 0 as std::ffi::c_int {
            r = flush_mbc_buf(mbc_pos);
            mbc_buf_index = r + 1 as std::ffi::c_int;
            mbc_buf_len = 0 as std::ffi::c_int;
            if r != 0 {
                return mbc_buf_index;
            }
        }
        pendc = ch;
        pendpos = pos;
        return 0 as std::ffi::c_int;
    }
    if utf_mode == 0 {
        r = do_append(ch, 0 as *const std::ffi::c_char, pos);
    } else {
        let mut current_block_41: u64;
        if mbc_buf_len == 0 as std::ffi::c_int {
            current_block_41 = 11600700225610697556;
        } else if c as std::ffi::c_int & 0xc0 as std::ffi::c_int == 0x80 as std::ffi::c_int {
            let fresh7 = mbc_buf_index;
            mbc_buf_index = mbc_buf_index + 1;
            mbc_buf[fresh7 as usize] = c;
            if mbc_buf_index < mbc_buf_len {
                return 0 as std::ffi::c_int;
            }
            if is_utf8_well_formed(mbc_buf.as_mut_ptr(), mbc_buf_index) as u64 != 0 {
                r = do_append(
                    get_wchar(mbc_buf.as_mut_ptr()),
                    mbc_buf.as_mut_ptr(),
                    mbc_pos,
                );
            } else {
                r = flush_mbc_buf(mbc_pos);
                mbc_buf_index = r;
            }
            mbc_buf_len = 0 as std::ffi::c_int;
            current_block_41 = 2873832966593178012;
        } else {
            r = flush_mbc_buf(mbc_pos);
            mbc_buf_index = r + 1 as std::ffi::c_int;
            mbc_buf_len = 0 as std::ffi::c_int;
            if r == 0 {
                current_block_41 = 11600700225610697556;
            } else {
                current_block_41 = 2873832966593178012;
            }
        }
        match current_block_41 {
            11600700225610697556 => {
                mbc_buf_index = 1 as std::ffi::c_int;
                *mbc_buf.as_mut_ptr() = c;
                if c as std::ffi::c_int & 0x80 as std::ffi::c_int == 0 as std::ffi::c_int {
                    r = do_append(ch, 0 as *const std::ffi::c_char, pos);
                } else if c as std::ffi::c_int & 0xc0 as std::ffi::c_int == 0xc0 as std::ffi::c_int
                    && !(c as std::ffi::c_int & 0xfe as std::ffi::c_int == 0xfe as std::ffi::c_int)
                {
                    mbc_buf_len = utf_len(c);
                    mbc_pos = pos;
                    return 0 as std::ffi::c_int;
                } else {
                    r = flush_mbc_buf(pos);
                }
            }
            _ => {}
        }
    }
    if r != 0 {
        r = if utf_mode == 0 {
            1 as std::ffi::c_int
        } else {
            mbc_buf_index
        };
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn pappend(mut c: std::ffi::c_char, mut pos: POSITION) -> std::ffi::c_int {
    if ff_starts_line < 0 as std::ffi::c_int {
        ff_starts_line =
            (c as std::ffi::c_int == 'L' as i32 & 0o37 as std::ffi::c_int) as std::ffi::c_int;
    }
    return pappend_b(c, pos, LFALSE);
}
#[no_mangle]
pub unsafe extern "C" fn line_is_ff() -> lbool {
    return (ff_starts_line == 1 as std::ffi::c_int) as std::ffi::c_int as lbool;
}
unsafe extern "C" fn store_control_char(
    mut ch: LWCHAR,
    mut rep: *const std::ffi::c_char,
    mut pos: POSITION,
) -> std::ffi::c_int {
    if ctldisp == 1 as std::ffi::c_int {
        if store_char(ch, 0 as std::ffi::c_int, rep, pos) != 0 {
            return 1 as std::ffi::c_int;
        }
    } else if store_prchar(ch, pos) != 0 {
        return 1 as std::ffi::c_int;
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn store_ansi(
    mut ch: LWCHAR,
    mut rep: *const std::ffi::c_char,
    mut pos: POSITION,
) -> std::ffi::c_int {
    match ansi_step2(
        line_ansi,
        ch,
        (pos != -(1 as std::ffi::c_int) as POSITION) as std::ffi::c_int as lbool,
    ) as std::ffi::c_uint
    {
        1 => {
            if store_char(ch, (1 as std::ffi::c_int) << 4 as std::ffi::c_int, rep, pos) != 0 {
                return 1 as std::ffi::c_int;
            }
            match ansi_osc8_state(line_ansi) as std::ffi::c_uint {
                2 | 3 => {
                    hlink_in_line = LTRUE;
                }
                _ => {}
            }
            last_ansi.xbuf_add_char(ch as std::ffi::c_char);
        }
        3 => {
            if store_char(ch, (1 as std::ffi::c_int) << 4 as std::ffi::c_int, rep, pos) != 0 {
                return 1 as std::ffi::c_int;
            }
            ansi_done(line_ansi);
            line_ansi = 0 as *mut ansi_state_0;
            last_ansi.xbuf_add_char(ch as std::ffi::c_char);
            last_ansis[curr_last_ansi as usize].set(&mut last_ansi);
            last_ansi.reset();
            curr_last_ansi = (curr_last_ansi + 1 as std::ffi::c_int) % 3 as std::ffi::c_int;
        }
        2 => {
            let mut start: *const std::ffi::c_char = if cshift < hshift {
                std::ptr::from_ref((&*shifted_ansi).char_data().first().unwrap())
            } else {
                linebuf.buf as *const std::ffi::c_char
            };
            let mut end: *mut size_t = if cshift < hshift {
                std::ptr::from_mut(&mut (shifted_ansi.data.len() as u64))
            } else {
                &mut linebuf.end
            };
            let mut p: *const std::ffi::c_char = start.offset(*end as isize);
            let mut bch: LWCHAR = 0;
            loop {
                bch = step_charc(&mut p, -(1 as std::ffi::c_int), start);
                if !(p > start
                    && (!(control_char(bch) as std::ffi::c_uint != 0
                        && (bch == ('[' as i32 & 0o37 as std::ffi::c_int) as LWCHAR
                            || bch == -101i32 as std::ffi::c_uchar as LWCHAR))
                        || {
                            let fresh8 = (*line_ansi).escs_in_seq;
                            (*line_ansi).escs_in_seq = ((*line_ansi).escs_in_seq).wrapping_sub(1);
                            fresh8 > 0 as std::ffi::c_int as std::ffi::c_uint
                        }))
                {
                    break;
                }
            }
            *end = p.offset_from(start) as std::ffi::c_long as size_t;
            last_ansi.reset();
            ansi_done(line_ansi);
            line_ansi = 0 as *mut ansi_state_0;
        }
        _ => {}
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn store_bs(
    mut ch: LWCHAR,
    mut rep: *const std::ffi::c_char,
    mut pos: POSITION,
) -> std::ffi::c_int {
    if proc_backspace == 2 as std::ffi::c_int
        || bs_mode == 2 as std::ffi::c_int && proc_backspace == 0 as std::ffi::c_int
    {
        return store_control_char(ch, rep, pos);
    }
    if linebuf.end > 0 as std::ffi::c_int as size_t
        && (linebuf.end <= linebuf.print
            && *(linebuf.buf)
                .offset((linebuf.end).wrapping_sub(1 as std::ffi::c_int as size_t) as isize)
                as std::ffi::c_int
                == '\0' as i32
            || linebuf.end > 0 as std::ffi::c_int as size_t
                && *(linebuf.attr)
                    .offset((linebuf.end).wrapping_sub(1 as std::ffi::c_int as size_t) as isize)
                    & ((1 as std::ffi::c_int) << 4 as std::ffi::c_int
                        | (1 as std::ffi::c_int) << 5 as std::ffi::c_int)
                    != 0)
    {
        if store_prchar('\u{8}' as i32 as LWCHAR, pos) != 0 {
            return 1 as std::ffi::c_int;
        }
    } else if proc_backspace == 0 as std::ffi::c_int && bs_mode == 1 as std::ffi::c_int {
        if store_char(ch, 0 as std::ffi::c_int, 0 as *const std::ffi::c_char, pos) != 0 {
            return 1 as std::ffi::c_int;
        }
    } else if proc_backspace == 1 as std::ffi::c_int
        || bs_mode == 0 as std::ffi::c_int && proc_backspace == 0 as std::ffi::c_int
    {
        overstrike = backc();
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn do_append(
    mut ch: LWCHAR,
    mut rep: *const std::ffi::c_char,
    mut pos: POSITION,
) -> std::ffi::c_int {
    let mut a: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut in_overstrike: std::ffi::c_int = overstrike;
    if (ctldisp == 2 as std::ffi::c_int || pos == -(1 as std::ffi::c_int) as POSITION)
        && line_ansi.is_null()
    {
        line_ansi = ansi_start(ch);
        if !line_ansi.is_null() {
            ansi_in_line = LTRUE;
        }
    }
    overstrike = 0 as std::ffi::c_int;
    if !line_ansi.is_null() {
        return store_ansi(ch, rep, pos);
    }
    if ch == '\u{8}' as i32 as LWCHAR {
        return store_bs(ch, rep, pos);
    }
    if in_overstrike > 0 as std::ffi::c_int {
        let mut prev_ch: LWCHAR = 0;
        overstrike = if utf_mode != 0 {
            -(1 as std::ffi::c_int)
        } else {
            0 as std::ffi::c_int
        };
        if utf_mode != 0 {
            prev_ch = get_wchar(&mut *(linebuf.buf).offset(linebuf.end as isize));
        } else {
            prev_ch = *(linebuf.buf).offset(linebuf.end as isize) as std::ffi::c_uchar as LWCHAR;
        }
        a = *(linebuf.attr).offset(linebuf.end as isize);
        if ch == prev_ch {
            if ch == '_' as i32 as LWCHAR {
                if a & ((1 as std::ffi::c_int) << 1 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 0 as std::ffi::c_int)
                    != 0 as std::ffi::c_int
                {
                    a |= (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                        | (1 as std::ffi::c_int) << 0 as std::ffi::c_int;
                } else if last_overstrike != 0 as std::ffi::c_int {
                    a |= last_overstrike;
                } else {
                    a |= (1 as std::ffi::c_int) << 1 as std::ffi::c_int;
                }
            } else {
                a |= (1 as std::ffi::c_int) << 1 as std::ffi::c_int;
            }
        } else if ch == '_' as i32 as LWCHAR {
            a |= (1 as std::ffi::c_int) << 0 as std::ffi::c_int;
            ch = prev_ch;
            rep = &mut *(linebuf.buf).offset(linebuf.end as isize) as *mut std::ffi::c_char;
        } else if prev_ch == '_' as i32 as LWCHAR {
            a |= (1 as std::ffi::c_int) << 0 as std::ffi::c_int;
        }
    } else if in_overstrike < 0 as std::ffi::c_int {
        if is_composing_char(ch) as std::ffi::c_uint != 0
            || is_combining_char(
                get_wchar(&mut *(linebuf.buf).offset(linebuf.end as isize)),
                ch,
            ) as std::ffi::c_uint
                != 0
        {
            a = last_overstrike;
        } else {
            overstrike = 0 as std::ffi::c_int;
        }
    }
    if ch == '\t' as i32 as LWCHAR {
        if proc_tab == 2 as std::ffi::c_int
            || bs_mode == 2 as std::ffi::c_int && proc_tab == 0 as std::ffi::c_int
        {
            return store_control_char(ch, rep, pos);
        }
        if store_tab(a, pos) != 0 {
            return 1 as std::ffi::c_int;
        }
        return 0 as std::ffi::c_int;
    }
    if (utf_mode == 0 || is_ascii_char(ch) as std::ffi::c_uint != 0)
        && control_char(ch) as std::ffi::c_uint != 0
    {
        return store_control_char(ch, rep, pos);
    } else if utf_mode != 0
        && ctldisp != 1 as std::ffi::c_int
        && is_ubin_char(ch) as std::ffi::c_uint != 0
    {
        if store_string(
            prutfchar(ch),
            (1 as std::ffi::c_int) << 5 as std::ffi::c_int,
            pos,
        ) != 0
        {
            return 1 as std::ffi::c_int;
        }
    } else if store_char(ch, a, rep, pos) != 0 {
        return 1 as std::ffi::c_int;
    }
    return 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pflushmbc() -> std::ffi::c_int {
    let mut r: std::ffi::c_int = 0 as std::ffi::c_int;
    if mbc_buf_len > 0 as std::ffi::c_int {
        r = flush_mbc_buf(mbc_pos);
        mbc_buf_len = 0 as std::ffi::c_int;
    }
    return r;
}
unsafe extern "C" fn add_attr_normal() {
    if !line_ansi.is_null() {
        match (*line_ansi).ostate as std::ffi::c_uint {
            2 | 6 | 7 | 3 => {
                addstr_linebuf(
                    b"\x1B\\\0" as *const u8 as *const std::ffi::c_char,
                    (1 as std::ffi::c_int) << 4 as std::ffi::c_int,
                    0 as std::ffi::c_int,
                );
            }
            _ => {}
        }
        ansi_done(line_ansi);
        line_ansi = 0 as *mut ansi_state_0;
    }
    if ctldisp != 2 as std::ffi::c_int || is_ansi_end('m' as i32 as LWCHAR) as u64 == 0 {
        return;
    }
    addstr_linebuf(
        b"\x1B[m\0" as *const u8 as *const std::ffi::c_char,
        (1 as std::ffi::c_int) << 4 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    if hlink_in_line as u64 != 0 {
        addstr_linebuf(
            b"\x1B]8;;\x1B\\\0" as *const u8 as *const std::ffi::c_char,
            (1 as std::ffi::c_int) << 4 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn pdone(mut endline: lbool, mut chopped: lbool, mut forw: lbool) {
    pflushmbc();
    linebuf.prev_end = if endline as u64 == 0 && chopped as u64 == 0 {
        linebuf.end
    } else {
        0 as std::ffi::c_int as size_t
    };
    if pendc != 0 && (pendc != '\r' as i32 as LWCHAR || endline as u64 == 0) {
        do_append(pendc, 0 as *const std::ffi::c_char, pendpos);
    }
    if chopped as std::ffi::c_uint != 0 && rscroll_char != 0 {
        let mut rscroll_utf8: [std::ffi::c_char; 7] = [0; 7];
        let mut up: *mut std::ffi::c_char = rscroll_utf8.as_mut_ptr();
        if end_column >= sc_width + cshift {
            end_column = right_column;
            linebuf.end = right_curr as size_t;
        }
        add_attr_normal();
        while end_column < sc_width - 1 as std::ffi::c_int + cshift {
            add_linebuf(
                ' ' as i32 as std::ffi::c_char,
                0 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
        }
        put_wchar(&mut up, rscroll_char);
        *up = '\0' as i32 as std::ffi::c_char;
        addstr_linebuf(
            rscroll_utf8.as_mut_ptr(),
            rscroll_attr,
            0 as std::ffi::c_int,
        );
        inc_end_column(1 as std::ffi::c_int);
    } else {
        add_attr_normal();
    }
    if status_line != 0 && line_mark_attr != 0 as std::ffi::c_int {
        while (end_column + 1 as std::ffi::c_int) < sc_width + cshift {
            add_linebuf(
                ' ' as i32 as std::ffi::c_char,
                line_mark_attr,
                1 as std::ffi::c_int,
            );
        }
    }
    if end_column < sc_width + cshift
        || auto_wrap == 0
        || endline as std::ffi::c_uint != 0 && ignaw != 0
        || ctldisp == 1 as std::ffi::c_int
    {
        add_linebuf(
            '\n' as i32 as std::ffi::c_char,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
    } else if ignaw != 0 && end_column >= sc_width + cshift && forw as std::ffi::c_uint != 0 {
        add_linebuf(
            ' ' as i32 as std::ffi::c_char,
            0 as std::ffi::c_int,
            1 as std::ffi::c_int,
        );
        add_linebuf(
            '\u{8}' as i32 as std::ffi::c_char,
            0 as std::ffi::c_int,
            -(1 as std::ffi::c_int),
        );
    }
    if auto_wrap != 0 && ignaw == 0 && end_column >= sc_width + cshift {
        clear_after_line = LTRUE;
    }
    set_linebuf(
        linebuf.end,
        '\0' as i32 as std::ffi::c_char,
        0 as std::ffi::c_int,
    );
}
unsafe extern "C" fn col_vs_pos(
    mut linepos: POSITION,
    mut cp: *mut col_pos,
    mut saved_pos: POSITION,
    mut saved_col: std::ffi::c_int,
) {
    let mut col: std::ffi::c_int = if saved_col < 0 as std::ffi::c_int {
        0 as std::ffi::c_int
    } else {
        saved_col
    };
    let mut prev_ch: LWCHAR = 0 as std::ffi::c_int as LWCHAR;
    let mut pansi: *mut ansi_state_0 = 0 as *mut ansi_state_0;
    let mut utf8_buf: [std::ffi::c_char; 6] = [0; 6];
    let mut utf8_len: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut chpos: POSITION = 0;
    if ch_seek(if saved_pos != -(1 as std::ffi::c_int) as POSITION {
        saved_pos
    } else {
        linepos
    }) != 0
    {
        return;
    }
    loop {
        let mut ich: std::ffi::c_int = 0;
        let mut ch: std::ffi::c_char = 0;
        let mut cw: std::ffi::c_int = 0 as std::ffi::c_int;
        chpos = ch_tell();
        ich = ch_forw_get();
        ch = ich as std::ffi::c_char;
        if ich == -(1 as std::ffi::c_int) || ch as std::ffi::c_int == '\n' as i32 {
            break;
        }
        if !pansi.is_null() {
            if ansi_step(pansi, ch as LWCHAR) as std::ffi::c_uint
                != ANSI_MID as std::ffi::c_int as std::ffi::c_uint
            {
                ansi_done(pansi);
                pansi = 0 as *mut ansi_state_0;
            }
        } else if ctldisp == 2 as std::ffi::c_int && {
            pansi = ansi_start(ch as LWCHAR);
            !pansi.is_null()
        } {
            ansi_step(pansi, ch as LWCHAR);
        } else if ch as std::ffi::c_int == '\u{8}' as i32 {
            if proc_backspace == 2 as std::ffi::c_int
                || bs_mode == 2 as std::ffi::c_int && proc_backspace == 0 as std::ffi::c_int
            {
                cw = strlen(prchar(ch as LWCHAR)) as std::ffi::c_int;
            } else {
                cw = if utf_mode != 0 && is_wide_char(prev_ch) as std::ffi::c_uint != 0 {
                    -(2 as std::ffi::c_int)
                } else {
                    -(1 as std::ffi::c_int)
                };
            }
        } else if ch as std::ffi::c_int == '\t' as i32 {
            if proc_tab == 2 as std::ffi::c_int
                || bs_mode == 2 as std::ffi::c_int && proc_tab == 0 as std::ffi::c_int
            {
                cw = strlen(prchar(ch as LWCHAR)) as std::ffi::c_int;
            } else {
                cw = tab_spaces(col);
            }
        } else if (utf_mode == 0 || is_ascii_char(ch as LWCHAR) as std::ffi::c_uint != 0)
            && control_char(ch as LWCHAR) as std::ffi::c_uint != 0
        {
            cw = strlen(prchar(ch as LWCHAR)) as std::ffi::c_int;
        } else if utf8_len < 6 as std::ffi::c_int {
            let fresh9 = utf8_len;
            utf8_len = utf8_len + 1;
            utf8_buf[fresh9 as usize] = ch;
            if is_utf8_well_formed(utf8_buf.as_mut_ptr(), utf8_len) as u64 != 0 {
                let mut wch: LWCHAR = get_wchar(utf8_buf.as_mut_ptr());
                let mut attr: std::ffi::c_int = 0 as std::ffi::c_int;
                utf8_len = 0 as std::ffi::c_int;
                if utf_mode != 0
                    && ctldisp != 1 as std::ffi::c_int
                    && is_ubin_char(wch) as std::ffi::c_uint != 0
                {
                    cw = strlen(prutfchar(wch)) as std::ffi::c_int;
                } else {
                    cw = pwidth(wch, attr, prev_ch, attr);
                }
                prev_ch = wch;
            }
        } else {
            utf8_len = 0 as std::ffi::c_int;
        }
        if (*cp).pos != -(1 as std::ffi::c_int) as POSITION && chpos == (*cp).pos {
            break;
        }
        if (*cp).col >= 0 as std::ffi::c_int && col >= (*cp).col && cw > 0 as std::ffi::c_int {
            break;
        } else {
            col += cw;
            prev_ch = ch as LWCHAR;
        }
    }
    (*cp).col = col;
    (*cp).pos = chpos;
}
#[no_mangle]
pub unsafe extern "C" fn col_from_pos(
    mut linepos: POSITION,
    mut spos: POSITION,
    mut saved_pos: POSITION,
    mut saved_col: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut cp: col_pos = col_pos { col: 0, pos: 0 };
    cp.pos = spos;
    cp.col = -(1 as std::ffi::c_int);
    col_vs_pos(linepos, &mut cp, saved_pos, saved_col);
    return cp.col;
}
#[no_mangle]
pub unsafe extern "C" fn pos_from_col(
    mut linepos: POSITION,
    mut col: std::ffi::c_int,
    mut saved_pos: POSITION,
    mut saved_col: std::ffi::c_int,
) -> POSITION {
    let mut cp: col_pos = col_pos { col: 0, pos: 0 };
    cp.col = col + hshift - line_pfx_width();
    cp.pos = -(1 as std::ffi::c_int) as POSITION;
    col_vs_pos(linepos, &mut cp, saved_pos, saved_col);
    return cp.pos;
}
#[no_mangle]
pub unsafe extern "C" fn set_attr_line(mut a: std::ffi::c_int) {
    let mut i: size_t = 0;
    i = linebuf.print;
    while i < linebuf.end {
        if *(linebuf.attr).offset(i as isize)
            & (16 as std::ffi::c_int - 1 as std::ffi::c_int) << 8 as std::ffi::c_int
            == 0 as std::ffi::c_int
            || a & (16 as std::ffi::c_int - 1 as std::ffi::c_int) << 8 as std::ffi::c_int
                == 0 as std::ffi::c_int
        {
            *(linebuf.attr).offset(i as isize) |= a;
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_status_col(mut c: std::ffi::c_char, mut attr: std::ffi::c_int) {
    set_pfx(0 as std::ffi::c_int as size_t, c, attr);
}
#[no_mangle]
pub unsafe extern "C" fn gline(mut i: size_t, mut ap: *mut std::ffi::c_int) -> std::ffi::c_int {
    if is_null_line as u64 != 0 {
        if twiddle != 0 {
            if i == 0 as std::ffi::c_int as size_t {
                *ap = (1 as std::ffi::c_int) << 1 as std::ffi::c_int;
                return '~' as i32;
            }
            i = i.wrapping_sub(1);
        }
        *ap = 0 as std::ffi::c_int;
        return if i != 0 { '\0' as i32 } else { '\n' as i32 };
    }
    if i < linebuf.pfx_end {
        *ap = linebuf.pfx_attr[i as usize];
        return linebuf.pfx[i as usize] as std::ffi::c_int;
    }
    i = i.wrapping_add((linebuf.print).wrapping_sub(linebuf.pfx_end));
    *ap = *(linebuf.attr).offset(i as isize);
    return *(linebuf.buf).offset(i as isize) as std::ffi::c_int & 0xff as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn should_clear_after_line() -> lbool {
    return clear_after_line;
}
#[no_mangle]
pub unsafe extern "C" fn null_line() {
    is_null_line = LTRUE;
    cshift = 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn forw_raw_line_len(
    mut curr_pos: POSITION,
    mut read_len: size_t,
    mut linep: *mut *const std::ffi::c_char,
    mut line_lenp: *mut size_t,
) -> POSITION {
    let mut n: size_t = 0;
    let mut c: std::ffi::c_int = 0;
    let mut new_pos: POSITION = 0;
    if curr_pos == -(1 as std::ffi::c_int) as POSITION || ch_seek(curr_pos) != 0 || {
        c = ch_forw_get();
        c == -(1 as std::ffi::c_int)
    } {
        return -(1 as std::ffi::c_int) as POSITION;
    }
    n = 0 as std::ffi::c_int as size_t;
    loop {
        if c == '\n' as i32
            || c == -(1 as std::ffi::c_int)
            || sigs
                & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
                != 0
        {
            new_pos = ch_tell();
            break;
        } else {
            if n >= size_linebuf.wrapping_sub(1 as std::ffi::c_int as size_t) {
                if expand_linebuf() != 0 {
                    new_pos = ch_tell() - 1 as std::ffi::c_int as POSITION;
                    break;
                }
            }
            let fresh10 = n;
            n = n.wrapping_add(1);
            *(linebuf.buf).offset(fresh10 as isize) = c as std::ffi::c_char;
            if read_len != -(1 as std::ffi::c_int) as size_t
                && read_len > 0 as std::ffi::c_int as size_t
                && n >= read_len
            {
                new_pos = ch_tell();
                break;
            } else {
                c = ch_forw_get();
            }
        }
    }
    *(linebuf.buf).offset(n as isize) = '\0' as i32 as std::ffi::c_char;
    if !linep.is_null() {
        *linep = linebuf.buf;
    }
    if !line_lenp.is_null() {
        *line_lenp = n;
    }
    return new_pos;
}
#[no_mangle]
pub unsafe extern "C" fn forw_raw_line(
    mut curr_pos: POSITION,
    mut linep: *mut *const std::ffi::c_char,
    mut line_lenp: *mut size_t,
) -> POSITION {
    return forw_raw_line_len(
        curr_pos,
        -(1 as std::ffi::c_int) as size_t,
        linep,
        line_lenp,
    );
}
#[no_mangle]
pub unsafe extern "C" fn back_raw_line(
    mut curr_pos: POSITION,
    mut linep: *mut *const std::ffi::c_char,
    mut line_lenp: *mut size_t,
) -> POSITION {
    let mut n: size_t = 0;
    let mut c: std::ffi::c_int = 0;
    let mut new_pos: POSITION = 0;
    if curr_pos == -(1 as std::ffi::c_int) as POSITION
        || curr_pos <= 0 as std::ffi::c_int as POSITION
        || ch_seek(curr_pos - 1 as std::ffi::c_int as POSITION) != 0
    {
        return -(1 as std::ffi::c_int) as POSITION;
    }
    n = size_linebuf;
    n = n.wrapping_sub(1);
    *(linebuf.buf).offset(n as isize) = '\0' as i32 as std::ffi::c_char;
    loop {
        c = ch_back_get();
        if c == '\n' as i32
            || sigs
                & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
                != 0
        {
            new_pos = ch_tell() + 1 as std::ffi::c_int as POSITION;
            break;
        } else if c == -(1 as std::ffi::c_int) {
            new_pos = 0 as std::ffi::c_int as POSITION;
            break;
        } else {
            if n <= 0 as std::ffi::c_int as size_t {
                let mut old_size_linebuf: size_t = size_linebuf;
                let mut fm: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
                let mut to: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
                if expand_linebuf() != 0 {
                    new_pos = ch_tell() + 1 as std::ffi::c_int as POSITION;
                    break;
                } else {
                    fm = (linebuf.buf)
                        .offset(old_size_linebuf as isize)
                        .offset(-(1 as std::ffi::c_int as isize));
                    to = (linebuf.buf)
                        .offset(size_linebuf as isize)
                        .offset(-(1 as std::ffi::c_int as isize));
                    while fm >= linebuf.buf {
                        *to = *fm;
                        fm = fm.offset(-1);
                        to = to.offset(-1);
                    }
                    n = size_linebuf.wrapping_sub(old_size_linebuf);
                }
            }
            n = n.wrapping_sub(1);
            *(linebuf.buf).offset(n as isize) = c as std::ffi::c_char;
        }
    }
    if !linep.is_null() {
        *linep = &mut *(linebuf.buf).offset(n as isize) as *mut std::ffi::c_char;
    }
    if !line_lenp.is_null() {
        *line_lenp = size_linebuf
            .wrapping_sub(1 as std::ffi::c_int as size_t)
            .wrapping_sub(n);
    }
    return new_pos;
}
#[no_mangle]
pub unsafe extern "C" fn skip_columns(
    mut cols: std::ffi::c_int,
    mut linep: *mut *const std::ffi::c_char,
    mut line_lenp: *mut size_t,
) -> std::ffi::c_int {
    let mut line: *const std::ffi::c_char = *linep;
    let mut eline: *const std::ffi::c_char = line.offset(*line_lenp as isize);
    let mut pch: LWCHAR = 0 as std::ffi::c_int as LWCHAR;
    let mut bytes: size_t = 0;
    while cols > 0 as std::ffi::c_int && line < eline {
        let mut ch: LWCHAR = step_charc(&mut line, 1 as std::ffi::c_int, eline);
        let mut pansi: *mut ansi_state_0 = ansi_start(ch);
        if !pansi.is_null() {
            skip_ansi(pansi, ch, &mut line, eline);
            ansi_done(pansi);
            pch = 0 as std::ffi::c_int as LWCHAR;
        } else {
            let mut w: std::ffi::c_int =
                pwidth(ch, 0 as std::ffi::c_int, pch, 0 as std::ffi::c_int);
            cols -= w;
            pch = ch;
        }
    }
    bytes = line.offset_from(*linep) as std::ffi::c_long as size_t;
    *linep = line;
    *line_lenp = (*line_lenp).wrapping_sub(bytes);
    return bytes as std::ffi::c_int;
}
unsafe extern "C" fn pappstr(mut str: *const std::ffi::c_char) -> std::ffi::c_int {
    while *str as std::ffi::c_int != '\0' as i32 {
        let fresh11 = str;
        str = str.offset(1);
        if pappend(*fresh11, -(1 as std::ffi::c_int) as POSITION) != 0 {
            return 1 as std::ffi::c_int;
        }
    }
    return 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn load_line(mut str: *const std::ffi::c_char) {
    let mut save_hshift: std::ffi::c_int = hshift;
    hshift = 0 as std::ffi::c_int;
    set_line_contig_pos(-(1 as std::ffi::c_int) as POSITION);
    loop {
        prewind(LFALSE);
        if pappstr(str) == 0 as std::ffi::c_int {
            break;
        }
        hshift += 1 as std::ffi::c_int;
    }
    set_linebuf(
        linebuf.end,
        '\0' as i32 as std::ffi::c_char,
        0 as std::ffi::c_int,
    );
    linebuf.prev_end = 0 as std::ffi::c_int as size_t;
    if ansi_in_line as u64 == 0 {
        let mut i: size_t = 0;
        i = linebuf.print;
        while i < linebuf.end {
            set_linebuf(
                i,
                *(linebuf.buf).offset(i as isize),
                (1 as std::ffi::c_int) << 3 as std::ffi::c_int
                    | (7 as std::ffi::c_int) << 8 as std::ffi::c_int,
            );
            i = i.wrapping_add(1);
        }
    }
    hshift = save_hshift;
}
#[no_mangle]
pub unsafe extern "C" fn rrshift() -> std::ffi::c_int {
    let mut pos: POSITION = 0;
    let mut save_width: std::ffi::c_int = 0;
    let mut sline: std::ffi::c_int = 0;
    let mut longest: std::ffi::c_int = 0 as std::ffi::c_int;
    save_width = sc_width;
    sc_width = 2147483647 as std::ffi::c_int;
    sline = 0 as std::ffi::c_int;
    while sline < sc_height {
        pos = position(sline);
        if pos != -(1 as std::ffi::c_int) as POSITION {
            break;
        }
        sline += 1;
    }
    while sline < sc_height && pos != -(1 as std::ffi::c_int) as POSITION {
        pos = forw_line(pos, 0 as *mut POSITION, 0 as *mut lbool);
        if end_column > longest {
            longest = end_column;
        }
        sline += 1;
    }
    sc_width = save_width;
    if longest < sc_width {
        return 0 as std::ffi::c_int;
    }
    return longest - sc_width;
}
unsafe extern "C" fn lookup_color_index(mut attr: std::ffi::c_int) -> std::ffi::c_int {
    let mut cx: std::ffi::c_int = 0;
    cx = 0 as std::ffi::c_int;
    while cx
        < (::core::mem::size_of::<[color_map; 19]>() as std::ffi::c_ulong)
            .wrapping_div(::core::mem::size_of::<color_map>() as std::ffi::c_ulong)
            as std::ffi::c_int
    {
        if color_map[cx as usize].attr == attr {
            return cx;
        }
        cx += 1;
    }
    return -(1 as std::ffi::c_int);
}
unsafe extern "C" fn color_index(mut attr: std::ffi::c_int) -> std::ffi::c_int {
    if use_color != 0
        && attr & (16 as std::ffi::c_int - 1 as std::ffi::c_int) << 8 as std::ffi::c_int != 0
    {
        return lookup_color_index(
            attr & (16 as std::ffi::c_int - 1 as std::ffi::c_int) << 8 as std::ffi::c_int,
        );
    }
    if attr & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
        return lookup_color_index((1 as std::ffi::c_int) << 0 as std::ffi::c_int);
    }
    if attr & (1 as std::ffi::c_int) << 1 as std::ffi::c_int != 0 {
        return lookup_color_index((1 as std::ffi::c_int) << 1 as std::ffi::c_int);
    }
    if attr & (1 as std::ffi::c_int) << 2 as std::ffi::c_int != 0 {
        return lookup_color_index((1 as std::ffi::c_int) << 2 as std::ffi::c_int);
    }
    if attr & (1 as std::ffi::c_int) << 3 as std::ffi::c_int != 0 {
        return lookup_color_index((1 as std::ffi::c_int) << 3 as std::ffi::c_int);
    }
    return -(1 as std::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn set_color_map(
    mut attr: std::ffi::c_int,
    mut colorstr: *const std::ffi::c_char,
) -> std::ffi::c_int {
    let mut cx: std::ffi::c_int = color_index(attr);
    if cx < 0 as std::ffi::c_int {
        return -(1 as std::ffi::c_int);
    }
    if (strlen(colorstr)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong)
        > ::core::mem::size_of::<[std::ffi::c_char; 12]>() as std::ffi::c_ulong
    {
        return -(1 as std::ffi::c_int);
    }
    if *colorstr as std::ffi::c_int != '\0' as i32
        && parse_color(
            colorstr,
            0 as *mut std::ffi::c_int,
            0 as *mut std::ffi::c_int,
            0 as *mut CHAR_ATTR,
        ) as std::ffi::c_uint
            == CT_NULL as std::ffi::c_int as std::ffi::c_uint
    {
        return -(1 as std::ffi::c_int);
    }
    strcpy((color_map[cx as usize].color).as_mut_ptr(), colorstr);
    return 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_color_map(mut attr: std::ffi::c_int) -> *const std::ffi::c_char {
    let mut cx: std::ffi::c_int = color_index(attr);
    if cx < 0 as std::ffi::c_int {
        return 0 as *const std::ffi::c_char;
    }
    return (color_map[cx as usize].color).as_mut_ptr();
}
