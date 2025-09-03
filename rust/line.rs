use crate::charset::get_wchar;
use crate::charset::is_combining_char;
use crate::charset::is_composing_char;
use crate::charset::is_ubin_char;
use crate::charset::is_utf8_well_formed;
use crate::charset::is_wide_char;
use crate::charset::prchar;
use crate::charset::prutfchar;
use crate::charset::put_wchar;
use crate::charset::step_char;
use crate::charset::step_charc;
use crate::decode::isnullenv;
use crate::decode::lgetenv;
use crate::xbuf::XBuffer;
use std::ffi::CString;
use std::sync::LazyLock;
extern "C" {
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
    fn utf_len(ch: std::ffi::c_char) -> std::ffi::c_int;
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
    static mut ctldisp: i32;
    static mut twiddle: bool;
    static mut status_col: bool;
    static mut status_col_width: i32;
    static mut linenum_width: std::ffi::c_int;
    static mut auto_wrap: std::ffi::c_int;
    static mut ignaw: std::ffi::c_int;
    static mut bo_s_width: std::ffi::c_int;
    static mut bo_e_width: std::ffi::c_int;
    static mut ul_s_width: i32;
    static mut ul_e_width: std::ffi::c_int;
    static mut bl_s_width: std::ffi::c_int;
    static mut bl_e_width: std::ffi::c_int;
    static mut so_s_width: std::ffi::c_int;
    static mut so_e_width: std::ffi::c_int;
    static mut sc_width: std::ffi::c_int;
    static mut sc_height: std::ffi::c_int;
    static mut utf_mode: bool;
    static mut start_attnpos: POSITION;
    static mut end_attnpos: POSITION;
    static mut rscroll_char: char;
    static mut rscroll_attr: std::ffi::c_int;
    static mut use_color: i32;
    static mut status_line: bool;
}
pub type __off_t = i64;
pub type off_t = __off_t;
pub type size_t = std::ffi::c_ulong;
pub type lbool = std::ffi::c_uint;
pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;
pub type LWCHAR = std::ffi::c_ulong;
pub type less_off_t = off_t;
pub type POSITION = i32;
pub type LINENUM = off_t;

/* Parsing position in an OSC8 link: "\e]8;PARAMS;URI\e\\" (final "\e\\" may be "\7") */
#[derive(Copy, Clone)]
enum OSC8_STATE {
    OSC_START = 0, /* Waiting for initial \e */
    OSC_INTRO,     /* Waiting for intro char, usually ']' */
    OSC_TYPENUM,   /* Reading OS command type */
    OSC_STRING,    /* Reading OS command string */
    OSC_END_CSI,   /* Waiting for backslash after the final ESC. */
    OSC_END,       /* At end */

    OSC8_PARAMS, /* In the OSC8 parameters */
    OSC8_URI,    /* In the OSC8 URI */
    OSC8_NOT,    /* This is not an OSC8 link */
}

/* ANSI states */
#[derive(PartialEq)]
enum ANSI_STATE {
    ANSI_NULL,
    ANSI_MID,
    ANSI_ERR,
    ANSI_END,
}

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
pub const EOI: i32 = -1;

const S_INTERRUPT: i32 = 1 << 0;
const S_SWINTERRUPT: i32 = 1 << 1;
const S_STOP: i32 = 1 << 2;
const S_WINCH: i32 = 1 << 3;

const CSI: u8 = 0o233;
const ESC: u8 = b']' & 0o037;

/*
 * Include file for interfacing to position.c modules.
 */
const TOP: i32 = 0;
const TOP_PLUS_ONE: i32 = 1;
const BOTTOM: i32 = -1;
const BOTTOM_PLUS_ONE: i32 = -2;
const MIDDLE: i32 = -3;
const BOTTOM_OFFSET: i32 = -4;

/* How should we handle backspaces? */
const BS_SPECIAL: i32 = 0; /* Do special things for underlining and bold */
const BS_NORMAL: i32 = 1; /* \b treated as normal char; actually output */
const BS_CONTROL: i32 = 2; /* \b treated as control char; prints as ^H */

fn control(ch: u8) -> u8 {
    ch & 0o037
}

#[derive(Copy, Clone)]
pub struct AnsiState {
    pub ostate: OSC8_STATE,
    pub otype: i32,
    pub escs_in_seq: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColPos {
    pub col: i32,
    pub pos: POSITION,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color_map {
    pub attr: i32,
    pub color: &'static str,
}

const NUM_LAST_ANSIS: usize = 3;
const LINEBUF_SIZE: usize = 1024; /* Initial max size of line in input file */
const TABSTOP_MAX: usize = 128; /* Max number of custom tab stops */

const MAX_LINENUM_WIDTH: usize = 16; /* Max width of a line number */
const MAX_STATUSCOL_WIDTH: usize = 4; /* Max width of the status column */
const MAX_PFX_WIDTH: usize = MAX_LINENUM_WIDTH + MAX_STATUSCOL_WIDTH + 1;

const OPT_OFF: i32 = 0;
const OPT_ON: i32 = 0;
const OPT_ONPLUS: i32 = 0;

const CHAR_BIT: usize = 8;
/*
 * Upper bound on the string length of an integer converted to string.
 * 302 / 1000 is ceil (log10 (2.0)).  Subtract 1 for the sign bit;
 * add 1 for integer division truncation; add 1 more for a minus sign.
 */
const fn int_strlen_bound<T>() -> usize
where
    T: Sized,
{
    (std::mem::size_of::<T>() * CHAR_BIT - 1) * 302 / 1000 + 1 + 1
}

struct LineBuf {
    buf: Vec<u8>,             /* Buffer which holds the current output line */
    attr: Vec<i32>,           /* Parallel to buf, to hold attributes */
    print: usize,             /* Index in buf of first printable char */
    end: usize,               /* Number of chars in buf */
    prev_end: usize,          /* Number of chars in buf for previous line */
    pfx: [u8; MAX_PFX_WIDTH], /* Holds status column and line number */
    pfx_attr: [i32; MAX_PFX_WIDTH],
    pfx_end: usize, /* Number of chars in pfx */
}

impl LineBuf {
    fn new() -> Self {
        LineBuf {
            buf: Vec::new(),
            attr: Vec::new(),
            print: 0,
            end: 0,
            prev_end: 0,
            pfx: [0; MAX_PFX_WIDTH],
            pfx_attr: [0; MAX_PFX_WIDTH],
            pfx_end: 0,
        }
    }
}

static mut linebuf: LineBuf = LineBuf {
    buf: Vec::new(),
    attr: Vec::new(),
    print: 0,
    end: 0,
    prev_end: 0,
    pfx: [0; MAX_PFX_WIDTH],
    pfx_attr: [0; MAX_PFX_WIDTH],
    pfx_end: 0,
};

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

static mut size_linebuf: usize = 0; /* Size of line buffer (and attr buffer) */
static mut line_ansi: Option<AnsiState> = None;
static mut ansi_in_line: bool = false;
static mut ff_starts_line: i32 = 0;
static mut hlink_in_line: bool = false;
static mut line_mark_attr: i32 = 0;
static mut cshift: i32 = 0; /* Current left-shift of output line buffer */
pub static mut hshift: i32 = 0; /* Desired left-shift of output line buffer */
pub static tabstops: [i32; TABSTOP_MAX] = [0; TABSTOP_MAX]; /* Custom tabstops */
pub static ntabstops: i32 = 1; /* Number of tabstops */
pub static tabdefault: i32 = 8; /* Default repeated tabstops */
pub static mut highest_hilite: POSITION = 0; /* Pos of last hilite in file found so far */
static mut line_pos: POSITION = 0;
static mut line_contig_pos: POSITION = NULL_POSITION; /* One after last byte processed */

static mut curr_last_ansi: i32 = 0;

static mut end_column: i32 = 0; /* Printable length, accounting for backspaces, etc. */
static mut right_curr: i32 = 0;
static mut right_column: std::ffi::c_int = 0;
static mut overstrike: i32 = 0; /* Next char should overstrike previous char */
static mut last_overstrike: std::ffi::c_int = 0 as std::ffi::c_int;
static mut is_null_line: bool = false; /* There is no current line */
static mut pendc: char = '\0';
static mut pendpos: POSITION = 0;
static mut end_ansi_chars: String = String::new();
static mut mid_ansi_chars: String = String::new();
static mut osc_ansi_chars: String = String::new();
static mut osc_ansi_allow_count: i32 = 0;
static mut osc_ansi_allow: *mut std::ffi::c_long =
    0 as *const std::ffi::c_long as *mut std::ffi::c_long;
static mut in_hilite: bool = false;
static mut clear_after_line: bool = false;
static mut mbc_buf: [u8; MAX_UTF_CHAR_LEN] = [0; MAX_UTF_CHAR_LEN];
static mut mbc_buf_len: std::ffi::c_int = 0 as std::ffi::c_int;
static mut mbc_buf_index: std::ffi::c_int = 0 as std::ffi::c_int;
static mut mbc_pos: POSITION = 0;
static mut saved_line_end: usize = 0;
static mut saved_end_column: i32 = 0;

const NULL_POSITION: POSITION = -1;

/* Special char bit-flags used to tell put_line() to do something special */
const AT_NORMAL: i32 = 0;
const AT_UNDERLINE: i32 = 1 << 0;
const AT_BOLD: i32 = 1 << 1;
const AT_BLINK: i32 = 1 << 2;
const AT_STANDOUT: i32 = 1 << 3;
const AT_ANSI: i32 = 1 << 4; /* Content-supplied "ANSI" escape sequence */
const AT_BINARY: i32 = 1 << 5; /* LESS*BINFMT representation */
const AT_HILITE: i32 = 1 << 6; /* Internal highlights (e.g., for search) */

const AT_COLOR_SHIFT: i32 = 8;
const AT_NUM_COLORS: i32 = 16;
const AT_COLOR: i32 = (AT_NUM_COLORS - 1) << AT_COLOR_SHIFT;
const AT_COLOR_ATTN: i32 = 1 << AT_COLOR_SHIFT;
const AT_COLOR_BIN: i32 = 2 << AT_COLOR_SHIFT;
const AT_COLOR_CTRL: i32 = 3 << AT_COLOR_SHIFT;
const AT_COLOR_ERROR: i32 = 4 << AT_COLOR_SHIFT;
const AT_COLOR_LINENUM: i32 = 5 << AT_COLOR_SHIFT;
const AT_COLOR_MARK: i32 = 6 << AT_COLOR_SHIFT;
const AT_COLOR_PROMPT: i32 = 7 << AT_COLOR_SHIFT;
const AT_COLOR_RSCROLL: i32 = 8 << AT_COLOR_SHIFT;
const AT_COLOR_HEADER: i32 = 9 << AT_COLOR_SHIFT;
const AT_COLOR_SEARCH: i32 = 10 << AT_COLOR_SHIFT;

const fn at_color_subsearch(i: i32) -> i32 {
    (10 + i) << AT_COLOR_SHIFT
}
const NUM_SEARCH_COLORS: i32 = AT_NUM_COLORS - 10 - 1;
const MAX_UTF_CHAR_LEN: usize = 6;

static mut color_map: LazyLock<[color_map; 19]> = LazyLock::new(|| {
    [
        color_map {
            attr: AT_UNDERLINE,
            color: "\0\0\0\0\0\0\0\0\0\0\0\0",
        },
        color_map {
            attr: AT_BOLD,
            color: "\0\0\0\0\0\0\0\0\0\0\0\0",
        },
        color_map {
            attr: AT_BLINK,
            color: "\0\0\0\0\0\0\0\0\0\0\0\0",
        },
        color_map {
            attr: AT_STANDOUT,
            color: "\0\0\0\0\0\0\0\0\0\0\0\0",
        },
        color_map {
            attr: AT_COLOR_ATTN,
            color: "Wm\0\0\0\0\0\0\0\0\0\0",
        },
        color_map {
            attr: AT_COLOR_BIN,
            color: "kR\0\0\0\0\0\0\0\0\0\0",
        },
        color_map {
            attr: AT_COLOR_CTRL,
            color: "kR\0\0\0\0\0\0\0\0\0\0",
        },
        color_map {
            attr: AT_COLOR_ERROR,
            color: "kY\0\0\0\0\0\0\0\0\0\0",
        },
        color_map {
            attr: AT_COLOR_LINENUM,
            color: "c\0\0\0\0\0\0\0\0\0\0\0",
        },
        color_map {
            attr: AT_COLOR_MARK,
            color: "Wb\0\0\0\0\0\0\0\0\0\0",
        },
        color_map {
            attr: AT_COLOR_PROMPT,
            color: "kC\0\0\0\0\0\0\0\0\0\0",
        },
        color_map {
            attr: AT_COLOR_RSCROLL,
            color: "kc\0\0\0\0\0\0\0\0\0\0",
        },
        color_map {
            attr: AT_COLOR_HEADER,
            color: "\0\0\0\0\0\0\0\0\0\0\0\0",
        },
        color_map {
            attr: AT_COLOR_SEARCH,
            color: "kG\0\0\0\0\0\0\0\0\0\0",
        },
        color_map {
            attr: at_color_subsearch(1),
            color: "ky\0\0\0\0\0\0\0\0\0\0",
        },
        color_map {
            attr: at_color_subsearch(2),
            color: "wb\0\0\0\0\0\0\0\0\0\0",
        },
        color_map {
            attr: at_color_subsearch(3),
            color: "YM\0\0\0\0\0\0\0\0\0\0",
        },
        color_map {
            attr: at_color_subsearch(4),
            color: "Yr\0\0\0\0\0\0\0\0\0\0",
        },
        color_map {
            attr: at_color_subsearch(5),
            color: "Wc\0\0\0\0\0\0\0\0\0\0",
        },
    ]
});

/*
 * Initialize from environment variables.
 */
pub unsafe extern "C" fn init_line() {
    let eac = lgetenv("LESSANSIENDCHARS");
    if isnullenv(&eac) {
        end_ansi_chars = "m".into();
    } else {
        end_ansi_chars = eac.unwrap();
    }

    let mac = lgetenv("LESSANSIMIDCHARS");
    if isnullenv(&mac) {
        mid_ansi_chars = "0123456789:;[?!\"'#%()*+ ".into();
    } else {
        mid_ansi_chars = mac.unwrap();
    }

    let iac = lgetenv("LESSANSIOSCCHARS");
    if isnullenv(&iac) {
        osc_ansi_chars = "".into();
    } else {
        osc_ansi_chars = iac.unwrap();
    }

    osc_ansi_allow_count = 0;
    let s = lgetenv("LESSANSIOSCALLOW");
    if !isnullenv(&s) {
        let mut xbuf = XBuffer::new(16);
        //let sp = s.unwrap().split(',');
        for ss in s.unwrap().split(',') {
            if let Ok(num) = ss.trim().parse::<i32>() {
                xbuf.add_data(&num.to_ne_bytes(), size_of::<i32>());
                osc_ansi_allow_count += 1;
            } else {
                break;
            }
        }
        osc_ansi_allow = xbuf.data.as_ptr() as *mut std::ffi::c_long;
    }

    size_linebuf = LINEBUF_SIZE;
    linebuf.buf = Vec::with_capacity(LINEBUF_SIZE);
    linebuf.attr = Vec::with_capacity(LINEBUF_SIZE);
    linebuf.buf.resize(LINEBUF_SIZE, 0);
    linebuf.attr.resize(LINEBUF_SIZE, 0);
    curr_last_ansi = 0;
}

/*
 * Expand the line buffer.
 */
unsafe fn expand_linebuf() {
    /* Double the size of the line buffer. */
    let new_size = size_linebuf * 2;
    linebuf.buf.resize(new_size, 0);
    linebuf.attr.resize(new_size, 0);
    size_linebuf = new_size;
}

/*
 * Is a character ASCII?
 */
pub unsafe extern "C" fn is_ascii_char(mut ch: LWCHAR) -> lbool {
    return (ch <= 0x7f as std::ffi::c_int as LWCHAR) as std::ffi::c_int as lbool;
}

unsafe extern "C" fn inc_end_column(mut w: i32) {
    if end_column > right_column && w > 0 {
        right_column = end_column;
        right_curr = linebuf.end as i32;
    }
    end_column += w;
}

pub unsafe extern "C" fn line_position() -> POSITION {
    line_pos
}

/*
 * Is this byte the next one after the previous byte processed?
 */
pub unsafe extern "C" fn is_line_contig_pos(mut pos: POSITION) -> bool {
    pos == line_contig_pos
}

/*
 * Set the position of the next byte to be processed.
 */
pub unsafe extern "C" fn set_line_contig_pos(mut pos: POSITION) {
    line_contig_pos = pos;
}

/*
 * Copy any ANSI sequences from line buffer to shifted_ansi.
 */
unsafe extern "C" fn pshift(mut end: usize) {
    for i in linebuf.print..end {
        if linebuf.attr[i] == AT_ANSI {
            shifted_ansi.xbuf_add_char(linebuf.buf[i] as i8);
        }
    }
}

/*
 * Rewind the line buffer.
 */
pub unsafe extern "C" fn prewind(mut contig: bool) {
    shifted_ansi.reset();
    if contig && linebuf.prev_end != 0 {
        pshift(linebuf.prev_end);
    }
    linebuf.print = 6; /* big enough for longest UTF-8 sequence */
    linebuf.pfx_end = 0;
    linebuf.end = 0;
    while linebuf.end < linebuf.print {
        linebuf.buf[linebuf.end] = b'\0';
        linebuf.attr[linebuf.end] = 0;
        linebuf.end = (linebuf.end).wrapping_add(1);
    }

    end_column = 0;
    right_curr = 0;
    right_column = 0;
    cshift = 0;
    overstrike = 0;
    last_overstrike;
    mbc_buf_len = 0;
    is_null_line = false;
    pendc = '\0';
    in_hilite = false;
    ansi_in_line = false;
    ff_starts_line = -1;
    hlink_in_line = false;
    clear_after_line = false;
    line_mark_attr = 0;
    line_pos = NULL_POSITION;
    last_ansi.reset();
    for ax in 0..NUM_LAST_ANSIS {
        last_ansis[ax as usize].reset();
    }
    curr_last_ansi = 0;
}

/*
 * Set a character in the line buffer.
 */
unsafe extern "C" fn set_linebuf(n: usize, ch: u8, attr: i32) {
    if n >= size_linebuf {
        /*
         * Won't fit in line buffer.
         * Try to expand it.
         */
        expand_linebuf();
    }
    linebuf.buf[n] = ch;
    linebuf.attr[n] = attr;
}

/*
 * Append a character to the line buffer.
 */
unsafe extern "C" fn add_linebuf(ch: u8, attr: i32, w: i32) {
    set_linebuf(linebuf.end, ch, attr);
    linebuf.end = (linebuf.end).wrapping_add(1);
    inc_end_column(w);
}

/*
 * Append a string to the line buffer.
 */
unsafe extern "C" fn addstr_linebuf(s: &[u8], attr: i32, cw: i32) {
    let mut i = 0;
    while s[i] != b'\0' {
        add_linebuf(s[i], attr, cw);
        i += 1;
    }
}

/*
 * Set a character in the line prefix buffer.
 */
unsafe extern "C" fn set_pfx(n: usize, ch: u8, attr: i32) {
    linebuf.pfx[n] = ch;
    linebuf.pfx_attr[n] = attr;
}

/*
 * Append a character to the line prefix buffer.
 */
unsafe extern "C" fn add_pfx(ch: u8, attr: i32) {
    set_pfx(linebuf.pfx_end, ch, attr);
    linebuf.pfx_end = (linebuf.pfx_end).wrapping_add(1);
}

/*
 * Insert the status column and line number into the line buffer.
 */
pub unsafe extern "C" fn plinestart(pos: POSITION) {
    let mut linenum: LINENUM = 0;
    if linenums == OPT_ONPLUS {
        /*
         * Get the line number and put it in the current line.
         * {{ Note: since find_linenum calls forw_raw_line,
         *    it may seek in the input file, requiring the caller
         *    of plinestart to re-seek if necessary. }}
         * {{ Since forw_raw_line modifies linebuf, we must
         *    do this first, before storing anything in linebuf. }}
         */
        linenum = find_linenum(pos);
    }
    /*
     * Display a status column if the -J option is set.
     */
    if status_col || status_line {
        let mut c = posmark(pos);
        if c != 0 {
            line_mark_attr = AT_HILITE | AT_COLOR_MARK;
        } else if start_attnpos != NULL_POSITION && pos >= start_attnpos && pos <= end_attnpos {
            line_mark_attr = AT_HILITE | AT_COLOR_ATTN;
        }
        if status_col {
            add_pfx(
                if c != 0 { c as u8 } else { b' ' }, /* column 0: status */
                line_mark_attr,
            );
            while linebuf.pfx_end < status_col_width as usize {
                add_pfx(b' ', 0);
            }
        }
    }
    /*
     * Display the line number at the start of each line
     * if the -N option is set.
     */
    if linenums == OPT_ONPLUS {
        let mut buf = String::new();
        let mut len = 0;
        let mut i = 0;
        linenum = vlinenum(linenum);
        if linenum == 0 {
            len = 0;
        } else {
            buf = linenum.to_string();
            len = buf.len();
        }
        for _ in 0..linenum_width {
            add_pfx(b' ', AT_NORMAL);
        }
        for i in 0..len {
            add_pfx(buf.as_bytes()[i], AT_BOLD | AT_COLOR_LINENUM);
        }
        add_pfx(b' ', AT_NORMAL);
    }
    end_column = linebuf.pfx_end as i32;
}

/*
 * Return the width of the line prefix (status column and line number).
 * {{ Actual line number can be wider than linenum_width. }}
 */
pub unsafe extern "C" fn line_pfx_width() -> i32 {
    let mut width = 0;
    if status_col {
        width += status_col_width;
    }
    if linenums == OPT_ONPLUS {
        width += linenum_width + 1;
    }
    width
}

/*
 * Shift line left so that the last char is just to the left
 * of the first visible column.
 */
pub unsafe extern "C" fn pshift_all() {
    pshift(linebuf.end);
    linebuf.end = linebuf.print;
    end_column = linebuf.pfx_end as i32;
    line_pos = NULL_POSITION;
}

/*
 * Return the printing width of the start (enter) sequence
 * for a given character attribute.
 */
unsafe extern "C" fn attr_swidth(a: i32) -> i32 {
    let mut w = 0;

    let a = apply_at_specials(a) as i32;

    if a & AT_UNDERLINE != 0 {
        w += ul_s_width;
    }
    if a & AT_BOLD != 0 {
        w += bo_s_width;
    }
    if a & AT_BLINK != 0 {
        w += bl_s_width;
    }
    if a & AT_STANDOUT != 0 {
        w += so_s_width;
    }
    w
}

/*
 * Return the printing width of the end (exit) sequence
 * for a given character attribute.
 */
unsafe extern "C" fn attr_ewidth(a: i32) -> i32 {
    let mut w = 0;

    let a = apply_at_specials(a);

    if a & AT_UNDERLINE != 0 {
        w += ul_e_width;
    }
    if a & AT_BOLD != 0 {
        w += bo_e_width;
    }
    if a & AT_BLINK != 0 {
        w += bl_e_width;
    }
    if a & AT_STANDOUT != 0 {
        w += so_e_width;
    }
    w
}

/*
 * Return the printing width of a given character and attribute,
 * if the character were added after prev_ch.
 * Adding a character with a given attribute may cause an enter or exit
 * attribute sequence to be inserted, so this must be taken into account.
 */
pub unsafe extern "C" fn pwidth(ch: char, a: i32, prev_ch: char, prev_a: i32) -> i32 {
    let mut w = 0;
    if ch == '\x08' {
        /*
         * Backspace moves backwards one or two positions.
         */
        if prev_a & (AT_ANSI | AT_BINARY) != 0 {
            return prchar('\x08').len() as i32;
        }
        return if utf_mode && is_wide_char(prev_ch) {
            -2
        } else {
            -1
        };
    }
    if !utf_mode || ch.is_ascii() {
        if ch.is_control() {
            /*
             * Control characters do unpredictable things,
             * so we don't even try to guess; say it doesn't move.
             * This can only happen if the -r flag is in effect.
             */
            return 0;
        }
    } else if is_composing_char(ch) || is_combining_char(prev_ch, ch) {
        /*
         * Composing and combining chars take up no space.
         *
         * Some terminals, upon failure to compose a
         * composing character with the character(s) that
         * precede(s) it will actually take up one end_column
         * for the composing character; there isn't much
         * we could do short of testing the (complex)
         * composition process ourselves and printing
         * a binary representation when it fails.
         */
        return 0;
    }
    /*
     * Other characters take one or two columns,
     * plus the width of any attribute enter/exit sequence.
     */
    w = 1;
    if is_wide_char(ch) {
        w += 1;
    }
    if linebuf.end > 0 && is_at_equiv(linebuf.attr[linebuf.end - 1], a) == 0 {
        w += attr_ewidth(linebuf.attr[linebuf.end - 1]);
    }
    if apply_at_specials(a) != AT_NORMAL
        && (linebuf.end == 0 || is_at_equiv(linebuf.attr[linebuf.end - 1], a) == 0)
    {
        w += attr_swidth(a);
    }
    return w;
}

/*
 * Delete to the previous base character in the line buffer.
 */
unsafe extern "C" fn backc() -> i32 {
    if linebuf.end == 0 {
        return 0;
    }
    let p = linebuf.end;
    let (mut ch, idx) = step_char(&linebuf.buf, -1, linebuf.end, 0);
    /* Skip back to the next nonzero-width char. */
    while p > 0 {
        linebuf.end = p;
        let (prev_ch, idx) = step_char(&linebuf.buf, -1, linebuf.end, 0);

        // Prevent out-of-bounds indexing into attr
        if linebuf.end == 0 || linebuf.end >= linebuf.attr.len() {
            break;
        }

        let width = pwidth(
            char::from_u32(ch as u32).unwrap(),
            linebuf.attr[linebuf.end],
            char::from_u32(prev_ch as u32).unwrap(),
            linebuf.attr[linebuf.end - 1],
        );
        end_column -= width;
        if width > 0 {
            break;
        }
        ch = prev_ch;
    }
    1
}

/*
 * Preserve the current position in the line buffer (for word wrapping).
 */
pub unsafe extern "C" fn savec() {
    saved_line_end = linebuf.end;
    saved_end_column = end_column;
}

/*
 * Restore the position in the line buffer (start of line for word wrapping).
 */
pub unsafe extern "C" fn loadc() {
    linebuf.end = saved_line_end;
    end_column = saved_end_column;
}

/*
 * Is a character the end of an ANSI escape sequence?
 */
pub unsafe extern "C" fn is_ansi_end(ch: char) -> bool {
    if !ch.is_ascii() {
        return false;
    }
    ch as i32 != 0 && end_ansi_chars.find(ch).is_some()
}

/*
 * Can a char appear in an ANSI escape sequence, before the end char?
 */
pub unsafe extern "C" fn is_ansi_middle(ch: char) -> bool {
    if !ch.is_ascii() {
        return false;
    }
    if is_ansi_end(ch) as u64 != 0 {
        return false;
    }
    ch as i32 != 0 && mid_ansi_chars.find(ch).is_some()
}

/*
 * Skip past an ANSI escape sequence.
 * pp is initially positioned just after the CSI_START char.
 */
pub unsafe extern "C" fn skip_ansi(
    pansi: &mut AnsiState,
    ch: char,
    s: &[u8],
    limit: usize,
) -> usize {
    ansi_step(pansi, ch);
    loop {
        let (ch, idx) = step_charc(s, 1, 0, limit);
        if !(idx < limit && ansi_step(pansi, ch) == ANSI_STATE::ANSI_MID) {
            return idx;
        }
    }
    /* Note that we discard final char, for which is_ansi_end is true. */
}

/*
 * Determine if a character starts an ANSI escape sequence.
 * If so, return an ansi_state struct; otherwise return NULL.
 */
pub unsafe extern "C" fn ansi_start(ch: char) -> Option<AnsiState> {
    if !(ch.is_control() && (ch as i32 == ('[' as i32 & 0o37) || ch as i32 == CSI as i32)) {
        return None;
    }
    Some(AnsiState {
        ostate: OSC8_STATE::OSC_START,
        otype: 0,
        escs_in_seq: 0,
    })
}

/*
 * Is a character a valid intro char for an OSC sequence?
 * An intro char is the one immediately after the ESC, usually ']'.
 */
unsafe extern "C" fn valid_osc_intro(ch: u8, content: bool) -> bool {
    if let Some(i) = osc_ansi_chars.find(ch as char) {
        return !content || osc_ansi_chars[i..].chars().next().unwrap() == '*';
    }
    false
}

/*
 * Is a given number a valid OSC type?
 */
unsafe extern "C" fn valid_osc_type(otype: i32, content: bool) -> bool {
    if !content {
        return true;
    }
    if otype == 8 {
        return true;
    }
    for i in 0..osc_ansi_allow_count {
        if *osc_ansi_allow.offset(i as isize) == otype as std::ffi::c_long {
            return true;
        }
    }
    false
}

/*
 * Helper function for ansi_step.
 */
unsafe extern "C" fn osc_return(
    pansi: &mut AnsiState,
    ostate: OSC8_STATE,
    astate: ANSI_STATE,
) -> ANSI_STATE {
    pansi.ostate = ostate;
    return astate;
}

fn is_csi_start(ch: char) -> bool {
    ch.is_control() && (ch == ESC as char || ch == CSI as char)
}

/*
 * Determine whether the next char in an ANSI escape sequence
 * ends the sequence.
 */
unsafe extern "C" fn ansi_step2(pansi: &mut AnsiState, ch: char, content: bool) -> ANSI_STATE {
    /*
     * Pass thru OS commands. Assume OSC commands do not move the cursor.
     * A "typed" OSC starts with ESC ] <integer> <semicolon>, followed by an
     * arbitrary string, and ends with a String Terminator (ESC-backslash or BEL).
     * An untyped OSC starts with ESC ] or ESC x where x is in osc_ansi_chars,
     * and ends with ST.
     * The only typed OSC we actually parse is OSC 8.
     */
    let mut current_block_33: u64;
    match pansi.ostate {
        OSC8_STATE::OSC_START => {
            if is_csi_start(ch) {
                return osc_return(pansi, OSC8_STATE::OSC_INTRO, ANSI_STATE::ANSI_MID);
            }
        }
        OSC8_STATE::OSC_INTRO => {
            if ch == ']' {
                return osc_return(pansi, OSC8_STATE::OSC_TYPENUM, ANSI_STATE::ANSI_MID);
            }
            if ch.is_ascii() && valid_osc_intro(ch as u8, content) {
                return osc_return(pansi, OSC8_STATE::OSC_STRING, ANSI_STATE::ANSI_MID);
            }
            if is_csi_start(ch) {
                return osc_return(pansi, OSC8_STATE::OSC_INTRO, ANSI_STATE::ANSI_MID);
            }
            /* ESC not followed by bracket; restart. */
            pansi.ostate = OSC8_STATE::OSC_START;
        }
        OSC8_STATE::OSC_TYPENUM => {
            if ch >= '0' && ch <= '9' {
                let (mulv, mul_oflow) = pansi.otype.overflowing_mul(10);
                pansi.otype = mulv;
                if !mul_oflow || {
                    let (addv, add_oflow) = pansi.otype.overflowing_add((ch as u8 - b'0') as i32);
                    pansi.otype = addv;
                    !add_oflow
                } {
                    return osc_return(pansi, OSC8_STATE::OSC_STRING, ANSI_STATE::ANSI_MID);
                }
                return osc_return(pansi, OSC8_STATE::OSC_TYPENUM, ANSI_STATE::ANSI_MID);
            }
            if ch == ';' {
                return osc_return(
                    pansi,
                    if pansi.otype == 8 {
                        OSC8_STATE::OSC8_PARAMS
                    } else {
                        OSC8_STATE::OSC_STRING
                    },
                    ANSI_STATE::ANSI_MID,
                );
            }
            /* OSC is untyped */
            if is_csi_start(ch) {
                return osc_return(pansi, OSC8_STATE::OSC_END_CSI, ANSI_STATE::ANSI_MID);
            }
            if ch == '7' {
                return osc_return(pansi, OSC8_STATE::OSC_END, ANSI_STATE::ANSI_END);
            }
            return osc_return(pansi, OSC8_STATE::OSC_STRING, ANSI_STATE::ANSI_MID);
        }
        OSC8_STATE::OSC8_PARAMS => {
            if ch == ';' {
                return osc_return(pansi, OSC8_STATE::OSC8_URI, ANSI_STATE::ANSI_MID);
            }
        }
        OSC8_STATE::OSC8_URI | OSC8_STATE::OSC_STRING => {
            /* Look for ST. */
            if ch == '7' {
                return osc_return(
                    pansi,
                    OSC8_STATE::OSC_END,
                    if valid_osc_type(pansi.otype, content) {
                        ANSI_STATE::ANSI_END
                    } else {
                        ANSI_STATE::ANSI_ERR
                    },
                );
            }
            if is_csi_start(ch) {
                pansi.escs_in_seq += 1;
                return osc_return(pansi, OSC8_STATE::OSC_END_CSI, ANSI_STATE::ANSI_MID);
            }
            /* Stay in same ostate */
            return ANSI_STATE::ANSI_MID;
        }
        OSC8_STATE::OSC_END_CSI => {
            /* Got ESC of ST, expect backslash next. */
            if ch == '\\' {
                return osc_return(
                    pansi,
                    OSC8_STATE::OSC_END,
                    (if valid_osc_type(pansi.otype, content) {
                        ANSI_STATE::ANSI_END
                    } else {
                        ANSI_STATE::ANSI_ERR
                    }),
                );
            }
            /* ESC not followed by backslash. */
            return osc_return(pansi, OSC8_STATE::OSC_STRING, ANSI_STATE::ANSI_MID);
        }
        OSC8_STATE::OSC_END => return ANSI_STATE::ANSI_END,
        OSC8_STATE::OSC8_NOT | _ => {
            /* cannot happen */
            panic!();
        }
    }
    /* Check for SGR sequences */
    if is_ansi_middle(ch) {
        return ANSI_STATE::ANSI_MID;
    }
    if is_ansi_end(ch) {
        return ANSI_STATE::ANSI_END;
    }
    ANSI_STATE::ANSI_ERR
}

pub unsafe extern "C" fn ansi_step(pansi: &mut AnsiState, ch: char) -> ANSI_STATE {
    ansi_step2(pansi, ch, true)
}

/*
 * Return the current OSC8 parsing state.
 */
pub unsafe extern "C" fn ansi_osc8_state(pansi: &AnsiState) -> OSC8_STATE {
    pansi.ostate
}

/*
 * Will w characters in attribute a fit on the screen?
 */
unsafe extern "C" fn fits_on_screen(mut w: i32, mut a: i32) -> bool {
    if ctldisp == OPT_ON {
        /* We're not counting, so say that everything fits. */
        return true;
    }
    end_column - cshift + w + attr_ewidth(a) <= sc_width
}

/*
 * Append a character and attribute to the line buffer.
 */
unsafe extern "C" fn store_char(ch: char, a: i32, rep: Option<&[u8]>, pos: POSITION) -> i32 {
    let mut w = 0;
    let mut i = 0;
    let mut replen = 0;
    let mut cs: [u8; 1] = [b'\0'; 1];
    let mut ov = 0;
    let mut rep = rep;
    let mut a = a;

    ov = a & (AT_UNDERLINE | AT_BOLD);
    if ov != AT_NORMAL {
        last_overstrike = ov;
    }
    let mut matches = 0;
    let mut resend_last = false;
    let mut hl_attr = 0;
    if pos != NULL_POSITION && a != AT_ANSI {
        hl_attr = is_hilited_attr(pos, pos + 1, 0, &mut matches);
        if hl_attr == 0 && status_line {
            hl_attr = line_mark_attr;
        }
    }
    if hl_attr != 0 {
        /*
         * This character should be highlighted.
         * Override the attribute passed in.
         */
        a |= hl_attr;
        if highest_hilite != NULL_POSITION && pos != NULL_POSITION && pos > highest_hilite {
            highest_hilite = pos;
        }
        in_hilite = true;
    } else {
        if in_hilite {
            /*
             * This is the first non-hilited char after a hilite.
             * Resend the last ANSI seq to restore color.
             */
            resend_last = true;
        }
        in_hilite = false;
    }
    if resend_last {
        for ai in 0..NUM_LAST_ANSIS {
            let ax = (curr_last_ansi as usize + ai) % NUM_LAST_ANSIS;
            for i in 0..last_ansis[ax].data.len() {
                store_char(last_ansis[ax].data[i] as char, AT_ANSI, None, pos);
            }
        }
    }
    if a == AT_ANSI {
        w = 0;
    } else {
        let (prev_ch, _) = if linebuf.end > 0 {
            step_char(&linebuf.buf, -1, linebuf.end, 0)
        } else {
            ('\0', 0)
        };
        let prev_a = if linebuf.end > 0 {
            linebuf.attr[linebuf.end - 1]
        } else {
            0
        };
        w = pwidth(ch, a, char::from_u32(prev_ch as u32).unwrap(), prev_a);
    }
    if !fits_on_screen(w, a) {
        return 1;
    }
    if rep.is_none() {
        cs[0] = ch as u8;
        rep = Some(&cs);
        replen = 1;
    } else {
        replen = utf_len(rep.unwrap()[0] as i8);
    }
    if cshift == hshift {
        if line_pos == NULL_POSITION {
            line_pos = pos;
        }
        if shifted_ansi.data.len() > 0 {
            /* Copy shifted ANSI sequences to beginning of line. */
            for i in 0..shifted_ansi.data.len() {
                add_linebuf(shifted_ansi.data[i], AT_ANSI, 0);
            }
            shifted_ansi.reset();
        }
    }

    /* Add the char to the buf, even if we will left-shift it next. */
    inc_end_column(w);
    for i in 0..replen {
        add_linebuf(rep.unwrap()[i as usize], a, 0);
    }
    if cshift < hshift {
        /* We haven't left-shifted enough yet. */
        if a == AT_ANSI {
            shifted_ansi.xbuf_add_char(ch as i8); /* Save ANSI attributes */
        }
        if linebuf.end > linebuf.print {
            /* Shift left enough to put last byte of this char at print-1. */
            for i in 0..linebuf.print {
                linebuf.buf[i] = linebuf.buf[i + replen as usize];
                linebuf.attr[i] = linebuf.attr[i + replen as usize];
            }
            linebuf.end -= replen as usize;
            cshift += w;
            /*
             * If the char we just left-shifted was double width,
             * the 2 spaces we shifted may be too much.
             * Represent the "half char" at start of line with a highlighted space.
             */
            while cshift > hshift {
                add_linebuf(b' ', rscroll_attr, 0);
                cshift -= 1;
            }
        }
    }
    0
}

unsafe extern "C" fn store_string(s: &[u8], a: i32, pos: POSITION) -> i32 {
    if !fits_on_screen(s.len() as i32, a) {
        return 1;
    }
    let mut i = 0;
    while s[i] != 0 {
        if store_char(s[i] as char, a, None, pos) != 0 {
            return 1;
        }
        i += 1;
    }
    0
}

/*
 * Return number of spaces from col to the next tab stop.
 */
unsafe extern "C" fn tab_spaces(col: i32) -> i32 {
    let mut to_tab = col - linebuf.pfx_end as i32;
    if ntabstops < 2 || to_tab >= tabstops[ntabstops as usize - 1] {
        to_tab = tabdefault - (to_tab - tabstops[ntabstops as usize - 1]) % tabdefault;
    } else {
        let mut i = (ntabstops - 2) as usize;
        while i >= 0 {
            if to_tab >= tabstops[i] {
                break;
            }
            i -= 1;
        }
        to_tab = tabstops[i + 1] - to_tab;
    }
    to_tab
}

/*
 * Append a tab to the line buffer.
 * Store spaces to represent the tab.
 */
unsafe extern "C" fn store_tab(attr: i32, pos: POSITION) -> i32 {
    let mut to_tab = tab_spaces(end_column);
    loop {
        if store_char(' ', attr, Some(b" "), pos) != 0 {
            return 1 as std::ffi::c_int;
        }
        to_tab -= 1;
        if !(to_tab > 0) {
            break;
        }
    }
    0
}

unsafe extern "C" fn store_prchar(c: char, pos: POSITION) -> i32 {
    /*
     * Convert to printable representation.
     */
    if store_string(prchar(c).as_bytes(), AT_BINARY | AT_COLOR_CTRL, pos) != 0 {
        return 1;
    }
    0
}

unsafe extern "C" fn flush_mbc_buf(mut pos: POSITION) -> std::ffi::c_int {
    for i in 0..mbc_buf_index {
        if store_prchar(mbc_buf[i as usize] as char, pos) != 0 {
            return mbc_buf_index - i;
        }
    }
    0
}

fn is_utf8_trail(c: u8) -> bool {
    c & 0xC0 == 0x80
}

fn is_ascii_octet(c: u8) -> bool {
    c & 0x80 == 0
}
fn is_utf8_invalid(c: u8) -> bool {
    c & 0xFE == 0xFE
}

fn is_utf8_lead(c: u8) -> bool {
    (c & 0xC0 == 0xC0) && !is_utf8_invalid(c)
}

/*
 * Append a character to the line buffer.
 * Expand tabs into spaces, handle underlining, boldfacing, etc.
 * Returns 0 if ok, 1 if couldn't fit in buffer.
 */
pub unsafe extern "C" fn pappend_b(c: u8, pos: POSITION, before_pendc: bool) -> i32 {
    let mut ch: char = (c & 0o377) as char;
    let mut r = 0;

    if pendc != '\0' && before_pendc {
        if ch == '\r' && pendc == '\r' {
            return 0;
        }
        if do_append(pendc, None, pendpos) != 0 {
            /*
             * Oops.  We've probably lost the char which
             * was in pendc, since caller won't back up.
             */
            return 1;
        }
        pendc = '\0';
    }
    if ch == '\r' && (proc_return == OPT_ON || bs_mode == BS_SPECIAL && proc_return == OPT_OFF) {
        if mbc_buf_len > 0 {
            /* utf_mode must be on */
            /* Flush incomplete (truncated) sequence. */
            r = flush_mbc_buf(mbc_pos);
            mbc_buf_index = r + 1;
            mbc_buf_len = 0;
            if r != 0 {
                return mbc_buf_index;
            }
        }
        /*
         * Don't put the CR into the buffer until we see
         * the next char.  If the next char is a newline,
         * discard the CR.
         */
        pendc = ch;
        pendpos = pos;
        return 0;
    }
    if !utf_mode {
        r = do_append(ch, None, pos);
    } else {
        /* Perform strict validation in all possible cases. */
        let mut current_block_41: u64;
        if mbc_buf_len == 0 {
            // FIXME
            current_block_41 = 11600700225610697556;
        } else if is_utf8_trail(c) {
            mbc_buf[mbc_buf_index as usize] = c;
            mbc_buf_index = mbc_buf_index + 1;
            if mbc_buf_index < mbc_buf_len {
                return 0;
            }
            if is_utf8_well_formed(&mbc_buf, mbc_buf_index as usize) {
                r = do_append(get_wchar(&mbc_buf), Some(&mbc_buf), mbc_pos);
            } else {
                /* Complete, but not shortest form, sequence. */
                r = flush_mbc_buf(mbc_pos);
                mbc_buf_index = r;
            }
            mbc_buf_len = 0 as std::ffi::c_int;
            // FIXME
            current_block_41 = 2873832966593178012;
        } else {
            /* Flush incomplete (truncated) sequence.  */
            r = flush_mbc_buf(mbc_pos);
            mbc_buf_index = r + 1;
            mbc_buf_len = 0;
            /* Handle new char.  */
            if r == 0 {
                // FIXME
                current_block_41 = 11600700225610697556;
            } else {
                // FIXME
                current_block_41 = 2873832966593178012;
            }
        }
        match current_block_41 {
            11600700225610697556 => {
                mbc_buf_index = 1;
                *mbc_buf.as_mut_ptr() = c;
                if is_ascii_octet(c) {
                    r = do_append(ch, None, pos);
                } else if is_utf8_lead(c) {
                    mbc_buf_len = utf_len(c as i8);
                    mbc_pos = pos;
                    return 0;
                } else {
                    /* UTF8_INVALID or stray UTF8_TRAIL */
                    r = flush_mbc_buf(pos);
                }
            }
            _ => {}
        }
    }
    if r != 0 {
        /* How many chars should caller back up? */
        r = if !utf_mode { 1 } else { mbc_buf_index };
    }
    r
}

pub unsafe extern "C" fn pappend(c: u8, pos: POSITION) -> i32 {
    if ff_starts_line < 0 {
        ff_starts_line = if c == b'L' & 0o37 { 1 } else { 0 };
    }
    return pappend_b(c, pos, false);
}

pub unsafe extern "C" fn line_is_ff() -> bool {
    ff_starts_line == 1
}

unsafe extern "C" fn store_control_char(ch: char, rep: Option<&[u8]>, pos: POSITION) -> i32 {
    if ctldisp == OPT_ON {
        /* Output the character itself. */
        if store_char(ch, 0, rep, pos) != 0 {
            return 1;
        }
    } else if store_prchar(ch, pos) != 0 {
        /* Output a printable representation of the character. */
        return 1;
    }
    0
}

unsafe extern "C" fn store_ansi(ch: char, rep: Option<&[u8]>, pos: POSITION) -> i32 {
    match ansi_step2(&mut line_ansi.unwrap(), ch, pos != NULL_POSITION) {
        ANSI_STATE::ANSI_MID => {
            if store_char(ch, AT_ANSI, rep, pos) != 0 {
                return 1;
            }
            match ansi_osc8_state(&line_ansi.unwrap()) {
                OSC8_STATE::OSC_TYPENUM | OSC8_STATE::OSC_STRING => {
                    hlink_in_line = true;
                }
                _ => {}
            }
            last_ansi.xbuf_add_char(ch as i8);
        }
        ANSI_STATE::ANSI_END => {
            if store_char(ch, AT_ANSI, rep, pos) != 0 {
                return 1;
            }
            // FIXME is this needed?
            // ansi_done(line_ansi.unwrap());
            line_ansi = None;
            last_ansi.xbuf_add_char(ch as i8);
            last_ansis[curr_last_ansi as usize].set(&mut last_ansi);
            last_ansi.reset();
            curr_last_ansi = (curr_last_ansi + 1) % NUM_LAST_ANSIS as i32;
        }
        ANSI_STATE::ANSI_ERR => {
            let mut start = 0;
            /* Remove whole unrecognized sequence.  */
            let mut buf = if cshift < hshift {
                &shifted_ansi.data
            } else {
                &linebuf.buf
            };
            let mut end = if cshift < hshift {
                shifted_ansi.data.len()
            } else {
                linebuf.end
            };
            let bch = '\0';
            let idx = 0;
            loop {
                let (bch, idx) = step_charc(buf, -1, end, 0);
                if !(idx > start
                    && (!(is_csi_start(bch) || {
                        let mut l_ansi = line_ansi.unwrap();
                        let cond = l_ansi.escs_in_seq > 0;
                        l_ansi.escs_in_seq -= 1;
                        line_ansi = Some(l_ansi);
                        cond
                    })))
                {
                    break;
                }
            }
            end = idx - start;
            last_ansi.reset();
            //ansi_done(line_ansi);
            line_ansi = None;
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn store_bs(ch: char, rep: Option<&[u8]>, pos: POSITION) -> i32 {
    if proc_backspace == OPT_ONPLUS || (bs_mode == BS_CONTROL && proc_backspace == OPT_OFF) {
        return store_control_char(ch, rep, pos);
    }
    if linebuf.end > 0
        && ((linebuf.end <= linebuf.print && linebuf.buf[linebuf.end - 1] == 0)
            || linebuf.end > 0 && (linebuf.attr[linebuf.end - 1] & (AT_ANSI | AT_BINARY)) != 0)
    {
        if store_prchar('\u{8}', pos) != 0 {
            return 1;
        }
    } else if proc_backspace == OPT_OFF && bs_mode == BS_NORMAL {
        if store_char(ch, AT_NORMAL, None, pos) != 0 {
            return 1;
        }
    } else if proc_backspace == OPT_ON || (bs_mode == BS_SPECIAL && proc_backspace == OPT_OFF) {
        overstrike = backc();
    }
    0
}

unsafe extern "C" fn do_append(ch: char, rep: Option<&[u8]>, pos: POSITION) -> i32 {
    let mut a = AT_NORMAL;
    let mut in_overstrike = overstrike;
    let mut rep = rep;
    let mut ch = ch;

    if (ctldisp == OPT_ONPLUS || pos == NULL_POSITION) && line_ansi.is_none() {
        line_ansi = ansi_start(ch);
        if !line_ansi.is_none() {
            ansi_in_line = true;
        }
    }
    overstrike = 0;
    if !line_ansi.is_none() {
        return store_ansi(ch, rep, pos);
    }
    if ch == '\x08' {
        return store_bs(ch, rep, pos);
    }
    if in_overstrike > 0 {
        /*
         * Overstrike the character at the current position
         * in the line buffer.  This will cause either
         * underline (if a "_" is overstruck),
         * bold (if an identical character is overstruck),
         * or just replacing the character in the buffer.
         */
        let mut prev_ch = '\0';
        overstrike = if utf_mode { -1 } else { 0 };
        if utf_mode {
            /* To be correct, this must be a base character.  */
            prev_ch = get_wchar(&linebuf.buf[linebuf.end..]);
        } else {
            prev_ch = linebuf.buf[linebuf.end] as char;
        }
        a = linebuf.attr[linebuf.end];
        if ch == prev_ch {
            /*
             * Overstriking a char with itself means make it bold.
             * But overstriking an underscore with itself is
             * ambiguous.  It could mean make it bold, or
             * it could mean make it underlined.
             * Use the previous overstrike to resolve it.
             */
            if ch == '_' {
                if (a & (AT_BOLD | AT_UNDERLINE)) != AT_NORMAL {
                    a |= AT_BOLD | AT_UNDERLINE;
                } else if last_overstrike != AT_NORMAL {
                    a |= last_overstrike;
                } else {
                    a |= AT_BOLD;
                }
            } else {
                a |= AT_BOLD;
            }
        } else if ch == '_' {
            a |= AT_UNDERLINE;
            ch = prev_ch;
            rep = Some(&linebuf.buf[linebuf.end..]);
        } else if prev_ch == '_' {
            a |= AT_UNDERLINE;
        }
        /* Else we replace prev_ch, but we keep its attributes.  */
    } else if in_overstrike < 0 {
        if is_composing_char(ch) || is_combining_char(get_wchar(&linebuf.buf[linebuf.end..]), ch) {
            /* Continuation of the same overstrike.  */
            a = last_overstrike;
        } else {
            overstrike = 0;
        }
    }
    if ch == '\t' {
        /*
         * Expand a tab into spaces.
         */

        if proc_tab == OPT_ONPLUS || (bs_mode == BS_CONTROL && proc_tab == OPT_OFF) {
            return store_control_char(ch, rep, pos);
        }
        if store_tab(a, pos) != 0 {
            return 1;
        }
        return 0;
    }
    if (!utf_mode || ch.is_ascii()) && ch.is_control() {
        return store_control_char(ch, rep, pos);
    } else if utf_mode && ctldisp != 1 && is_ubin_char(ch) {
        if store_string(prutfchar(ch), AT_BINARY, pos) != 0 {
            return 1;
        }
    } else if store_char(ch, a, rep, pos) != 0 {
        return 1;
    }
    0
}

pub unsafe extern "C" fn pflushmbc() -> i32 {
    let mut r = 0;
    if mbc_buf_len > 0 {
        /* Flush incomplete (truncated) sequence.  */
        r = flush_mbc_buf(mbc_pos);
        mbc_buf_len = 0;
    }
    r
}

/*
 * Switch to normal attribute at end of line.
 */
unsafe extern "C" fn add_attr_normal() {
    if !line_ansi.is_none() {
        match line_ansi.unwrap().ostate {
            OSC8_STATE::OSC_TYPENUM
            | OSC8_STATE::OSC8_PARAMS
            | OSC8_STATE::OSC8_URI
            | OSC8_STATE::OSC_STRING => {
                addstr_linebuf(b"\x1B\\\0", AT_ANSI, 0);
            }
            _ => {}
        }
        //ansi_done(line_ansi);
        line_ansi = None;
    }
    if ctldisp != OPT_ONPLUS || !is_ansi_end('m') {
        return;
    }
    addstr_linebuf(b"\x1B[m\0", AT_ANSI, 0);
    if hlink_in_line {
        /* Don't send hyperlink clear if we know we don't need to. */
        addstr_linebuf(b"\x1B]8;;\x1B\\\0", AT_ANSI, 0);
    }
}

/*
 * Terminate the line in the line buffer.
 */
pub unsafe extern "C" fn pdone(endline: bool, chopped: bool, forw: bool) {
    pflushmbc();
    linebuf.prev_end = if !endline && !chopped { linebuf.end } else { 0 };
    if pendc != '\0' && (pendc != '\r' || !endline) {
        /*
         * If we had a pending character, put it in the buffer.
         * But discard a pending CR if we are at end of line
         * (that is, discard the CR in a CR/LF sequence).
         */
        do_append(pendc, None, pendpos);
    }
    if chopped && rscroll_char != '\0' {
        let mut rscroll_utf8: [u8; MAX_UTF_CHAR_LEN + 1] = [0; MAX_UTF_CHAR_LEN + 1];
        let mut up = 0;

        /*
         * Display the right scrolling char.
         * If we've already filled the rightmost screen char
         * (in the buffer), overwrite it.
         */
        if end_column >= sc_width + cshift {
            /* We've already written in the rightmost char. */
            end_column = right_column;
            linebuf.end = right_curr as usize;
        }
        add_attr_normal();
        while end_column < sc_width - 1 + cshift {
            /*
             * Space to last (rightmost) char on screen.
             * This may be necessary if the char we overwrote
             * was double-width.
             */
            add_linebuf(b' ', 0, 1);
        }
        /* Print rscroll char. */
        let idx = put_wchar(&mut rscroll_utf8, rscroll_char);
        rscroll_utf8[idx] = b'\0';
        addstr_linebuf(&rscroll_utf8, rscroll_attr, 0);
        inc_end_column(1); /* assume rscroll_char is single-width */
    } else {
        add_attr_normal();
    }

    /*
     * If we're coloring a status line, fill out the line with spaces.
     */
    if status_line && line_mark_attr != 0 {
        while (end_column + 1) < sc_width + cshift {
            add_linebuf(b' ', line_mark_attr, 1);
        }
    }

    /*
     * Add a newline if necessary,
     * and append a '\0' to the end of the line.
     * We output a newline if we're not at the right edge of the screen,
     * or if the terminal doesn't auto wrap,
     * or if this is really the end of the line AND the terminal ignores
     * a newline at the right edge.
     * (In the last case we don't want to output a newline if the terminal
     * doesn't ignore it since that would produce an extra blank line.
     * But we do want to output a newline if the terminal ignores it in case
     * the next line is blank.  In that case the single newline output for
     * that blank line would be ignored!)
     */
    if end_column < sc_width + cshift
        || auto_wrap != 0
        || (endline && ignaw != 0)
        || ctldisp == OPT_ON
    {
        add_linebuf(b'\n', AT_NORMAL, 0);
    } else if ignaw != 0 && end_column >= sc_width + cshift && forw {
        /*
         * Terminals with "ignaw" don't wrap until they *really* need
         * to, i.e. when the character *after* the last one to fit on a
         * line is output. But they are too hard to deal with when they
         * get in the state where a full screen width of characters
         * have been output but the cursor is sitting on the right edge
         * instead of at the start of the next line.
         * So we nudge them into wrapping by outputting a space
         * character plus a backspace.  But do this only if moving
         * forward; if we're moving backward and drawing this line at
         * the top of the screen, the space would overwrite the first
         * char on the next line.  We don't need to do this "nudge"
         * at the top of the screen anyway.
         */
        add_linebuf(b' ', 0, 1);
        add_linebuf(b'\x08', 0, -1);
    }
    /*
     * If a terminal moves the cursor to the next line immediately after
     * writing into the last char of a line, the following line may get
     * colored with the last char's background color before the color
     * reset sequence is sent. Clear the line to reset the background color.
     */
    if auto_wrap != 0 && ignaw == 0 && end_column >= sc_width + cshift {
        clear_after_line = true;
    }
    set_linebuf(linebuf.end, b'\0', 0);
}

/*
 * Return the column number (screen position) of a given file position in its line.
 * linepos = position of first char in line
 * spos = position of char being queried
 * saved_pos = position of a known column, or NULL_POSITION if no known column
 * saved_col = column number of a known column, or -1 if no known column
 *
 * This attempts to mimic the logic in pappend() and the store_*() functions.
 * Duplicating this complicated logic is not a good design.
 */
unsafe extern "C" fn col_vs_pos(
    linepos: POSITION,
    cp: &mut ColPos,
    saved_pos: POSITION,
    saved_col: i32,
) {
    let mut col = if saved_col < 0 { 0 } else { saved_col };
    let mut prev_ch = '\0';
    let mut pansi: Option<AnsiState> = None;
    let mut utf8_buf: [u8; MAX_UTF_CHAR_LEN] = [0; MAX_UTF_CHAR_LEN];
    let mut utf8_len = 0;
    let mut chpos: POSITION = 0;
    if ch_seek(if saved_pos != NULL_POSITION {
        saved_pos
    } else {
        linepos
    }) != 0
    {
        return;
    }
    loop {
        let mut ich = '\0';
        let mut ch = '\0';
        let mut cw: i64 = 0;

        chpos = ch_tell();
        ich = char::from_u32(ch_forw_get() as u32).unwrap();
        ch = ich;
        if ich as i32 == EOI || ch == '\n' {
            break;
        }
        if !pansi.is_none() {
            if ansi_step(&mut pansi.unwrap(), ch) != ANSI_STATE::ANSI_MID {
                //ansi_done(pansi);
                pansi = None;
            }
        } else if ctldisp == OPT_ONPLUS && {
            pansi = ansi_start(ch);
            !pansi.is_none()
        } {
            /* start of ansi sequence */
            ansi_step(&mut (pansi.unwrap()), ch);
        } else if ch == '\u{8}' {
            if proc_backspace == OPT_ONPLUS || (bs_mode == BS_CONTROL && proc_backspace == OPT_OFF)
            {
                cw = prchar(ch).len() as i64;
            } else {
                cw = if utf_mode && is_wide_char(prev_ch) {
                    -2
                } else {
                    -1
                };
            }
        } else if ch == '\t' {
            if proc_tab == OPT_ONPLUS || (bs_mode == BS_CONTROL && proc_tab == OPT_OFF) {
                cw = prchar(ch).len() as i64;
            } else {
                cw = tab_spaces(col) as i64;
            }
        } else if (!utf_mode || ch.is_ascii()) && ch.is_control() {
            cw = prchar(ch).len() as i64;
        } else if utf8_len < MAX_UTF_CHAR_LEN {
            utf8_buf[utf8_len] = ch as u8;
            utf8_len += 1;
            if is_utf8_well_formed(&utf8_buf, utf8_len) {
                let mut wch = get_wchar(&utf8_buf);
                let mut attr = 0; /* {{ ignoring attribute is not correct for magic cookie terminals }} */
                utf8_len = 0;
                if utf_mode && ctldisp != OPT_ON && is_ubin_char(wch) {
                    cw = prutfchar(wch).len() as i64;
                } else {
                    cw = pwidth(wch, attr, prev_ch as char, attr) as i64;
                }
                prev_ch = wch;
            }
        } else {
            utf8_len = 0; /* flush invalid UTF-8 */
        }
        if cp.pos != -1 && chpos == cp.pos {
            /* found the position we want */
            break;
        }
        if cp.col >= 0 && col >= cp.col && cw > 0 {
            /* found the column we want */
            break;
        } else {
            col += cw as i32;
            prev_ch = ch;
        }
    }
    cp.col = col;
    cp.pos = chpos;
}

pub unsafe extern "C" fn col_from_pos(
    linepos: POSITION,
    spos: POSITION,
    saved_pos: POSITION,
    saved_col: i32,
) -> i32 {
    let mut cp = ColPos { col: 0, pos: 0 };
    cp.pos = spos;
    cp.col = NULL_POSITION;
    col_vs_pos(linepos, &mut cp, saved_pos, saved_col);
    return cp.col;
}

pub unsafe extern "C" fn pos_from_col(
    linepos: POSITION,
    col: std::ffi::c_int,
    saved_pos: POSITION,
    saved_col: i32,
) -> POSITION {
    let mut cp = ColPos { col: 0, pos: 0 };
    cp.col = col + hshift - line_pfx_width();
    cp.pos = NULL_POSITION;
    col_vs_pos(linepos, &mut cp, saved_pos, saved_col);
    return cp.pos;
}

/*
 * Set an attribute on each char of the line in the line buffer.
 */
pub unsafe extern "C" fn set_attr_line(mut a: i32) {
    for i in linebuf.print..linebuf.end {
        if (linebuf.attr[i] & AT_COLOR) == 0 || (a & AT_COLOR) == 0 {
            linebuf.attr[i] |= a;
        }
    }
}

/*
 * Set the char to be displayed in the status column.
 */
pub unsafe extern "C" fn set_status_col(c: u8, attr: i32) {
    set_pfx(0, c, attr);
}

/*
 * Get a character from the current line.
 * Return the character as the function return value,
 * and the character attribute in *ap.
 */
pub unsafe extern "C" fn gline(i: usize, ap: &mut i32) -> i32 {
    let mut i = i;
    if is_null_line {
        /*
         * If there is no current line, we pretend the line is
         * either "~" or "", depending on the "twiddle" flag.
         */
        if twiddle {
            if i == 0 {
                *ap = AT_BOLD;
                return b'~' as i32;
            }
            i -= 1;
        }
        /* Make sure we're back to AT_NORMAL before the '\n'.  */
        *ap = 0;
        return if i != 0 { b'\0' as i32 } else { b'\n' as i32 };
    }
    if i < linebuf.pfx_end {
        *ap = linebuf.pfx_attr[i];
        return linebuf.pfx[i as usize] as i32;
    }
    i += linebuf.print - linebuf.pfx_end;
    *ap = linebuf.attr[i];
    (linebuf.buf[i] & 0xFF) as i32
}

/*
 * Should we clear to end of line after printing this line?
 */
pub unsafe extern "C" fn should_clear_after_line() -> bool {
    clear_after_line
}

/*
 * Indicate that there is no current line.
 */
pub unsafe extern "C" fn null_line() {
    is_null_line = true;
    cshift = 0;
}

/*
 * Analogous to forw_line(), but deals with "raw lines":
 * lines which are not split for screen width.
 * {{ This is supposed to be more efficient than forw_line(). }}
 */
pub unsafe extern "C" fn forw_raw_line_len(
    curr_pos: POSITION,
    read_len: isize,
) -> (POSITION, usize, usize) {
    let mut n = 0;
    let mut c = 0;
    let mut new_pos = 0;

    if curr_pos == NULL_POSITION || ch_seek(curr_pos) != 0 || {
        c = ch_forw_get();
        c == EOI
    } {
        return (NULL_POSITION, 0, 0);
    }
    loop {
        if c == b'\n' as i32 || c == EOI || (sigs & (S_INTERRUPT | S_SWINTERRUPT | S_STOP)) != 0 {
            new_pos = ch_tell();
            break;
        } else {
            if n >= size_linebuf - 1 {
                expand_linebuf();
            }
            linebuf.buf[n] = 0;
            n += 1;
            if read_len != -1 && read_len > 0 && n >= read_len as usize {
                new_pos = ch_tell();
                break;
            } else {
                c = ch_forw_get();
            }
        }
    }
    linebuf.buf[n] = 0;
    (new_pos, 0, n)
}

pub unsafe extern "C" fn forw_raw_line(curr_pos: POSITION) -> (POSITION, usize, usize) {
    forw_raw_line_len(curr_pos, -1)
}

/*
 * Analogous to back_line(), but deals with "raw lines".
 * {{ This is supposed to be more efficient than back_line(). }}
 * returns absolute position, new line position, new line_size
 */
pub unsafe extern "C" fn back_raw_line(curr_pos: POSITION) -> (POSITION, usize, usize) {
    let mut n = 0;
    let mut c = 0;
    let mut new_pos = 0;
    if curr_pos == NULL_POSITION || curr_pos <= 0 || ch_seek(curr_pos - 1) != 0 {
        return (NULL_POSITION, 0, 0);
    }
    n = size_linebuf;
    n -= 1;
    linebuf.buf[n] = 0;
    loop {
        c = ch_back_get();
        if c == '\n' as i32 || sigs & (S_INTERRUPT | S_SWINTERRUPT | S_STOP) != 0 {
            /*
             * This is the newline ending the previous line.
             * We have hit the beginning of the line.
             */
            new_pos = ch_tell() + 1;
            break;
        } else if c == EOI {
            /*
             * We have hit the beginning of the file.
             * This must be the first line in the file.
             * This must, of course, be the beginning of the line.
             */
            new_pos = 0;
            break;
        }
        if n <= 0 {
            let mut old_size_linebuf = size_linebuf;
            let mut fm = 0;
            let mut to = 0;
            expand_linebuf();
            /*
             * Shift the data to the end of the new linebuf.
             */
            fm = old_size_linebuf - 1;
            to = size_linebuf - 1;
            while fm >= linebuf.buf.len() {
                linebuf.buf[to] = linebuf.buf[fm];
                fm -= 1;
                to -= 1;
            }
            n = size_linebuf.wrapping_sub(old_size_linebuf);
        }
        n -= 1;
        linebuf.buf[n] = c as u8;
    }
    (new_pos, n, size_linebuf - 1 - n)
}

/*
 * Skip cols printable columns at the start of line.
 * Return number of bytes skipped.
 */
pub unsafe extern "C" fn skip_columns(
    cols: i32,
    line: &[u8],
    line_lenp: &mut usize,
) -> (i32, usize) {
    let mut eline = *line_lenp;
    let mut pch: char = '\0';
    let mut bytes = 0;
    let mut l_idx = 0;
    let mut cols = cols;

    // FIXME index vs ptr
    while cols > 0 && l_idx < eline {
        let (mut ch, size) = step_charc(&line, 1, 0, eline);
        l_idx = size;
        let mut pansi = ansi_start(ch);
        if !pansi.is_none() {
            skip_ansi(&mut pansi.unwrap(), ch, &line[l_idx..], eline);
            //ansi_done(pansi);
            pch = 0 as char;
        } else {
            let mut w = pwidth(ch, 0, pch, 0);
            cols -= w;
            pch = ch;
        }
    }
    bytes = l_idx;
    *line_lenp -= bytes;
    (bytes as i32, l_idx)
}

/*
 * Append a string to the line buffer.
 */
unsafe extern "C" fn pappstr(string: &[u8]) -> i32 {
    for c in string {
        if pappend(*c, NULL_POSITION) != 0 {
            /* Doesn't fit on screen. */
            return 1;
        }
    }
    0
}

/*
 * Load a string into the line buffer.
 * If the string is too long to fit on the screen,
 * truncate the beginning of the string to fit.
 */
pub unsafe extern "C" fn load_line(string: &[u8]) {
    let mut save_hshift = hshift;
    hshift = 0;

    /* We're overwriting the line buffer, so what's in it will no longer be contiguous. */
    set_line_contig_pos(NULL_POSITION);

    loop {
        prewind(false);
        if pappstr(string) == 0 {
            break;
        }
        /*
         * Didn't fit on screen; increase left shift by one.
         * {{ This gets very inefficient if the string
         * is much longer than the screen width. }}
         */
        hshift += 1;
    }
    set_linebuf(linebuf.end, b'\0', 0);
    linebuf.prev_end = 0;

    /* Color the prompt unless it has ansi sequences in it. */
    if !ansi_in_line {
        for i in linebuf.print..linebuf.end {
            set_linebuf(i, linebuf.buf[i], AT_STANDOUT | AT_COLOR_PROMPT);
        }
    }
    hshift = save_hshift;
}

/*
 * Find the shift necessary to show the end of the longest displayed line.
 */
pub unsafe extern "C" fn rrshift() -> std::ffi::c_int {
    let mut pos = 0;
    let mut save_width = 0;
    let mut sline = 0;
    let mut longest = 0;

    save_width = sc_width;
    sc_width = i32::MAX; /* so forw_line() won't chop */

    for sline in TOP..sc_height {
        pos = position(sline);
        if pos != NULL_POSITION {
            break;
        }
    }
    while sline < sc_height && pos != NULL_POSITION {
        pos = forw_line(pos, &mut 0, &mut LFALSE);
        if end_column > longest {
            longest = end_column;
        }
        sline += 1;
    }
    sc_width = save_width;
    if longest < sc_width {
        return 0;
    }
    return longest - sc_width;
}

/*
 * Get the color_map index associated with a given attribute.
 */
unsafe extern "C" fn lookup_color_index(attr: i32) -> i32 {
    for cx in 0..color_map.len() {
        if color_map[cx].attr == attr {
            return cx as i32;
        }
    }
    -1
}

unsafe extern "C" fn color_index(attr: i32) -> i32 {
    if (use_color != 0) && (attr & AT_COLOR != 0) {
        return lookup_color_index(attr & AT_COLOR);
    }
    if attr & AT_UNDERLINE != 0 {
        return lookup_color_index(AT_UNDERLINE);
    }
    if attr & AT_BOLD != 0 {
        return lookup_color_index(AT_BOLD);
    }
    if attr & AT_BLINK != 0 {
        return lookup_color_index(AT_BLINK);
    }
    if attr & AT_STANDOUT != 0 {
        return lookup_color_index(AT_STANDOUT);
    }
    -1
}

/*
 * Set the color string to use for a given attribute.
 */
pub unsafe extern "C" fn set_color_map(attr: i32, colorstr: &'static str) -> i32 {
    let mut cx = color_index(attr);
    if cx < 0 {
        return -1;
    }
    if colorstr.len() + 1 > color_map[cx as usize].color.len() {
        return -1;
    }
    if colorstr.chars().nth(0).unwrap() != '\0'
        && parse_color(
            CString::new(colorstr).unwrap().as_ptr(),
            0 as *mut std::ffi::c_int,
            0 as *mut std::ffi::c_int,
            0 as *mut CHAR_ATTR,
        ) == CT_NULL
    {
        return -1;
    }
    color_map[cx as usize].color = colorstr;
    0
}

/*
* Get the color string to use for a given attribute.
*/
pub unsafe extern "C" fn get_color_map(mut attr: i32) -> Option<String> {
    let mut cx = color_index(attr);
    if cx < 0 {
        return None;
    }
    Some(String::from(color_map[cx as usize].color))
}
