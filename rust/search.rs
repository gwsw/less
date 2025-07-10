use ::c2rust_bitfields;
use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ansi_state_0;
    pub type re_dfa_t;
    fn snprintf(
        _: *mut std::ffi::c_char,
        _: std::ffi::c_ulong,
        _: *const std::ffi::c_char,
        _: ...
    ) -> std::ffi::c_int;
    fn pclose(__stream: *mut FILE) -> std::ffi::c_int;
    fn popen(__command: *const std::ffi::c_char, __modes: *const std::ffi::c_char) -> *mut FILE;
    fn iswupper(__wc: wint_t) -> std::ffi::c_int;
    fn free(_: *mut std::ffi::c_void);
    fn abs(_: std::ffi::c_int) -> std::ffi::c_int;
    fn strcpy(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strncpy(
        _: *mut std::ffi::c_char,
        _: *const std::ffi::c_char,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_char;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strncmp(
        _: *const std::ffi::c_char,
        _: *const std::ffi::c_char,
        _: std::ffi::c_ulong,
    ) -> std::ffi::c_int;
    fn strchr(_: *const std::ffi::c_char, _: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn saven(s: *const std::ffi::c_char, n: size_t) -> *mut std::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
    fn skipsp(s: *mut std::ffi::c_char) -> *mut std::ffi::c_char;
    fn lower_left();
    fn goto_line(sindex: std::ffi::c_int);
    fn clear_eol();
    fn ch_end_seek() -> std::ffi::c_int;
    fn ch_length() -> POSITION;
    fn ch_getflags() -> std::ffi::c_int;
    fn step_charc(
        pp: *mut *const std::ffi::c_char,
        dir: std::ffi::c_int,
        limit: *const std::ffi::c_char,
    ) -> LWCHAR;
    fn set_mlist(mlist: *mut std::ffi::c_void, cmdflags: std::ffi::c_int);
    fn cmd_lastpattern() -> *const std::ffi::c_char;
    fn screen_trashed();
    fn cvt_length(len: size_t, ops: std::ffi::c_int) -> size_t;
    fn cvt_alloc_chpos(len: size_t) -> *mut std::ffi::c_int;
    fn cvt_text(
        odst: *mut std::ffi::c_char,
        osrc: *const std::ffi::c_char,
        chpos: *mut std::ffi::c_int,
        lenp: *mut size_t,
        ops: std::ffi::c_int,
    );
    fn lgetenv(var: *const std::ffi::c_char) -> *const std::ffi::c_char;
    fn isnullenv(s: *const std::ffi::c_char) -> lbool;
    fn edit(filename: *const std::ffi::c_char) -> std::ffi::c_int;
    fn readfd(fd: *mut FILE) -> *mut std::ffi::c_char;
    fn overlay_header() -> std::ffi::c_int;
    fn forw_line(curr_pos: POSITION, p_linepos: *mut POSITION, p_newline: *mut lbool) -> POSITION;
    fn repaint();
    fn jump_loc(pos: POSITION, sline: std::ffi::c_int);
    fn line_pfx_width() -> std::ffi::c_int;
    fn ansi_start(ch: LWCHAR) -> *mut ansi_state_0;
    fn ansi_step(pansi: *mut ansi_state_0, ch: LWCHAR) -> ansi_state;
    fn ansi_osc8_state(pansi: *mut ansi_state_0) -> osc8_state;
    fn ansi_done(pansi: *mut ansi_state_0);
    fn col_from_pos(
        linepos: POSITION,
        spos: POSITION,
        saved_pos: POSITION,
        saved_col: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn pos_from_col(
        linepos: POSITION,
        col: std::ffi::c_int,
        saved_pos: POSITION,
        saved_col: std::ffi::c_int,
    ) -> POSITION;
    fn forw_raw_line(
        curr_pos: POSITION,
        linep: *mut *const std::ffi::c_char,
        line_lenp: *mut size_t,
    ) -> POSITION;
    fn back_raw_line(
        curr_pos: POSITION,
        linep: *mut *const std::ffi::c_char,
        line_lenp: *mut size_t,
    ) -> POSITION;
    fn skip_columns(
        cols: std::ffi::c_int,
        linep: *mut *const std::ffi::c_char,
        line_lenp: *mut size_t,
    ) -> std::ffi::c_int;
    fn add_lnum(linenum: LINENUM, pos: POSITION);
    fn find_linenum(pos: POSITION) -> LINENUM;
    fn find_pos(linenum: LINENUM) -> POSITION;
    fn lsystem(cmd: *const std::ffi::c_char, donemsg: *const std::ffi::c_char);
    fn chop_line() -> std::ffi::c_int;
    fn put_line(forw_scroll: lbool);
    fn flush();
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn compile_pattern(
        pattern: *const std::ffi::c_char,
        search_type: std::ffi::c_int,
        show_error: std::ffi::c_int,
        comp_pattern: *mut *mut regex_t,
    ) -> std::ffi::c_int;
    fn uncompile_pattern(pattern: *mut *mut regex_t);
    fn is_null_pattern(pattern: *mut regex_t) -> lbool;
    fn match_pattern(
        pattern: *mut regex_t,
        tpattern: *const std::ffi::c_char,
        line: *const std::ffi::c_char,
        line_len: size_t,
        line_off: size_t,
        sp: *mut *const std::ffi::c_char,
        ep: *mut *const std::ffi::c_char,
        nsp: std::ffi::c_int,
        notbol: std::ffi::c_int,
        search_type: std::ffi::c_int,
    ) -> lbool;
    fn position(sindex: std::ffi::c_int) -> POSITION;
    fn onscreen(pos: POSITION) -> std::ffi::c_int;
    fn empty_screen() -> std::ffi::c_int;
    fn get_scrpos(scrpos: *mut scrpos, where_0: std::ffi::c_int);
    fn sindex_from_sline(sline: std::ffi::c_int) -> std::ffi::c_int;
    fn pr_expand(proto: *const std::ffi::c_char) -> *const std::ffi::c_char;
    static mut sigs: std::ffi::c_int;
    static mut how_search: std::ffi::c_int;
    static mut caseless: std::ffi::c_int;
    static mut linenums: std::ffi::c_int;
    static mut jump_sline: std::ffi::c_int;
    static mut bs_mode: std::ffi::c_int;
    static mut proc_backspace: std::ffi::c_int;
    static mut proc_return: std::ffi::c_int;
    static mut ctldisp: std::ffi::c_int;
    static mut status_col: std::ffi::c_int;
    static mut ml_search: *mut std::ffi::c_void;
    static mut start_attnpos: POSITION;
    static mut end_attnpos: POSITION;
    static mut sc_width: std::ffi::c_int;
    static mut sc_height: std::ffi::c_int;
    static mut hshift: std::ffi::c_int;
    static mut match_shift: std::ffi::c_int;
    static mut nosearch_header_lines: std::ffi::c_int;
    static mut nosearch_header_cols: std::ffi::c_int;
    static mut header_lines: std::ffi::c_int;
    static mut header_cols: std::ffi::c_int;
    static mut rscroll_char: LWCHAR;
    static mut hilite_search: std::ffi::c_int;
    static mut squished: lbool;
    static mut can_goto_line: std::ffi::c_int;
}
pub type __off_t = std::ffi::c_long;
pub type __off64_t = std::ffi::c_long;
pub type off_t = __off_t;
pub type size_t = std::ffi::c_ulong;
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
pub type wint_t = std::ffi::c_uint;
pub type lbool = std::ffi::c_uint;
pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;
pub type LWCHAR = std::ffi::c_ulong;
pub type less_off_t = off_t;
pub type POSITION = less_off_t;
pub type LINENUM = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scrpos {
    pub pos: POSITION,
    pub ln: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union parg {
    pub p_string: *const std::ffi::c_char,
    pub p_int: std::ffi::c_int,
    pub p_linenum: LINENUM,
    pub p_char: std::ffi::c_char,
}
pub type PARG = parg;
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
pub type ansi_state = std::ffi::c_uint;
pub const ANSI_END: ansi_state = 3;
pub const ANSI_ERR: ansi_state = 2;
pub const ANSI_MID: ansi_state = 1;
pub const ANSI_NULL: ansi_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hilite_tree {
    pub first: *mut hilite_storage,
    pub current: *mut hilite_storage,
    pub root: *mut hilite_node,
    pub lookaside: *mut hilite_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hilite_node {
    pub parent: *mut hilite_node,
    pub left: *mut hilite_node,
    pub right: *mut hilite_node,
    pub prev: *mut hilite_node,
    pub next: *mut hilite_node,
    pub red: std::ffi::c_int,
    pub r: hilite,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hilite {
    pub hl_startpos: POSITION,
    pub hl_endpos: POSITION,
    pub hl_attr: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hilite_storage {
    pub capacity: size_t,
    pub used: size_t,
    pub next: *mut hilite_storage,
    pub nodes: *mut hilite_node,
}
pub type __re_long_size_t = std::ffi::c_ulong;
pub type reg_syntax_t = std::ffi::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub __buffer: *mut re_dfa_t,
    pub __allocated: __re_long_size_t,
    pub __used: __re_long_size_t,
    pub __syntax: reg_syntax_t,
    pub __fastmap: *mut std::ffi::c_char,
    pub __translate: *mut std::ffi::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "__can_be_null", ty = "std::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "__regs_allocated", ty = "std::ffi::c_uint", bits = "1..=2")]
    #[bitfield(name = "__fastmap_accurate", ty = "std::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "__no_sub", ty = "std::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "__not_bol", ty = "std::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "__not_eol", ty = "std::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "__newline_anchor", ty = "std::ffi::c_uint", bits = "7..=7")]
    pub __can_be_null___regs_allocated___fastmap_accurate___no_sub___not_bol___not_eol___newline_anchor:
        [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pattern_info {
    pub compiled: *mut regex_t,
    pub text: *mut std::ffi::c_char,
    pub search_type: std::ffi::c_int,
    pub is_ucase_pattern: lbool,
    pub next: *mut pattern_info,
}
pub const OSC8_NO_MATCH: osc8_match = 0;
pub type osc8_match = std::ffi::c_uint;
pub const OSC8_ALREADY: osc8_match = 2;
pub const OSC8_MATCH: osc8_match = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osc8_parse_info {
    pub osc8_start: *const std::ffi::c_char,
    pub osc8_end: *const std::ffi::c_char,
    pub params_start: *const std::ffi::c_char,
    pub params_end: *const std::ffi::c_char,
    pub uri_start: *const std::ffi::c_char,
    pub uri_end: *const std::ffi::c_char,
}
static mut hide_hilite: lbool = LFALSE;
static mut prep_startpos: POSITION = 0;
static mut prep_endpos: POSITION = 0;
#[no_mangle]
pub static mut header_start_pos: POSITION = -(1 as std::ffi::c_int) as POSITION;
static mut header_end_pos: POSITION = 0;
#[no_mangle]
pub static mut search_wrapped: lbool = LFALSE;
#[no_mangle]
pub static mut osc8_linepos: POSITION = -(1 as std::ffi::c_int) as POSITION;
#[no_mangle]
pub static mut osc8_match_start: POSITION = -(1 as std::ffi::c_int) as POSITION;
#[no_mangle]
pub static mut osc8_match_end: POSITION = -(1 as std::ffi::c_int) as POSITION;
#[no_mangle]
pub static mut osc8_params_start: POSITION = -(1 as std::ffi::c_int) as POSITION;
#[no_mangle]
pub static mut osc8_params_end: POSITION = -(1 as std::ffi::c_int) as POSITION;
#[no_mangle]
pub static mut osc8_uri_start: POSITION = -(1 as std::ffi::c_int) as POSITION;
#[no_mangle]
pub static mut osc8_uri_end: POSITION = -(1 as std::ffi::c_int) as POSITION;
#[no_mangle]
pub static mut osc8_text_start: POSITION = -(1 as std::ffi::c_int) as POSITION;
#[no_mangle]
pub static mut osc8_text_end: POSITION = -(1 as std::ffi::c_int) as POSITION;
#[no_mangle]
pub static mut osc8_path: *mut std::ffi::c_char =
    0 as *const std::ffi::c_char as *mut std::ffi::c_char;
#[no_mangle]
pub static mut osc8_uri: *mut std::ffi::c_char =
    0 as *const std::ffi::c_char as *mut std::ffi::c_char;
#[no_mangle]
pub static mut osc8_search_param: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut hilite_anchor: hilite_tree = {
    let mut init = hilite_tree {
        first: 0 as *const hilite_storage as *mut hilite_storage,
        current: 0 as *const hilite_storage as *mut hilite_storage,
        root: 0 as *const hilite_node as *mut hilite_node,
        lookaside: 0 as *const hilite_node as *mut hilite_node,
    };
    init
};
static mut filter_anchor: hilite_tree = {
    let mut init = hilite_tree {
        first: 0 as *const hilite_storage as *mut hilite_storage,
        current: 0 as *const hilite_storage as *mut hilite_storage,
        root: 0 as *const hilite_node as *mut hilite_node,
        lookaside: 0 as *const hilite_node as *mut hilite_node,
    };
    init
};
static mut filter_infos: *mut pattern_info = 0 as *const pattern_info as *mut pattern_info;
static mut search_info: pattern_info = pattern_info {
    compiled: 0 as *const regex_t as *mut regex_t,
    text: 0 as *const std::ffi::c_char as *mut std::ffi::c_char,
    search_type: 0,
    is_ucase_pattern: LFALSE,
    next: 0 as *const pattern_info as *mut pattern_info,
};
#[no_mangle]
pub static mut is_caseless: std::ffi::c_int = 0;
unsafe extern "C" fn is_ucase(mut str: *const std::ffi::c_char) -> lbool {
    let mut str_end: *const std::ffi::c_char = str.offset(strlen(str) as isize);
    let mut ch: LWCHAR = 0;
    while str < str_end {
        ch = step_charc(&mut str, 1 as std::ffi::c_int, str_end);
        if iswupper(ch as wint_t) != 0 {
            return LTRUE;
        }
    }
    return LFALSE;
}
unsafe extern "C" fn clear_pattern(mut info: *mut pattern_info) {
    if !((*info).text).is_null() {
        free((*info).text as *mut std::ffi::c_void);
    }
    (*info).text = 0 as *mut std::ffi::c_char;
    uncompile_pattern(&mut (*info).compiled);
}
unsafe extern "C" fn set_pattern(
    mut info: *mut pattern_info,
    mut pattern: *const std::ffi::c_char,
    mut search_type: std::ffi::c_int,
    mut show_error: std::ffi::c_int,
) -> std::ffi::c_int {
    (*info).is_ucase_pattern = (if pattern.is_null() {
        LFALSE as std::ffi::c_int as std::ffi::c_uint
    } else {
        is_ucase(pattern) as std::ffi::c_uint
    }) as lbool;
    is_caseless =
        if (*info).is_ucase_pattern as std::ffi::c_uint != 0 && caseless != 2 as std::ffi::c_int {
            0 as std::ffi::c_int
        } else {
            caseless
        };
    if pattern.is_null() {
        (*info).compiled = 0 as *mut regex_t;
    } else if compile_pattern(pattern, search_type, show_error, &mut (*info).compiled)
        < 0 as std::ffi::c_int
    {
        return -(1 as std::ffi::c_int);
    }
    if !((*info).text).is_null() {
        free((*info).text as *mut std::ffi::c_void);
    }
    (*info).text = 0 as *mut std::ffi::c_char;
    if !pattern.is_null() {
        (*info).text = ecalloc(
            1 as std::ffi::c_int as size_t,
            (strlen(pattern)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong),
        ) as *mut std::ffi::c_char;
        strcpy((*info).text, pattern);
    }
    (*info).search_type = search_type;
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn init_pattern(mut info: *mut pattern_info) {
    (*info).compiled = 0 as *mut regex_t;
    (*info).text = 0 as *mut std::ffi::c_char;
    (*info).search_type = 0 as std::ffi::c_int;
    (*info).next = 0 as *mut pattern_info;
}
#[no_mangle]
pub unsafe extern "C" fn init_search() {
    init_pattern(&mut search_info);
}
#[no_mangle]
pub unsafe extern "C" fn get_cvt_ops(mut search_type: std::ffi::c_int) -> std::ffi::c_int {
    let mut ops: std::ffi::c_int = 0 as std::ffi::c_int;
    if is_caseless != 0
        && (LTRUE as std::ffi::c_int == 0
            || search_type & (1 as std::ffi::c_int) << 12 as std::ffi::c_int != 0)
    {
        ops |= 0o1 as std::ffi::c_int;
    }
    if proc_backspace == 1 as std::ffi::c_int
        || bs_mode == 0 as std::ffi::c_int && proc_backspace == 0 as std::ffi::c_int
    {
        ops |= 0o2 as std::ffi::c_int;
    }
    if proc_return == 1 as std::ffi::c_int
        || bs_mode != 2 as std::ffi::c_int && proc_backspace == 0 as std::ffi::c_int
    {
        ops |= 0o4 as std::ffi::c_int;
    }
    if ctldisp == 2 as std::ffi::c_int {
        ops |= 0o10 as std::ffi::c_int;
    }
    return ops;
}
unsafe extern "C" fn prev_pattern(mut info: *mut pattern_info) -> lbool {
    if (*info).search_type & (1 as std::ffi::c_int) << 12 as std::ffi::c_int == 0 as std::ffi::c_int
    {
        return (is_null_pattern((*info).compiled) as u64 == 0) as std::ffi::c_int as lbool;
    }
    return ((*info).text != 0 as *mut std::ffi::c_void as *mut std::ffi::c_char) as std::ffi::c_int
        as lbool;
}
#[no_mangle]
pub unsafe extern "C" fn repaint_hilite(mut on: lbool) {
    let mut sindex: std::ffi::c_int = 0;
    let mut pos: POSITION = 0;
    let mut save_hide_hilite: lbool = LFALSE;
    if squished as u64 != 0 {
        repaint();
    }
    save_hide_hilite = hide_hilite;
    if on as u64 == 0 {
        if hide_hilite as u64 != 0 {
            return;
        }
        hide_hilite = LTRUE;
    }
    if can_goto_line == 0 {
        repaint();
        hide_hilite = save_hide_hilite;
        return;
    }
    sindex = 0 as std::ffi::c_int;
    while sindex < 0 as std::ffi::c_int + sc_height - 1 as std::ffi::c_int {
        pos = position(sindex);
        if !(pos == -(1 as std::ffi::c_int) as POSITION) {
            forw_line(pos, 0 as *mut POSITION, 0 as *mut lbool);
            goto_line(sindex);
            clear_eol();
            put_line(LFALSE);
        }
        sindex += 1;
    }
    overlay_header();
    lower_left();
    hide_hilite = save_hide_hilite;
}
#[no_mangle]
pub unsafe extern "C" fn clear_attn() {
    let mut sindex: std::ffi::c_int = 0;
    let mut old_start_attnpos: POSITION = 0;
    let mut old_end_attnpos: POSITION = 0;
    let mut pos: POSITION = 0;
    let mut epos: POSITION = 0;
    let mut moved: std::ffi::c_int = 0 as std::ffi::c_int;
    if start_attnpos == -(1 as std::ffi::c_int) as POSITION {
        return;
    }
    old_start_attnpos = start_attnpos;
    old_end_attnpos = end_attnpos;
    end_attnpos = -(1 as std::ffi::c_int) as POSITION;
    start_attnpos = end_attnpos;
    if can_goto_line == 0 {
        repaint();
        return;
    }
    if squished as u64 != 0 {
        repaint();
    }
    sindex = 0 as std::ffi::c_int;
    while sindex < 0 as std::ffi::c_int + sc_height - 1 as std::ffi::c_int {
        pos = position(sindex);
        if !(pos == -(1 as std::ffi::c_int) as POSITION) {
            epos = position(sindex + 1 as std::ffi::c_int);
            if pos <= old_end_attnpos
                && (epos == -(1 as std::ffi::c_int) as POSITION || epos > old_start_attnpos)
            {
                forw_line(pos, 0 as *mut POSITION, 0 as *mut lbool);
                goto_line(sindex);
                clear_eol();
                put_line(LFALSE);
                moved = 1 as std::ffi::c_int;
            }
        }
        sindex += 1;
    }
    if overlay_header() != 0 {
        moved = 1 as std::ffi::c_int;
    }
    if moved != 0 {
        lower_left();
    }
}
#[no_mangle]
pub unsafe extern "C" fn undo_search(mut clear: lbool) {
    clear_pattern(&mut search_info);
    undo_osc8();
    if clear as u64 != 0 {
        clr_hilite();
    } else {
        if (hilite_anchor.first).is_null() {
            error(
                b"No previous regular expression\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
            return;
        }
        hide_hilite = (hide_hilite as u64 == 0) as std::ffi::c_int as lbool;
    }
    repaint_hilite(LTRUE);
}
#[no_mangle]
pub unsafe extern "C" fn undo_osc8() {
    osc8_linepos = -(1 as std::ffi::c_int) as POSITION;
}
#[no_mangle]
pub unsafe extern "C" fn clr_hlist(mut anchor: *mut hilite_tree) {
    let mut hls: *mut hilite_storage = 0 as *mut hilite_storage;
    let mut nexthls: *mut hilite_storage = 0 as *mut hilite_storage;
    hls = (*anchor).first;
    while !hls.is_null() {
        nexthls = (*hls).next;
        free((*hls).nodes as *mut std::ffi::c_void);
        free(hls as *mut std::ffi::c_void);
        hls = nexthls;
    }
    (*anchor).first = 0 as *mut hilite_storage;
    (*anchor).current = 0 as *mut hilite_storage;
    (*anchor).root = 0 as *mut hilite_node;
    (*anchor).lookaside = 0 as *mut hilite_node;
    prep_endpos = -(1 as std::ffi::c_int) as POSITION;
    prep_startpos = prep_endpos;
}
#[no_mangle]
pub unsafe extern "C" fn clr_hilite() {
    clr_hlist(&mut hilite_anchor);
}
#[no_mangle]
pub unsafe extern "C" fn clr_filter() {
    clr_hlist(&mut filter_anchor);
}
unsafe extern "C" fn hlist_find(
    mut anchor: *mut hilite_tree,
    mut pos: POSITION,
) -> *mut hilite_node {
    let mut n: *mut hilite_node = 0 as *mut hilite_node;
    let mut m: *mut hilite_node = 0 as *mut hilite_node;
    if !((*anchor).lookaside).is_null() {
        let mut steps: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut hit: std::ffi::c_int = 0 as std::ffi::c_int;
        n = (*anchor).lookaside;
        loop {
            if pos < (*n).r.hl_endpos {
                if ((*n).prev).is_null() || pos >= (*(*n).prev).r.hl_endpos {
                    hit = 1 as std::ffi::c_int;
                    break;
                }
            } else if ((*n).next).is_null() {
                n = 0 as *mut hilite_node;
                hit = 1 as std::ffi::c_int;
                break;
            }
            if steps >= 2 as std::ffi::c_int {
                break;
            }
            steps += 1;
            if pos < (*n).r.hl_endpos {
                n = (*n).prev;
                (*anchor).lookaside = n;
            } else {
                n = (*n).next;
                (*anchor).lookaside = n;
            }
        }
        if hit != 0 {
            return n;
        }
    }
    n = (*anchor).root;
    m = 0 as *mut hilite_node;
    while !n.is_null() {
        if pos < (*n).r.hl_startpos {
            if ((*n).left).is_null() {
                break;
            }
            m = n;
            n = (*n).left;
        } else {
            if !(pos >= (*n).r.hl_endpos) {
                break;
            }
            if !((*n).right).is_null() {
                n = (*n).right;
            } else {
                if !m.is_null() {
                    n = m;
                } else {
                    m = n;
                    n = 0 as *mut hilite_node;
                }
                break;
            }
        }
    }
    if !n.is_null() {
        (*anchor).lookaside = n;
    } else if !m.is_null() {
        (*anchor).lookaside = m;
    }
    return n;
}
unsafe extern "C" fn hilited_range_attr(mut pos: POSITION, mut epos: POSITION) -> std::ffi::c_int {
    let mut n: *mut hilite_node = hlist_find(&mut hilite_anchor, pos);
    if n.is_null() {
        return 0 as std::ffi::c_int;
    }
    if epos != -(1 as std::ffi::c_int) as POSITION && epos <= (*n).r.hl_startpos {
        return 0 as std::ffi::c_int;
    }
    return (*n).r.hl_attr;
}
#[no_mangle]
pub unsafe extern "C" fn set_header(mut pos: POSITION) {
    header_start_pos = if header_lines == 0 as std::ffi::c_int {
        -(1 as std::ffi::c_int) as POSITION
    } else {
        pos
    };
    if header_start_pos != -(1 as std::ffi::c_int) as POSITION {
        let mut ln: std::ffi::c_int = 0;
        ln = 0 as std::ffi::c_int;
        while ln < header_lines {
            pos = forw_raw_line(pos, 0 as *mut *const std::ffi::c_char, 0 as *mut size_t);
            if pos == -(1 as std::ffi::c_int) as POSITION {
                break;
            }
            ln += 1;
        }
        header_end_pos = pos;
    }
}
unsafe extern "C" fn pos_in_header(mut pos: POSITION) -> lbool {
    return (header_start_pos != -(1 as std::ffi::c_int) as POSITION
        && pos >= header_start_pos
        && pos < header_end_pos) as std::ffi::c_int as lbool;
}
#[no_mangle]
pub unsafe extern "C" fn is_filtered(mut pos: POSITION) -> lbool {
    let mut n: *mut hilite_node = 0 as *mut hilite_node;
    if is_filtering() as u64 == 0 {
        return LFALSE;
    }
    if pos_in_header(pos) as u64 != 0 {
        return LFALSE;
    }
    n = hlist_find(&mut filter_anchor, pos);
    return (!n.is_null() && pos >= (*n).r.hl_startpos) as std::ffi::c_int as lbool;
}
#[no_mangle]
pub unsafe extern "C" fn next_unfiltered(mut pos: POSITION) -> POSITION {
    if is_filtering() as u64 == 0 {
        return pos;
    }
    if pos_in_header(pos) as u64 != 0 {
        return pos;
    }
    flush();
    while pos != -(1 as std::ffi::c_int) as POSITION {
        prep_hilite(
            pos,
            -(1 as std::ffi::c_int) as POSITION,
            1 as std::ffi::c_int,
        );
        if is_filtered(pos) as u64 == 0 {
            break;
        }
        pos = forw_raw_line(pos, 0 as *mut *const std::ffi::c_char, 0 as *mut size_t);
    }
    return pos;
}
unsafe extern "C" fn shift_visible(
    mut line_pos: POSITION,
    mut start_off: size_t,
    mut end_off: size_t,
) {
    let mut start_pos: POSITION = (line_pos as size_t).wrapping_add(start_off) as POSITION;
    let mut end_pos: POSITION = (line_pos as size_t).wrapping_add(end_off) as POSITION;
    let mut start_col: std::ffi::c_int = col_from_pos(
        line_pos,
        start_pos,
        -(1 as std::ffi::c_int) as POSITION,
        -(1 as std::ffi::c_int),
    );
    let mut end_col: std::ffi::c_int = col_from_pos(line_pos, end_pos, start_pos, start_col);
    let mut swidth: std::ffi::c_int = sc_width
        - line_pfx_width()
        - (if rscroll_char != 0 {
            1 as std::ffi::c_int
        } else {
            0 as std::ffi::c_int
        });
    let mut new_hshift: std::ffi::c_int = 0;
    if start_col < 0 as std::ffi::c_int || end_col < 0 as std::ffi::c_int {
        return;
    }
    if end_col < swidth {
        new_hshift = 0 as std::ffi::c_int;
    } else if start_col > hshift && end_col < hshift + swidth {
        new_hshift = hshift;
    } else {
        let mut eol_col: std::ffi::c_int = col_from_pos(
            line_pos,
            -(1 as std::ffi::c_int) as POSITION,
            end_pos,
            end_col,
        ) - swidth;
        if start_col >= eol_col {
            new_hshift = eol_col;
        } else {
            new_hshift = if start_col < match_shift {
                0 as std::ffi::c_int
            } else {
                start_col - match_shift
            };
        }
    }
    if new_hshift != hshift {
        hshift = new_hshift;
        screen_trashed();
    }
}
#[no_mangle]
pub unsafe extern "C" fn is_hilited_attr(
    mut pos: POSITION,
    mut epos: POSITION,
    mut nohide: std::ffi::c_int,
    mut p_matches: *mut std::ffi::c_int,
) -> std::ffi::c_int {
    let mut attr: std::ffi::c_int = 0;
    if !p_matches.is_null() {
        *p_matches = 0 as std::ffi::c_int;
    }
    if status_col == 0
        && start_attnpos != -(1 as std::ffi::c_int) as POSITION
        && pos <= end_attnpos
        && (epos == -(1 as std::ffi::c_int) as POSITION || epos > start_attnpos)
    {
        return (1 as std::ffi::c_int) << 6 as std::ffi::c_int
            | (1 as std::ffi::c_int) << 8 as std::ffi::c_int;
    }
    if osc8_linepos != -(1 as std::ffi::c_int) as POSITION
        && pos < osc8_text_end
        && (epos == -(1 as std::ffi::c_int) as POSITION || epos > osc8_text_start)
    {
        return (1 as std::ffi::c_int) << 6 as std::ffi::c_int
            | (10 as std::ffi::c_int) << 8 as std::ffi::c_int;
    }
    attr = hilited_range_attr(pos, epos);
    if attr == 0 as std::ffi::c_int {
        return 0 as std::ffi::c_int;
    }
    if p_matches.is_null() {
        return attr;
    }
    *p_matches = 1 as std::ffi::c_int;
    if hilite_search == 0 as std::ffi::c_int {
        return 0 as std::ffi::c_int;
    }
    if nohide == 0 && hide_hilite as std::ffi::c_uint != 0 {
        return 0 as std::ffi::c_int;
    }
    return attr;
}
unsafe extern "C" fn hlist_getstorage(mut anchor: *mut hilite_tree) -> *mut hilite_storage {
    let mut capacity: size_t = 1 as std::ffi::c_int as size_t;
    let mut s: *mut hilite_storage = 0 as *mut hilite_storage;
    if !((*anchor).current).is_null() {
        if (*(*anchor).current).used < (*(*anchor).current).capacity {
            return (*anchor).current;
        }
        capacity = (*(*anchor).current).capacity * 2 as std::ffi::c_int as size_t;
    }
    s = ecalloc(
        1 as std::ffi::c_int as size_t,
        ::core::mem::size_of::<hilite_storage>() as std::ffi::c_ulong,
    ) as *mut hilite_storage;
    (*s).nodes = ecalloc(
        capacity,
        ::core::mem::size_of::<hilite_node>() as std::ffi::c_ulong,
    ) as *mut hilite_node;
    (*s).capacity = capacity;
    (*s).used = 0 as std::ffi::c_int as size_t;
    (*s).next = 0 as *mut hilite_storage;
    if !((*anchor).current).is_null() {
        (*(*anchor).current).next = s;
    } else {
        (*anchor).first = s;
    }
    (*anchor).current = s;
    return s;
}
unsafe extern "C" fn hlist_getnode(mut anchor: *mut hilite_tree) -> *mut hilite_node {
    let mut s: *mut hilite_storage = hlist_getstorage(anchor);
    let fresh0 = (*s).used;
    (*s).used = ((*s).used).wrapping_add(1);
    return &mut *((*s).nodes).offset(fresh0 as isize) as *mut hilite_node;
}
unsafe extern "C" fn hlist_rotate_left(mut anchor: *mut hilite_tree, mut n: *mut hilite_node) {
    let mut np: *mut hilite_node = (*n).parent;
    let mut nr: *mut hilite_node = (*n).right;
    let mut nrl: *mut hilite_node = (*(*n).right).left;
    if !np.is_null() {
        if n == (*np).left {
            (*np).left = nr;
        } else {
            (*np).right = nr;
        }
    } else {
        (*anchor).root = nr;
    }
    (*nr).left = n;
    (*n).right = nrl;
    (*nr).parent = np;
    (*n).parent = nr;
    if !nrl.is_null() {
        (*nrl).parent = n;
    }
}
unsafe extern "C" fn hlist_rotate_right(mut anchor: *mut hilite_tree, mut n: *mut hilite_node) {
    let mut np: *mut hilite_node = (*n).parent;
    let mut nl: *mut hilite_node = (*n).left;
    let mut nlr: *mut hilite_node = (*(*n).left).right;
    if !np.is_null() {
        if n == (*np).right {
            (*np).right = nl;
        } else {
            (*np).left = nl;
        }
    } else {
        (*anchor).root = nl;
    }
    (*nl).right = n;
    (*n).left = nlr;
    (*nl).parent = np;
    (*n).parent = nl;
    if !nlr.is_null() {
        (*nlr).parent = n;
    }
}
unsafe extern "C" fn add_hilite(mut anchor: *mut hilite_tree, mut hl: *mut hilite) {
    let mut p: *mut hilite_node = 0 as *mut hilite_node;
    let mut n: *mut hilite_node = 0 as *mut hilite_node;
    let mut u: *mut hilite_node = 0 as *mut hilite_node;
    if (*hl).hl_startpos >= (*hl).hl_endpos {
        return;
    }
    p = (*anchor).root;
    if p.is_null() {
        n = hlist_getnode(anchor);
        (*n).r = *hl;
        (*anchor).root = n;
        (*anchor).lookaside = n;
        return;
    }
    loop {
        if (*hl).hl_startpos < (*p).r.hl_startpos {
            if (*hl).hl_endpos > (*p).r.hl_startpos && (*hl).hl_attr == (*p).r.hl_attr {
                (*hl).hl_endpos = (*p).r.hl_startpos;
            }
            if ((*p).left).is_null() {
                break;
            }
            p = (*p).left;
        } else {
            if (*hl).hl_startpos < (*p).r.hl_endpos && (*hl).hl_attr == (*p).r.hl_attr {
                (*hl).hl_startpos = (*p).r.hl_endpos;
                if (*hl).hl_startpos >= (*hl).hl_endpos {
                    return;
                }
            }
            if ((*p).right).is_null() {
                break;
            }
            p = (*p).right;
        }
    }
    if (*hl).hl_startpos < (*p).r.hl_startpos {
        if (*hl).hl_attr == (*p).r.hl_attr {
            if (*hl).hl_endpos == (*p).r.hl_startpos {
                (*p).r.hl_startpos = (*hl).hl_startpos;
                return;
            }
            if !((*p).prev).is_null() && (*(*p).prev).r.hl_endpos == (*hl).hl_startpos {
                (*(*p).prev).r.hl_endpos = (*hl).hl_endpos;
                return;
            }
        }
        n = hlist_getnode(anchor);
        (*p).left = n;
        (*n).next = p;
        if !((*p).prev).is_null() {
            (*n).prev = (*p).prev;
            (*(*p).prev).next = n;
        }
        (*p).prev = n;
    } else {
        if (*hl).hl_attr == (*p).r.hl_attr {
            if (*p).r.hl_endpos == (*hl).hl_startpos {
                (*p).r.hl_endpos = (*hl).hl_endpos;
                return;
            }
            if !((*p).next).is_null() && (*hl).hl_endpos == (*(*p).next).r.hl_startpos {
                (*(*p).next).r.hl_startpos = (*hl).hl_startpos;
                return;
            }
        }
        n = hlist_getnode(anchor);
        (*p).right = n;
        (*n).prev = p;
        if !((*p).next).is_null() {
            (*n).next = (*p).next;
            (*(*p).next).prev = n;
        }
        (*p).next = n;
    }
    (*n).parent = p;
    (*n).red = 1 as std::ffi::c_int;
    (*n).r = *hl;
    loop {
        if ((*n).parent).is_null() {
            (*n).red = 0 as std::ffi::c_int;
            break;
        } else {
            if (*(*n).parent).red == 0 {
                break;
            }
            u = (*(*(*n).parent).parent).left;
            if (*n).parent == u {
                u = (*(*(*n).parent).parent).right;
            }
            if !u.is_null() && (*u).red != 0 {
                (*(*n).parent).red = 0 as std::ffi::c_int;
                (*u).red = 0 as std::ffi::c_int;
                n = (*(*n).parent).parent;
                (*n).red = 1 as std::ffi::c_int;
            } else {
                if n == (*(*n).parent).right && (*n).parent == (*(*(*n).parent).parent).left {
                    hlist_rotate_left(anchor, (*n).parent);
                    n = (*n).left;
                } else if n == (*(*n).parent).left && (*n).parent == (*(*(*n).parent).parent).right
                {
                    hlist_rotate_right(anchor, (*n).parent);
                    n = (*n).right;
                }
                (*(*n).parent).red = 0 as std::ffi::c_int;
                (*(*(*n).parent).parent).red = 1 as std::ffi::c_int;
                if n == (*(*n).parent).left {
                    hlist_rotate_right(anchor, (*(*n).parent).parent);
                } else {
                    hlist_rotate_left(anchor, (*(*n).parent).parent);
                }
                break;
            }
        }
    }
}
unsafe extern "C" fn create_hilites(
    mut linepos: POSITION,
    mut line: *const std::ffi::c_char,
    mut sp: *const std::ffi::c_char,
    mut ep: *const std::ffi::c_char,
    mut attr: std::ffi::c_int,
    mut chpos: *mut std::ffi::c_int,
) {
    let mut start_index: size_t = sp.offset_from(line) as std::ffi::c_long as size_t;
    let mut end_index: size_t = ep.offset_from(line) as std::ffi::c_long as size_t;
    let mut hl: hilite = hilite {
        hl_startpos: 0,
        hl_endpos: 0,
        hl_attr: 0,
    };
    let mut i: size_t = 0;
    hl.hl_startpos = linepos + *chpos.offset(start_index as isize) as POSITION;
    hl.hl_attr = attr;
    i = start_index.wrapping_add(1 as std::ffi::c_int as size_t);
    while i <= end_index {
        if *chpos.offset(i as isize)
            != *chpos.offset(i.wrapping_sub(1 as std::ffi::c_int as size_t) as isize)
                + 1 as std::ffi::c_int
            || i == end_index
        {
            hl.hl_endpos = linepos
                + *chpos.offset(i.wrapping_sub(1 as std::ffi::c_int as size_t) as isize)
                    as POSITION
                + 1 as std::ffi::c_int as POSITION;
            add_hilite(&mut hilite_anchor, &mut hl);
            if i < end_index {
                hl.hl_startpos = linepos + *chpos.offset(i as isize) as POSITION;
            }
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn hilite_line(
    mut linepos: POSITION,
    mut line: *const std::ffi::c_char,
    mut line_len: size_t,
    mut chpos: *mut std::ffi::c_int,
    mut sp: *mut *const std::ffi::c_char,
    mut ep: *mut *const std::ffi::c_char,
    mut nsp: std::ffi::c_int,
) {
    let mut line_off: size_t = 0 as std::ffi::c_int as size_t;
    loop {
        let mut lep: *const std::ffi::c_char = *sp.offset(0 as std::ffi::c_int as isize);
        let mut i: std::ffi::c_int = 0;
        if (*sp.offset(0 as std::ffi::c_int as isize)).is_null()
            || (*ep.offset(0 as std::ffi::c_int as isize)).is_null()
        {
            break;
        }
        i = 1 as std::ffi::c_int;
        while i < nsp {
            if (*sp.offset(i as isize)).is_null() || (*ep.offset(i as isize)).is_null() {
                break;
            }
            if *ep.offset(i as isize) > *sp.offset(i as isize) {
                create_hilites(
                    linepos,
                    line,
                    lep,
                    *sp.offset(i as isize),
                    (1 as std::ffi::c_int) << 6 as std::ffi::c_int
                        | (10 as std::ffi::c_int) << 8 as std::ffi::c_int,
                    chpos,
                );
                create_hilites(
                    linepos,
                    line,
                    *sp.offset(i as isize),
                    *ep.offset(i as isize),
                    (1 as std::ffi::c_int) << 6 as std::ffi::c_int
                        | 10 as std::ffi::c_int + i << 8 as std::ffi::c_int,
                    chpos,
                );
                lep = *ep.offset(i as isize);
            }
            i += 1;
        }
        create_hilites(
            linepos,
            line,
            lep,
            *ep.offset(0 as std::ffi::c_int as isize),
            (1 as std::ffi::c_int) << 6 as std::ffi::c_int
                | (10 as std::ffi::c_int) << 8 as std::ffi::c_int,
            chpos,
        );
        if *ep.offset(0 as std::ffi::c_int as isize)
            > &*line.offset(line_off as isize) as *const std::ffi::c_char
        {
            line_off = (*ep.offset(0 as std::ffi::c_int as isize)).offset_from(line)
                as std::ffi::c_long as size_t;
        } else {
            if !(line_off != line_len) {
                break;
            }
            line_off = line_off.wrapping_add(1);
        }
        if !(match_pattern(
            search_info.compiled,
            search_info.text,
            line,
            line_len,
            line_off,
            sp,
            ep,
            nsp,
            1 as std::ffi::c_int,
            search_info.search_type,
        ) as u64
            != 0)
        {
            break;
        }
    }
}
unsafe extern "C" fn hilite_screen() {
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    get_scrpos(&mut scrpos, 0 as std::ffi::c_int);
    if scrpos.pos == -(1 as std::ffi::c_int) as POSITION {
        return;
    }
    prep_hilite(
        scrpos.pos,
        position(-(2 as std::ffi::c_int)),
        -(1 as std::ffi::c_int),
    );
    repaint_hilite(LTRUE);
}
#[no_mangle]
pub unsafe extern "C" fn chg_hilite() {
    clr_hilite();
    hide_hilite = LFALSE;
    if hilite_search == 2 as std::ffi::c_int {
        hilite_screen();
    }
}
unsafe extern "C" fn search_pos(mut search_type: std::ffi::c_int) -> POSITION {
    let mut pos: POSITION = 0;
    let mut sindex: std::ffi::c_int = 0;
    if empty_screen() != 0 {
        if search_type & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
            pos = 0 as std::ffi::c_int as POSITION;
        } else {
            pos = ch_length();
            if pos == -(1 as std::ffi::c_int) as POSITION {
                ch_end_seek();
                pos = ch_length();
            }
        }
        sindex = 0 as std::ffi::c_int;
    } else {
        let mut add_one: lbool = LFALSE;
        if how_search == 1 as std::ffi::c_int {
            if search_type & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
                sindex = sc_height - 1 as std::ffi::c_int;
            } else {
                sindex = 0 as std::ffi::c_int;
            }
        } else if how_search == 2 as std::ffi::c_int
            && search_type & (1 as std::ffi::c_int) << 14 as std::ffi::c_int == 0
        {
            if search_type & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
                sindex = 0 as std::ffi::c_int;
            } else {
                sindex = sc_height - 1 as std::ffi::c_int;
            }
        } else {
            sindex = sindex_from_sline(jump_sline);
            if search_type & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
                add_one = LTRUE;
            }
        }
        pos = position(sindex);
        if add_one as u64 != 0 {
            pos = forw_raw_line(pos, 0 as *mut *const std::ffi::c_char, 0 as *mut size_t);
        }
    }
    if search_type & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
        while pos == -(1 as std::ffi::c_int) as POSITION {
            sindex += 1;
            if sindex >= sc_height {
                break;
            }
            pos = position(sindex);
        }
    } else {
        while pos == -(1 as std::ffi::c_int) as POSITION {
            sindex -= 1;
            if sindex < 0 as std::ffi::c_int {
                break;
            }
            pos = position(sindex);
        }
    }
    return pos;
}
unsafe extern "C" fn matches_filters(
    mut pos: POSITION,
    mut cline: *mut std::ffi::c_char,
    mut line_len: size_t,
    mut chpos: *mut std::ffi::c_int,
    mut linepos: POSITION,
    mut sp: *mut *const std::ffi::c_char,
    mut ep: *mut *const std::ffi::c_char,
    mut nsp: std::ffi::c_int,
) -> lbool {
    let mut filter: *mut pattern_info = 0 as *mut pattern_info;
    filter = filter_infos;
    while !filter.is_null() {
        let mut line_filter: lbool = match_pattern(
            (*filter).compiled,
            (*filter).text,
            cline,
            line_len,
            0 as std::ffi::c_int as size_t,
            sp,
            ep,
            nsp,
            0 as std::ffi::c_int,
            (*filter).search_type,
        );
        if line_filter as u64 != 0 {
            let mut hl: hilite = hilite {
                hl_startpos: 0,
                hl_endpos: 0,
                hl_attr: 0,
            };
            hl.hl_startpos = linepos;
            hl.hl_endpos = pos;
            add_hilite(&mut filter_anchor, &mut hl);
            free(cline as *mut std::ffi::c_void);
            free(chpos as *mut std::ffi::c_void);
            return LTRUE;
        }
        filter = (*filter).next;
    }
    return LFALSE;
}
unsafe extern "C" fn get_lastlinepos(
    mut pos: POSITION,
    mut tpos: POSITION,
    mut sheight: std::ffi::c_int,
) -> POSITION {
    let mut nlines: std::ffi::c_int = 0;
    flush();
    nlines = 0 as std::ffi::c_int;
    loop {
        let mut npos: POSITION = forw_line(pos, 0 as *mut POSITION, 0 as *mut lbool);
        if npos > tpos {
            if nlines < sheight {
                return -(1 as std::ffi::c_int) as POSITION;
            }
            return pos;
        }
        pos = npos;
        nlines += 1;
    }
}
unsafe extern "C" fn osc8_parse(
    mut line: *const std::ffi::c_char,
    mut line_end: *const std::ffi::c_char,
    mut pop: *mut osc8_parse_info,
) -> lbool {
    let mut oline: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut ch: LWCHAR = 0;
    let mut pansi: *mut ansi_state_0 = 0 as *mut ansi_state_0;
    (*pop).params_end = 0 as *const std::ffi::c_char;
    (*pop).params_start = (*pop).params_end;
    (*pop).uri_end = (*pop).params_start;
    (*pop).uri_start = (*pop).uri_end;
    (*pop).osc8_end = (*pop).uri_start;
    (*pop).osc8_start = (*pop).osc8_end;
    oline = line;
    ch = step_charc(&mut line, 1 as std::ffi::c_int, line_end);
    pansi = ansi_start(ch);
    if pansi.is_null() {
        return LFALSE;
    }
    (*pop).osc8_start = oline;
    loop {
        let mut astate: ansi_state = ansi_step(pansi, ch);
        let mut ostate: osc8_state = ansi_osc8_state(pansi);
        if ostate as std::ffi::c_uint == OSC8_NOT as std::ffi::c_int as std::ffi::c_uint {
            break;
        }
        match ostate as std::ffi::c_uint {
            6 => {
                if ((*pop).params_start).is_null() {
                    (*pop).params_start = line;
                }
            }
            7 => {
                if ((*pop).uri_start).is_null() {
                    (*pop).params_end = oline;
                    (*pop).uri_start = line;
                }
            }
            4 => {
                if ((*pop).uri_end).is_null() {
                    (*pop).uri_end = oline;
                }
            }
            5 => {
                ansi_done(pansi);
                if ((*pop).params_start).is_null() || ((*pop).uri_start).is_null() {
                    return LFALSE;
                }
                (*pop).osc8_end = line;
                if ((*pop).uri_end).is_null() {
                    (*pop).uri_end = oline;
                }
                if ((*pop).params_end).is_null() {
                    (*pop).params_end = oline;
                }
                return LTRUE;
            }
            _ => {}
        }
        if astate as std::ffi::c_uint != ANSI_MID as std::ffi::c_int as std::ffi::c_uint
            || line >= line_end
        {
            break;
        }
        oline = line;
        ch = step_charc(&mut line, 1 as std::ffi::c_int, line_end);
    }
    ansi_done(pansi);
    return LFALSE;
}
unsafe extern "C" fn osc8_param_match(
    mut linepos: POSITION,
    mut line: *const std::ffi::c_char,
    mut op1: *const osc8_parse_info,
    mut op2: *const osc8_parse_info,
    mut param: *const std::ffi::c_char,
    mut clickpos: POSITION,
) -> lbool {
    let mut param_len: size_t = 0;
    let mut p: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    if clickpos != -(1 as std::ffi::c_int) as POSITION {
        return (clickpos as size_t
            >= (linepos as size_t)
                .wrapping_add(
                    ((*op1).osc8_start).offset_from(line) as std::ffi::c_long as size_t,
                )
            && (clickpos as size_t)
                < (linepos as size_t)
                    .wrapping_add(
                        ((*op2).osc8_end).offset_from(line) as std::ffi::c_long as size_t,
                    )) as std::ffi::c_int as lbool;
    }
    if param.is_null() {
        return LTRUE;
    }
    param_len = strlen(param);
    p = (*op1).params_start;
    while p.offset(param_len as isize) <= (*op1).params_end {
        if strncmp(p, param, param_len) == 0 as std::ffi::c_int {
            p = p.offset(param_len as isize);
            if p == (*op1).params_end || *p as std::ffi::c_int == ':' as i32 {
                return LTRUE;
            }
        }
        while p < (*op1).params_end && *p as std::ffi::c_int != ':' as i32 {
            p = p.offset(1);
        }
        while p < (*op1).params_end && *p as std::ffi::c_int == ':' as i32 {
            p = p.offset(1);
        }
    }
    return LFALSE;
}
unsafe extern "C" fn osc8_empty_uri(mut op: *const osc8_parse_info) -> lbool {
    return ((*op).uri_end == (*op).uri_start
        || (*op).uri_end == ((*op).uri_start).offset(1 as std::ffi::c_int as isize)
            && *((*op).uri_start).offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                == '#' as i32) as std::ffi::c_int as lbool;
}
unsafe extern "C" fn osc8_search_line1(
    mut search_type: std::ffi::c_int,
    mut linepos: POSITION,
    mut spos: POSITION,
    mut line: *const std::ffi::c_char,
    mut line_len: size_t,
    mut param: *const std::ffi::c_char,
    mut clickpos: POSITION,
) -> osc8_match {
    let mut line_end: *const std::ffi::c_char =
        &*line.offset(line_len as isize) as *const std::ffi::c_char;
    let mut op1: osc8_parse_info = osc8_parse_info {
        osc8_start: 0 as *const std::ffi::c_char,
        osc8_end: 0 as *const std::ffi::c_char,
        params_start: 0 as *const std::ffi::c_char,
        params_end: 0 as *const std::ffi::c_char,
        uri_start: 0 as *const std::ffi::c_char,
        uri_end: 0 as *const std::ffi::c_char,
    };
    let mut op2: osc8_parse_info = osc8_parse_info {
        osc8_start: 0 as *const std::ffi::c_char,
        osc8_end: 0 as *const std::ffi::c_char,
        params_start: 0 as *const std::ffi::c_char,
        params_end: 0 as *const std::ffi::c_char,
        uri_start: 0 as *const std::ffi::c_char,
        uri_end: 0 as *const std::ffi::c_char,
    };
    let mut linep: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let min_osc8_size: size_t = 6 as std::ffi::c_int as size_t;
    if search_type & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
        linep = line;
        loop {
            if linep.offset(min_osc8_size as isize) > line_end {
                return OSC8_NO_MATCH;
            }
            if osc8_parse(linep, line_end, &mut op1) as std::ffi::c_uint != 0
                && (osc8_empty_uri(&mut op1) as u64 == 0 || !param.is_null())
            {
                let mut linep2: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
                linep2 = op1.osc8_end;
                while linep2 < line_end {
                    if osc8_parse(linep2, line_end, &mut op2) as u64 != 0 {
                        break;
                    }
                    linep2 = linep2.offset(1);
                }
                if linep2 == line_end {
                    op2.osc8_start = line_end;
                    op2.osc8_end = op2.osc8_start;
                }
                if (op2.osc8_start > op1.osc8_end || !param.is_null())
                    && osc8_param_match(linepos, line, &mut op1, &mut op2, param, clickpos)
                        as std::ffi::c_uint
                        != 0
                {
                    break;
                }
            }
            linep = linep.offset(1);
        }
    } else {
        op2.osc8_start = line_end;
        op2.osc8_end = op2.osc8_start;
        linep = line_end.offset(-(min_osc8_size as isize));
        loop {
            if linep < line {
                return OSC8_NO_MATCH;
            }
            if osc8_parse(linep, line_end, &mut op1) as u64 != 0 {
                if (osc8_empty_uri(&mut op1) as u64 == 0 && op2.osc8_start > op1.osc8_end
                    || !param.is_null())
                    && osc8_param_match(linepos, line, &mut op1, &mut op2, param, clickpos)
                        as std::ffi::c_uint
                        != 0
                {
                    break;
                }
                op2 = op1;
            }
            linep = linep.offset(-1);
        }
    }
    if !param.is_null() {
        return OSC8_MATCH;
    }
    if osc8_linepos == linepos
        && osc8_match_start as size_t
            == (spos as size_t)
                .wrapping_add((op1.osc8_start).offset_from(line) as std::ffi::c_long as size_t)
    {
        return OSC8_ALREADY;
    }
    osc8_linepos = linepos;
    osc8_match_start = (spos as size_t)
        .wrapping_add((op1.osc8_start).offset_from(line) as std::ffi::c_long as size_t)
        as POSITION;
    osc8_match_end = (spos as size_t)
        .wrapping_add((op2.osc8_start).offset_from(line) as std::ffi::c_long as size_t)
        as POSITION;
    osc8_params_start = (spos as size_t)
        .wrapping_add((op1.params_start).offset_from(line) as std::ffi::c_long as size_t)
        as POSITION;
    osc8_params_end = (spos as size_t)
        .wrapping_add((op1.params_end).offset_from(line) as std::ffi::c_long as size_t)
        as POSITION;
    osc8_uri_start = (spos as size_t)
        .wrapping_add((op1.uri_start).offset_from(line) as std::ffi::c_long as size_t)
        as POSITION;
    osc8_uri_end = (spos as size_t)
        .wrapping_add((op1.uri_end).offset_from(line) as std::ffi::c_long as size_t)
        as POSITION;
    osc8_text_start = (spos as size_t)
        .wrapping_add((op1.osc8_end).offset_from(line) as std::ffi::c_long as size_t)
        as POSITION;
    osc8_text_end = (spos as size_t)
        .wrapping_add((op2.osc8_start).offset_from(line) as std::ffi::c_long as size_t)
        as POSITION;
    osc8_uri = saven(
        op1.uri_start,
        (op1.uri_end).offset_from(op1.uri_start) as std::ffi::c_long as size_t,
    );
    return OSC8_MATCH;
}
unsafe extern "C" fn osc8_search_line(
    mut search_type: std::ffi::c_int,
    mut linepos: POSITION,
    mut line: *const std::ffi::c_char,
    mut line_len: size_t,
    mut param: *const std::ffi::c_char,
    mut clickpos: POSITION,
    mut matches: *mut std::ffi::c_int,
) -> osc8_match {
    while *matches > 0 as std::ffi::c_int {
        let mut spos: POSITION = linepos;
        let mut sline: *const std::ffi::c_char = line;
        let mut sline_len: size_t = line_len;
        let mut r: osc8_match = OSC8_NO_MATCH;
        if linepos == osc8_linepos && clickpos == -(1 as std::ffi::c_int) as POSITION {
            if search_type & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
                let mut off: size_t = (osc8_match_end - linepos) as size_t;
                spos = (spos as size_t).wrapping_add(off) as POSITION as POSITION;
                sline = sline.offset(off as isize);
                sline_len = sline_len.wrapping_sub(off);
            } else {
                sline_len = (osc8_match_start - linepos) as size_t;
            }
        }
        r = osc8_search_line1(
            search_type,
            linepos,
            spos,
            sline,
            sline_len,
            param,
            clickpos,
        );
        if r as std::ffi::c_uint == OSC8_NO_MATCH as std::ffi::c_int as std::ffi::c_uint {
            break;
        }
        *matches -= 1;
        if *matches <= 0 as std::ffi::c_int {
            return r;
        }
    }
    return OSC8_NO_MATCH;
}
unsafe extern "C" fn osc8_shift_visible() {
    if chop_line() != 0 {
        let mut start_off: size_t = (osc8_match_start - osc8_linepos) as size_t;
        let mut end_off: size_t = (osc8_match_end - osc8_linepos) as size_t;
        shift_visible(osc8_linepos, start_off, end_off);
    }
}
unsafe extern "C" fn search_range(
    mut pos: POSITION,
    mut endpos: POSITION,
    mut search_type: std::ffi::c_int,
    mut matches: std::ffi::c_int,
    mut maxlines: std::ffi::c_int,
    mut plinepos: *mut POSITION,
    mut pendpos: *mut POSITION,
    mut plastlinepos: *mut POSITION,
) -> std::ffi::c_int {
    let mut line: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut cline: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut line_len: size_t = 0;
    let mut linenum: LINENUM = 0;
    let mut sp: [*const std::ffi::c_char; 7] = [0 as *const std::ffi::c_char; 7];
    let mut ep: [*const std::ffi::c_char; 7] = [0 as *const std::ffi::c_char; 7];
    let mut line_match: lbool = LFALSE;
    let mut cvt_ops: std::ffi::c_int = 0;
    let mut cvt_len: size_t = 0;
    let mut chpos: *mut std::ffi::c_int = 0 as *mut std::ffi::c_int;
    let mut linepos: POSITION = 0;
    let mut oldpos: POSITION = 0;
    let mut skip_bytes: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut swidth: size_t = (sc_width - line_pfx_width()) as size_t;
    let mut sheight: size_t = (sc_height - sindex_from_sline(jump_sline)) as size_t;
    linenum = find_linenum(pos);
    if nosearch_header_lines != 0 && linenum <= header_lines as LINENUM {
        linenum = (header_lines + 1 as std::ffi::c_int) as LINENUM;
        pos = find_pos(linenum);
    }
    if pos == -(1 as std::ffi::c_int) as POSITION {
        return -(1 as std::ffi::c_int);
    }
    oldpos = pos;
    if search_type & (1 as std::ffi::c_int) << 15 as std::ffi::c_int != 0
        && endpos == -(1 as std::ffi::c_int) as POSITION
    {
        endpos = pos;
    }
    flush();
    loop {
        if sigs
            & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
            != 0
        {
            return -(1 as std::ffi::c_int);
        }
        if endpos != -(1 as std::ffi::c_int) as POSITION
            && search_type & (1 as std::ffi::c_int) << 15 as std::ffi::c_int == 0
            && (search_type & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 && pos >= endpos
                || search_type & (1 as std::ffi::c_int) << 1 as std::ffi::c_int != 0
                    && pos <= endpos)
            || maxlines == 0 as std::ffi::c_int
        {
            if !pendpos.is_null() {
                *pendpos = pos;
            }
            return matches;
        }
        if maxlines > 0 as std::ffi::c_int {
            maxlines -= 1;
        }
        if search_type & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
            linepos = pos;
            pos = forw_raw_line(pos, &mut line, &mut line_len);
            if linenum != 0 as std::ffi::c_int as LINENUM {
                linenum += 1;
            }
        } else {
            pos = back_raw_line(pos, &mut line, &mut line_len);
            linepos = pos;
            if linenum != 0 as std::ffi::c_int as LINENUM {
                linenum -= 1;
            }
        }
        if pos == -(1 as std::ffi::c_int) as POSITION {
            if search_type & (1 as std::ffi::c_int) << 15 as std::ffi::c_int != 0 {
                if search_type & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
                    pos = 0 as std::ffi::c_int as POSITION;
                } else {
                    pos = ch_length();
                    if pos == -(1 as std::ffi::c_int) as POSITION {
                        ch_end_seek();
                        pos = ch_length();
                    }
                }
                if pos != -(1 as std::ffi::c_int) as POSITION {
                    search_wrapped = LTRUE;
                    search_type &= !((1 as std::ffi::c_int) << 15 as std::ffi::c_int);
                    linenum = find_linenum(pos);
                    continue;
                }
            }
            if !pendpos.is_null() {
                *pendpos = oldpos;
            }
            return matches;
        } else {
            if linenums != 0 && abs((pos - oldpos) as std::ffi::c_int) > 2048 as std::ffi::c_int {
                add_lnum(linenum, pos);
            }
            oldpos = pos;
            if is_filtered(linepos) as u64 != 0 {
                continue;
            }
            if nosearch_header_cols != 0 {
                skip_bytes = skip_columns(header_cols, &mut line, &mut line_len);
            }
            if search_type & (1 as std::ffi::c_int) << 16 as std::ffi::c_int != 0 {
                if osc8_search_line(
                    search_type,
                    linepos,
                    line,
                    line_len,
                    osc8_search_param,
                    -(1 as std::ffi::c_int) as POSITION,
                    &mut matches,
                ) as std::ffi::c_uint
                    != OSC8_NO_MATCH as std::ffi::c_int as std::ffi::c_uint
                {
                    if !plinepos.is_null() {
                        *plinepos = linepos;
                    }
                    osc8_shift_visible();
                    return 0 as std::ffi::c_int;
                }
            } else {
                cvt_ops = get_cvt_ops(search_type);
                cvt_len = cvt_length(line_len, cvt_ops);
                cline = ecalloc(1 as std::ffi::c_int as size_t, cvt_len) as *mut std::ffi::c_char;
                chpos = cvt_alloc_chpos(cvt_len);
                cvt_text(cline, line, chpos, &mut line_len, cvt_ops);
                if !filter_infos.is_null()
                    && (search_type & (1 as std::ffi::c_int) << 4 as std::ffi::c_int != 0
                        || prep_startpos == -(1 as std::ffi::c_int) as POSITION
                        || linepos < prep_startpos
                        || linepos >= prep_endpos)
                {
                    if matches_filters(
                        pos,
                        cline,
                        line_len,
                        chpos,
                        linepos,
                        sp.as_mut_ptr(),
                        ep.as_mut_ptr(),
                        16 as std::ffi::c_int - 10 as std::ffi::c_int - 1 as std::ffi::c_int
                            + 2 as std::ffi::c_int,
                    ) as u64
                        != 0
                    {
                        continue;
                    }
                }
                if prev_pattern(&mut search_info) as u64 != 0 {
                    line_match = match_pattern(
                        search_info.compiled,
                        search_info.text,
                        cline,
                        line_len,
                        0 as std::ffi::c_int as size_t,
                        sp.as_mut_ptr(),
                        ep.as_mut_ptr(),
                        16 as std::ffi::c_int - 10 as std::ffi::c_int - 1 as std::ffi::c_int
                            + 2 as std::ffi::c_int,
                        0 as std::ffi::c_int,
                        search_type,
                    );
                    if line_match as u64 != 0 {
                        if search_type & (1 as std::ffi::c_int) << 4 as std::ffi::c_int != 0 {
                            hilite_line(
                                linepos + skip_bytes as POSITION,
                                cline,
                                line_len,
                                chpos,
                                sp.as_mut_ptr(),
                                ep.as_mut_ptr(),
                                16 as std::ffi::c_int
                                    - 10 as std::ffi::c_int
                                    - 1 as std::ffi::c_int
                                    + 2 as std::ffi::c_int,
                            );
                        } else {
                            matches -= 1;
                            if matches <= 0 as std::ffi::c_int {
                                if hilite_search == 1 as std::ffi::c_int {
                                    clr_hilite();
                                    hilite_line(
                                        linepos + skip_bytes as POSITION,
                                        cline,
                                        line_len,
                                        chpos,
                                        sp.as_mut_ptr(),
                                        ep.as_mut_ptr(),
                                        16 as std::ffi::c_int
                                            - 10 as std::ffi::c_int
                                            - 1 as std::ffi::c_int
                                            + 2 as std::ffi::c_int,
                                    );
                                }
                                if chop_line() != 0 {
                                    if !(sp[0 as std::ffi::c_int as usize]).is_null()
                                        && !(ep[0 as std::ffi::c_int as usize]).is_null()
                                    {
                                        let mut start_off: size_t =
                                            (sp[0 as std::ffi::c_int as usize]).offset_from(cline)
                                                as std::ffi::c_long
                                                as size_t;
                                        let mut end_off: size_t =
                                            (ep[0 as std::ffi::c_int as usize]).offset_from(cline)
                                                as std::ffi::c_long
                                                as size_t;
                                        shift_visible(
                                            linepos,
                                            *chpos.offset(start_off as isize) as size_t,
                                            *chpos.offset(end_off as isize) as size_t,
                                        );
                                    }
                                } else if !plastlinepos.is_null() {
                                    if !(ep[0 as std::ffi::c_int as usize]).is_null() {
                                        let mut end_off_0: size_t =
                                            (ep[0 as std::ffi::c_int as usize]).offset_from(cline)
                                                as std::ffi::c_long
                                                as size_t;
                                        if end_off_0
                                            >= swidth * sheight / 4 as std::ffi::c_int as size_t
                                        {
                                            *plastlinepos = get_lastlinepos(
                                                linepos,
                                                linepos
                                                    + *chpos.offset(end_off_0 as isize) as POSITION,
                                                sheight as std::ffi::c_int,
                                            );
                                        }
                                    }
                                }
                                free(cline as *mut std::ffi::c_void);
                                free(chpos as *mut std::ffi::c_void);
                                if !plinepos.is_null() {
                                    *plinepos = linepos;
                                }
                                return 0 as std::ffi::c_int;
                            }
                        }
                    }
                }
                free(cline as *mut std::ffi::c_void);
                free(chpos as *mut std::ffi::c_void);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn osc8_search(
    mut search_type: std::ffi::c_int,
    mut param: *const std::ffi::c_char,
    mut matches: std::ffi::c_int,
) {
    let mut pos: POSITION = 0;
    let mut match_0: std::ffi::c_int = 0;
    let mut curr_sindex: std::ffi::c_int = -(1 as std::ffi::c_int);
    if osc8_linepos != -(1 as std::ffi::c_int) as POSITION && {
        curr_sindex = onscreen(osc8_linepos);
        curr_sindex >= 0 as std::ffi::c_int
    } {
        let mut line: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
        let mut line_len: size_t = 0;
        pos = forw_raw_line(osc8_linepos, &mut line, &mut line_len);
        if pos != -(1 as std::ffi::c_int) as POSITION {
            if osc8_search_line(
                search_type,
                osc8_linepos,
                line,
                line_len,
                param,
                -(1 as std::ffi::c_int) as POSITION,
                &mut matches,
            ) as std::ffi::c_uint
                != OSC8_NO_MATCH as std::ffi::c_int as std::ffi::c_uint
            {
                osc8_shift_visible();
                repaint_hilite(LTRUE);
                return;
            }
        }
        search_type |= (1 as std::ffi::c_int) << 14 as std::ffi::c_int;
    }
    if curr_sindex >= 0 as std::ffi::c_int {
        pos = osc8_linepos;
    } else {
        pos = search_pos(search_type);
    }
    if pos == -(1 as std::ffi::c_int) as POSITION {
        error(
            b"Nothing to search\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        return;
    }
    osc8_search_param = param;
    match_0 = search_range(
        pos,
        -(1 as std::ffi::c_int) as POSITION,
        search_type | (1 as std::ffi::c_int) << 16 as std::ffi::c_int,
        matches,
        -(1 as std::ffi::c_int),
        &mut pos,
        0 as *mut POSITION,
        0 as *mut POSITION,
    );
    osc8_search_param = 0 as *const std::ffi::c_char;
    if match_0 != 0 as std::ffi::c_int {
        error(
            b"OSC 8 link not found\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        return;
    }
    if onscreen(pos) < 0 as std::ffi::c_int {
        jump_loc(pos, jump_sline);
    }
    repaint_hilite(LTRUE);
}
#[no_mangle]
pub unsafe extern "C" fn osc8_click(
    mut sindex: std::ffi::c_int,
    mut col: std::ffi::c_int,
) -> lbool {
    let mut linepos: POSITION = position(sindex);
    let mut clickpos: POSITION = 0;
    let mut line: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut line_len: size_t = 0;
    let mut matches: std::ffi::c_int = 1 as std::ffi::c_int;
    let mut r: std::ffi::c_int = 0;
    if linepos == -(1 as std::ffi::c_int) as POSITION {
        return LFALSE;
    }
    clickpos = pos_from_col(
        linepos,
        col,
        -(1 as std::ffi::c_int) as POSITION,
        -(1 as std::ffi::c_int),
    );
    if clickpos == -(1 as std::ffi::c_int) as POSITION {
        return LFALSE;
    }
    if forw_raw_line(linepos, &mut line, &mut line_len) == -(1 as std::ffi::c_int) as POSITION {
        return LFALSE;
    }
    r = osc8_search_line(
        (1 as std::ffi::c_int) << 0 as std::ffi::c_int
            | (1 as std::ffi::c_int) << 16 as std::ffi::c_int,
        linepos,
        line,
        line_len,
        0 as *const std::ffi::c_char,
        clickpos,
        &mut matches,
    ) as std::ffi::c_int;
    if r != OSC8_NO_MATCH as std::ffi::c_int {
        repaint_hilite(LTRUE);
        if r == OSC8_ALREADY as std::ffi::c_int {
            osc8_open();
        }
        return LTRUE;
    }
    return LFALSE;
}
unsafe extern "C" fn scheme_length(
    mut uri: *const std::ffi::c_char,
    mut uri_len: size_t,
) -> size_t {
    let mut plen: size_t = 0;
    plen = 0 as std::ffi::c_int as size_t;
    while plen < uri_len {
        if *uri.offset(plen as isize) as std::ffi::c_int == ':' as i32 {
            return plen;
        }
        plen = plen.wrapping_add(1);
    }
    return 0 as std::ffi::c_int as size_t;
}
unsafe extern "C" fn bad_uri(mut uri: *const std::ffi::c_char, mut uri_len: size_t) -> lbool {
    let mut i: size_t = 0;
    i = 0 as std::ffi::c_int as size_t;
    while i < uri_len {
        if !(strchr(
            b"'\"\0" as *const u8 as *const std::ffi::c_char,
            *uri.offset(i as isize) as std::ffi::c_int,
        ))
        .is_null()
        {
            return LTRUE;
        }
        i = i.wrapping_add(1);
    }
    return LFALSE;
}
unsafe extern "C" fn osc8_read_selected(mut op: *mut osc8_parse_info) -> lbool {
    let mut line: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut line_len: size_t = 0;
    let mut pos: POSITION = 0;
    pos = forw_raw_line(osc8_linepos, &mut line, &mut line_len);
    if pos == -(1 as std::ffi::c_int) as POSITION {
        return LFALSE;
    }
    (*op).osc8_start =
        &*line.offset((osc8_match_start - osc8_linepos) as isize) as *const std::ffi::c_char;
    (*op).osc8_end =
        &*line.offset((osc8_match_end - osc8_linepos) as isize) as *const std::ffi::c_char;
    (*op).params_start =
        &*line.offset((osc8_params_start - osc8_linepos) as isize) as *const std::ffi::c_char;
    (*op).params_end =
        &*line.offset((osc8_params_end - osc8_linepos) as isize) as *const std::ffi::c_char;
    (*op).uri_start =
        &*line.offset((osc8_uri_start - osc8_linepos) as isize) as *const std::ffi::c_char;
    (*op).uri_end =
        &*line.offset((osc8_uri_end - osc8_linepos) as isize) as *const std::ffi::c_char;
    return LTRUE;
}
#[no_mangle]
pub unsafe extern "C" fn osc8_open() {
    let mut op: osc8_parse_info = osc8_parse_info {
        osc8_start: 0 as *const std::ffi::c_char,
        osc8_end: 0 as *const std::ffi::c_char,
        params_start: 0 as *const std::ffi::c_char,
        params_end: 0 as *const std::ffi::c_char,
        uri_start: 0 as *const std::ffi::c_char,
        uri_end: 0 as *const std::ffi::c_char,
    };
    let mut env_name: [std::ffi::c_char; 64] = [0; 64];
    let mut scheme_len: size_t = 0;
    let mut handler: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut open_cmd: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut uri_len: size_t = 0;
    let mut hf: *mut FILE = 0 as *mut FILE;
    static mut env_name_pfx: *const std::ffi::c_char =
        b"LESS_OSC8_\0" as *const u8 as *const std::ffi::c_char;
    if osc8_linepos == -(1 as std::ffi::c_int) as POSITION {
        error(
            b"No OSC8 link selected\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        return;
    }
    if osc8_read_selected(&mut op) as u64 == 0 {
        error(
            b"Cannot find OSC8 link\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        return;
    }
    uri_len = (op.uri_end).offset_from(op.uri_start) as std::ffi::c_long as size_t;
    scheme_len = scheme_length(op.uri_start, uri_len);
    if scheme_len == 0 as std::ffi::c_int as size_t
        && *(op.uri_start).offset(0 as std::ffi::c_int as isize) as std::ffi::c_int == '#' as i32
    {
        let mut param: *mut std::ffi::c_char = ecalloc(
            uri_len.wrapping_add(3 as std::ffi::c_int as size_t),
            ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
        ) as *mut std::ffi::c_char;
        strcpy(param, b"id=\0" as *const u8 as *const std::ffi::c_char);
        strncpy(
            param.offset(3 as std::ffi::c_int as isize),
            (op.uri_start).offset(1 as std::ffi::c_int as isize),
            uri_len.wrapping_sub(1 as std::ffi::c_int as size_t),
        );
        *param.offset(uri_len.wrapping_add(2 as std::ffi::c_int as size_t) as isize) =
            '\0' as i32 as std::ffi::c_char;
        osc8_search(
            (1 as std::ffi::c_int) << 0 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 15 as std::ffi::c_int,
            param,
            1 as std::ffi::c_int,
        );
        free(param as *mut std::ffi::c_void);
        return;
    }
    if bad_uri(op.uri_start, uri_len) as u64 != 0 {
        error(
            b"Cannot open link containing quote characters\0" as *const u8
                as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        return;
    }
    snprintf(
        env_name.as_mut_ptr(),
        ::core::mem::size_of::<[std::ffi::c_char; 64]>() as std::ffi::c_ulong,
        b"%s%.*s\0" as *const u8 as *const std::ffi::c_char,
        env_name_pfx,
        scheme_len as std::ffi::c_int,
        op.uri_start,
    );
    handler = lgetenv(env_name.as_mut_ptr());
    if isnullenv(handler) as std::ffi::c_uint != 0
        || strcmp(handler, b"-\0" as *const u8 as *const std::ffi::c_char) == 0 as std::ffi::c_int
    {
        handler = lgetenv(b"LESS_OSC8_ANY\0" as *const u8 as *const std::ffi::c_char);
    }
    if isnullenv(handler) as u64 != 0 {
        let mut parg: PARG = parg {
            p_string: 0 as *const std::ffi::c_char,
        };
        parg.p_string = env_name.as_mut_ptr().offset(strlen(env_name_pfx) as isize);
        error(
            b"No handler for \"%s\" link type\0" as *const u8 as *const std::ffi::c_char,
            &mut parg,
        );
        return;
    }
    osc8_path = saven(op.uri_start, uri_len);
    hf = popen(
        pr_expand(handler),
        b"r\0" as *const u8 as *const std::ffi::c_char,
    );
    free(osc8_path as *mut std::ffi::c_void);
    osc8_path = 0 as *mut std::ffi::c_char;
    if hf.is_null() {
        let mut parg_0: PARG = parg {
            p_string: 0 as *const std::ffi::c_char,
        };
        parg_0.p_string = env_name.as_mut_ptr();
        error(
            b"Cannot execute protocol handler in %s\0" as *const u8 as *const std::ffi::c_char,
            &mut parg_0,
        );
        return;
    }
    open_cmd = readfd(hf);
    pclose(hf);
    if strncmp(
        open_cmd,
        b":e\0" as *const u8 as *const std::ffi::c_char,
        2 as std::ffi::c_int as std::ffi::c_ulong,
    ) == 0 as std::ffi::c_int
    {
        edit(skipsp(&mut *open_cmd.offset(2 as std::ffi::c_int as isize)));
    } else {
        lsystem(
            open_cmd,
            b"link done\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    free(open_cmd as *mut std::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn osc8_jump() {
    if osc8_linepos == -(1 as std::ffi::c_int) as POSITION {
        error(
            b"No OSC8 link selected\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        return;
    }
    jump_loc(osc8_linepos, jump_sline);
}
unsafe extern "C" fn hist_pattern(mut search_type: std::ffi::c_int) -> std::ffi::c_int {
    let mut pattern: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    set_mlist(ml_search, 0 as std::ffi::c_int);
    pattern = cmd_lastpattern();
    if pattern.is_null() {
        return 0 as std::ffi::c_int;
    }
    if set_pattern(&mut search_info, pattern, search_type, 1 as std::ffi::c_int)
        < 0 as std::ffi::c_int
    {
        return -(1 as std::ffi::c_int);
    }
    if hilite_search == 2 as std::ffi::c_int && hide_hilite as u64 == 0 {
        hilite_screen();
    }
    return 1 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn chg_caseless() {
    if search_info.is_ucase_pattern as u64 == 0 {
        is_caseless = caseless;
        if LTRUE as std::ffi::c_int == 0 {
            return;
        }
    }
    clear_pattern(&mut search_info);
    hist_pattern(search_info.search_type);
}
#[no_mangle]
pub unsafe extern "C" fn search(
    mut search_type: std::ffi::c_int,
    mut pattern: *const std::ffi::c_char,
    mut n: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut pos: POSITION = 0;
    let mut opos: POSITION = 0;
    let mut lastlinepos: POSITION = -(1 as std::ffi::c_int) as POSITION;
    if pattern.is_null() || *pattern as std::ffi::c_int == '\0' as i32 {
        search_type |= (1 as std::ffi::c_int) << 14 as std::ffi::c_int;
        if prev_pattern(&mut search_info) as u64 == 0 {
            let mut r: std::ffi::c_int = hist_pattern(search_type);
            if r == 0 as std::ffi::c_int {
                error(
                    b"No previous regular expression\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
            }
            if r <= 0 as std::ffi::c_int {
                return -(1 as std::ffi::c_int);
            }
        }
        if search_type & (1 as std::ffi::c_int) << 12 as std::ffi::c_int
            != search_info.search_type & (1 as std::ffi::c_int) << 12 as std::ffi::c_int
        {
            error(
                b"Please re-enter search pattern\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
            return -(1 as std::ffi::c_int);
        }
        if hilite_search == 1 as std::ffi::c_int || status_col != 0 {
            repaint_hilite(LFALSE);
        }
        if hilite_search == 2 as std::ffi::c_int && hide_hilite as std::ffi::c_uint != 0 {
            hide_hilite = LFALSE;
            hilite_screen();
        }
        hide_hilite = LFALSE;
    } else {
        let mut show_error: std::ffi::c_int =
            (search_type & (1 as std::ffi::c_int) << 3 as std::ffi::c_int == 0) as std::ffi::c_int;
        if set_pattern(&mut search_info, pattern, search_type, show_error) < 0 as std::ffi::c_int {
            return -(1 as std::ffi::c_int);
        }
        if hilite_search != 0 || status_col != 0 {
            repaint_hilite(LFALSE);
            hide_hilite = LFALSE;
            clr_hilite();
        }
        if hilite_search == 2 as std::ffi::c_int || status_col != 0 {
            hilite_screen();
        }
    }
    pos = search_pos(search_type);
    opos = position(sindex_from_sline(jump_sline));
    if pos == -(1 as std::ffi::c_int) as POSITION {
        if search_type & (1 as std::ffi::c_int) << 9 as std::ffi::c_int != 0 {
            return n;
        }
        if hilite_search == 1 as std::ffi::c_int || status_col != 0 {
            repaint_hilite(LTRUE);
        }
        error(
            b"Nothing to search\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        return -(1 as std::ffi::c_int);
    }
    n = search_range(
        pos,
        -(1 as std::ffi::c_int) as POSITION,
        search_type,
        n,
        -(1 as std::ffi::c_int),
        &mut pos,
        0 as *mut std::ffi::c_void as *mut POSITION,
        &mut lastlinepos,
    );
    if sigs
        & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
            | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
            | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
        != 0
    {
        return -(1 as std::ffi::c_int);
    }
    if n != 0 as std::ffi::c_int {
        if (hilite_search == 1 as std::ffi::c_int || status_col != 0) && n > 0 as std::ffi::c_int {
            repaint_hilite(LTRUE);
        }
        return n;
    }
    if search_type & (1 as std::ffi::c_int) << 2 as std::ffi::c_int == 0 {
        if lastlinepos != -(1 as std::ffi::c_int) as POSITION {
            jump_loc(lastlinepos, -(1 as std::ffi::c_int));
        } else if pos != opos {
            jump_loc(pos, jump_sline);
        }
    }
    if hilite_search == 1 as std::ffi::c_int || status_col != 0 {
        repaint_hilite(LTRUE);
    }
    return 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn prep_hilite(
    mut spos: POSITION,
    mut epos: POSITION,
    mut maxlines: std::ffi::c_int,
) {
    let mut nprep_startpos: POSITION = prep_startpos;
    let mut nprep_endpos: POSITION = prep_endpos;
    let mut new_epos: POSITION = 0;
    let mut max_epos: POSITION = 0;
    let mut result: std::ffi::c_int = 0;
    let mut i: std::ffi::c_int = 0;
    if prev_pattern(&mut search_info) as u64 == 0 && is_filtering() as u64 == 0 {
        return;
    }
    spos = back_raw_line(
        spos + 1 as std::ffi::c_int as POSITION,
        0 as *mut *const std::ffi::c_char,
        0 as *mut size_t,
    );
    if maxlines < 0 as std::ffi::c_int {
        max_epos = -(1 as std::ffi::c_int) as POSITION;
    } else {
        max_epos = spos;
        i = 0 as std::ffi::c_int;
        while i < maxlines {
            max_epos = forw_raw_line(
                max_epos,
                0 as *mut *const std::ffi::c_char,
                0 as *mut size_t,
            );
            i += 1;
        }
    }
    if epos == -(1 as std::ffi::c_int) as POSITION
        || max_epos != -(1 as std::ffi::c_int) as POSITION && epos > max_epos
    {
        epos = max_epos;
    }
    if prep_startpos == -(1 as std::ffi::c_int) as POSITION
        || epos != -(1 as std::ffi::c_int) as POSITION && epos < prep_startpos
        || spos > prep_endpos
    {
        clr_hilite();
        clr_filter();
        nprep_startpos = spos;
    } else {
        if epos != -(1 as std::ffi::c_int) as POSITION && epos <= prep_endpos {
            epos = prep_startpos;
        }
        if spos < prep_startpos {
            nprep_startpos = spos;
        } else {
            spos = prep_endpos;
        }
    }
    if epos == -(1 as std::ffi::c_int) as POSITION || epos > spos {
        let mut search_type: std::ffi::c_int = (1 as std::ffi::c_int) << 0 as std::ffi::c_int
            | (1 as std::ffi::c_int) << 4 as std::ffi::c_int;
        search_type |= search_info.search_type & (1 as std::ffi::c_int) << 12 as std::ffi::c_int;
        loop {
            result = search_range(
                spos,
                epos,
                search_type,
                0 as std::ffi::c_int,
                maxlines,
                0 as *mut std::ffi::c_void as *mut POSITION,
                &mut new_epos,
                0 as *mut std::ffi::c_void as *mut POSITION,
            );
            if result < 0 as std::ffi::c_int {
                return;
            }
            if prep_endpos == -(1 as std::ffi::c_int) as POSITION || new_epos > prep_endpos {
                nprep_endpos = new_epos;
            }
            if prep_endpos == -(1 as std::ffi::c_int) as POSITION || nprep_endpos > prep_endpos {
                if new_epos >= nprep_endpos
                    && is_filtered(new_epos - 1 as std::ffi::c_int as POSITION) as std::ffi::c_uint
                        != 0
                {
                    spos = nprep_endpos;
                    epos = forw_raw_line(
                        nprep_endpos,
                        0 as *mut *const std::ffi::c_char,
                        0 as *mut size_t,
                    );
                    if epos == -(1 as std::ffi::c_int) as POSITION {
                        break;
                    }
                    maxlines = 1 as std::ffi::c_int;
                    continue;
                }
            }
            if !(prep_startpos == -(1 as std::ffi::c_int) as POSITION
                || nprep_startpos < prep_startpos)
            {
                break;
            }
            if !(nprep_startpos > 0 as std::ffi::c_int as POSITION
                && is_filtered(nprep_startpos) as std::ffi::c_uint != 0)
            {
                break;
            }
            epos = nprep_startpos;
            spos = back_raw_line(
                nprep_startpos,
                0 as *mut *const std::ffi::c_char,
                0 as *mut size_t,
            );
            if spos == -(1 as std::ffi::c_int) as POSITION {
                break;
            }
            nprep_startpos = spos;
            maxlines = 1 as std::ffi::c_int;
        }
    }
    prep_startpos = nprep_startpos;
    prep_endpos = nprep_endpos;
}
#[no_mangle]
pub unsafe extern "C" fn set_filter_pattern(
    mut pattern: *const std::ffi::c_char,
    mut search_type: std::ffi::c_int,
) {
    let mut filter: *mut pattern_info = 0 as *mut pattern_info;
    clr_filter();
    if pattern.is_null() || *pattern as std::ffi::c_int == '\0' as i32 {
        filter = filter_infos;
        while !filter.is_null() {
            let mut next_filter: *mut pattern_info = (*filter).next;
            clear_pattern(filter);
            free(filter as *mut std::ffi::c_void);
            filter = next_filter;
        }
        filter_infos = 0 as *mut pattern_info;
    } else {
        filter = ecalloc(
            1 as std::ffi::c_int as size_t,
            ::core::mem::size_of::<pattern_info>() as std::ffi::c_ulong,
        ) as *mut pattern_info;
        init_pattern(filter);
        if set_pattern(filter, pattern, search_type, 1 as std::ffi::c_int) < 0 as std::ffi::c_int {
            free(filter as *mut std::ffi::c_void);
            return;
        }
        (*filter).next = filter_infos;
        filter_infos = filter;
    }
    screen_trashed();
}
#[no_mangle]
pub unsafe extern "C" fn is_filtering() -> lbool {
    if ch_getflags() & 0o10 as std::ffi::c_int != 0 {
        return LFALSE;
    }
    return (filter_infos != 0 as *mut std::ffi::c_void as *mut pattern_info) as std::ffi::c_int
        as lbool;
}
