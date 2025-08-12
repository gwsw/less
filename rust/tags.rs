use crate::decode::lgetenv;
use ::c2rust_bitfields;
use ::libc;
use std::ffi::CString;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn fclose(__stream: *mut FILE) -> std::ffi::c_int;
    fn fopen(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> *mut FILE;
    fn sprintf(_: *mut std::ffi::c_char, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn fgetc(__stream: *mut FILE) -> std::ffi::c_int;
    fn fgets(
        __s: *mut std::ffi::c_char,
        __n: std::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut std::ffi::c_char;
    fn pclose(__stream: *mut FILE) -> std::ffi::c_int;
    fn popen(__command: *const std::ffi::c_char, __modes: *const std::ffi::c_char) -> *mut FILE;
    fn open(__file: *const std::ffi::c_char, __oflag: std::ffi::c_int, _: ...) -> std::ffi::c_int;
    fn close(__fd: std::ffi::c_int) -> std::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const std::ffi::c_ushort;
    fn strtol(
        _: *const std::ffi::c_char,
        _: *mut *mut std::ffi::c_char,
        _: std::ffi::c_int,
    ) -> std::ffi::c_long;
    fn free(_: *mut std::ffi::c_void);
    fn strcpy(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strncmp(
        _: *const std::ffi::c_char,
        _: *const std::ffi::c_char,
        _: std::ffi::c_ulong,
    ) -> std::ffi::c_int;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn save(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
    fn skipsp(s: *mut std::ffi::c_char) -> *mut std::ffi::c_char;
    fn cvt_length(len: size_t, ops: std::ffi::c_int) -> size_t;
    fn cvt_alloc_chpos(len: size_t) -> *mut std::ffi::c_int;
    fn cvt_text(
        odst: *mut std::ffi::c_char,
        osrc: *const std::ffi::c_char,
        chpos: *mut std::ffi::c_int,
        lenp: *mut size_t,
        ops: std::ffi::c_int,
    );
    fn isnullenv(s: *const std::ffi::c_char) -> lbool;
    fn edit(filename: *const std::ffi::c_char) -> std::ffi::c_int;
    fn shell_unquote(str: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn shell_quote(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn forw_raw_line(
        curr_pos: POSITION,
        linep: *mut *const std::ffi::c_char,
        line_lenp: *mut size_t,
    ) -> POSITION;
    fn add_lnum(linenum: LINENUM, pos: POSITION);
    fn find_linenum(pos: POSITION) -> LINENUM;
    fn find_pos(linenum: LINENUM) -> POSITION;
    fn getnum(
        sp: *mut *mut std::ffi::c_char,
        printopt: *const std::ffi::c_char,
        errp: *mut lbool,
    ) -> std::ffi::c_int;
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    static mut linenums: std::ffi::c_int;
    static mut sigs: std::ffi::c_int;
    static mut ctldisp: std::ffi::c_int;
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
pub type C2RustUnnamed = std::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub struct tag {
    pub next: *mut tag,
    pub prev: *mut tag,
    pub tag_file: *mut std::ffi::c_char,
    pub tag_linenum: LINENUM,
    pub tag_pattern: *mut std::ffi::c_char,
    pub tag_endline: lbool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct taglist {
    pub tl_first: *mut tag,
    pub tl_last: *mut tag,
}
pub const T_GTAGS: C2RustUnnamed_0 = 2;
pub const T_CTAGS: C2RustUnnamed_0 = 0;
pub const T_CTAGS_X: C2RustUnnamed_0 = 1;
pub const T_GPATH: C2RustUnnamed_0 = 5;
pub const T_GSYMS: C2RustUnnamed_0 = 4;
pub const T_GRTAGS: C2RustUnnamed_0 = 3;
pub const TAG_NOTYPE: tag_result = 3;
pub const TAG_NOTAG: tag_result = 2;
pub const TAG_NOFILE: tag_result = 1;
pub const TAG_INTR: tag_result = 4;
pub const TAG_FOUND: tag_result = 0;
pub type tag_result = std::ffi::c_uint;
pub type C2RustUnnamed_0 = std::ffi::c_uint;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const std::ffi::c_char) -> std::ffi::c_int {
    return strtol(
        __nptr,
        0 as *mut std::ffi::c_void as *mut *mut std::ffi::c_char,
        10 as std::ffi::c_int,
    ) as std::ffi::c_int;
}
#[no_mangle]
pub static mut ztags: [std::ffi::c_char; 5] =
    unsafe { *::core::mem::transmute::<&[u8; 5], &[std::ffi::c_char; 5]>(b"tags\0") };
#[no_mangle]
pub static mut tags: *const std::ffi::c_char = unsafe { ztags.as_ptr() };
static mut total: std::ffi::c_int = 0;
static mut curseq: std::ffi::c_int = 0;
static mut taglist: taglist = unsafe {
    {
        let mut init = taglist {
            tl_first: &taglist as *const taglist as *mut taglist as *mut tag,
            tl_last: &taglist as *const taglist as *mut taglist as *mut tag,
        };
        init
    }
};
static mut curtag: *mut tag = 0 as *const tag as *mut tag;
#[no_mangle]
pub unsafe extern "C" fn cleantags() {
    let mut tp: *mut tag = 0 as *mut tag;
    loop {
        tp = taglist.tl_first;
        if !(tp != &mut taglist as *mut taglist as *mut tag) {
            break;
        }
        (*(*tp).next).prev = (*tp).prev;
        (*(*tp).prev).next = (*tp).next;
        free((*tp).tag_file as *mut std::ffi::c_void);
        free((*tp).tag_pattern as *mut std::ffi::c_void);
        free(tp as *mut std::ffi::c_void);
    }
    curtag = 0 as *mut tag;
    curseq = 0 as std::ffi::c_int;
    total = curseq;
}
unsafe extern "C" fn maketagent(
    mut file: *const std::ffi::c_char,
    mut linenum: LINENUM,
    mut pattern: *const std::ffi::c_char,
    mut endline: lbool,
) -> *mut tag {
    let mut tp: *mut tag = 0 as *mut tag;
    tp = ecalloc(
        ::core::mem::size_of::<tag>() as std::ffi::c_ulong,
        1 as std::ffi::c_int as size_t,
    ) as *mut tag;
    (*tp).tag_file = ecalloc(
        (strlen(file)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong),
        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_char;
    strcpy((*tp).tag_file, file);
    (*tp).tag_linenum = linenum;
    (*tp).tag_endline = endline;
    if pattern.is_null() {
        (*tp).tag_pattern = 0 as *mut std::ffi::c_char;
    } else {
        (*tp).tag_pattern = ecalloc(
            (strlen(pattern)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong),
            ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
        ) as *mut std::ffi::c_char;
        strcpy((*tp).tag_pattern, pattern);
    }
    return tp;
}
#[no_mangle]
pub unsafe extern "C" fn gettagtype() -> std::ffi::c_int {
    let mut f: std::ffi::c_int = 0;
    if strcmp(tags, b"GTAGS\0" as *const u8 as *const std::ffi::c_char) == 0 as std::ffi::c_int {
        return T_GTAGS as std::ffi::c_int;
    }
    if strcmp(tags, b"GRTAGS\0" as *const u8 as *const std::ffi::c_char) == 0 as std::ffi::c_int {
        return T_GRTAGS as std::ffi::c_int;
    }
    if strcmp(tags, b"GSYMS\0" as *const u8 as *const std::ffi::c_char) == 0 as std::ffi::c_int {
        return T_GSYMS as std::ffi::c_int;
    }
    if strcmp(tags, b"GPATH\0" as *const u8 as *const std::ffi::c_char) == 0 as std::ffi::c_int {
        return T_GPATH as std::ffi::c_int;
    }
    if strcmp(tags, b"-\0" as *const u8 as *const std::ffi::c_char) == 0 as std::ffi::c_int {
        return T_CTAGS_X as std::ffi::c_int;
    }
    f = open(tags, 0 as std::ffi::c_int);
    if f >= 0 as std::ffi::c_int {
        close(f);
        return T_CTAGS as std::ffi::c_int;
    }
    return T_GTAGS as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn findtag(mut tag: *const std::ffi::c_char) {
    let mut type_0: std::ffi::c_int = gettagtype();
    let mut result: tag_result = TAG_FOUND;
    if type_0 == T_CTAGS as std::ffi::c_int {
        result = findctag(tag);
    } else {
        result = findgtag(tag, type_0);
    }
    match result as std::ffi::c_uint {
        1 => {
            error(
                b"No tags file\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
        }
        2 => {
            error(
                b"No such tag in tags file\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
        }
        3 => {
            error(
                b"unknown tag type\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
        }
        0 | 4 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn tagsearch() -> POSITION {
    if curtag.is_null() {
        return -(1 as std::ffi::c_int) as POSITION;
    }
    if (*curtag).tag_linenum != 0 as std::ffi::c_int as LINENUM {
        return gtagsearch();
    } else {
        return ctagsearch();
    };
}
#[no_mangle]
pub unsafe extern "C" fn nexttag(mut n: std::ffi::c_int) -> *const std::ffi::c_char {
    let mut tagfile: *const std::ffi::c_char = 0 as *mut std::ffi::c_void as *mut std::ffi::c_char;
    loop {
        let fresh0 = n;
        n = n - 1;
        if !(fresh0 > 0 as std::ffi::c_int) {
            break;
        }
        tagfile = nextgtag();
    }
    return tagfile;
}
#[no_mangle]
pub unsafe extern "C" fn prevtag(mut n: std::ffi::c_int) -> *const std::ffi::c_char {
    let mut tagfile: *const std::ffi::c_char = 0 as *mut std::ffi::c_void as *mut std::ffi::c_char;
    loop {
        let fresh1 = n;
        n = n - 1;
        if !(fresh1 > 0 as std::ffi::c_int) {
            break;
        }
        tagfile = prevgtag();
    }
    return tagfile;
}
#[no_mangle]
pub unsafe extern "C" fn ntags() -> std::ffi::c_int {
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn curr_tag() -> std::ffi::c_int {
    return curseq;
}
unsafe extern "C" fn findctag(mut tag: *const std::ffi::c_char) -> tag_result {
    let mut p: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut q: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut taglen: size_t = 0;
    let mut taglinenum: LINENUM = 0;
    let mut tagfile: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut tagpattern: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut tagendline: lbool = LFALSE;
    let mut search_char: std::ffi::c_int = 0;
    let mut err: lbool = LFALSE;
    let mut tline: [std::ffi::c_char; 1024] = [0; 1024];
    let mut tp: *mut tag = 0 as *mut tag;
    p = shell_unquote(tags);
    f = fopen(p, b"r\0" as *const u8 as *const std::ffi::c_char);
    free(p as *mut std::ffi::c_void);
    if f.is_null() {
        return TAG_NOFILE;
    }
    cleantags();
    total = 0 as std::ffi::c_int;
    taglen = strlen(tag);
    while !(fgets(
        tline.as_mut_ptr(),
        ::core::mem::size_of::<[std::ffi::c_char; 1024]>() as std::ffi::c_ulong as std::ffi::c_int,
        f,
    ))
    .is_null()
    {
        if !(tline[0 as std::ffi::c_int as usize] as std::ffi::c_int == '!' as i32) {
            if strncmp(tag, tline.as_mut_ptr(), taglen) != 0 as std::ffi::c_int
                || !(tline[taglen as usize] as std::ffi::c_int == ' ' as i32
                    || tline[taglen as usize] as std::ffi::c_int == '\t' as i32)
            {
                continue;
            }
            tagpattern = 0 as *mut std::ffi::c_char;
            p = skipsp(tline.as_mut_ptr().offset(taglen as isize));
            if !(*p as std::ffi::c_int == '\0' as i32) {
                tagfile = p;
                while !(*p as std::ffi::c_int == ' ' as i32 || *p as std::ffi::c_int == '\t' as i32)
                    && *p as std::ffi::c_int != '\0' as i32
                {
                    p = p.offset(1);
                }
                let fresh2 = p;
                p = p.offset(1);
                *fresh2 = '\0' as i32 as std::ffi::c_char;
                p = skipsp(p);
                if !(*p as std::ffi::c_int == '\0' as i32) {
                    tagendline = LFALSE;
                    taglinenum = getnum(&mut p, 0 as *const std::ffi::c_char, &mut err) as LINENUM;
                    if err as u64 != 0 {
                        taglinenum = 0 as std::ffi::c_int as LINENUM;
                        let fresh3 = p;
                        p = p.offset(1);
                        search_char = *fresh3 as std::ffi::c_int;
                        if *p as std::ffi::c_int == '^' as i32 {
                            p = p.offset(1);
                        }
                        q = p;
                        tagpattern = q;
                        while *p as std::ffi::c_int != search_char
                            && *p as std::ffi::c_int != '\0' as i32
                        {
                            if *p as std::ffi::c_int == '\\' as i32 {
                                p = p.offset(1);
                            }
                            if q != p {
                                let fresh4 = p;
                                p = p.offset(1);
                                let fresh5 = q;
                                q = q.offset(1);
                                *fresh5 = *fresh4;
                            } else {
                                q = q.offset(1);
                                p = p.offset(1);
                            }
                        }
                        tagendline = (*q.offset(-(1 as std::ffi::c_int) as isize)
                            as std::ffi::c_int
                            == '$' as i32) as std::ffi::c_int
                            as lbool;
                        if tagendline as u64 != 0 {
                            q = q.offset(-1);
                        }
                        *q = '\0' as i32 as std::ffi::c_char;
                    }
                    tp = maketagent(tagfile, taglinenum, tagpattern, tagendline);
                    (*tp).next = &mut taglist as *mut taglist as *mut tag;
                    (*tp).prev = taglist.tl_last;
                    (*taglist.tl_last).next = tp;
                    taglist.tl_last = tp;
                    total += 1;
                }
            }
        }
    }
    fclose(f);
    if total == 0 as std::ffi::c_int {
        return TAG_NOTAG;
    }
    curtag = taglist.tl_first;
    curseq = 1 as std::ffi::c_int;
    return TAG_FOUND;
}
#[no_mangle]
pub unsafe extern "C" fn edit_tagfile() -> std::ffi::c_int {
    if curtag.is_null() {
        return 1 as std::ffi::c_int;
    }
    return edit((*curtag).tag_file);
}
unsafe extern "C" fn curtag_match(
    mut line: *const std::ffi::c_char,
    mut linepos: POSITION,
) -> std::ffi::c_int {
    let mut len: size_t = strlen((*curtag).tag_pattern);
    if strncmp((*curtag).tag_pattern, line, len) == 0 as std::ffi::c_int
        && ((*curtag).tag_endline as u64 == 0
            || *line.offset(len as isize) as std::ffi::c_int == '\0' as i32
            || *line.offset(len as isize) as std::ffi::c_int == '\r' as i32)
    {
        (*curtag).tag_linenum = find_linenum(linepos);
        return 1 as std::ffi::c_int;
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn ctagsearch() -> POSITION {
    let mut pos: POSITION = 0;
    let mut linepos: POSITION = 0;
    let mut linenum: LINENUM = 0;
    let mut line_len: size_t = 0;
    let mut line: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut found: std::ffi::c_int = 0;
    pos = 0 as std::ffi::c_int as POSITION;
    linenum = find_linenum(pos);
    found = 0 as std::ffi::c_int;
    while found == 0 {
        if sigs
            & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
            != 0
        {
            return -(1 as std::ffi::c_int) as POSITION;
        }
        linepos = pos;
        pos = forw_raw_line(pos, &mut line, &mut line_len);
        if linenum != 0 as std::ffi::c_int as LINENUM {
            linenum += 1;
        }
        if pos == -(1 as std::ffi::c_int) as POSITION {
            error(
                b"Tag not found\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
            return -(1 as std::ffi::c_int) as POSITION;
        }
        if linenums != 0 {
            add_lnum(linenum, pos);
        }
        if ctldisp != 2 as std::ffi::c_int {
            if curtag_match(line, linepos) != 0 {
                found = 1 as std::ffi::c_int;
            }
        } else {
            let mut cvt_ops: std::ffi::c_int = 0o10 as std::ffi::c_int;
            let mut cvt_len: size_t = cvt_length(line_len, cvt_ops);
            let mut chpos: *mut std::ffi::c_int = cvt_alloc_chpos(cvt_len);
            let mut cline: *mut std::ffi::c_char =
                ecalloc(1 as std::ffi::c_int as size_t, cvt_len) as *mut std::ffi::c_char;
            cvt_text(cline, line, chpos, &mut line_len, cvt_ops);
            if curtag_match(cline, linepos) != 0 {
                found = 1 as std::ffi::c_int;
            }
            free(chpos as *mut std::ffi::c_void);
            free(cline as *mut std::ffi::c_void);
        }
    }
    return linepos;
}
unsafe extern "C" fn findgtag(
    mut tag: *const std::ffi::c_char,
    mut type_0: std::ffi::c_int,
) -> tag_result {
    let mut buf: [std::ffi::c_char; 1024] = [0; 1024];
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut tp: *mut tag = 0 as *mut tag;
    if type_0 != T_CTAGS_X as std::ffi::c_int && tag.is_null() {
        return TAG_NOFILE;
    }
    cleantags();
    total = 0 as std::ffi::c_int;
    if type_0 == T_CTAGS_X as std::ffi::c_int {
        fp = stdin;
        tags = ztags.as_ptr();
    } else {
        let mut command: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
        let mut flag: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
        let mut qtag: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
        let cmd;
        if let Ok(c) = lgetenv("LESSGLOBALTAGS") {
            cmd = CString::new(c).unwrap().as_ptr();
        } else {
            return TAG_NOFILE;
        }
        match type_0 {
            2 => {
                flag = b"\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char;
            }
            3 => {
                flag = b"r\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char;
            }
            4 => {
                flag = b"s\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char;
            }
            5 => {
                flag = b"P\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char;
            }
            _ => return TAG_NOTYPE,
        }
        qtag = shell_quote(tag);
        if qtag.is_null() {
            qtag = save(tag);
        }
        command = ecalloc(
            (strlen(cmd))
                .wrapping_add(strlen(flag))
                .wrapping_add(strlen(qtag))
                .wrapping_add(5 as std::ffi::c_int as std::ffi::c_ulong),
            ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
        ) as *mut std::ffi::c_char;
        sprintf(
            command,
            b"%s -x%s %s\0" as *const u8 as *const std::ffi::c_char,
            cmd,
            flag,
            qtag,
        );
        free(qtag as *mut std::ffi::c_void);
        fp = popen(command, b"r\0" as *const u8 as *const std::ffi::c_char);
        free(command as *mut std::ffi::c_void);
    }
    if !fp.is_null() {
        while !(fgets(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[std::ffi::c_char; 1024]>() as std::ffi::c_ulong
                as std::ffi::c_int,
            fp,
        ))
        .is_null()
        {
            let mut name: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
            let mut file: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
            let mut line: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
            let mut len: size_t = 0;
            if sigs != 0 {
                if fp != stdin {
                    pclose(fp);
                }
                return TAG_INTR;
            }
            len = strlen(buf.as_mut_ptr());
            if len > 0 as std::ffi::c_int as size_t
                && buf[len.wrapping_sub(1 as std::ffi::c_int as size_t) as usize] as std::ffi::c_int
                    == '\n' as i32
            {
                buf[len.wrapping_sub(1 as std::ffi::c_int as size_t) as usize] =
                    '\0' as i32 as std::ffi::c_char;
            } else {
                let mut c: std::ffi::c_int = 0;
                loop {
                    c = fgetc(fp);
                    if !(c != '\n' as i32 && c != -(1 as std::ffi::c_int)) {
                        break;
                    }
                }
            }
            if getentry(buf.as_mut_ptr(), &mut name, &mut file, &mut line) != 0 {
                break;
            }
            tp = maketagent(
                file,
                atoi(line) as LINENUM,
                0 as *const std::ffi::c_char,
                LFALSE,
            );
            (*tp).next = &mut taglist as *mut taglist as *mut tag;
            (*tp).prev = taglist.tl_last;
            (*taglist.tl_last).next = tp;
            taglist.tl_last = tp;
            total += 1;
        }
        if fp != stdin {
            if pclose(fp) != 0 {
                curtag = 0 as *mut tag;
                curseq = 0 as std::ffi::c_int;
                total = curseq;
                return TAG_NOFILE;
            }
        }
    }
    tp = taglist.tl_first;
    if tp == &mut taglist as *mut taglist as *mut tag {
        return TAG_NOTAG;
    }
    curtag = tp;
    curseq = 1 as std::ffi::c_int;
    return TAG_FOUND;
}
static mut circular: std::ffi::c_int = 0 as std::ffi::c_int;
unsafe extern "C" fn nextgtag() -> *const std::ffi::c_char {
    let mut tp: *mut tag = 0 as *mut tag;
    if curtag.is_null() {
        return 0 as *const std::ffi::c_char;
    }
    tp = (*curtag).next;
    if tp == &mut taglist as *mut taglist as *mut tag {
        if circular == 0 {
            return 0 as *const std::ffi::c_char;
        }
        curtag = taglist.tl_first;
        curseq = 1 as std::ffi::c_int;
    } else {
        curtag = tp;
        curseq += 1;
    }
    return (*curtag).tag_file;
}
unsafe extern "C" fn prevgtag() -> *const std::ffi::c_char {
    let mut tp: *mut tag = 0 as *mut tag;
    if curtag.is_null() {
        return 0 as *const std::ffi::c_char;
    }
    tp = (*curtag).prev;
    if tp == &mut taglist as *mut taglist as *mut tag {
        if circular == 0 {
            return 0 as *const std::ffi::c_char;
        }
        curtag = taglist.tl_last;
        curseq = total;
    } else {
        curtag = tp;
        curseq -= 1;
    }
    return (*curtag).tag_file;
}
unsafe extern "C" fn gtagsearch() -> POSITION {
    if curtag.is_null() {
        return -(1 as std::ffi::c_int) as POSITION;
    }
    return find_pos((*curtag).tag_linenum);
}
unsafe extern "C" fn getentry(
    mut buf: *mut std::ffi::c_char,
    mut tag: *mut *const std::ffi::c_char,
    mut file: *mut *const std::ffi::c_char,
    mut line: *mut *const std::ffi::c_char,
) -> std::ffi::c_int {
    let mut p: *mut std::ffi::c_char = buf;
    *tag = p;
    while *p as std::ffi::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as std::ffi::c_uchar as std::ffi::c_int as isize)
            as std::ffi::c_int
            & _ISspace as std::ffi::c_int as std::ffi::c_ushort as std::ffi::c_int
            == 0
    {
        p = p.offset(1);
    }
    if *p as std::ffi::c_int == 0 as std::ffi::c_int {
        return -(1 as std::ffi::c_int);
    }
    let fresh6 = p;
    p = p.offset(1);
    *fresh6 = 0 as std::ffi::c_int as std::ffi::c_char;
    while *p as std::ffi::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as std::ffi::c_uchar as std::ffi::c_int as isize)
            as std::ffi::c_int
            & _ISspace as std::ffi::c_int as std::ffi::c_ushort as std::ffi::c_int
            != 0
    {
        p = p.offset(1);
    }
    if *p as std::ffi::c_int == 0 as std::ffi::c_int {
        return -(1 as std::ffi::c_int);
    }
    if *(*__ctype_b_loc()).offset(*p as std::ffi::c_uchar as std::ffi::c_int as isize)
        as std::ffi::c_int
        & _ISdigit as std::ffi::c_int as std::ffi::c_ushort as std::ffi::c_int
        == 0
    {
        while *p as std::ffi::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as std::ffi::c_uchar as std::ffi::c_int as isize)
                as std::ffi::c_int
                & _ISspace as std::ffi::c_int as std::ffi::c_ushort as std::ffi::c_int
                == 0
        {
            p = p.offset(1);
        }
        while *p as std::ffi::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as std::ffi::c_uchar as std::ffi::c_int as isize)
                as std::ffi::c_int
                & _ISspace as std::ffi::c_int as std::ffi::c_ushort as std::ffi::c_int
                != 0
        {
            p = p.offset(1);
        }
    }
    if *(*__ctype_b_loc()).offset(*p as std::ffi::c_uchar as std::ffi::c_int as isize)
        as std::ffi::c_int
        & _ISdigit as std::ffi::c_int as std::ffi::c_ushort as std::ffi::c_int
        == 0
    {
        return -(1 as std::ffi::c_int);
    }
    *line = p;
    *line = p;
    while *p as std::ffi::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as std::ffi::c_uchar as std::ffi::c_int as isize)
            as std::ffi::c_int
            & _ISspace as std::ffi::c_int as std::ffi::c_ushort as std::ffi::c_int
            == 0
    {
        p = p.offset(1);
    }
    if *p as std::ffi::c_int == 0 as std::ffi::c_int {
        return -(1 as std::ffi::c_int);
    }
    let fresh7 = p;
    p = p.offset(1);
    *fresh7 = 0 as std::ffi::c_int as std::ffi::c_char;
    while *p as std::ffi::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as std::ffi::c_uchar as std::ffi::c_int as isize)
            as std::ffi::c_int
            & _ISspace as std::ffi::c_int as std::ffi::c_ushort as std::ffi::c_int
            != 0
    {
        p = p.offset(1);
    }
    if *p as std::ffi::c_int == 0 as std::ffi::c_int {
        return -(1 as std::ffi::c_int);
    }
    *file = p;
    *file = p;
    while *p as std::ffi::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as std::ffi::c_uchar as std::ffi::c_int as isize)
            as std::ffi::c_int
            & _ISspace as std::ffi::c_int as std::ffi::c_ushort as std::ffi::c_int
            == 0
    {
        p = p.offset(1);
    }
    if *p as std::ffi::c_int == 0 as std::ffi::c_int {
        return -(1 as std::ffi::c_int);
    }
    *p = 0 as std::ffi::c_int as std::ffi::c_char;
    if strlen(*tag) != 0
        && strlen(*line) != 0
        && strlen(*file) != 0
        && atoi(*line) > 0 as std::ffi::c_int
    {
        return 0 as std::ffi::c_int;
    }
    return -(1 as std::ffi::c_int);
}
