use crate::evar::expand_evars;
use crate::screen::special_key_str;
use crate::util::ptr_to_str;
use crate::xbuf::XBuffer;
use std::env;
use std::env::VarError;
use std::ffi::CStr;
use std::ffi::CString;
use std::sync::LazyLock;

/*
 * Routines to decode user commands.
 *
 * This is all table driven.
 * A command table is a sequence of command descriptors.
 * Each command descriptor is a sequence of bytes with the following format:
 *     <c1><c2>...<cN><0><action>
 * The characters c1,c2,...,cN are the command string; that is,
 * the characters which the user must type.
 * It is terminated by a null <0> byte.
 * The byte after the null byte is the action code associated
 * with the command string.
 * If an action byte is OR-ed with A_EXTRA, this indicates
 * that the option byte is followed by an extra string.
 *
 * There may be many command tables.
 * The first (default) table is built-in.
 * Other tables are read in from "lesskey" files.
 * All the tables are linked together and are searched in order.
 */
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
    fn save(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn secure_allow(features: std::ffi::c_int) -> std::ffi::c_int;
    fn cmd_exec();
    fn screen_trashed();
    fn getcc() -> std::ffi::c_char;
    fn ungetcc(c: std::ffi::c_char);
    fn ungetsc(s: *const std::ffi::c_char);
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
    static mut mousecap: i32;
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
pub struct tablelist {
    pub t_next: *mut tablelist,
    pub t_start: *mut std::ffi::c_uchar,
    pub t_end: *mut std::ffi::c_uchar,
}
#[derive(Clone)]
#[repr(C)]
pub struct lesskey_table {
    pub names: *const lesskey_cmdname,
    pub buf: XBuffer,
    pub is_var: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lesskey_cmdname {
    pub cn_name: *const std::ffi::c_char,
    pub cn_action: std::ffi::c_int,
}
#[derive(Clone)]
#[repr(C)]
pub struct lesskey_tables {
    pub currtable: *mut lesskey_table,
    pub cmdtable: lesskey_table,
    pub edittable: lesskey_table,
    pub vartable: lesskey_table,
}
static mut allow_drag: bool = true;

/* Special keys (keys which output different strings on different terminals) */
const MAX_USERCMD: u32 = 1000;
const MAX_CMDLEN: usize = 16;

const A_B_LINE: u8 = 2;
const A_B_SCREEN: u8 = 3;
const A_B_SCROLL: u8 = 4;
const A_B_SEARCH: u8 = 5;
const A_DIGIT: u8 = 6;
const A_DISP_OPTION: u8 = 7;
const A_DEBUG: u8 = 8;
const A_EXAMINE: u8 = 9;
const A_FIRSTCMD: u8 = 10;
const A_FREPAINT: u8 = 11;
const A_F_LINE: u8 = 12;
const A_F_SCREEN: u8 = 13;
const A_F_SCROLL: u8 = 14;
const A_F_SEARCH: u8 = 15;
const A_GOEND: u8 = 16;
const A_GOLINE: u8 = 17;
const A_GOMARK: u8 = 18;
const A_HELP: u8 = 19;
const A_NEXT_FILE: u8 = 20;
const A_PERCENT: u8 = 21;
const A_BF_SCREEN: u8 = 22;
const A_PREV_FILE: u8 = 23;
const A_QUIT: u8 = 24;
const A_REPAINT: u8 = 25;
const A_SETMARK: u8 = 26;
const A_SHELL: u8 = 27;
const A_STAT: u8 = 28;
const A_FF_LINE: u8 = 29;
const A_BF_LINE: u8 = 30;
const A_VERSION: u8 = 31;
const A_VISUAL: u8 = 32;
const A_F_WINDOW: u8 = 33;
const A_B_WINDOW: u8 = 34;
const A_F_BRACKET: u8 = 35;
const A_B_BRACKET: u8 = 36;
const A_PIPE: u8 = 37;
const A_INDEX_FILE: u8 = 38;
const A_UNDO_SEARCH: u8 = 39;
const A_FF_SCREEN: u8 = 40;
const A_LSHIFT: u8 = 41;
const A_RSHIFT: u8 = 42;
const A_AGAIN_SEARCH: u8 = 43;
const A_T_AGAIN_SEARCH: u8 = 44;
const A_REVERSE_SEARCH: u8 = 45;
const A_T_REVERSE_SEARCH: u8 = 46;
const A_OPT_TOGGLE: u8 = 47;
const A_OPT_SET: u8 = 48;
const A_OPT_UNSET: u8 = 49;
const A_F_FOREVER: u8 = 50;
const A_GOPOS: u8 = 51;
const A_REMOVE_FILE: u8 = 52;
const A_NEXT_TAG: u8 = 53;
const A_PREV_TAG: u8 = 54;
const A_FILTER: u8 = 55;
const A_F_UNTIL_HILITE: u8 = 56;
const A_GOEND_BUF: u8 = 57;
const A_LLSHIFT: u8 = 58;
const A_RRSHIFT: u8 = 59;
const A_F_NEWLINE: u8 = 60;
const A_B_NEWLINE: u8 = 61;
const A_CLRMARK: u8 = 62;
const A_SETMARKBOT: u8 = 63;
const A_X11MOUSE_IN: u8 = 64;
const A_F_MOUSE: u8 = 66;
const A_B_MOUSE: u8 = 67;
/* Note "X116" refers to extended (1006) X11 mouse reporting. */
const A_X116MOUSE_IN: u8 = 68;
const A_PSHELL: u8 = 69;
const A_CLR_SEARCH: u8 = 70;
const A_OSC8_F_SEARCH: u8 = 71;
const A_OSC8_B_SEARCH: u8 = 72;
const A_OSC8_OPEN: u8 = 73;
const A_OSC8_JUMP: u8 = 74;
const A_START_PASTE: u8 = 75; /* must not overlap EC_* */
const A_END_PASTE: u8 = 76; /* must not overlap EC_* */

/* These values must not conflict with any A_* or EC_* value. */
const A_INVALID: u8 = 100;
const A_NOACTION: u8 = 101;
const A_UINVALID: u8 = 102;
const A_END_LIST: u8 = 103;
const A_SPECIAL_KEY: u8 = 104;
const A_PREFIX: u8 = 105;
const A_SKIP: u8 = 127;

const A_EXTRA: u8 = 0o200;

/* Special keys (keys which output different strings on different terminals) */
const SK_RIGHT_ARROW: u8 = 1;
const SK_LEFT_ARROW: u8 = 2;
const SK_UP_ARROW: u8 = 3;
const SK_DOWN_ARROW: u8 = 4;
const SK_PAGE_UP: u8 = 5;
const SK_PAGE_DOWN: u8 = 6;
const SK_HOME: u8 = 7;
const SK_END: u8 = 8;
const SK_DELETE: u8 = 9;
const SK_INSERT: u8 = 10;
const SK_CTL_LEFT_ARROW: u8 = 11;
const SK_CTL_RIGHT_ARROW: u8 = 12;
const SK_CTL_DELETE: u8 = 13;
const SK_F1: u8 = 14;
const SK_BACKTAB: u8 = 15;
const SK_CTL_BACKSPACE: u8 = 16;
const SK_BACKSPACE: u8 = 17;
const SK_CONTROL_K: u8 = 40;
const SK_SPECIAL_KEY: u8 = b'K' & 0o37;

/* Line editing characters */
const EC_BACKSPACE: u8 = 1;
const EC_LINEKILL: u8 = 2;
const EC_RIGHT: u8 = 3;
const EC_LEFT: u8 = 4;
const EC_W_LEFT: u8 = 5;
const EC_W_RIGHT: u8 = 6;
const EC_INSERT: u8 = 7;
const EC_DELETE: u8 = 8;
const EC_HOME: u8 = 9;
const EC_END: u8 = 10;
const EC_W_BACKSPACE: u8 = 11;
const EC_W_DELETE: u8 = 12;
const EC_UP: u8 = 13;
const EC_DOWN: u8 = 14;
const EC_EXPAND: u8 = 15;
const EC_F_COMPLETE: u8 = 17;
const EC_B_COMPLETE: u8 = 18;
const EC_LITERAL: u8 = 19;
const EC_ABORT: u8 = 20;
const EC_X11MOUSE: u8 = 21;
const EC_X116MOUSE: u8 = 22;
const EC_START_PASTE: u8 = A_START_PASTE;
const EC_END_PASTE: u8 = A_END_PASTE;

const EC_UINVALID: u8 = 102;

/* Environment variable stuff */
const EV_OK: u8 = 0o01;

const ESC: u8 = b'[' & 0o37;
const SK: u8 = b'K' & 0o37;

const OPT_OFF: u8 = 0;
const OPT_ON: u8 = 1;
const OPT_ONPLUS: u8 = 2;

/* X11 mouse reporting definitions */
const X11MOUSE_BUTTON1: u8 = 0; /* Left button press */
const X11MOUSE_BUTTON2: u8 = 1; /* Middle button press */
const X11MOUSE_BUTTON3: u8 = 2; /* Right button press */
const X11MOUSE_BUTTON_REL: u8 = 3; /* Button release */
const X11MOUSE_DRAG: u8 = 0x20; /* Drag with button down */
const X11MOUSE_WHEEL_UP: u8 = 0x40; /* Wheel scroll up */
const X11MOUSE_WHEEL_DOWN: u8 = 0x41; /* Wheel scroll down */
const X11MOUSE_OFFSET: u8 = 0x20; /* Added to button & pos bytes to create a char */

/* Flags for editchar() */
const ECF_PEEK: u8 = 0o1;
const ECF_NOHISTORY: u8 = 0o2;
const ECF_NOCOMPLETE: u8 = 0o4;
const ECF_NORIGHTLEFT: u8 = 0o10;

/* Security features. */
const SF_EDIT: i32 = 1 << 1; /* Edit file (v) */
const SF_EXAMINE: i32 = 1 << 2; /* Examine file (:e) */
const SF_GLOB: i32 = 1 << 3; /* Expand file pattern */
const SF_HISTORY: i32 = 1 << 4; /* History file */
const SF_LESSKEY: i32 = 1 << 5; /* Lesskey files */
const SF_LESSOPEN: i32 = 1 << 6; /* LESSOPEN */
const SF_LOGFILE: i32 = 1 << 7; /* Log file (s, -o) */
const SF_PIPE: i32 = 1 << 8; /* Pipe (|) */
const SF_SHELL: i32 = 1 << 9; /* Shell command (!) */
const SF_STOP: i32 = 1 << 10; /* Stop signal */
const SF_TAGS: i32 = 1 << 11; /* Tags */
const SF_OSC8_OPEN: i32 = 1 << 12; /* OSC8 open */

const BAD_LSEEK: i64 = -1;
const SEEK_SET: i32 = 0;
const SEEK_END: i32 = 2;

/*
 * Format of a lesskey file:
 *
 *      LESSKEY_MAGIC (4 bytes)
 *       sections...
 *      END_LESSKEY_MAGIC (4 bytes)
 *
 * Each section is:
 *
 *      section_MAGIC (1 byte)
 *      section_length (2 bytes)
 *      key table (section_length bytes)
 */
const C0_LESSKEY_MAGIC: u8 = b'\0';
const C1_LESSKEY_MAGIC: u8 = b'M';
const C2_LESSKEY_MAGIC: u8 = b'+';
const C3_LESSKEY_MAGIC: u8 = b'G';

const CMD_SECTION: u8 = b'c';
const EDIT_SECTION: u8 = b'e';
const VAR_SECTION: u8 = b'v';
const END_SECTION: u8 = b'x';

const C0_END_LESSKEY_MAGIC: u8 = b'E';
const C1_END_LESSKEY_MAGIC: u8 = b'n';
const C2_END_LESSKEY_MAGIC: u8 = b'd';

const NULL_POSITION: i32 = -1;
const KRADIX: u32 = 64;

#[derive(Debug)]
struct Table {
    table: Vec<u8>,
}

impl Table {
    fn from(buf: &[u8]) -> Table {
        Table { table: buf.into() }
    }
}

#[derive(Debug)]
pub struct Tables {
    pub fcmd_tables: Vec<Table>,
    pub ecmd_tables: Vec<Table>,
    pub var_tables: Vec<Table>,
    pub sysvar_tables: Vec<Table>,
}

impl Tables {
    pub fn new() -> Self {
        Tables {
            fcmd_tables: Vec::new(),
            ecmd_tables: Vec::new(),
            var_tables: Vec::new(),
            sysvar_tables: Vec::new(),
        }
    }
}

fn table_from_xbuf(xbuf: XBuffer) -> Table {
    Table {
        table: xbuf.data.into(),
    }
}

/*
 * Command table is ordered roughly according to expected
 * frequency of use, so the common commands are near the beginning.
 */
#[rustfmt::skip]
static mut cmdtable: LazyLock<Vec<u8>> = LazyLock::new(|| {
    vec![
    b'\r',0,                         A_F_LINE,
    b'\n',0,                         A_F_LINE,
    b'e',0,                          A_F_LINE,
    b'j',0,                          A_F_LINE,
    SK, SK_DOWN_ARROW, 6, 1, 1, 1, 0, A_F_LINE,
    b'E' & 0o37, 0,                 A_F_LINE,
    b'N' & 0o37, 0,                 A_F_LINE,
    b'k',0,                         A_B_LINE,
    b'y',0,                         A_B_LINE,
    b'Y' & 0o37, 0,                  A_B_LINE,
    SK, SK_CONTROL_K, 6, 1, 1, 1, 0, A_F_LINE,
    b'P' & 0o37, 0,                 A_B_LINE,
    SK, SK_UP_ARROW, 6, 1, 1, 1, 0, A_F_LINE,
    b'J',0,                         A_FF_LINE,
    b'K',0,                         A_BF_LINE,
    b'Y',0,                         A_BF_LINE,
    b'd',0,                         A_F_SCROLL,
    b'D' & 0o37, 0,                 A_F_SCROLL,
    b'u',0,                         A_B_SCROLL,
    b'U' & 0o37 , 0,                A_B_SCROLL,
    ESC,b'[',b'M',0,                A_X11MOUSE_IN,
    b'[' & 0o37,b'[',b'<',0,        A_X116MOUSE_IN,
    b' ',0,                         A_F_SCREEN,
    b'f',0,                         A_F_SCREEN,
    b'F' & 0o37,0,                  A_F_SCREEN,
    b'V' & 0o37,0,                  A_F_SCREEN,
    SK, SK_PAGE_DOWN, 6, 1, 1, 1, 0, A_F_SCREEN,
    b'b',0,                         A_B_SCREEN,
    b'B' & 0o37,0,                  A_B_SCREEN,
    ESC,b'v',0,                     A_B_SCREEN,
    SK, SK_PAGE_UP, 6, 1, 1, 1, 0,  A_B_SCREEN,
    b'z',0,                         A_F_WINDOW,
    b'w',0,                         A_B_WINDOW,
    ESC,b' ',0,                     A_FF_SCREEN,
    ESC,b'b',0,                     A_BF_SCREEN,
    ESC,b'j',0,                     A_F_NEWLINE,
    ESC,b'k',0,                     A_B_NEWLINE,
    b'F',0,                         A_F_FOREVER,
    ESC,b'F',0,                     A_F_UNTIL_HILITE,
    b'R',0,                         A_FREPAINT,
    b'r',0,                         A_REPAINT,
    b'R' & 0o37,0,                  A_REPAINT,
    b'L' & 0o37,0,                  A_REPAINT,
    ESC,b'u',0,                     A_UNDO_SEARCH,
    ESC,b'U',0,                     A_CLR_SEARCH,
    b'g',0,                         A_GOLINE,
    SK, SK_HOME, 6, 1, 1, 1, 0,     A_GOLINE,
    b'<',0,                         A_GOLINE,
    ESC,b'<',0,                     A_GOLINE,
    b'p',0,                         A_PERCENT,
    b'%',0,                         A_PERCENT,
    ESC,b'(',0,                     A_LSHIFT,
    ESC,b')',0,                     A_RSHIFT,
    ESC,b'{',0,                     A_LLSHIFT,
    ESC,b'}',0,                     A_RRSHIFT,
    SK, SK_RIGHT_ARROW, 6, 1, 1, 1, 0, A_RSHIFT,
    SK, SK_LEFT_ARROW, 6, 1, 1, 1, 0, A_LSHIFT,
    SK, SK_CTL_RIGHT_ARROW, 6, 1, 1, 1, 0, A_RRSHIFT,
    SK, SK_CTL_LEFT_ARROW, 6, 1, 1, 1, 0, A_LLSHIFT,
    b'{',0,                         A_F_BRACKET|A_EXTRA,        b'{',b'}',0,
    b'}',0,                         A_B_BRACKET|A_EXTRA,        b'{',b'}',0,
    b'(',0,                         A_F_BRACKET|A_EXTRA,        b'(',b')',0,
    b')',0,                         A_B_BRACKET|A_EXTRA,        b'(',b')',0,
    b'[',0,                         A_F_BRACKET|A_EXTRA,        b'[',b']',0,
    b']',0,                         A_B_BRACKET|A_EXTRA,        b'[',b']',0,
    ESC,b'F' & 0o37,0,              A_F_BRACKET,
    ESC,b'B' & 0o37,0,              A_B_BRACKET,
    b'G',0,                         A_GOEND,
    ESC,b'G',0,                     A_GOEND_BUF,
    ESC,b'>',0,                     A_GOEND,
    b'>',0,                         A_GOEND,
    SK, SK_END, 6, 1, 1, 1, 0,      A_GOEND,
    b'P',0,                         A_GOPOS,

    b'0',0,                         A_DIGIT,
    b'1',0,                         A_DIGIT,
    b'2',0,                         A_DIGIT,
    b'3',0,                         A_DIGIT,
    b'4',0,                         A_DIGIT,
    b'5',0,                         A_DIGIT,
    b'6',0,                         A_DIGIT,
    b'7',0,                         A_DIGIT,
    b'8',0,                         A_DIGIT,
    b'9',0,                         A_DIGIT,
    b'.',0,                         A_DIGIT,

    b'=',0,                         A_STAT,
    b'G' & 0o37,0,                  A_STAT,
    b':', b'f',0,                   A_STAT,
    b'/',0,                         A_F_SEARCH,
    b'?',0,                         A_B_SEARCH,
    ESC,b'/',0,                     A_F_SEARCH|A_EXTRA,        b'*',0,
    ESC,b'?',0,                     A_B_SEARCH|A_EXTRA,        b'*',0,
    b'n',0,                         A_AGAIN_SEARCH,
    ESC,b'n',0,                     A_T_AGAIN_SEARCH,
    b'N',0,                         A_REVERSE_SEARCH,
    ESC,b'N',0,                     A_T_REVERSE_SEARCH,
    b'&',0,                         A_FILTER,
    b'm',0,                         A_SETMARK,
    b'M',0,                         A_SETMARKBOT,
    ESC,b'm',0,                     A_CLRMARK,
    b'\'',0,                        A_GOMARK,
    b'X' & 0o37,b'X' & 0o37,0,      A_GOMARK,
    b'E',0,                         A_EXAMINE,
    b':',b'e',0,                    A_EXAMINE,
    b'X' & 0o37, b'V' & 0o37,0,     A_EXAMINE,
    b':',b'n',0,                    A_NEXT_FILE,
    b':',b'p',0,                    A_PREV_FILE,
    b'O' & 0o37,b'N' & 0o37,0,      A_OSC8_F_SEARCH,
    b'O' & 0o37,b'n',0,             A_OSC8_F_SEARCH,
    b'O' & 0o37,b'P'& 0o37,0,       A_OSC8_B_SEARCH,
    b'O' & 0o37,b'p',0,             A_OSC8_B_SEARCH,
    b'O' & 0o37,b'O' & 0o37,0,      A_OSC8_OPEN,
    b'O' & 0o37,b'o',0,             A_OSC8_OPEN,
    b'O' & 0o37,b'L' & 0o37,0,      A_OSC8_JUMP,
    b'O' & 0o37,b'l',0,             A_OSC8_JUMP,
    b't',0,                         A_NEXT_TAG,
    b'T',0,                         A_PREV_TAG,
    b':',b'x',0,                    A_INDEX_FILE,
    b':',b'd',0,                    A_REMOVE_FILE,
    b'-',0,                         A_OPT_TOGGLE,
    b':',b't',0,                    A_OPT_TOGGLE|A_EXTRA,        b't',0,
    b's',0,                         A_OPT_TOGGLE|A_EXTRA,        b'o',0,
    b'_',0,                         A_DISP_OPTION,
    b'|',0,                         A_PIPE,
    b'v',0,                         A_VISUAL,
    b'!',0,                         A_SHELL,
    b'#',0,                         A_PSHELL,
    b'+',0,                         A_FIRSTCMD,
    ESC,b'[',b'2',b'0',b'0',b'~',0,      A_START_PASTE,
    ESC,b'[',b'2',b'0',b'1',b'~',0,      A_END_PASTE,

    b'H',0,                         A_HELP,
    b'h',0,                         A_HELP,
    SK, SK_F1, 6, 1, 1, 1, 0,       A_HELP,
    b'V',0,                         A_VERSION,
    b'q',0,                         A_QUIT,
    b'Q',0,                         A_QUIT,
    b':',b'q',0,                    A_QUIT,
    b':',b'Q',0,                    A_QUIT,
    b'Z',b'Z',0,                    A_QUIT,
]
});

#[rustfmt::skip]
static mut edittable: LazyLock<Vec<u8>> = LazyLock::new(|| {
    vec![
    b'\t',0,                        EC_F_COMPLETE,  /* TAB */
    0o17,0,                         EC_B_COMPLETE,  /* BACKTAB */
    SK, SK_BACKTAB, 6, 1, 1, 1, 0,  EC_B_COMPLETE,  /* BACKTAB */
    ESC,b'\t',0,                    EC_B_COMPLETE,  /* ESC TAB */
    b'L' & 0o37,0,                  EC_EXPAND,      /* CTRL-L */
    b'V' & 0o37,0,                  EC_LITERAL,     /* BACKSLASH */
    b'A' & 0o37,0,                  EC_LITERAL,     /* BACKSLASH */
    ESC, b'l',0,                    EC_RIGHT,       /* ESC l */
    SK, SK_RIGHT_ARROW, 6, 1, 1, 1, 0, EC_RIGHT, /* RIGHTARROW */
    ESC,b'h',0,                     EC_LEFT,        /* ESC h */
    SK, SK_LEFT_ARROW, 6, 1, 1, 1, 0, EC_LEFT, /* LEFTARROW */
    ESC,b'b',0,                     EC_W_LEFT,      /* ESC b */
    ESC,SK, SK_LEFT_ARROW, 6, 1, 1, 1, 0, EC_W_LEFT, /* ESC LEFTARROW */
    SK, SK_CTL_LEFT_ARROW, 6, 1, 1, 1, 0, EC_W_LEFT, /* CTRL-LEFTARROW */
    ESC,b'w',0,                     EC_W_RIGHT,     /* ESC w */
    ESC,SK, SK_RIGHT_ARROW, 6, 1, 1, 1, 0, EC_W_RIGHT, /* ESC RIGHTARROW */
    SK, SK_CTL_RIGHT_ARROW, 6, 1, 1, 1, 0, EC_W_RIGHT, /* CTRL-RIGHTARROW */
    ESC,b'i',0,                     EC_INSERT,        /* ESC i */
    SK, SK_INSERT, 6, 1, 1, 1, 0,   EC_INSERT,        /* INSERT */
    ESC,b'x',0,                     EC_DELETE,        /* ESC x */
    SK, SK_DELETE, 6, 1, 1, 1, 0,   EC_DELETE,        /* DELETE */
    ESC,b'X',0,                     EC_W_DELETE,      /* ESC X */
    ESC,SK, SK_DELETE, 6, 1, 1, 1, 0, EC_W_DELETE, /* ESC DELETE */
    SK, SK_CTL_DELETE, 6, 1, 1, 1, 0, EC_W_DELETE, /* CTRL-DELETE */
    SK, SK_CTL_BACKSPACE, 6, 1, 1, 1, 0, EC_W_BACKSPACE, /* CTRL-BACKSPACE */
    ESC,SK, SK_BACKSPACE, 6, 1, 1, 1, 0, EC_W_BACKSPACE, /* ESC BACKSPACE */
    ESC,b'0',0,                     EC_HOME,        /* ESC 0 */
    SK, SK_HOME, 6, 1, 1, 1, 0,     EC_HOME,        /* HOME */
    ESC,b'$',0,                     EC_END,         /* ESC $ */
    SK, SK_END, 6, 1, 1, 1, 0,      EC_END,         /* END */
    ESC,b'k',0,                     EC_UP,          /* ESC k */
    SK, SK_UP_ARROW, 6, 1, 1, 1, 0, EC_UP, /* UPARROW */
    ESC,b'j',0,                     EC_DOWN,        /* ESC j */
    SK, SK_DOWN_ARROW, 6, 1, 1, 1, 0, EC_DOWN, /* DOWNARROW */
    b'G' & 0o37,0,                  EC_ABORT,       /* CTRL-G */
    ESC,b'[',b'M',0,                EC_X11MOUSE,    /* X11 mouse report */
    ESC,b'[',b'<',0,                EC_X116MOUSE,   /* X11 1006 mouse report */
    ESC,b'[',b'2',b'0',b'0',b'~',0, A_START_PASTE,  /* open paste bracket */
    ESC,b'[',b'2',b'0',b'1',b'~',0, A_END_PASTE,    /* close paste bracket */
]
});

#[rustfmt::skip]
static mut dflt_vartable: LazyLock<Vec<u8>> = LazyLock::new(|| {
    vec![
        b'L',b'E',b'S',b'S',b'_',b'O',b'S',b'C',b'8',b'_',b'm',b'a',b'n', 0, EV_OK|A_EXTRA,
            /* echo '%o' | sed -e "s,^man\:\\([^(]*\\)( *\\([^)]*\\)\.*,-man '\\2' '\\1'," -e"t X" -e"s,\.*,-echo Invalid man link," -e"\: X" */
            b'e',b'c',b'h',b'o',b' ',b'\'',b'%',b'o',b'\'',b' ',b'|',b' ',b's',b'e',b'd',b' ',b'-',b'e',b' ',b'"',b's',b',',b'^',b'm',b'a',b'n',b'\\',b':',b'\\',b'\\',b'(',b'[',b'^',b'(',b']',b'*',b'\\',b'\\',b')',b'(',b' ',b'*',b'\\',b'\\',b'(',b'[',b'^',b')',b']',b'*',b'\\',b'\\',b')',b'\\',b'.',b'*',b',',b'-',b'm',b'a',b'n',b' ',b'\'',b'\\',b'\\',b'2',b'\'',b' ',b'\'',b'\\',b'\\',b'1',b'\'',b',',b'"',b' ',b'-',b'e',b'"',b't',b' ',b'X',b'"',b' ',b'-',b'e',b'"',b's',b',',b'\\',b'.',b'*',b',',b'-',b'e',b'c',b'h',b'o',b' ',b'I',b'n',b'v',b'a',b'l',b'i',b'd',b' ',b'm',b'a',b'n',b' ',b'l',b'i',b'n',b'k',b',',b'"',b' ',b'-',b'e',b'"',b'\\',b':',b' ',b'X',b'"',
            0,

        b'L',b'E',b'S',b'S',b'_',b'O',b'S',b'C',b'8',b'_',b'f',b'i',b'l',b'e', 0, EV_OK|A_EXTRA,
            /* eval `echo '%o' | sed -e "s,^file://\\([^/]*\\)\\(.*\\),_H=\\1;_P=\\2;_E=0," -e"t X" -e"s,.*,_E=1," -e": X"`; if [ "$_E" = 1 ]; then echo -echo Invalid file link; elif [ -z "$_H" -o "$_H" = localhost -o "$_H" = $HOSTNAME ]; then echo ":e $_P"; else echo -echo Cannot open remote file on "$_H"; fi */
            b'e',b'v',b'a',b'l',b' ',b'`',b'e',b'c',b'h',b'o',b' ',b'\'',b'%',b'o',b'\'',b' ',b'|',b' ',b's',b'e',b'd',b' ',b'-',b'e',b' ',b'"',b's',b',',b'^',b'f',b'i',b'l',b'e',b'\\',b':',b'/',b'/',b'\\',b'\\',b'(',b'[',b'^',b'/',b']',b'*',b'\\',b'\\',b')',b'\\',b'\\',b'(',b'\\',b'.',b'*',b'\\',b'\\',b')',b',',b'_',b'H',b'=',b'\\',b'\\',b'1',b';',b'_',b'P',b'=',b'\\',b'\\',b'2',b';',b'_',b'E',b'=',b'0',b',',b'"',b' ',b'-',b'e',b'"',b't',b' ',b'X',b'"',b' ',b'-',b'e',b'"',b's',b',',b'\\',b'.',b'*',b',',b'_',b'E',b'=',b'1',b',',b'"',b' ',b'-',b'e',b'"',b'\\',b':',b' ',b'X',b'"',b'`',b';',b' ',b'i',b'f',b' ',b'[',b' ',b'"',b'$',b'_',b'E',b'"',b' ',b'=',b' ',b'1',b' ',b']',b';',b' ',b't',b'h',b'e',b'n',b' ',b'e',b'c',b'h',b'o',b' ',b'-',b'e',b'c',b'h',b'o',b' ',b'I',b'n',b'v',b'a',b'l',b'i',b'd',b' ',b'f',b'i',b'l',b'e',b' ',b'l',b'i',b'n',b'k',b';',b' ',b'e',b'l',b'i',b'f',b' ',b'[',b' ',b'-',b'z',b' ',b'"',b'$',b'_',b'H',b'"',b' ',b'-',b'o',b' ',b'"',b'$',b'_',b'H',b'"',b' ',b'=',b' ',b'l',b'o',b'c',b'a',b'l',b'h',b'o',b's',b't',b' ',b'-',b'o',b' ',b'"',b'$',b'_',b'H',b'"',b' ',b'=',b' ',b'$',b'H',b'O',b'S',b'T',b'N',b'A',b'M',b'E',b' ',b']',b';',b' ',b't',b'h',b'e',b'n',b' ',b'e',b'c',b'h',b'o',b' ',b'"',b'\\',b':',b'e',b' ',b'$',b'_',b'P',b'"',b';',b' ',b'e',b'l',b's',b'e',b' ',b'e',b'c',b'h',b'o',b' ',b'-',b'e',b'c',b'h',b'o',b' ',b'C',b'a',b'n',b'n',b'o',b't',b' ',b'o',b'p',b'e',b'n',b' ',b'r',b'e',b'm',b'o',b't',b'e',b' ',b'f',b'i',b'l',b'e',b' ',b'o',b'n',b' ',b'"',b'$',b'_',b'H',b'"',b';',b' ',b'f',b'i',
            0,
    ]
}
);

/*
 * Expand special key abbreviations in a command table.
 */
unsafe extern "C" fn expand_special_keys(table: &mut Table) {
    let mut fm = 0;
    let mut to = fm;
    let len = table.table.len();
    while fm < len {
        /*
         * Rewrite each command in the table with any
         * special key abbreviations expanded.
         */
        to = fm;
        while to < fm && table.table[fm] != 0 {
            if table.table[fm] != SK_SPECIAL_KEY {
                table.table[to] = table.table[fm];
                to += 1;
                fm += 1;
                continue;
            }

            if fm + 2 > len {
                break; // not enough data
            }

            /*
             * After SK_SPECIAL_KEY, next byte is the type
             * of special key (one of the SK_* constants),
             * and the byte after that is the number of bytes,
             * N, reserved by the abbreviation (including the
             * SK_SPECIAL_KEY and key type bytes).
             * Replace all N bytes with the actual bytes
             * output by the special key on this terminal.
             */

            let key_type = table.table[fm + 1];
            let klen = table.table[fm + 2] as usize;
            let repl = special_key_str(key_type);
            fm += klen;
            let replacement = match repl {
                Some(r) if r.len() <= klen => r.as_bytes(),
                _ => &[0xFF], // fallback
            };
            for &b in replacement {
                if to < len {
                    table.table[to] = b;
                    to += 1;
                }
            }
        }

        // write null terminator
        if to < table.table.len() {
            table.table[to] = 0;
            to += 1;
        }

        /*
         * Fill any unused bytes between end of command and
         * the action byte with A_SKIP.
         */
        while to <= fm && to < len {
            table.table[to] = A_SKIP;
            to += 1;
        }

        fm += 1;
        if fm >= len {
            break;
        }

        let a = table.table[fm] & 0xFF;
        if a & A_EXTRA != 0 {
            // skip over extra data (null-terminated string)
            while fm < table.table.len() && table.table[fm] != 0 {
                fm += 1;
            }
            fm += 1; // skip the null terminator
        }
    }
}

/*
 * Expand special key abbreviations in a list of command tables.
 */
unsafe extern "C" fn expand_cmd_table(t_list: &mut [Table]) {
    for t in t_list {
        expand_special_keys(t);
    }
}

/*
 * Expand special key abbreviations in all command tables.
 */
pub unsafe extern "C" fn expand_cmd_tables(tables: &mut Tables) {
    expand_cmd_table(&mut tables.fcmd_tables);
    expand_cmd_table(&mut tables.ecmd_tables);
    expand_cmd_table(&mut tables.var_tables);
    expand_cmd_table(&mut tables.sysvar_tables);
}

/*
 * Initialize the command lists.
 */
pub unsafe extern "C" fn init_cmds(tables: &mut Tables) {
    add_fcmd_table(tables, Table::from(&cmdtable));
    add_ecmd_table(tables, Table::from(&edittable));
    add_sysvar_table(tables, &mut dflt_vartable, dflt_vartable.len());
    add_hometable(
        tables,
        Some(lesskey as unsafe extern "C" fn(&mut Tables, &[u8], bool) -> i32),
        None,
        b"/usr/local/bin/.sysless\0" as *const u8 as *const std::ffi::c_char,
        true,
    );
    if add_hometable(
        tables,
        Some(lesskey_src as unsafe extern "C" fn(&mut Tables, &[u8], bool) -> i32),
        Some("LESSKEYIN_SYSTEM"),
        b"/usr/local/etc/syslesskey\0" as *const u8 as *const std::ffi::c_char,
        true,
    ) != 0 as std::ffi::c_int
    {
        add_hometable(
            tables,
            Some(lesskey as unsafe extern "C" fn(&mut Tables, &[u8], bool) -> i32),
            Some("LESSKEY_SYSTEM"),
            b"/usr/local/etc/sysless\0" as *const u8 as *const std::ffi::c_char,
            true,
        );
    }
    if add_hometable(
        tables,
        Some(lesskey_src as unsafe extern "C" fn(&mut Tables, &[u8], bool) -> i32),
        Some("LESSKEYIN"),
        b".lesskey\0" as *const u8 as *const std::ffi::c_char,
        false,
    ) != 0 as std::ffi::c_int
    {
        add_hometable(
            tables,
            Some(lesskey as unsafe extern "C" fn(&mut Tables, &[u8], bool) -> i32),
            Some("LESSKEY"),
            b".less\0" as *const u8 as *const std::ffi::c_char,
            false,
        );
    }
    add_content_table(
        tables,
        Some(lesskey_content as unsafe extern "C" fn(&mut Tables, &[u8], bool) -> i32),
        "LESSKEY_CONTENT_SYSTEM",
        true,
    );
    add_content_table(
        tables,
        Some(lesskey_content as unsafe extern "C" fn(&mut Tables, &[u8], bool) -> i32),
        "LESSKEY_CONTENT",
        false,
    );
}
unsafe extern "C" fn add_cmd_table(tables: &mut Vec<Table>, table: Table) {
    tables.push(table);
}

unsafe extern "C" fn pop_cmd_table(table: &mut Vec<Table>) {
    table.pop().unwrap();
}

pub unsafe extern "C" fn add_fcmd_table(tables: &mut Tables, table: Table) {
    add_cmd_table(&mut tables.fcmd_tables, table);
}

pub unsafe extern "C" fn add_ecmd_table(tables: &mut Tables, table: Table) {
    add_cmd_table(&mut tables.fcmd_tables, table);
}

/*
 * Add an environment variable table.
 */
unsafe extern "C" fn add_var_table(tables: &mut Tables, buf: &mut [u8], len: usize) {
    // TODO review hardcoded value
    let mut xbuf = XBuffer::new(16);
    expand_evars(tables, buf, len, &mut xbuf);
    add_cmd_table(&mut tables.var_tables, table_from_xbuf(xbuf));
}

pub unsafe extern "C" fn add_uvar_table(tables: &mut Tables, buf: &mut [u8], len: usize) {
    add_var_table(tables, buf, len);
}

pub unsafe extern "C" fn add_sysvar_table(tables: &mut Tables, buf: &mut [u8], len: usize) {
    add_var_table(tables, buf, len);
}

/*
 * Return action for a mouse wheel down event.
 */
unsafe extern "C" fn mouse_wheel_down() -> i32 {
    return if mousecap == OPT_ONPLUS as i32 {
        A_F_MOUSE as i32
    } else {
        A_B_MOUSE as i32
    };
}

/*
 * Return action for a mouse wheel up event.
 */
unsafe extern "C" fn mouse_wheel_up() -> i32 {
    return if mousecap == OPT_ONPLUS as i32 {
        A_F_MOUSE as i32
    } else {
        A_B_MOUSE as i32
    };
}

/*
 * Return action for the left mouse button trigger.
 */
unsafe extern "C" fn mouse_button_left(x: i32, y: i32, down: bool, drag: bool) -> i32 {
    static mut last_drag_y: i32 = -1;
    static mut last_click_y: i32 = -1;
    if down && !drag {
        last_click_y = y;
        last_drag_y = last_click_y;
    }
    if allow_drag && drag && last_drag_y >= 0 {
        /* Drag text up/down */
        if y > last_drag_y {
            cmd_exec();
            backward(y - last_drag_y, LFALSE, LFALSE, LFALSE);
            last_drag_y = y;
        } else if y < last_drag_y {
            cmd_exec();
            forward(last_drag_y - y, LFALSE, LFALSE, LFALSE);
            last_drag_y = y;
        }
    } else if !down {
        if osc8_click(y, x) as u64 != 0 {
            return A_NOACTION as i32;
        }
        if y < sc_height - 1 && y == last_click_y {
            setmark('#' as i32 as std::ffi::c_char, y);
            screen_trashed();
        }
    }
    A_NOACTION as i32
}

/*
 * Return action for the right mouse button trigger.
 */
unsafe extern "C" fn mouse_button_right(x: i32, y: i32, down: bool, drag: bool) -> i32 {
    /*
     * {{ unlike mouse_button_left, we could return an action,
     *    but keep it near mouse_button_left for readability. }}
     */
    if !down && y < sc_height - 1 {
        gomark('#' as i32 as std::ffi::c_char);
        screen_trashed();
    }
    A_NOACTION as i32
}

/*
 * Read a decimal integer. Return the integer and set *pterm to the terminating char.
 */
unsafe extern "C" fn getcc_int(pterm: &mut Option<u8>) -> Result<i32, ()> {
    let mut num = 0;
    let mut digits = 0;
    loop {
        let mut ch = getcc() as u8;
        if ch < b'0' || ch > b'9' {
            if !pterm.is_none() {
                *pterm = Some(ch);
            }
            if digits == 0 {
                return Ok(-1);
            }
            return Ok(num);
        }
        let digit = (ch - b'0') as i32;

        // check for overflow
        num = num
            .checked_mul(10)
            .and_then(|n| n.checked_add(digit))
            .ok_or(())?;
        digits += 1;
    }
}

unsafe extern "C" fn x11mouse_button(btn: i32, x: i32, y: i32, down: bool, drag: bool) -> i32 {
    match btn as u8 {
        X11MOUSE_BUTTON1 => return mouse_button_left(x, y, down, drag),
        X11MOUSE_BUTTON2 | X11MOUSE_BUTTON3 => return mouse_button_right(x, y, down, drag),
        _ => {}
    }
    A_NOACTION as i32
}

/*
 * Read suffix of mouse input and return the action to take.
 * The prefix ("\e[M") has already been read.
 */
unsafe extern "C" fn x11mouse_action(skip: bool) -> i32 {
    static mut prev_b: i32 = X11MOUSE_BUTTON_REL as i32;
    let mut x = 0;
    let mut y = 0;
    let mut b = getcc() as i32 - 0x20;
    let drag = (b & (X11MOUSE_DRAG as i32)) != 0;

    b &= !(X11MOUSE_DRAG as i32);
    x = getcc() as i32 - (X11MOUSE_OFFSET as i32) - 1;
    y = getcc() as i32 - (X11MOUSE_OFFSET as i32) - 1;
    if skip {
        return A_NOACTION as i32;
    }
    match b as u8 {
        X11MOUSE_WHEEL_DOWN => return mouse_wheel_down(),
        X11MOUSE_WHEEL_UP => return mouse_wheel_up(),
        X11MOUSE_BUTTON1 | X11MOUSE_BUTTON2 | X11MOUSE_BUTTON3 => {
            prev_b = b;
            return x11mouse_button(b, x, y, true, drag);
        }
        X11MOUSE_BUTTON_REL => return x11mouse_button(prev_b, x, y, false, drag),
        _ => {}
    }
    A_NOACTION as i32
}

/*
 * Read suffix of mouse input and return the action to take.
 * The prefix ("\e[<") has already been read.
 */
unsafe extern "C" fn x116mouse_action(skip: bool) -> i32 {
    let mut ch: u8 = 0;
    let mut x = 0;
    let mut y = 0;
    let mut b = getcc_int(&mut Some(ch)).unwrap();
    let mut drag = ((b as u8) & X11MOUSE_DRAG) != 0;
    b &= !(X11MOUSE_DRAG as i32);
    if b < 0 || ch != b';' {
        return A_NOACTION as i32;
    }
    x = getcc_int(&mut Some(ch)).unwrap() - 1;
    if x < 0 || ch != b';' {
        return A_NOACTION as i32;
    }
    y = getcc_int(&mut Some(ch)).unwrap() - 1;
    if y < 0 {
        return A_NOACTION as i32;
    }
    if skip {
        return A_NOACTION as i32;
    }
    match b as u8 {
        X11MOUSE_WHEEL_DOWN => return mouse_wheel_down(),
        X11MOUSE_WHEEL_UP => return mouse_wheel_up(),
        X11MOUSE_BUTTON1 | X11MOUSE_BUTTON2 | X11MOUSE_BUTTON3 => {
            let mut down = ch == b'M';
            let mut up = ch == b'm';
            if up || down {
                return x11mouse_button(b, x, y, down, drag);
            }
        }
        _ => {}
    }
    A_NOACTION as i32
}

fn strncmp_u8(s1: &[u8], s2: &[u8], n: usize) -> i32 {
    let mut i = 0;
    let mut c1 = 0;
    let mut c2 = 0;
    loop {
        c1 = s1[i] as i32;
        c2 = s2[i] as i32;
        if c1 == 0 || c1 != c2 {
            return c1 - c2;
        }
        i += 1;
        if i >= n {
            break;
        }
    }
    c1 - c2
}

/*
 * Return the largest N such that the first N chars of goal
 * are equal to the last N chars of s.
 */
unsafe extern "C" fn cmd_match(goal: &[u8], s: &[u8]) -> usize {
    let slen = s.len();
    let mut len = slen;
    while len > 0 {
        if strncmp_u8(&s[slen - len..], goal, len) == 0 {
            break;
        }
        len -= 1;
    }
    len
}

/*
 * Return pointer to next command table entry.
 * Also return the action and the extra string from the entry.
 */
unsafe extern "C" fn cmd_next_entry(
    mut entry: &[u8],
    action: &mut Option<u8>,
    extra: &mut Option<usize>,
    cmdlen: &mut Option<usize>,
) -> usize {
    let mut i = 0;

    // 1. Find end of command (null-terminated)
    while i < entry.len() && entry[i] != 0 {
        i += 1;
    }

    if !cmdlen.is_none() {
        *cmdlen = Some(i);
    }
    i += 1; // skip the null byte

    // 2. Skip A_SKIP bytes to find action
    while i < entry.len() && entry[i] == A_SKIP {
        i += 1;
    }

    let mut a = entry[i];

    // 3. Handle extra data if A_EXTRA is set
    if !extra.is_none() {
        *extra = Some(i);
    }
    if (a & A_EXTRA) != 0 {
        // Find end of extra (null-terminated)
        while i < entry.len() && entry[i] != 0 {
            i += 1;
        }

        i += 1; // skip the null byte
        a &= !A_EXTRA;
    }
    if !action.is_none() {
        *action = Some(a);
    }
    i
}

/*
 * Search a single command table for the command string in cmd.
 */
unsafe extern "C" fn cmd_search(
    cmd: &[u8],
    table: &Table,
    mut extra: &mut Option<usize>,
    mut mlen: &mut Option<usize>,
) -> i32 {
    let mut action = A_INVALID as i32;
    let mut match_len = 0;
    if !extra.is_none() {
        *extra = Some(0);
    }
    let i = 0;
    while i < table.table.len() {
        let mut taction: i32 = 0;
        let mut textra: usize = 0;
        let mut cmdlen = 0;
        let mut m = cmd_match(&table.table, cmd);
        let i = cmd_next_entry(
            &table.table[i..],
            &mut Some(taction as u8),
            &mut Some(textra),
            &mut Some(cmdlen),
        );
        if taction == A_END_LIST as i32 {
            return -action;
        }
        if m >= match_len {
            if m == cmdlen {
                /* (last chars of) cmd matches this table entry */
                action = taction;
                if !extra.is_none() {
                    *extra = Some(textra);
                }
            } else if m > 0 && action == A_INVALID as i32 {
                /* cmd is a prefix of this table entry */
                action = A_PREFIX as i32;
            }
            match_len = m;
        }
    }
    if !mlen.is_none() {
        *mlen = Some(match_len as usize);
    }
    action
}

/*
 * Decode a command character and return the associated action.
 * If "extra" string exists it's index and the the table index
 * where it is found will be returned
 */
unsafe extern "C" fn cmd_decode(
    tlist: &[Table],
    cmd: &[u8],
    mut extra_idx: &mut Option<(usize, usize)>,
) -> i32 {
    let mut action = A_INVALID as i32;
    let mut match_len = 0;

    /*
     * Search for the cmd thru all the command tables.
     * If we find it more than once, take the last one.
     */
    let mut spi = 0;
    *extra_idx = None;
    let mut t_idx = 0;
    for t in tlist {
        let mut tsp = 0;
        let mut mlen = 0;
        let mut taction = cmd_search(cmd, t, &mut Some(tsp), &mut Some(mlen));
        if mlen >= match_len {
            match_len = mlen;
            if taction == A_UINVALID as i32 {
                taction = A_INVALID as i32;
            }
            if taction != A_INVALID as i32 {
                spi = tsp;
                *extra_idx = Some((spi, t_idx));
                if taction < 0 {
                    action = -taction;
                    break;
                } else {
                    action = taction;
                }
            }
        }
        t_idx += 1;
    }
    if action == A_X11MOUSE_IN as i32 {
        action = x11mouse_action(false);
    } else if action == A_X116MOUSE_IN as i32 {
        action = x116mouse_action(false);
    }
    return action as i32;
}

/*
 * Decode a command from the cmdtables list.
 */
pub unsafe extern "C" fn fcmd_decode(
    tables: &Tables,
    cmd: &[u8],
    mut sp: &mut Option<(usize, usize)>,
) -> i32 {
    return cmd_decode(&tables.fcmd_tables, cmd, sp);
}

/*
 * Decode a command from the edittables list.
 */
pub unsafe extern "C" fn ecmd_decode(
    tables: &Tables,
    cmd: &[u8],
    sp: &mut Option<(usize, usize)>,
) -> i32 {
    return cmd_decode(&tables.ecmd_tables, cmd, sp);
}

/*
 * Get the value of an environment variable.
 * Looks first in the lesskey file, then in the real environment.
 */
pub fn lgetenv(key: &str) -> Result<String, VarError> {
    // FIXME add the lesskey file lookup
    /*
    let mut a: i32;
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    a = cmd_decode(list_var_tables, var, &mut s);
    if a == 0o1 as std::ffi::c_int {
        return s;
    }
    */
    env::var(key)
    /*
    a = cmd_decode(list_sysvar_tables, var, &mut s);
    if a == 0o1 as std::ffi::c_int {
        return s;
    }
    */
}

/*
 * Like lgetenv, but also uses a buffer partially filled with an env table.
 */
pub unsafe extern "C" fn lgetenv_ext(
    tables: &mut Tables,
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
    add_uvar_table(
        tables,
        CStr::from_ptr(env_buf as *const i8)
            .to_str()
            .unwrap()
            .to_owned()
            .as_bytes_mut(),
        env_end as usize,
    );
    r = CString::new(lgetenv(var).unwrap()).unwrap().as_ptr();
    pop_cmd_table(&mut tables.var_tables);
    return r;
}

pub unsafe extern "C" fn isnullenv(r: &Result<String, VarError>) -> bool {
    r.is_err() || r.as_ref().unwrap() != ""
}

unsafe extern "C" fn gint(buf: &[u8]) -> usize {
    let mut n = 0;
    n = buf[0] as usize;
    n += (buf[1] as usize) * KRADIX as usize;
    return n;
}

/*
 * Process an old (pre-v241) lesskey file.
 */
unsafe extern "C" fn old_lesskey(tables: &mut Tables, mut buf: &[u8], mut len: usize) -> i32 {
    /*
     * Old-style lesskey file.
     * The file must end with either
     *     ...,cmd,0,action
     * or  ...,cmd,0,action|A_EXTRA,string,0
     * So the last byte or the second to last byte must be zero.
     */
    if buf[len - 1] != b'\0' && buf[len - 2] != b'\0' {
        return -1;
    }
    add_fcmd_table(tables, Table::from(buf));
    return 0;
}

/*
 * Process a new (post-v241) lesskey file.
 */
unsafe extern "C" fn new_lesskey(tables: &mut Tables, buf: &mut [u8], sysvar: bool) -> i32 {
    let mut p: usize = 0;
    let mut end = 0;
    let mut c = 0;
    let mut n: usize = 0;

    let len = buf.len();
    if buf[len - 3] != C0_END_LESSKEY_MAGIC
        || buf[len - 2] != C1_END_LESSKEY_MAGIC
        || buf[len - 1] != C2_END_LESSKEY_MAGIC
    {
        return -1;
    }
    p += 4;
    end = len;
    loop {
        c = buf[p];
        p += 1;
        match c {
            CMD_SECTION => {
                n = gint(&mut buf[p..]);
                if p + n >= end {
                    return -1;
                }
                add_fcmd_table(tables, Table::from(&buf[p..]));
                p += n;
            }
            EDIT_SECTION => {
                n = gint(&mut buf[p..]);
                if p + n >= end {
                    return -1;
                }
                add_ecmd_table(tables, Table::from(&buf[p..]));
                p += n;
            }
            VAR_SECTION => {
                n = gint(&mut &buf[p..]);
                if p + n >= end {
                    return -1;
                }
                if sysvar {
                    add_sysvar_table(tables, &mut buf[p..], n as usize);
                } else {
                    add_uvar_table(tables, &mut buf[p..], n as usize);
                }
                p += n;
            }
            END_SECTION => return 0,
            _ => {
                /*
                 * Unrecognized section type.
                 */
                return -1;
            }
        }
    }
}

/*
 * Set up a user command table, based on a "lesskey" file.
 */
pub unsafe extern "C" fn lesskey(tables: &mut Tables, filename: &[u8], sysvar: bool) -> i32 {
    let mut buf: *mut std::ffi::c_uchar = 0 as *mut std::ffi::c_uchar;
    let mut len: POSITION = 0;
    let mut n = 0;
    let mut f: std::ffi::c_int = 0;

    if secure_allow(SF_LESSKEY) != 0 {
        return 1;
    }
    /*
     * Try to open the lesskey file.
     */
    f = open(filename.as_ptr() as *const std::ffi::c_char, 0);
    if f < 0 {
        return 1;
    }

    /*
     * Read the file into a buffer.
     * We first figure out the size of the file and allocate space for it.
     * {{ Minimal error checking is done here.
     *    A garbage .less file will produce strange results.
     *    To avoid a large amount of error checking code here, we
     *    rely on the lesskey program to generate a good .less file. }}
     */
    len = filesize(f);
    if len == (NULL_POSITION as i64) || len < 3 {
        /*
         * Bad file (valid file must have at least 3 chars).
         */
        close(f);
        return -1;
    }
    buf = calloc(
        len as size_t,
        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_uchar;
    if buf.is_null() {
        close(f);
        return -1;
    }
    if lseek(f, 0, SEEK_SET) == BAD_LSEEK {
        free(buf as *mut std::ffi::c_void);
        close(f);
        return -1;
    }

    n = read(f, buf as *mut std::ffi::c_void, len as size_t);
    close(f);
    if n != len {
        free(buf as *mut std::ffi::c_void);
        return -1;
    }
    let rbuf = std::slice::from_raw_parts_mut(buf, len as usize);
    /*
     * Figure out if this is an old-style (before version 241)
     * or new-style lesskey file format.
     */
    if len < 4
        || rbuf[0] != C0_LESSKEY_MAGIC
        || rbuf[1] != C1_LESSKEY_MAGIC
        || rbuf[2] != C2_LESSKEY_MAGIC
        || rbuf[3] != C3_LESSKEY_MAGIC
    {
        return old_lesskey(tables, rbuf, len as usize);
    }
    return new_lesskey(tables, rbuf, sysvar);
}

unsafe extern "C" fn lesskey_text(
    ttables: &mut Tables,
    filename: &[u8],
    sysvar: bool,
    content: bool,
) -> i32 {
    let mut r = 0;
    static mut tables: LazyLock<lesskey_tables> = LazyLock::new(|| lesskey_tables {
        currtable: 0 as *const lesskey_table as *mut lesskey_table,
        cmdtable: lesskey_table {
            names: 0 as *const lesskey_cmdname,
            buf: XBuffer::new(16),
            is_var: 0,
        },
        edittable: lesskey_table {
            names: 0 as *const lesskey_cmdname,
            buf: XBuffer::new(16),
            is_var: 0,
        },
        vartable: lesskey_table {
            names: 0 as *const lesskey_cmdname,
            buf: XBuffer::new(16),
            is_var: 0,
        },
    });
    if !secure_allow(SF_LESSKEY) == 0 {
        return 1;
    }
    r = if content {
        parse_lesskey_content(filename.as_ptr() as *const std::ffi::c_char, &mut *tables)
    } else {
        parse_lesskey(filename.as_ptr() as *const std::ffi::c_char, &mut *tables)
    };
    if r != 0 {
        return r;
    }
    add_fcmd_table(ttables, Table::from(&tables.cmdtable.buf.data));
    add_ecmd_table(ttables, Table::from(&tables.edittable.buf.data));
    if sysvar {
        add_sysvar_table(
            ttables,
            &mut tables.vartable.buf.data,
            tables.vartable.buf.data.len(),
        );
    } else {
        add_uvar_table(
            ttables,
            &mut tables.vartable.buf.data,
            tables.vartable.buf.data.len(),
        );
    }
    return 0;
}

pub unsafe extern "C" fn lesskey_src(tables: &mut Tables, filename: &[u8], sysvar: bool) -> i32 {
    return lesskey_text(tables, filename, sysvar, false);
}

pub unsafe extern "C" fn lesskey_content(tables: &mut Tables, content: &[u8], sysvar: bool) -> i32 {
    return lesskey_text(tables, content, sysvar, true);
}

pub unsafe extern "C" fn lesskey_parse_error(mut s: *mut std::ffi::c_char) {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    parg.p_string = s;
    error(b"%s\0" as *const u8 as *const std::ffi::c_char, &mut parg);
}

/*
 * Add a lesskey file.
 */
unsafe extern "C" fn add_hometable(
    tables: &mut Tables,
    mut call_lesskey: Option<unsafe extern "C" fn(&mut Tables, &[u8], bool) -> i32>,
    envname: Option<&str>,
    mut def_filename: *const std::ffi::c_char,
    mut sysvar: bool,
) -> std::ffi::c_int {
    let mut filename: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut efilename: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut r: std::ffi::c_int = 0;
    if let Some(name) = envname {
        if let Ok(efilename) = lgetenv(name) {
            filename = save(CString::new(efilename).unwrap().as_ptr());
        }
    } else if sysvar {
        filename = save(def_filename);
    } else {
        if let Ok(xdg) = lgetenv("XDG_CONFIG_HOME") {
            filename = dirfile(
                CString::new(xdg).unwrap().as_ptr(),
                &*def_filename.offset(1 as std::ffi::c_int as isize),
                1 as std::ffi::c_int,
            );
        }
        if filename.is_null() {
            if let Ok(home) = lgetenv("HOME") {
                let mut cfg_dir: *mut std::ffi::c_char = dirfile(
                    CString::new(home).unwrap().as_ptr(),
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
        .expect("non-null function pointer")(
        tables,
        ptr_to_str(filename).unwrap().as_bytes(),
        sysvar,
    );
    free(filename as *mut std::ffi::c_void);
    return r;
}

/*
 * Add the content of a lesskey source file.
 */
unsafe extern "C" fn add_content_table(
    tables: &mut Tables,
    call_lesskey: Option<unsafe extern "C" fn(&mut Tables, &[u8], bool) -> i32>,
    envname: &str,
    sysvar: bool,
) {
    if let Ok(content) = lgetenv(envname) {
        lesskey_content(tables, content.as_bytes(), sysvar);
    }
}

/*
 * See if a char is a special line-editing command.
 */
pub unsafe extern "C" fn editchar(tables: &Tables, c: u8, flags: i32) -> i32 {
    let mut action = 0;
    let mut nch = 0;
    let mut sidx: Option<(usize, usize)> = Some((0, 0));
    let mut usercmd: [u8; MAX_CMDLEN + 1] = [0; MAX_CMDLEN + 1];

    /*
     * An editing character could actually be a sequence of characters;
     * for example, an escape sequence sent by pressing the uparrow key.
     * To match the editing string, we use the command decoder
     * but give it the edit-commands command table
     * This table is constructed to match the user's keyboard.
     */
    if c == (erase_char as u8) || c == (erase2_char as u8) {
        return EC_BACKSPACE as i32;
    }
    if c == kill_char as u8 {
        return EC_LINEKILL as i32;
    }
    loop {
        let ch = if nch > 0 { getcc() as u8 } else { c };
        usercmd[nch] = ch;
        usercmd[nch + 1] = b'\0';
        nch += 1;
        action = ecmd_decode(tables, &usercmd, &mut sidx);
        if !(action == A_PREFIX as i32 && nch < MAX_CMDLEN) {
            break;
        }
    }
    if action == EC_X11MOUSE as i32 {
        return x11mouse_action(true);
    }
    if action == EC_X116MOUSE as i32 {
        return x116mouse_action(true);
    }
    if flags & (ECF_NORIGHTLEFT as i32) != 0 {
        match action as u8 {
            EC_RIGHT | EC_LEFT => {
                action = A_INVALID as i32;
            }
            _ => {}
        }
    }
    if flags & (ECF_NOHISTORY as i32) != 0 {
        /*
         * The caller says there is no history list.
         * Reject any history-manipulation action.
         */
        match action as u8 {
            EC_UP | EC_DOWN => {
                action = A_INVALID as i32;
            }
            _ => {}
        }
    }
    if flags & (ECF_NOCOMPLETE as i32) != 0 {
        match action as u8 {
            EC_F_COMPLETE | EC_B_COMPLETE | EC_EXPAND => {
                action = A_INVALID as i32;
            }
            _ => {}
        }
    }
    if (flags & (ECF_PEEK as i32) != 0) || (action == A_INVALID as i32) {
        /*
         * We're just peeking, or we didn't understand the command.
         * Unget all the characters we read in the loop above.
         * This does NOT include the original character that was
         * passed in as a parameter.
         */
        while nch > 1 {
            nch -= 1;
            ungetcc(usercmd[nch as usize] as i8);
        }
    } else if !sidx.is_none() {
        ungetsc(tables.ecmd_tables[sidx.unwrap().1].table[sidx.unwrap().0..].as_ptr() as *const i8);
    }
    return action;
}
