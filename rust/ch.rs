use ::libc;
extern "C" {
    fn lseek(__fd: std::ffi::c_int, __offset: __off_t, __whence: std::ffi::c_int) -> __off_t;
    fn close(__fd: std::ffi::c_int) -> std::ffi::c_int;
    fn write(__fd: std::ffi::c_int, __buf: *const std::ffi::c_void, __n: size_t) -> ssize_t;
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
    fn secure_allow(features: std::ffi::c_int) -> std::ffi::c_int;
    fn clear_eol();
    fn screen_trashed_num(trashed: std::ffi::c_int);
    fn filesize(f: std::ffi::c_int) -> POSITION;
    fn curr_ifile_changed() -> lbool;
    fn get_filestate(ifile: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn set_filestate(ifile: *mut std::ffi::c_void, filestate: *mut std::ffi::c_void);
    fn iread(fd: std::ffi::c_int, buf: *mut std::ffi::c_uchar, len: size_t) -> ssize_t;
    fn sleep_ms(ms: std::ffi::c_int);
    fn flush();
    fn putstr(s: *const std::ffi::c_char);
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn ierror(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn ixerror(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn wait_message() -> *const std::ffi::c_char;
    static mut autobuf: std::ffi::c_int;
    static mut sigs: std::ffi::c_int;
    static mut follow_mode: std::ffi::c_int;
    static mut waiting_for_data: lbool;
    static helpdata: [std::ffi::c_char; 0];
    static size_helpdata: std::ffi::c_int;
    static mut curr_ifile: *mut std::ffi::c_void;
    static mut logfile: std::ffi::c_int;
    static mut namelogfile: *mut std::ffi::c_char;
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
pub struct filestate {
    pub buflist: bufnode,
    pub hashtbl: [bufnode; 1024],
    pub file: std::ffi::c_int,
    pub flags: std::ffi::c_int,
    pub fpos: POSITION,
    pub nbufs: std::ffi::c_int,
    pub block: BLOCKNUM,
    pub offset: size_t,
    pub fsize: POSITION,
}
pub type BLOCKNUM = POSITION;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bufnode {
    pub next: *mut bufnode,
    pub prev: *mut bufnode,
    pub hnext: *mut bufnode,
    pub hprev: *mut bufnode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf {
    pub node: bufnode,
    pub block: BLOCKNUM,
    pub datasize: size_t,
    pub data: [std::ffi::c_uchar; 8192],
}
#[no_mangle]
pub static mut ignore_eoi: std::ffi::c_int = 0;
static mut thisfile: *mut filestate = 0 as *const filestate as *mut filestate;
static mut ch_ungotchar: std::ffi::c_uchar = 0;
static mut ch_have_ungotchar: lbool = LFALSE;
static mut maxbufs: std::ffi::c_int = -(1 as std::ffi::c_int);
unsafe extern "C" fn ch_position(mut block: BLOCKNUM, mut offset: size_t) -> POSITION {
    return block * 8192 as std::ffi::c_int as BLOCKNUM + offset as POSITION;
}
unsafe extern "C" fn ch_get() -> std::ffi::c_int {
    let mut read_again: lbool = LFALSE;
    let mut len: POSITION = 0;
    let mut pos: POSITION = 0;
    let mut read_pipe_at_eof: lbool = LFALSE;
    let mut current_block: u64;
    let mut bp: *mut buf = 0 as *mut buf;
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    let mut n: ssize_t = 0;
    let mut h: std::ffi::c_int = 0;
    if thisfile.is_null() {
        return -(1 as std::ffi::c_int);
    }
    if (*thisfile).buflist.next != &mut (*thisfile).buflist as *mut bufnode {
        bp = (*thisfile).buflist.next as *mut buf;
        if (*thisfile).block == (*bp).block && (*thisfile).offset < (*bp).datasize {
            return (*bp).data[(*thisfile).offset as usize] as std::ffi::c_int;
        }
    }
    waiting_for_data = LFALSE;
    h = ((*thisfile).block & (1024 as std::ffi::c_int - 1 as std::ffi::c_int) as BLOCKNUM)
        as std::ffi::c_int;
    bn = (*thisfile).hashtbl[h as usize].hnext;
    loop {
        if !(bn != &mut *((*thisfile).hashtbl).as_mut_ptr().offset(h as isize) as *mut bufnode) {
            current_block = 12800627514080957624;
            break;
        }
        bp = bn as *mut buf;
        if (*bp).block == (*thisfile).block {
            if (*thisfile).offset >= (*bp).datasize {
                current_block = 12800627514080957624;
                break;
            } else {
                current_block = 4410251971103114040;
                break;
            }
        } else {
            bn = (*bn).hnext;
        }
    }
    match current_block {
        12800627514080957624 => {
            if sigs
                & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
                != 0
            {
                return -(1 as std::ffi::c_int);
            }
            if bn == &mut *((*thisfile).hashtbl).as_mut_ptr().offset(h as isize) as *mut bufnode {
                if (*thisfile).buflist.prev == &mut (*thisfile).buflist as *mut bufnode
                    || (*((*thisfile).buflist.prev as *mut buf)).block
                        != -(1 as std::ffi::c_int) as BLOCKNUM
                {
                    if autobuf != 0 && (*thisfile).flags & 0o1 as std::ffi::c_int == 0
                        || (maxbufs < 0 as std::ffi::c_int || (*thisfile).nbufs < maxbufs)
                    {
                        if ch_addbuf() != 0 {
                            autobuf = 0 as std::ffi::c_int;
                        }
                    }
                }
                bn = (*thisfile).buflist.prev;
                bp = bn as *mut buf;
                (*(*bn).hnext).hprev = (*bn).hprev;
                (*(*bn).hprev).hnext = (*bn).hnext;
                (*bp).block = (*thisfile).block;
                (*bp).datasize = 0 as std::ffi::c_int as size_t;
                (*bn).hnext = (*thisfile).hashtbl[h as usize].hnext;
                (*bn).hprev =
                    &mut *((*thisfile).hashtbl).as_mut_ptr().offset(h as isize) as *mut bufnode;
                (*(*thisfile).hashtbl[h as usize].hnext).hprev = bn;
                (*thisfile).hashtbl[h as usize].hnext = bn;
            }
            current_block = 4761528863920922185;
        }
        _ => {}
    }
    loop {
        match current_block {
            4410251971103114040 => {
                if (*thisfile).buflist.next != bn {
                    (*(*bn).next).prev = (*bn).prev;
                    (*(*bn).prev).next = (*bn).next;
                    (*bn).next = (*thisfile).buflist.next;
                    (*bn).prev = &mut (*thisfile).buflist;
                    (*(*thisfile).buflist.next).prev = bn;
                    (*thisfile).buflist.next = bn;
                    (*(*bn).hnext).hprev = (*bn).hprev;
                    (*(*bn).hprev).hnext = (*bn).hnext;
                    (*bn).hnext = (*thisfile).hashtbl[h as usize].hnext;
                    (*bn).hprev =
                        &mut *((*thisfile).hashtbl).as_mut_ptr().offset(h as isize) as *mut bufnode;
                    (*(*thisfile).hashtbl[h as usize].hnext).hprev = bn;
                    (*thisfile).hashtbl[h as usize].hnext = bn;
                }
                if (*thisfile).offset < (*bp).datasize {
                    break;
                } else {
                    current_block = 4761528863920922185;
                }
            }
            _ => {
                read_again = LFALSE;
                len = 0;
                pos = ch_position((*thisfile).block, (*bp).datasize);
                read_pipe_at_eof = LFALSE;
                len = ch_length();
                if len != -(1 as std::ffi::c_int) as POSITION && pos >= len {
                    ch_resize();
                    len = ch_length();
                    if len != -(1 as std::ffi::c_int) as POSITION && pos >= len {
                        if (*thisfile).flags & (0o1 as std::ffi::c_int | 0o10 as std::ffi::c_int)
                            != 0
                        {
                            return -(1 as std::ffi::c_int);
                        }
                        read_pipe_at_eof = LTRUE;
                    }
                }
                if pos != (*thisfile).fpos {
                    if (*thisfile).flags & 0o1 as std::ffi::c_int == 0 {
                        return '?' as i32;
                    }
                    if lseek((*thisfile).file, pos, 0 as std::ffi::c_int)
                        == -(1 as std::ffi::c_int) as off_t
                    {
                        error(
                            b"seek error\0" as *const u8 as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void as *mut PARG,
                        );
                        clear_eol();
                        return -(1 as std::ffi::c_int);
                    }
                    (*thisfile).fpos = pos;
                }
                if ch_have_ungotchar as u64 != 0 {
                    (*bp).data[(*bp).datasize as usize] = ch_ungotchar;
                    n = 1 as std::ffi::c_int as ssize_t;
                    ch_have_ungotchar = LFALSE;
                } else if (*thisfile).flags & 0o10 as std::ffi::c_int != 0 {
                    (*bp).data[(*bp).datasize as usize] =
                        *helpdata.as_ptr().offset((*thisfile).fpos as isize) as std::ffi::c_uchar;
                    n = 1 as std::ffi::c_int as ssize_t;
                } else {
                    n = iread(
                        (*thisfile).file,
                        &mut *((*bp).data).as_mut_ptr().offset((*bp).datasize as isize),
                        (8192 as std::ffi::c_int as size_t).wrapping_sub((*bp).datasize),
                    );
                }
                read_again = LFALSE;
                if n == -(2 as std::ffi::c_int) as ssize_t {
                    if (*thisfile).flags & 0o1 as std::ffi::c_int != 0 {
                        (*thisfile).fsize = pos;
                    }
                    return -(1 as std::ffi::c_int);
                }
                if n == -(3 as std::ffi::c_int) as ssize_t {
                    read_again = LTRUE;
                    n = 0 as std::ffi::c_int as ssize_t;
                }
                if n < 0 as std::ffi::c_int as ssize_t {
                    error(
                        b"read error\0" as *const u8 as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void as *mut PARG,
                    );
                    clear_eol();
                    n = 0 as std::ffi::c_int as ssize_t;
                }
                if secure_allow((1 as std::ffi::c_int) << 7 as std::ffi::c_int) != 0 {
                    if logfile >= 0 as std::ffi::c_int && n > 0 as std::ffi::c_int as ssize_t {
                        write(
                            logfile,
                            &mut *((*bp).data).as_mut_ptr().offset((*bp).datasize as isize)
                                as *mut std::ffi::c_uchar
                                as *const std::ffi::c_void,
                            n as size_t,
                        );
                    }
                }
                (*thisfile).fpos += n;
                (*bp).datasize = ((*bp).datasize).wrapping_add(n as size_t);
                if read_pipe_at_eof as u64 != 0 {
                    ch_set_eof();
                }
                if n == 0 as std::ffi::c_int as ssize_t {
                    if read_again as u64 == 0 {
                        (*thisfile).fsize = pos;
                    }
                    if ignore_eoi != 0 || read_again as std::ffi::c_uint != 0 {
                        if waiting_for_data as u64 == 0 {
                            let mut parg: PARG = parg {
                                p_string: 0 as *const std::ffi::c_char,
                            };
                            parg.p_string = wait_message();
                            ixerror(b"%s\0" as *const u8 as *const std::ffi::c_char, &mut parg);
                            waiting_for_data = LTRUE;
                        }
                        sleep_ms(50 as std::ffi::c_int);
                    }
                    if ignore_eoi != 0
                        && follow_mode == 1 as std::ffi::c_int
                        && curr_ifile_changed() as std::ffi::c_uint != 0
                    {
                        screen_trashed_num(2 as std::ffi::c_int);
                        return -(1 as std::ffi::c_int);
                    }
                    if sigs != 0 {
                        return -(1 as std::ffi::c_int);
                    }
                    if read_pipe_at_eof as u64 != 0 {
                        return -(1 as std::ffi::c_int);
                    }
                }
                current_block = 4410251971103114040;
            }
        }
    }
    return (*bp).data[(*thisfile).offset as usize] as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ch_ungetchar(mut c: std::ffi::c_int) {
    if c < 0 as std::ffi::c_int {
        ch_have_ungotchar = LFALSE;
    } else {
        if ch_have_ungotchar as u64 != 0 {
            error(
                b"ch_ungetchar overrun\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
        }
        ch_ungotchar = c as std::ffi::c_uchar;
        ch_have_ungotchar = LTRUE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn end_logfile() {
    static mut tried: lbool = LFALSE;
    if logfile < 0 as std::ffi::c_int {
        return;
    }
    if tried as u64 == 0 && (*thisfile).fsize == -(1 as std::ffi::c_int) as POSITION {
        tried = LTRUE;
        ierror(
            b"Finishing logfile\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        while ch_forw_get() != -(1 as std::ffi::c_int) {
            if sigs
                & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
                != 0
            {
                break;
            }
        }
    }
    close(logfile);
    logfile = -(1 as std::ffi::c_int);
    free(namelogfile as *mut std::ffi::c_void);
    namelogfile = 0 as *mut std::ffi::c_char;
    putstr(b"\n\0" as *const u8 as *const std::ffi::c_char);
    flush();
}
#[no_mangle]
pub unsafe extern "C" fn sync_logfile() {
    let mut bp: *mut buf = 0 as *mut buf;
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    let mut warned: lbool = LFALSE;
    let mut h: std::ffi::c_int = 0;
    let mut block: BLOCKNUM = 0;
    let mut nblocks: BLOCKNUM = 0;
    if logfile < 0 as std::ffi::c_int {
        return;
    }
    nblocks = ((*thisfile).fpos + 8192 as std::ffi::c_int as POSITION
        - 1 as std::ffi::c_int as POSITION)
        / 8192 as std::ffi::c_int as POSITION;
    block = 0 as std::ffi::c_int as BLOCKNUM;
    while block < nblocks {
        let mut wrote: lbool = LFALSE;
        h = (block & (1024 as std::ffi::c_int - 1 as std::ffi::c_int) as BLOCKNUM)
            as std::ffi::c_int;
        bn = (*thisfile).hashtbl[h as usize].hnext;
        while bn != &mut *((*thisfile).hashtbl).as_mut_ptr().offset(h as isize) as *mut bufnode {
            bp = bn as *mut buf;
            if (*bp).block == block {
                write(
                    logfile,
                    ((*bp).data).as_mut_ptr() as *const std::ffi::c_void,
                    (*bp).datasize,
                );
                wrote = LTRUE;
                break;
            } else {
                bn = (*bn).hnext;
            }
        }
        if wrote as u64 == 0 && warned as u64 == 0 {
            error(
                b"Warning: log file is incomplete\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
            warned = LTRUE;
        }
        block += 1;
    }
}
unsafe extern "C" fn buffered(mut block: BLOCKNUM) -> lbool {
    let mut bp: *mut buf = 0 as *mut buf;
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    let mut h: std::ffi::c_int = 0;
    h = (block & (1024 as std::ffi::c_int - 1 as std::ffi::c_int) as BLOCKNUM) as std::ffi::c_int;
    bn = (*thisfile).hashtbl[h as usize].hnext;
    while bn != &mut *((*thisfile).hashtbl).as_mut_ptr().offset(h as isize) as *mut bufnode {
        bp = bn as *mut buf;
        if (*bp).block == block {
            return LTRUE;
        }
        bn = (*bn).hnext;
    }
    return LFALSE;
}
#[no_mangle]
pub unsafe extern "C" fn ch_seek(mut pos: POSITION) -> std::ffi::c_int {
    let mut new_block: BLOCKNUM = 0;
    let mut len: POSITION = 0;
    if thisfile.is_null() {
        return 0 as std::ffi::c_int;
    }
    len = ch_length();
    if pos < 0 as std::ffi::c_int as POSITION
        || len != -(1 as std::ffi::c_int) as POSITION && pos > len
    {
        return 1 as std::ffi::c_int;
    }
    new_block = pos / 8192 as std::ffi::c_int as POSITION;
    if (*thisfile).flags & 0o1 as std::ffi::c_int == 0
        && pos != (*thisfile).fpos
        && buffered(new_block) as u64 == 0
    {
        if (*thisfile).fpos > pos {
            return 1 as std::ffi::c_int;
        }
        while (*thisfile).fpos < pos {
            if ch_forw_get() == -(1 as std::ffi::c_int) {
                return 1 as std::ffi::c_int;
            }
            if sigs
                & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
                != 0
            {
                return 1 as std::ffi::c_int;
            }
        }
        return 0 as std::ffi::c_int;
    }
    (*thisfile).block = new_block;
    (*thisfile).offset = (pos % 8192 as std::ffi::c_int as POSITION) as size_t;
    return 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ch_end_seek() -> std::ffi::c_int {
    let mut len: POSITION = 0;
    if thisfile.is_null() {
        return 0 as std::ffi::c_int;
    }
    if (*thisfile).flags & 0o1 as std::ffi::c_int != 0 {
        (*thisfile).fsize = filesize((*thisfile).file);
    }
    len = ch_length();
    if len != -(1 as std::ffi::c_int) as POSITION {
        return ch_seek(len);
    }
    while ch_forw_get() != -(1 as std::ffi::c_int) {
        if sigs
            & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
            != 0
        {
            return 1 as std::ffi::c_int;
        }
    }
    return 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ch_end_buffer_seek() -> std::ffi::c_int {
    let mut bp: *mut buf = 0 as *mut buf;
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    let mut buf_pos: POSITION = 0;
    let mut end_pos: POSITION = 0;
    if thisfile.is_null() || (*thisfile).flags & 0o1 as std::ffi::c_int != 0 {
        return ch_end_seek();
    }
    end_pos = 0 as std::ffi::c_int as POSITION;
    bn = (*thisfile).buflist.next;
    while bn != &mut (*thisfile).buflist as *mut bufnode {
        bp = bn as *mut buf;
        buf_pos = ch_position((*bp).block, (*bp).datasize);
        if buf_pos > end_pos {
            end_pos = buf_pos;
        }
        bn = (*bn).next;
    }
    return ch_seek(end_pos);
}
#[no_mangle]
pub unsafe extern "C" fn ch_beg_seek() -> std::ffi::c_int {
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    let mut firstbn: *mut bufnode = 0 as *mut bufnode;
    if ch_seek(0 as std::ffi::c_int as POSITION) == 0 as std::ffi::c_int {
        return 0 as std::ffi::c_int;
    }
    firstbn = (*thisfile).buflist.next;
    if firstbn == &mut (*thisfile).buflist as *mut bufnode {
        return 1 as std::ffi::c_int;
    }
    bn = (*thisfile).buflist.next;
    while bn != &mut (*thisfile).buflist as *mut bufnode {
        if (*(bn as *mut buf)).block < (*(firstbn as *mut buf)).block {
            firstbn = bn;
        }
        bn = (*bn).next;
    }
    (*thisfile).block = (*(firstbn as *mut buf)).block;
    (*thisfile).offset = 0 as std::ffi::c_int as size_t;
    return 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ch_length() -> POSITION {
    if thisfile.is_null() {
        return -(1 as std::ffi::c_int) as POSITION;
    }
    if ignore_eoi != 0 {
        return -(1 as std::ffi::c_int) as POSITION;
    }
    if (*thisfile).flags & 0o10 as std::ffi::c_int != 0 {
        return size_helpdata as POSITION;
    }
    if (*thisfile).flags & 0o20 as std::ffi::c_int != 0 {
        return 0 as std::ffi::c_int as POSITION;
    }
    return (*thisfile).fsize;
}
#[no_mangle]
pub unsafe extern "C" fn ch_resize() {
    let mut fsize: POSITION = 0;
    if (*thisfile).flags & 0o1 as std::ffi::c_int == 0 {
        return;
    }
    fsize = filesize((*thisfile).file);
    if fsize != -(1 as std::ffi::c_int) as POSITION {
        (*thisfile).fsize = fsize;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ch_tell() -> POSITION {
    if thisfile.is_null() {
        return -(1 as std::ffi::c_int) as POSITION;
    }
    return ch_position((*thisfile).block, (*thisfile).offset);
}
#[no_mangle]
pub unsafe extern "C" fn ch_forw_get() -> std::ffi::c_int {
    let mut c: std::ffi::c_int = 0;
    if thisfile.is_null() {
        return -(1 as std::ffi::c_int);
    }
    c = ch_get();
    if c == -(1 as std::ffi::c_int) {
        return -(1 as std::ffi::c_int);
    }
    if (*thisfile).offset < (8192 as std::ffi::c_int - 1 as std::ffi::c_int) as size_t {
        (*thisfile).offset = ((*thisfile).offset).wrapping_add(1);
        (*thisfile).offset;
    } else {
        (*thisfile).block += 1;
        (*thisfile).block;
        (*thisfile).offset = 0 as std::ffi::c_int as size_t;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn ch_back_get() -> std::ffi::c_int {
    if thisfile.is_null() {
        return -(1 as std::ffi::c_int);
    }
    if (*thisfile).offset > 0 as std::ffi::c_int as size_t {
        (*thisfile).offset = ((*thisfile).offset).wrapping_sub(1);
        (*thisfile).offset;
    } else {
        if (*thisfile).block <= 0 as std::ffi::c_int as BLOCKNUM {
            return -(1 as std::ffi::c_int);
        }
        if (*thisfile).flags & 0o1 as std::ffi::c_int == 0
            && buffered((*thisfile).block - 1 as std::ffi::c_int as BLOCKNUM) as u64 == 0
        {
            return -(1 as std::ffi::c_int);
        }
        (*thisfile).block -= 1;
        (*thisfile).block;
        (*thisfile).offset = (8192 as std::ffi::c_int - 1 as std::ffi::c_int) as size_t;
    }
    return ch_get();
}
#[no_mangle]
pub unsafe extern "C" fn ch_setbufspace(mut bufspace: ssize_t) {
    if bufspace < 0 as std::ffi::c_int as ssize_t {
        maxbufs = -(1 as std::ffi::c_int);
    } else {
        let mut lbufk: size_t = (8192 as std::ffi::c_int / 1024 as std::ffi::c_int) as size_t;
        maxbufs = (bufspace as size_t / lbufk).wrapping_add(
            (bufspace as size_t % lbufk != 0 as std::ffi::c_int as size_t) as std::ffi::c_int
                as size_t,
        ) as std::ffi::c_int;
        if maxbufs < 1 as std::ffi::c_int {
            maxbufs = 1 as std::ffi::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ch_flush() {
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    if thisfile.is_null() {
        return;
    }
    if (*thisfile).flags & 0o1 as std::ffi::c_int == 0 {
        (*thisfile).fsize = -(1 as std::ffi::c_int) as POSITION;
        return;
    }
    bn = (*thisfile).buflist.next;
    while bn != &mut (*thisfile).buflist as *mut bufnode {
        (*(bn as *mut buf)).block = -(1 as std::ffi::c_int) as BLOCKNUM;
        bn = (*bn).next;
    }
    (*thisfile).fpos = 0 as std::ffi::c_int as POSITION;
    (*thisfile).block = 0 as std::ffi::c_int as BLOCKNUM;
    (*thisfile).offset = 0 as std::ffi::c_int as size_t;
    if (*thisfile).flags & 0o40 as std::ffi::c_int != 0 {
        (*thisfile).fsize = -(1 as std::ffi::c_int) as POSITION;
        (*thisfile).flags &= !(0o1 as std::ffi::c_int);
    } else {
        (*thisfile).fsize = if (*thisfile).flags & 0o10 as std::ffi::c_int != 0 {
            size_helpdata as POSITION
        } else {
            filesize((*thisfile).file)
        };
    }
    if lseek(
        (*thisfile).file,
        0 as std::ffi::c_int as less_off_t,
        0 as std::ffi::c_int,
    ) == -(1 as std::ffi::c_int) as off_t
    {
        error(
            b"seek error to 0\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
    }
}
unsafe extern "C" fn ch_addbuf() -> std::ffi::c_int {
    let mut bp: *mut buf = 0 as *mut buf;
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    bp = calloc(
        1 as std::ffi::c_int as std::ffi::c_ulong,
        ::core::mem::size_of::<buf>() as std::ffi::c_ulong,
    ) as *mut buf;
    if bp.is_null() {
        return 1 as std::ffi::c_int;
    }
    (*thisfile).nbufs += 1;
    (*thisfile).nbufs;
    (*bp).block = -(1 as std::ffi::c_int) as BLOCKNUM;
    bn = &mut (*bp).node;
    (*bn).next = &mut (*thisfile).buflist;
    (*bn).prev = (*thisfile).buflist.prev;
    (*(*thisfile).buflist.prev).next = bn;
    (*thisfile).buflist.prev = bn;
    (*bn).hnext = (*thisfile).hashtbl[0 as std::ffi::c_int as usize].hnext;
    (*bn).hprev = &mut *((*thisfile).hashtbl)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize) as *mut bufnode;
    (*(*thisfile).hashtbl[0 as std::ffi::c_int as usize].hnext).hprev = bn;
    (*thisfile).hashtbl[0 as std::ffi::c_int as usize].hnext = bn;
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn init_hashtbl() {
    let mut h: std::ffi::c_int = 0;
    h = 0 as std::ffi::c_int;
    while h < 1024 as std::ffi::c_int {
        (*thisfile).hashtbl[h as usize].hnext =
            &mut *((*thisfile).hashtbl).as_mut_ptr().offset(h as isize) as *mut bufnode;
        (*thisfile).hashtbl[h as usize].hprev =
            &mut *((*thisfile).hashtbl).as_mut_ptr().offset(h as isize) as *mut bufnode;
        h += 1;
    }
}
unsafe extern "C" fn ch_delbufs() {
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    while (*thisfile).buflist.next != &mut (*thisfile).buflist as *mut bufnode {
        bn = (*thisfile).buflist.next;
        (*(*bn).next).prev = (*bn).prev;
        (*(*bn).prev).next = (*bn).next;
        free(bn as *mut buf as *mut std::ffi::c_void);
    }
    (*thisfile).nbufs = 0 as std::ffi::c_int;
    init_hashtbl();
}
#[no_mangle]
pub unsafe extern "C" fn seekable(mut f: std::ffi::c_int) -> std::ffi::c_int {
    return (lseek(f, 1 as std::ffi::c_int as less_off_t, 0 as std::ffi::c_int)
        != -(1 as std::ffi::c_int) as off_t) as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ch_set_eof() {
    if (*thisfile).fsize != -(1 as std::ffi::c_int) as POSITION
        && (*thisfile).fsize < (*thisfile).fpos
    {
        (*thisfile).fsize = (*thisfile).fpos;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ch_init(
    mut f: std::ffi::c_int,
    mut flags: std::ffi::c_int,
    mut nread: ssize_t,
) {
    thisfile = get_filestate(curr_ifile) as *mut filestate;
    if thisfile.is_null() {
        thisfile = ecalloc(
            1 as std::ffi::c_int as size_t,
            ::core::mem::size_of::<filestate>() as std::ffi::c_ulong,
        ) as *mut filestate;
        (*thisfile).buflist.prev = &mut (*thisfile).buflist;
        (*thisfile).buflist.next = (*thisfile).buflist.prev;
        (*thisfile).nbufs = 0 as std::ffi::c_int;
        (*thisfile).flags = flags;
        (*thisfile).fpos = 0 as std::ffi::c_int as POSITION;
        (*thisfile).block = 0 as std::ffi::c_int as BLOCKNUM;
        (*thisfile).offset = 0 as std::ffi::c_int as size_t;
        (*thisfile).file = -(1 as std::ffi::c_int);
        (*thisfile).fsize = -(1 as std::ffi::c_int) as POSITION;
        init_hashtbl();
        if flags & 0o1 as std::ffi::c_int != 0 && seekable(f) == 0 {
            (*thisfile).flags &= !(0o1 as std::ffi::c_int);
        }
        set_filestate(curr_ifile, thisfile as *mut std::ffi::c_void);
    }
    if (*thisfile).file == -(1 as std::ffi::c_int) {
        (*thisfile).file = f;
    }
    (*thisfile).fsize = if flags & 0o10 as std::ffi::c_int != 0 {
        size_helpdata as POSITION
    } else {
        filesize((*thisfile).file)
    };
    if (*thisfile).fsize == 0 as std::ffi::c_int as POSITION
        && nread > 0 as std::ffi::c_int as ssize_t
    {
        (*thisfile).flags |= 0o40 as std::ffi::c_int;
    }
    ch_flush();
}
#[no_mangle]
pub unsafe extern "C" fn ch_close() {
    let mut keepstate: lbool = LFALSE;
    if thisfile.is_null() {
        return;
    }
    if (*thisfile).flags
        & (0o1 as std::ffi::c_int | 0o4 as std::ffi::c_int | 0o10 as std::ffi::c_int)
        != 0
        && (*thisfile).flags & 0o2 as std::ffi::c_int == 0
    {
        ch_delbufs();
    } else {
        keepstate = LTRUE;
    }
    if (*thisfile).flags & 0o2 as std::ffi::c_int == 0 {
        if (*thisfile).flags & (0o4 as std::ffi::c_int | 0o10 as std::ffi::c_int) == 0 {
            close((*thisfile).file);
        }
        (*thisfile).file = -(1 as std::ffi::c_int);
    } else {
        keepstate = LTRUE;
    }
    if keepstate as u64 == 0 {
        free(thisfile as *mut std::ffi::c_void);
        thisfile = 0 as *mut filestate;
        set_filestate(curr_ifile, 0 as *mut std::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ch_getflags() -> std::ffi::c_int {
    if thisfile.is_null() {
        return 0 as std::ffi::c_int;
    }
    return (*thisfile).flags;
}
