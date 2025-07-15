use crate::util::ptr_to_str;
use ::c2rust_bitfields;
use ::libc;
use std::ptr;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn free(_: *mut std::ffi::c_void);
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn postoa(_: POSITION, _: *mut std::ffi::c_char, _: std::ffi::c_int);
    fn lstrtoic(
        _: *const std::ffi::c_char,
        _: *mut *const std::ffi::c_char,
        _: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn lstrtoposc(
        _: *const std::ffi::c_char,
        _: *mut *const std::ffi::c_char,
        _: std::ffi::c_int,
    ) -> POSITION;
    fn save(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn bell();
    fn ch_end_seek() -> std::ffi::c_int;
    fn ch_tell() -> POSITION;
    fn ch_getflags() -> std::ffi::c_int;
    fn edit_ifile(ifile: *mut std::ffi::c_void) -> std::ffi::c_int;
    fn lrealpath(path: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn prev_ifile(h: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn get_ifile(
        filename: *const std::ffi::c_char,
        prev: *mut std::ffi::c_void,
    ) -> *mut std::ffi::c_void;
    fn get_real_filename(ifile: *mut std::ffi::c_void) -> *const std::ffi::c_char;
    fn jump_loc(pos: POSITION, sline: std::ffi::c_int);
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn get_scrpos(scrpos: *mut scrpos, where_0: std::ffi::c_int);
    static mut curr_ifile: *mut std::ffi::c_void;
    static mut sc_height: std::ffi::c_int;
    static mut jump_sline: std::ffi::c_int;
    static mut perma_marks: std::ffi::c_int;
}
pub type __off_t = std::ffi::c_long;
pub type __off64_t = std::ffi::c_long;
pub type off_t = __off_t;
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
pub type less_off_t = off_t;
pub type POSITION = i32;
pub type LINENUM = i32;
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
    pub p_char: u8,
}
pub type PARG = parg;

/*
 * The table of marks.
 * Each mark is identified by a lowercase or uppercase letter.
 * The final one is lmark, for the "last mark"; addressed by the apostrophe.
 */
const NMARKS: u32 = (2 * 26) + 2; /* a-z, A-Z, mousemark, lastmark */
const NUMARKS: u32 = (2 * 26) + 1; /* user marks (not lastmark) */
const MOUSEMARK: u32 = NMARKS - 2;
const LASTMARK: u32 = NMARKS - 1;

const CH_HELPFILE: i32 = 0o10;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mark {
    pub m_letter: u8,
    pub m_ifile: *mut std::ffi::c_void,
    pub m_filename: *mut std::ffi::c_char,
    pub m_scrpos: scrpos,
}
static mut marks: [mark; 54] = [mark {
    m_letter: 0,
    m_ifile: 0 as *const std::ffi::c_void as *mut std::ffi::c_void,
    m_filename: 0 as *const std::ffi::c_char as *mut std::ffi::c_char,
    m_scrpos: scrpos { pos: 0, ln: 0 },
}; 54];

/*
 * Initialize a mark struct.
 */
#[no_mangle]
pub static mut marks_modified: bool = false;
unsafe extern "C" fn cmark(
    m: &mut mark,
    mut ifile: *mut std::ffi::c_void,
    mut pos: POSITION,
    mut ln: i32,
) {
    m.m_ifile = ifile;
    m.m_scrpos.pos = pos;
    m.m_scrpos.ln = ln;
    if !(m.m_filename).is_null() {
        free(m.m_filename as *mut std::ffi::c_void);
    }
    m.m_filename = 0 as *mut std::ffi::c_char;
}

/*
 * Initialize the mark table to show no marks are set.
 */
#[no_mangle]
pub unsafe extern "C" fn init_mark() {
    for i in 0..NMARKS {
        let mut letter: u8;
        match i {
            MOUSEMARK => {
                letter = b'#';
            }
            LASTMARK => {
                letter = b'\'';
            }
            _ => {
                letter = (if i < 26 {
                    b'a' + i as u8
                } else {
                    b'A' + i as u8 - 26
                })
            }
        }
        marks[i as usize].m_letter = letter;
        cmark(&mut marks[i as usize], 0 as *mut std::ffi::c_void, -1, -1);
    }
}

/*
 * Set m_ifile and clear m_filename.
 */
unsafe extern "C" fn mark_set_ifile(m: &mut mark, mut ifile: *mut std::ffi::c_void) {
    m.m_ifile = ifile;
    free(m.m_filename as *mut std::ffi::c_void);
    m.m_filename = 0 as *mut std::ffi::c_char;
}

/*
 * Populate the m_ifile member of a mark struct from m_filename.
 */
unsafe extern "C" fn mark_get_ifile(m: &mut mark) {
    if m.m_ifile != 0 as *mut std::ffi::c_void {
        return;
    }
    mark_set_ifile(
        m,
        get_ifile(m.m_filename, prev_ifile(0 as *mut std::ffi::c_void)),
    );
}

/*
 * Return the user mark struct identified by a character.
 */
unsafe extern "C" fn getumark<'a>(mut c: u8) -> Option<&'a mut mark> {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    match c {
        b'a'..=b'z' => return Some(&mut marks[(c - b'a') as usize]),
        b'A'..=b'Z' => return Some(&mut marks[(c - b'A' + 26) as usize]),
        b'\'' => return Some(&mut marks[LASTMARK as usize]),
        b'#' => return Some(&mut marks[MOUSEMARK as usize]),
        _ => {}
    }
    parg.p_char = c;
    error(
        b"Invalid mark letter %c\0" as *const u8 as *const std::ffi::c_char,
        &mut parg,
    );
    None
}

/*
 * Get the mark structure identified by a character.
 * The mark struct may either be in the mark table (user mark)
 * or may be constructed on the fly for certain characters like ^, $.
 */
unsafe extern "C" fn getmark<'a>(mut c: u8) -> Option<&'a mut mark> {
    static mut sm: mark = mark {
        m_letter: 0,
        m_ifile: 0 as *const std::ffi::c_void as *mut std::ffi::c_void,
        m_filename: 0 as *const std::ffi::c_char as *mut std::ffi::c_char,
        m_scrpos: scrpos { pos: 0, ln: 0 },
    };
    let mut m = &mut sm;
    match c {
        b'^' => {
            /*
             * Beginning of the current file.
             */
            m = &mut sm;
            cmark(
                m,
                curr_ifile,
                0 as std::ffi::c_int as POSITION,
                0 as std::ffi::c_int,
            );
        }
        b'$' => {
            /*
             * End of the current file.
             */
            if ch_end_seek() != 0 {
                error(
                    b"Cannot seek to end of file\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                return None;
            }
            m = &mut sm;
            cmark(m, curr_ifile, ch_tell(), sc_height);
        }
        b'.' => {
            /*
             * Current position in the current file.
             */
            m = &mut sm;
            get_scrpos(&mut (*m).m_scrpos, 0 as std::ffi::c_int);
            cmark(m, curr_ifile, (*m).m_scrpos.pos, (*m).m_scrpos.ln);
        }
        b'\'' => {
            /*
             * The "last mark".
             */
            m = &mut marks[LASTMARK as usize];
        }
        _ => {
            /*
             * Must be a user-defined mark.
             */
            if let Some(m) = getumark(c) {
                if m.m_scrpos.pos == -1 {
                    error(
                        b"Mark not set\0" as *const u8 as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void as *mut PARG,
                    );
                    return None;
                }
            }
        }
    }
    Some(m)
}

/*
 * Is a mark letter invalid?
 */
#[no_mangle]
pub unsafe extern "C" fn badmark(mut c: u8) -> bool {
    getmark(c).is_none()
}

/*
 * Set a user-defined mark.
 */
#[no_mangle]
pub unsafe extern "C" fn setmark(mut c: u8, mut where_0: i32) {
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    if let Some(m) = getumark(c) {
        get_scrpos(&mut scrpos, where_0);
        if scrpos.pos == -1 {
            bell();
            return;
        }
        cmark(m, curr_ifile, scrpos.pos, scrpos.ln);
        marks_modified = true;
    }
}

/*
 * Clear a user-defined mark.
 */
#[no_mangle]
pub unsafe extern "C" fn clrmark(mut c: u8) {
    if let Some(m) = getumark(c) {
        if m.m_scrpos.pos == -1 {
            bell();
            return;
        }
        m.m_scrpos.pos = -1;
        marks_modified = true;
    }
}

/*
 * Set lmark (the mark named by the apostrophe).
 */
#[no_mangle]
pub unsafe extern "C" fn lastmark() {
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    if ch_getflags() & CH_HELPFILE != 0 {
        return;
    }
    get_scrpos(&mut scrpos, 0 as std::ffi::c_int);
    if scrpos.pos == -1 {
        return;
    }
    cmark(
        &mut marks[LASTMARK as usize],
        curr_ifile,
        scrpos.pos,
        scrpos.ln,
    );
    marks_modified = true;
}

/*
 * Go to a mark.
 */
#[no_mangle]
pub unsafe extern "C" fn gomark(mut c: u8) {
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    if let Some(m) = getmark(c) {
        /*
         * If we're trying to go to the lastmark and
         * it has not been set to anything yet,
         * set it to the beginning of the current file.
         * {{ Couldn't we instead set marks[LASTMARK] in edit()? }}
         */
        if ptr::eq(m, &mut marks[LASTMARK as usize]) && m.m_scrpos.pos == -1 {
            cmark(m, curr_ifile, 0 as std::ffi::c_int as POSITION, jump_sline);
        }
        mark_get_ifile(m);

        /* Save scrpos; if it's LASTMARK it could change in edit_ifile. */
        scrpos = m.m_scrpos;
        if m.m_ifile != curr_ifile {
            /*
             * Not in the current file; edit the correct file.
             */
            if edit_ifile(m.m_ifile) != 0 {
                return;
            }
        }
        jump_loc(scrpos.pos, scrpos.ln);
    }
}

/*
 * Return the position associated with a given mark letter.
 *
 * We don't return which screen line the position
 * is associated with, but this doesn't matter much,
 * because it's always the first non-blank line on the screen.
 */
#[no_mangle]
pub unsafe extern "C" fn markpos(mut c: u8) -> POSITION {
    if let Some(m) = getmark(c) {
        if m.m_ifile != curr_ifile {
            error(
                b"Mark not in current file\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
            return -1;
        }
        return m.m_scrpos.pos;
    } else {
        return -1;
    }
}

/*
 * Return the mark associated with a given position, if any.
 */
#[no_mangle]
pub unsafe extern "C" fn posmark(mut pos: POSITION) -> u8 {
    /* Only user marks */
    for i in 0..NUMARKS {
        if marks[i as usize].m_ifile == curr_ifile && marks[i as usize].m_scrpos.pos == pos {
            if i < 26 {
                return b'a' + i as u8;
            }
            if i < 26 * 2 {
                return b'A' + (i as u8 - 26);
            }
            return b'#';
        }
    }
    0
}

/*
 * Clear the marks associated with a specified ifile.
 */
#[no_mangle]
pub unsafe extern "C" fn unmark(mut ifile: *mut std::ffi::c_void) {
    for i in 0..NMARKS {
        if marks[i as usize].m_ifile == ifile {
            marks[i as usize].m_scrpos.pos = -1;
        }
    }
}

/*
 * Check if any marks refer to a specified ifile vi m_filename
 * rather than m_ifile.
 */
#[no_mangle]
pub unsafe extern "C" fn mark_check_ifile(mut ifile: *mut std::ffi::c_void) {
    let mut filename: *const std::ffi::c_char = get_real_filename(ifile);
    for i in 0..NMARKS {
        let mut m = &mut marks[i as usize];
        let mut mark_filename: *mut std::ffi::c_char = m.m_filename;
        if !mark_filename.is_null() {
            mark_filename = lrealpath(mark_filename);
            if strcmp(filename, mark_filename) == 0 as std::ffi::c_int {
                mark_set_ifile(m, ifile);
            }
            free(mark_filename as *mut std::ffi::c_void);
        }
    }
}

/*
 * Save marks to history file.
 */
#[no_mangle]
pub unsafe extern "C" fn save_marks(mut fout: *mut FILE, mut hdr: *const std::ffi::c_char) {
    let mut i: std::ffi::c_int = 0;
    if perma_marks == 0 {
        return;
    }
    fprintf(fout, b"%s\n\0" as *const u8 as *const std::ffi::c_char, hdr);
    for i in 0..NMARKS {
        let mut filename: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
        let mut m = &mut marks[i as usize];
        let mut pos_str: [i8; 23] = [0; 23];
        if !m.m_scrpos.pos == -1 {
            postoa(m.m_scrpos.pos, pos_str.as_mut_ptr(), 10);
            filename = m.m_filename;
            if filename.is_null() {
                filename = get_real_filename(m.m_ifile);
            }
            if strcmp(filename, b"-\0" as *const u8 as *const std::ffi::c_char) != 0 {
                fprintf(
                    fout,
                    b"m %c %d %s %s\n\0" as *const u8 as *const std::ffi::c_char,
                    m.m_letter as std::ffi::c_int,
                    m.m_scrpos.ln,
                    pos_str.as_mut_ptr(),
                    filename,
                );
            }
        }
    }
}

/*
 * Restore one mark from the history file.
 */
#[no_mangle]
pub unsafe extern "C" fn restore_mark(mut line: *const std::ffi::c_char) {
    let m: &mut mark;
    let mut ln = 0;
    let mut pos: POSITION = 0;

    let fresh0 = line;
    line = line.offset(1);
    if *fresh0 as std::ffi::c_int != 'm' as i32 {
        return;
    }
    while *line as std::ffi::c_int == ' ' as i32 {
        line = line.offset(1);
    }
    let fresh1 = line;
    line = line.offset(1);
    let mm = getumark(*fresh1 as u8);
    if mm.is_none() {
        return;
    } else {
        m = mm.unwrap();
    }
    while *line as std::ffi::c_int == ' ' as i32 {
        line = line.offset(1);
    }
    ln = lstrtoic(line, &mut line, 10 as std::ffi::c_int);
    if ln < 0 {
        return;
    }
    if ln < 1 {
        ln = 1;
    }
    if ln > sc_height {
        ln = sc_height;
    }
    while *line as std::ffi::c_int == ' ' as i32 {
        line = line.offset(1);
    }
    pos = lstrtoposc(line, &mut line, 10 as std::ffi::c_int);
    if pos < 0 as std::ffi::c_int as POSITION {
        return;
    }
    while *line as std::ffi::c_int == ' ' as i32 {
        line = line.offset(1);
    }
    cmark(m, 0 as *mut std::ffi::c_void, pos, ln);
    m.m_filename = save(line);
}
