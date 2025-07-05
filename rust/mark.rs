use ::libc;
use ::c2rust_bitfields;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mark {
    pub m_letter: std::ffi::c_char,
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
#[no_mangle]
pub static mut marks_modified: std::ffi::c_int = 0 as std::ffi::c_int;
unsafe extern "C" fn cmark(
    mut m: *mut mark,
    mut ifile: *mut std::ffi::c_void,
    mut pos: POSITION,
    mut ln: std::ffi::c_int,
) {
    (*m).m_ifile = ifile;
    (*m).m_scrpos.pos = pos;
    (*m).m_scrpos.ln = ln;
    if !((*m).m_filename).is_null() {
        free((*m).m_filename as *mut std::ffi::c_void);
    }
    (*m).m_filename = 0 as *mut std::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn init_mark() {
    let mut i: std::ffi::c_int = 0;
    i = 0 as std::ffi::c_int;
    while i < 2 as std::ffi::c_int * 26 as std::ffi::c_int + 2 as std::ffi::c_int {
        let mut letter: std::ffi::c_char = 0;
        match i {
            52 => {
                letter = '#' as i32 as std::ffi::c_char;
            }
            53 => {
                letter = '\'' as i32 as std::ffi::c_char;
            }
            _ => {
                letter = (if i < 26 as std::ffi::c_int {
                    'a' as i32 + i
                } else {
                    'A' as i32 + i - 26 as std::ffi::c_int
                }) as std::ffi::c_char;
            }
        }
        marks[i as usize].m_letter = letter;
        cmark(
            &mut *marks.as_mut_ptr().offset(i as isize),
            0 as *mut std::ffi::c_void,
            -(1 as std::ffi::c_int) as POSITION,
            -(1 as std::ffi::c_int),
        );
        i += 1;
        i;
    }
}
unsafe extern "C" fn mark_set_ifile(mut m: *mut mark, mut ifile: *mut std::ffi::c_void) {
    (*m).m_ifile = ifile;
    free((*m).m_filename as *mut std::ffi::c_void);
    (*m).m_filename = 0 as *mut std::ffi::c_char;
}
unsafe extern "C" fn mark_get_ifile(mut m: *mut mark) {
    if (*m).m_ifile != 0 as *mut std::ffi::c_void {
        return;
    }
    mark_set_ifile(
        m,
        get_ifile((*m).m_filename, prev_ifile(0 as *mut std::ffi::c_void)),
    );
}
unsafe extern "C" fn getumark(mut c: std::ffi::c_char) -> *mut mark {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    if c as std::ffi::c_int >= 'a' as i32 && c as std::ffi::c_int <= 'z' as i32 {
        return &mut *marks
            .as_mut_ptr()
            .offset((c as std::ffi::c_int - 'a' as i32) as isize) as *mut mark;
    }
    if c as std::ffi::c_int >= 'A' as i32 && c as std::ffi::c_int <= 'Z' as i32 {
        return &mut *marks
            .as_mut_ptr()
            .offset((c as std::ffi::c_int - 'A' as i32 + 26 as std::ffi::c_int) as isize)
            as *mut mark;
    }
    if c as std::ffi::c_int == '\'' as i32 {
        return &mut *marks
            .as_mut_ptr()
            .offset(
                (2 as std::ffi::c_int * 26 as std::ffi::c_int + 2 as std::ffi::c_int
                    - 1 as std::ffi::c_int) as isize,
            ) as *mut mark;
    }
    if c as std::ffi::c_int == '#' as i32 {
        return &mut *marks
            .as_mut_ptr()
            .offset(
                (2 as std::ffi::c_int * 26 as std::ffi::c_int + 2 as std::ffi::c_int
                    - 2 as std::ffi::c_int) as isize,
            ) as *mut mark;
    }
    parg.p_char = c;
    error(
        b"Invalid mark letter %c\0" as *const u8 as *const std::ffi::c_char,
        &mut parg,
    );
    return 0 as *mut mark;
}
unsafe extern "C" fn getmark(mut c: std::ffi::c_char) -> *mut mark {
    let mut m: *mut mark = 0 as *mut mark;
    static mut sm: mark = mark {
        m_letter: 0,
        m_ifile: 0 as *const std::ffi::c_void as *mut std::ffi::c_void,
        m_filename: 0 as *const std::ffi::c_char as *mut std::ffi::c_char,
        m_scrpos: scrpos { pos: 0, ln: 0 },
    };
    match c as std::ffi::c_int {
        94 => {
            m = &mut sm;
            cmark(m, curr_ifile, 0 as std::ffi::c_int as POSITION, 0 as std::ffi::c_int);
        }
        36 => {
            if ch_end_seek() != 0 {
                error(
                    b"Cannot seek to end of file\0" as *const u8
                        as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                return 0 as *mut mark;
            }
            m = &mut sm;
            cmark(m, curr_ifile, ch_tell(), sc_height);
        }
        46 => {
            m = &mut sm;
            get_scrpos(&mut (*m).m_scrpos, 0 as std::ffi::c_int);
            cmark(m, curr_ifile, (*m).m_scrpos.pos, (*m).m_scrpos.ln);
        }
        39 => {
            m = &mut *marks
                .as_mut_ptr()
                .offset(
                    (2 as std::ffi::c_int * 26 as std::ffi::c_int + 2 as std::ffi::c_int
                        - 1 as std::ffi::c_int) as isize,
                ) as *mut mark;
        }
        _ => {
            m = getumark(c);
            if !m.is_null() {
                if (*m).m_scrpos.pos == -(1 as std::ffi::c_int) as POSITION {
                    error(
                        b"Mark not set\0" as *const u8 as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void as *mut PARG,
                    );
                    return 0 as *mut mark;
                }
            }
        }
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn badmark(mut c: std::ffi::c_char) -> std::ffi::c_int {
    return (getmark(c) == 0 as *mut std::ffi::c_void as *mut mark) as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn setmark(mut c: std::ffi::c_char, mut where_0: std::ffi::c_int) {
    let mut m: *mut mark = 0 as *mut mark;
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    m = getumark(c);
    if m.is_null() {
        return;
    }
    get_scrpos(&mut scrpos, where_0);
    if scrpos.pos == -(1 as std::ffi::c_int) as POSITION {
        bell();
        return;
    }
    cmark(m, curr_ifile, scrpos.pos, scrpos.ln);
    marks_modified = 1 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clrmark(mut c: std::ffi::c_char) {
    let mut m: *mut mark = 0 as *mut mark;
    m = getumark(c);
    if m.is_null() {
        return;
    }
    if (*m).m_scrpos.pos == -(1 as std::ffi::c_int) as POSITION {
        bell();
        return;
    }
    (*m).m_scrpos.pos = -(1 as std::ffi::c_int) as POSITION;
    marks_modified = 1 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lastmark() {
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    if ch_getflags() & 0o10 as std::ffi::c_int != 0 {
        return;
    }
    get_scrpos(&mut scrpos, 0 as std::ffi::c_int);
    if scrpos.pos == -(1 as std::ffi::c_int) as POSITION {
        return;
    }
    cmark(
        &mut *marks
            .as_mut_ptr()
            .offset(
                (2 as std::ffi::c_int * 26 as std::ffi::c_int + 2 as std::ffi::c_int
                    - 1 as std::ffi::c_int) as isize,
            ),
        curr_ifile,
        scrpos.pos,
        scrpos.ln,
    );
    marks_modified = 1 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gomark(mut c: std::ffi::c_char) {
    let mut m: *mut mark = 0 as *mut mark;
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    m = getmark(c);
    if m.is_null() {
        return;
    }
    if m
        == &mut *marks
            .as_mut_ptr()
            .offset(
                (2 as std::ffi::c_int * 26 as std::ffi::c_int + 2 as std::ffi::c_int
                    - 1 as std::ffi::c_int) as isize,
            ) as *mut mark && (*m).m_scrpos.pos == -(1 as std::ffi::c_int) as POSITION
    {
        cmark(m, curr_ifile, 0 as std::ffi::c_int as POSITION, jump_sline);
    }
    mark_get_ifile(m);
    scrpos = (*m).m_scrpos;
    if (*m).m_ifile != curr_ifile {
        if edit_ifile((*m).m_ifile) != 0 {
            return;
        }
    }
    jump_loc(scrpos.pos, scrpos.ln);
}
#[no_mangle]
pub unsafe extern "C" fn markpos(mut c: std::ffi::c_char) -> POSITION {
    let mut m: *mut mark = 0 as *mut mark;
    m = getmark(c);
    if m.is_null() {
        return -(1 as std::ffi::c_int) as POSITION;
    }
    if (*m).m_ifile != curr_ifile {
        error(
            b"Mark not in current file\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        return -(1 as std::ffi::c_int) as POSITION;
    }
    return (*m).m_scrpos.pos;
}
#[no_mangle]
pub unsafe extern "C" fn posmark(mut pos: POSITION) -> std::ffi::c_char {
    let mut i: std::ffi::c_uchar = 0;
    i = 0 as std::ffi::c_int as std::ffi::c_uchar;
    while (i as std::ffi::c_int)
        < 2 as std::ffi::c_int * 26 as std::ffi::c_int + 1 as std::ffi::c_int
    {
        if marks[i as usize].m_ifile == curr_ifile
            && marks[i as usize].m_scrpos.pos == pos
        {
            if (i as std::ffi::c_int) < 26 as std::ffi::c_int {
                return ('a' as i32 + i as std::ffi::c_int) as std::ffi::c_char;
            }
            if (i as std::ffi::c_int) < 26 as std::ffi::c_int * 2 as std::ffi::c_int {
                return ('A' as i32 + (i as std::ffi::c_int - 26 as std::ffi::c_int))
                    as std::ffi::c_char;
            }
            return '#' as i32 as std::ffi::c_char;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as std::ffi::c_int as std::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn unmark(mut ifile: *mut std::ffi::c_void) {
    let mut i: std::ffi::c_int = 0;
    i = 0 as std::ffi::c_int;
    while i < 2 as std::ffi::c_int * 26 as std::ffi::c_int + 2 as std::ffi::c_int {
        if marks[i as usize].m_ifile == ifile {
            marks[i as usize].m_scrpos.pos = -(1 as std::ffi::c_int) as POSITION;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mark_check_ifile(mut ifile: *mut std::ffi::c_void) {
    let mut i: std::ffi::c_int = 0;
    let mut filename: *const std::ffi::c_char = get_real_filename(ifile);
    i = 0 as std::ffi::c_int;
    while i < 2 as std::ffi::c_int * 26 as std::ffi::c_int + 2 as std::ffi::c_int {
        let mut m: *mut mark = &mut *marks.as_mut_ptr().offset(i as isize) as *mut mark;
        let mut mark_filename: *mut std::ffi::c_char = (*m).m_filename;
        if !mark_filename.is_null() {
            mark_filename = lrealpath(mark_filename);
            if strcmp(filename, mark_filename) == 0 as std::ffi::c_int {
                mark_set_ifile(m, ifile);
            }
            free(mark_filename as *mut std::ffi::c_void);
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn save_marks(
    mut fout: *mut FILE,
    mut hdr: *const std::ffi::c_char,
) {
    let mut i: std::ffi::c_int = 0;
    if perma_marks == 0 {
        return;
    }
    fprintf(fout, b"%s\n\0" as *const u8 as *const std::ffi::c_char, hdr);
    i = 0 as std::ffi::c_int;
    while i < 2 as std::ffi::c_int * 26 as std::ffi::c_int + 2 as std::ffi::c_int {
        let mut filename: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
        let mut m: *mut mark = &mut *marks.as_mut_ptr().offset(i as isize) as *mut mark;
        let mut pos_str: [std::ffi::c_char; 23] = [0; 23];
        if !((*m).m_scrpos.pos == -(1 as std::ffi::c_int) as POSITION) {
            postoa((*m).m_scrpos.pos, pos_str.as_mut_ptr(), 10 as std::ffi::c_int);
            filename = (*m).m_filename;
            if filename.is_null() {
                filename = get_real_filename((*m).m_ifile);
            }
            if strcmp(filename, b"-\0" as *const u8 as *const std::ffi::c_char)
                != 0 as std::ffi::c_int
            {
                fprintf(
                    fout,
                    b"m %c %d %s %s\n\0" as *const u8 as *const std::ffi::c_char,
                    (*m).m_letter as std::ffi::c_int,
                    (*m).m_scrpos.ln,
                    pos_str.as_mut_ptr(),
                    filename,
                );
            }
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn restore_mark(mut line: *const std::ffi::c_char) {
    let mut m: *mut mark = 0 as *mut mark;
    let mut ln: std::ffi::c_int = 0;
    let mut pos: POSITION = 0;
    let fresh0 = line;
    line = line.offset(1);
    if *fresh0 as std::ffi::c_int != 'm' as i32 {
        return;
    }
    while *line as std::ffi::c_int == ' ' as i32 {
        line = line.offset(1);
        line;
    }
    let fresh1 = line;
    line = line.offset(1);
    m = getumark(*fresh1);
    if m.is_null() {
        return;
    }
    while *line as std::ffi::c_int == ' ' as i32 {
        line = line.offset(1);
        line;
    }
    ln = lstrtoic(line, &mut line, 10 as std::ffi::c_int);
    if ln < 0 as std::ffi::c_int {
        return;
    }
    if ln < 1 as std::ffi::c_int {
        ln = 1 as std::ffi::c_int;
    }
    if ln > sc_height {
        ln = sc_height;
    }
    while *line as std::ffi::c_int == ' ' as i32 {
        line = line.offset(1);
        line;
    }
    pos = lstrtoposc(line, &mut line, 10 as std::ffi::c_int);
    if pos < 0 as std::ffi::c_int as POSITION {
        return;
    }
    while *line as std::ffi::c_int == ' ' as i32 {
        line = line.offset(1);
        line;
    }
    cmark(m, 0 as *mut std::ffi::c_void, pos, ln);
    (*m).m_filename = save(line);
}
