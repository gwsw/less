use ::libc;
extern "C" {
    fn free(_: *mut std::ffi::c_void);
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn save(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
    fn lrealpath(path: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn unmark(ifile: *mut std::ffi::c_void);
    fn mark_check_ifile(ifile: *mut std::ffi::c_void);
    static mut curr_ifile: *mut std::ffi::c_void;
}
pub type __off_t = std::ffi::c_long;
pub type off_t = __off_t;
pub type size_t = std::ffi::c_ulong;
pub type less_off_t = off_t;
pub type POSITION = less_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scrpos {
    pub pos: POSITION,
    pub ln: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifile {
    pub h_next: *mut ifile,
    pub h_prev: *mut ifile,
    pub h_filename: *mut std::ffi::c_char,
    pub h_rfilename: *mut std::ffi::c_char,
    pub h_filestate: *mut std::ffi::c_void,
    pub h_index: std::ffi::c_int,
    pub h_hold: std::ffi::c_int,
    pub h_opened: std::ffi::c_char,
    pub h_scrpos: scrpos,
    pub h_altpipe: *mut std::ffi::c_void,
    pub h_altfilename: *mut std::ffi::c_char,
}
static mut anchor: ifile = unsafe {
    {
        let mut init = ifile {
            h_next: &anchor as *const ifile as *mut ifile,
            h_prev: &anchor as *const ifile as *mut ifile,
            h_filename: 0 as *const std::ffi::c_char as *mut std::ffi::c_char,
            h_rfilename: 0 as *const std::ffi::c_char as *mut std::ffi::c_char,
            h_filestate: 0 as *const std::ffi::c_void as *mut std::ffi::c_void,
            h_index: 0 as std::ffi::c_int,
            h_hold: 0 as std::ffi::c_int,
            h_opened: '\0' as i32 as std::ffi::c_char,
            h_scrpos: {
                let mut init = scrpos {
                    pos: -(1 as std::ffi::c_int) as POSITION,
                    ln: 0 as std::ffi::c_int,
                };
                init
            },
            h_altpipe: 0 as *const std::ffi::c_void as *mut std::ffi::c_void,
            h_altfilename: 0 as *const std::ffi::c_char as *mut std::ffi::c_char,
        };
        init
    }
};
static mut ifiles: std::ffi::c_int = 0 as std::ffi::c_int;
unsafe extern "C" fn incr_index(mut p: *mut ifile, mut incr: std::ffi::c_int) {
    while p != &mut anchor as *mut ifile {
        (*p).h_index += incr;
        p = (*p).h_next;
    }
}
unsafe extern "C" fn link_ifile(mut p: *mut ifile, mut prev: *mut ifile) {
    if prev.is_null() {
        prev = &mut anchor;
    }
    (*p).h_next = (*prev).h_next;
    (*p).h_prev = prev;
    (*(*prev).h_next).h_prev = p;
    (*prev).h_next = p;
    (*p).h_index = (*prev).h_index + 1 as std::ffi::c_int;
    incr_index((*p).h_next, 1 as std::ffi::c_int);
    ifiles += 1;
}
unsafe extern "C" fn unlink_ifile(mut p: *mut ifile) {
    (*(*p).h_next).h_prev = (*p).h_prev;
    (*(*p).h_prev).h_next = (*p).h_next;
    incr_index((*p).h_next, -(1 as std::ffi::c_int));
    ifiles -= 1;
}
unsafe extern "C" fn new_ifile(
    mut filename: *const std::ffi::c_char,
    mut prev: *mut ifile,
) -> *mut ifile {
    let mut p: *mut ifile = 0 as *mut ifile;
    p = ecalloc(
        1 as std::ffi::c_int as size_t,
        ::core::mem::size_of::<ifile>() as std::ffi::c_ulong,
    ) as *mut ifile;
    (*p).h_filename = save(filename);
    (*p).h_rfilename = lrealpath(filename);
    (*p).h_scrpos.pos = -(1 as std::ffi::c_int) as POSITION;
    (*p).h_opened = 0 as std::ffi::c_int as std::ffi::c_char;
    (*p).h_hold = 0 as std::ffi::c_int;
    (*p).h_filestate = 0 as *mut std::ffi::c_void;
    (*p).h_altfilename = 0 as *mut std::ffi::c_char;
    (*p).h_altpipe = 0 as *mut std::ffi::c_void;
    link_ifile(p, prev);
    mark_check_ifile(p as *mut std::ffi::c_void);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn del_ifile(mut h: *mut std::ffi::c_void) {
    let mut p: *mut ifile = 0 as *mut ifile;
    if h == 0 as *mut std::ffi::c_void {
        return;
    }
    unmark(h);
    if h == curr_ifile {
        curr_ifile = getoff_ifile(curr_ifile);
    }
    p = h as *mut ifile;
    unlink_ifile(p);
    free((*p).h_rfilename as *mut std::ffi::c_void);
    free((*p).h_filename as *mut std::ffi::c_void);
    free(p as *mut std::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn next_ifile(mut h: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    let mut p: *mut ifile = 0 as *mut ifile;
    p = if h == 0 as *mut std::ffi::c_void {
        &mut anchor
    } else {
        h as *mut ifile
    };
    if (*p).h_next == &mut anchor as *mut ifile {
        return 0 as *mut std::ffi::c_void;
    }
    return (*p).h_next as *mut std::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn prev_ifile(mut h: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    let mut p: *mut ifile = 0 as *mut ifile;
    p = if h == 0 as *mut std::ffi::c_void {
        &mut anchor
    } else {
        h as *mut ifile
    };
    if (*p).h_prev == &mut anchor as *mut ifile {
        return 0 as *mut std::ffi::c_void;
    }
    return (*p).h_prev as *mut std::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn getoff_ifile(mut ifile: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    let mut newifile: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    newifile = prev_ifile(ifile);
    if newifile != 0 as *mut std::ffi::c_void {
        return newifile;
    }
    newifile = next_ifile(ifile);
    if newifile != 0 as *mut std::ffi::c_void {
        return newifile;
    }
    return 0 as *mut std::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn nifile() -> std::ffi::c_int {
    return ifiles;
}
unsafe extern "C" fn find_ifile(mut filename: *const std::ffi::c_char) -> *mut ifile {
    let mut p: *mut ifile = 0 as *mut ifile;
    let mut rfilename: *mut std::ffi::c_char = lrealpath(filename);
    p = anchor.h_next;
    while p != &mut anchor as *mut ifile {
        if strcmp(rfilename, (*p).h_rfilename) == 0 as std::ffi::c_int {
            if strlen(filename) < strlen((*p).h_filename) {
                free((*p).h_filename as *mut std::ffi::c_void);
                (*p).h_filename = save(filename);
            }
            break;
        } else {
            p = (*p).h_next;
        }
    }
    free(rfilename as *mut std::ffi::c_void);
    if p == &mut anchor as *mut ifile {
        p = 0 as *mut ifile;
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn get_ifile(
    mut filename: *const std::ffi::c_char,
    mut prev: *mut std::ffi::c_void,
) -> *mut std::ffi::c_void {
    let mut p: *mut ifile = 0 as *mut ifile;
    p = find_ifile(filename);
    if p.is_null() {
        p = new_ifile(filename, prev as *mut ifile);
    }
    return p as *mut std::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn get_filename(mut ifile: *mut std::ffi::c_void) -> *const std::ffi::c_char {
    if ifile.is_null() {
        return 0 as *const std::ffi::c_char;
    }
    return (*(ifile as *mut ifile)).h_filename;
}
#[no_mangle]
pub unsafe extern "C" fn get_real_filename(
    mut ifile: *mut std::ffi::c_void,
) -> *const std::ffi::c_char {
    if ifile.is_null() {
        return 0 as *const std::ffi::c_char;
    }
    return (*(ifile as *mut ifile)).h_rfilename;
}
#[no_mangle]
pub unsafe extern "C" fn get_index(mut ifile: *mut std::ffi::c_void) -> std::ffi::c_int {
    return (*(ifile as *mut ifile)).h_index;
}
#[no_mangle]
pub unsafe extern "C" fn store_pos(mut ifile: *mut std::ffi::c_void, mut scrpos: *mut scrpos) {
    (*(ifile as *mut ifile)).h_scrpos = *scrpos;
}
#[no_mangle]
pub unsafe extern "C" fn get_pos(mut ifile: *mut std::ffi::c_void, mut scrpos: *mut scrpos) {
    *scrpos = (*(ifile as *mut ifile)).h_scrpos;
}
#[no_mangle]
pub unsafe extern "C" fn set_open(mut ifile: *mut std::ffi::c_void) {
    (*(ifile as *mut ifile)).h_opened = 1 as std::ffi::c_int as std::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn opened(mut ifile: *mut std::ffi::c_void) -> std::ffi::c_int {
    return (*(ifile as *mut ifile)).h_opened as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn hold_ifile(mut ifile: *mut std::ffi::c_void, mut incr: std::ffi::c_int) {
    (*(ifile as *mut ifile)).h_hold += incr;
}
#[no_mangle]
pub unsafe extern "C" fn held_ifile(mut ifile: *mut std::ffi::c_void) -> std::ffi::c_int {
    return (*(ifile as *mut ifile)).h_hold;
}
#[no_mangle]
pub unsafe extern "C" fn get_filestate(mut ifile: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    return (*(ifile as *mut ifile)).h_filestate;
}
#[no_mangle]
pub unsafe extern "C" fn set_filestate(
    mut ifile: *mut std::ffi::c_void,
    mut filestate: *mut std::ffi::c_void,
) {
    let ref mut fresh0 = (*(ifile as *mut ifile)).h_filestate;
    *fresh0 = filestate;
}
#[no_mangle]
pub unsafe extern "C" fn set_altpipe(
    mut ifile: *mut std::ffi::c_void,
    mut p: *mut std::ffi::c_void,
) {
    let ref mut fresh1 = (*(ifile as *mut ifile)).h_altpipe;
    *fresh1 = p;
}
#[no_mangle]
pub unsafe extern "C" fn get_altpipe(mut ifile: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    return (*(ifile as *mut ifile)).h_altpipe;
}
#[no_mangle]
pub unsafe extern "C" fn set_altfilename(
    mut ifile: *mut std::ffi::c_void,
    mut altfilename: *mut std::ffi::c_char,
) {
    let mut p: *mut ifile = ifile as *mut ifile;
    if !((*p).h_altfilename).is_null() && (*p).h_altfilename != altfilename {
        free((*p).h_altfilename as *mut std::ffi::c_void);
    }
    (*p).h_altfilename = altfilename;
}
#[no_mangle]
pub unsafe extern "C" fn get_altfilename(
    mut ifile: *mut std::ffi::c_void,
) -> *mut std::ffi::c_char {
    return (*(ifile as *mut ifile)).h_altfilename;
}
