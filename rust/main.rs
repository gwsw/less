use crate::charset::init_charset;
use crate::decode::{expand_cmd_tables, init_cmds, Tables};
use crate::decode::{isnullenv, lgetenv};
use crate::defs::*;
use crate::ifile::{IFile, IFileHandle, IFileManager};
use crate::line::init_line;
use crate::mark::Marks;
use std::env;
use std::ffi::CString;
extern "C" {
    fn snprintf(
        _: *mut std::ffi::c_char,
        _: std::ffi::c_ulong,
        _: *const std::ffi::c_char,
        _: ...
    ) -> std::ffi::c_int;
    fn isatty(__fd: std::ffi::c_int) -> std::ffi::c_int;
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn exit(_: std::ffi::c_int) -> !;
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
    fn raw_mode(on: std::ffi::c_int);
    fn get_term();
    fn init();
    fn deinit();
    fn interactive() -> std::ffi::c_int;
    fn clear_bot();
    fn init_cmdhist();
    fn save_cmdhist();
    fn commands();
    fn check_altpipe_error();
    fn edit(filename: *const std::ffi::c_char) -> std::ffi::c_int;
    fn edit_first() -> std::ffi::c_int;
    fn edit_next(n: std::ffi::c_int) -> std::ffi::c_int;
    fn cat_file();
    fn last_component(name: *const std::ffi::c_char) -> *const std::ffi::c_char;
    fn get_one_screen() -> lbool;
    fn nifile() -> std::ffi::c_int;
    fn repaint();
    fn opt_header(type_0: std::ffi::c_int, s: *const std::ffi::c_char);
    fn scan_option(s: *const std::ffi::c_char, is_env: lbool);
    fn isoptpending() -> lbool;
    fn nopendopt();
    fn init_unsupport();
    fn init_option();
    fn init_poll();
    fn get_time() -> time_t;
    fn flush();
    fn set_output(fd: std::ffi::c_int);
    fn putchr(ch: std::ffi::c_int) -> std::ffi::c_int;
    fn less_printf(fmt: *const std::ffi::c_char, parg: *mut PARG) -> std::ffi::c_int;
    fn get_return();
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn init_prompt();
    fn init_search();
    fn init_signals(on: std::ffi::c_int);
    fn findtag(tag: *const std::ffi::c_char);
    fn tagsearch() -> POSITION;
    fn edit_tagfile() -> std::ffi::c_int;
    fn open_getchr();
    fn close_getchr();
    static mut tags: *mut std::ffi::c_char;
    static mut tagoption: *mut std::ffi::c_char;
    static mut jump_sline: std::ffi::c_int;
    static mut less_is_more: std::ffi::c_int;
    static mut missing_cap: lbool;
    static mut know_dumb: std::ffi::c_int;
    static mut quit_if_one_screen: std::ffi::c_int;
    static mut no_init: std::ffi::c_int;
    static mut errmsgs: std::ffi::c_int;
    static mut redraw_on_quit: std::ffi::c_int;
    static mut term_init_done: std::ffi::c_int;
    static mut first_time: lbool;
}
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
pub struct secure_feature {
    pub name: *const std::ffi::c_char,
    pub sf_value: std::ffi::c_int,
}
#[no_mangle]
pub static mut every_first_cmd: *mut std::ffi::c_char =
    0 as *const std::ffi::c_char as *mut std::ffi::c_char;
#[no_mangle]
pub static mut new_file: lbool = LFALSE;
#[no_mangle]
pub static mut is_tty: std::ffi::c_int = 0;
#[no_mangle]
pub static mut initial_scrpos: scrpos = scrpos { pos: 0, ln: 0 };
#[no_mangle]
pub static mut start_attnpos: POSITION = -(1 as std::ffi::c_int) as POSITION;
#[no_mangle]
pub static mut end_attnpos: POSITION = -(1 as std::ffi::c_int) as POSITION;
#[no_mangle]
pub static mut wscroll: std::ffi::c_int = 0;
pub static mut progname: String = String::new();
#[no_mangle]
pub static mut quitting: lbool = LFALSE;
#[no_mangle]
pub static mut dohelp: std::ffi::c_int = 0;
#[no_mangle]
pub static mut init_header: *mut std::ffi::c_char =
    0 as *const std::ffi::c_char as *mut std::ffi::c_char;
static mut secure_allow_features: i32 = 0;
#[no_mangle]
pub static mut logfile: std::ffi::c_int = -(1 as std::ffi::c_int);
#[no_mangle]
pub static mut force_logfile: lbool = LFALSE;
#[no_mangle]
pub static mut namelogfile: *mut std::ffi::c_char =
    0 as *const std::ffi::c_char as *mut std::ffi::c_char;
#[no_mangle]
pub static mut editor: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
#[no_mangle]
pub static mut editproto: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
#[no_mangle]
pub static mut less_start_time: time_t = 0;
#[no_mangle]
pub static mut one_screen: std::ffi::c_int = 0;

#[no_mangle]
pub static mut curr_ifile: Option<IFileHandle> = None;
#[no_mangle]
pub static mut old_ifile: Option<IFileHandle> = None;

const FAKE_HELPFILE: &'static str = "@/\\less/\\help/\\file/\\@";
const FAKE_EMPTYFILE: &'static str = "@/\\less/\\empty/\\file/\\@";

pub struct Less {
    pub marks: Marks,
    pub ifiles: IFileManager,
    //pub curr_ifile: Option<IFile>,
    //pub old_ifile: Option<IFile>,
}

impl Less {
    pub fn new(marks: Marks) -> Self {
        Less {
            marks: marks,
            ifiles: IFileManager::new(),
            //curr_ifile: None,
            //old_ifile: None,
        }
    }

    pub fn marks_ref(&self) -> &Marks {
        &self.marks
    }

    pub fn marks_ref_mut(&mut self) -> &mut Marks {
        &mut self.marks
    }
}

unsafe extern "C" fn security_feature_error(
    mut type_0: *const std::ffi::c_char,
    mut len: size_t,
    mut name: *const std::ffi::c_char,
) -> std::ffi::c_int {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    let mut msglen: size_t = len
        .wrapping_add(strlen(type_0))
        .wrapping_add(64 as std::ffi::c_int as std::ffi::c_ulong);
    let mut msg: *mut std::ffi::c_char = ecalloc(
        msglen,
        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_char;
    snprintf(
        msg,
        msglen,
        b"LESSSECURE_ALLOW: %s feature name \"%.*s\"\0" as *const u8 as *const std::ffi::c_char,
        type_0,
        len as std::ffi::c_int,
        name,
    );
    parg.p_string = msg;
    error(b"%s\0" as *const u8 as *const std::ffi::c_char, &mut parg);
    free(msg as *mut std::ffi::c_void);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn security_feature(
    mut name: *const std::ffi::c_char,
    mut len: size_t,
) -> std::ffi::c_int {
    static mut features: [secure_feature; 12] = [
        {
            let mut init = secure_feature {
                name: b"edit\0" as *const u8 as *const std::ffi::c_char,
                sf_value: (1 as std::ffi::c_int) << 1 as std::ffi::c_int,
            };
            init
        },
        {
            let mut init = secure_feature {
                name: b"examine\0" as *const u8 as *const std::ffi::c_char,
                sf_value: (1 as std::ffi::c_int) << 2 as std::ffi::c_int,
            };
            init
        },
        {
            let mut init = secure_feature {
                name: b"glob\0" as *const u8 as *const std::ffi::c_char,
                sf_value: (1 as std::ffi::c_int) << 3 as std::ffi::c_int,
            };
            init
        },
        {
            let mut init = secure_feature {
                name: b"history\0" as *const u8 as *const std::ffi::c_char,
                sf_value: (1 as std::ffi::c_int) << 4 as std::ffi::c_int,
            };
            init
        },
        {
            let mut init = secure_feature {
                name: b"lesskey\0" as *const u8 as *const std::ffi::c_char,
                sf_value: (1 as std::ffi::c_int) << 5 as std::ffi::c_int,
            };
            init
        },
        {
            let mut init = secure_feature {
                name: b"lessopen\0" as *const u8 as *const std::ffi::c_char,
                sf_value: (1 as std::ffi::c_int) << 6 as std::ffi::c_int,
            };
            init
        },
        {
            let mut init = secure_feature {
                name: b"logfile\0" as *const u8 as *const std::ffi::c_char,
                sf_value: (1 as std::ffi::c_int) << 7 as std::ffi::c_int,
            };
            init
        },
        {
            let mut init = secure_feature {
                name: b"osc8\0" as *const u8 as *const std::ffi::c_char,
                sf_value: (1 as std::ffi::c_int) << 12 as std::ffi::c_int,
            };
            init
        },
        {
            let mut init = secure_feature {
                name: b"pipe\0" as *const u8 as *const std::ffi::c_char,
                sf_value: (1 as std::ffi::c_int) << 8 as std::ffi::c_int,
            };
            init
        },
        {
            let mut init = secure_feature {
                name: b"shell\0" as *const u8 as *const std::ffi::c_char,
                sf_value: (1 as std::ffi::c_int) << 9 as std::ffi::c_int,
            };
            init
        },
        {
            let mut init = secure_feature {
                name: b"stop\0" as *const u8 as *const std::ffi::c_char,
                sf_value: (1 as std::ffi::c_int) << 10 as std::ffi::c_int,
            };
            init
        },
        {
            let mut init = secure_feature {
                name: b"tags\0" as *const u8 as *const std::ffi::c_char,
                sf_value: (1 as std::ffi::c_int) << 11 as std::ffi::c_int,
            };
            init
        },
    ];
    let mut i: std::ffi::c_int = 0;
    let mut match_0: std::ffi::c_int = -(1 as std::ffi::c_int);
    i = 0 as std::ffi::c_int;
    while i
        < (::core::mem::size_of::<[secure_feature; 12]>() as std::ffi::c_ulong)
            .wrapping_div(::core::mem::size_of::<secure_feature>() as std::ffi::c_ulong)
            as std::ffi::c_int
    {
        if strncmp(features[i as usize].name, name, len) == 0 as std::ffi::c_int {
            if match_0 >= 0 as std::ffi::c_int {
                return security_feature_error(
                    b"ambiguous\0" as *const u8 as *const std::ffi::c_char,
                    len,
                    name,
                );
            }
            match_0 = i;
        }
        i += 1;
    }
    if match_0 < 0 as std::ffi::c_int {
        return security_feature_error(
            b"invalid\0" as *const u8 as *const std::ffi::c_char,
            len,
            name,
        );
    }
    return features[match_0 as usize].sf_value;
}

/*
 * Set the secure_allow_features bitmask, which controls
 * whether certain secure features are allowed.
 */
unsafe extern "C" fn init_secure() {
    let mut s = lgetenv("LESSSECURE");
    if isnullenv(&s) {
        secure_allow_features = !0; /* allow everything */
    } else {
        secure_allow_features = 0; /* allow nothing */
    }

    let ls = lgetenv("LESSSECURE_ALLOW");
    if !isnullenv(&ls) {
        let mut s = ls.unwrap();
        let mut s = s.as_mut_str();
        loop {
            let mut estr: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
            let mut c = s.chars().next().unwrap();
            while c == ' ' || c == ',' {
                c = s.chars().next().unwrap();
            }
            if c == '\0' {
                break;
            }
            let mut start = 0;
            if let Some(i) = s.find(',') {
                start = i;
            } else {
                start = s.len();
            }
            while start > 0 && s.chars().nth(start - 1).unwrap() == ' ' {
                /* trim trailing spaces */
                start -= 1;
            }
            secure_allow_features |= security_feature(
                CString::new(String::from(&mut *s)).unwrap().as_ptr(),
                start as u64,
            );
            let (_, r) = s.split_at_mut(start);
            s = r;
        }
    }
}

fn is_opt_string(s: &str) -> bool {
    (s.starts_with("-") || s.starts_with("+")) && s.len() > 1
}

unsafe fn main_0() -> i32 {
    let args: Vec<String> = env::args().collect();
    let mut ifile: Option<IFileHandle> = None;
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    progname = args[0].clone();
    init_secure();
    /*
     * Process command line arguments and LESS environment arguments.
     * Command line arguments override environment arguments.
     */
    is_tty = isatty(1 as std::ffi::c_int);
    let mut less = Less::new(Marks::new());
    let marks = less.marks_ref_mut();
    let mut ifiles = less.ifiles;
    less.marks.init();
    let mut tables = Tables::new();
    init_cmds(&mut tables);
    init_poll();
    init_charset();
    init_line();
    init_cmdhist();
    init_option();
    init_search();
    if strcmp(
        last_component(CString::new(progname.clone()).unwrap().as_ptr()),
        b"more\0" as *const u8 as *const std::ffi::c_char,
    ) == 0 as std::ffi::c_int
        && lgetenv("LESS_IS_MORE").is_err()
    {
        less_is_more = 1 as std::ffi::c_int;
    }
    init_prompt();
    init_unsupport();
    let ss = lgetenv(if less_is_more != 0 { "MORE" } else { "LESS" });
    if ss.is_err() {
        scan_option(s, LTRUE);
    } else {
        s = CString::new(ss.unwrap()).unwrap().as_ptr();
    }
    for arg in args.iter() {
        if !(is_opt_string(arg.as_str()) || isoptpending() != 0) {
            break;
        }
        if arg.starts_with("--") {
            break;
        }
        scan_option(CString::new(arg.clone()).unwrap().as_ptr(), LFALSE);
    }
    if isoptpending() as u64 != 0 {
        nopendopt();
        quit(0 as std::ffi::c_int);
    }
    get_term();
    expand_cmd_tables(&mut tables);
    let ed = lgetenv("VISUAL");
    if ed.is_err() {
        let edit = lgetenv("EDITOR");
        if edit.is_ok() {
            editor = CString::new(edit.unwrap()).unwrap().as_ptr();
        } else {
            editor = b"vi\0" as *const u8 as *const std::ffi::c_char;
        }
    } else {
        editor = CString::new(ed.unwrap()).unwrap().as_ptr();
    }
    let editp = lgetenv("LESSEDIT");
    if editp.is_err() {
        editproto = b"%E ?lm+%lm. %g\0" as *const u8 as *const std::ffi::c_char;
    } else {
        editproto = CString::new(editp.unwrap()).unwrap().as_ptr();
    }

    /*
     * Call get_ifile with all the command line filenames
     * to "register" them with the ifile system.
     */
    if dohelp != 0 {
        ifile = Some(ifiles.get_ifile(FAKE_HELPFILE, ifile));
    }
    for arg in args.iter() {
        let _ = ifiles.get_ifile(arg, ifile);
        ifile = ifiles.prev_ifile(None);
    }
    if is_tty == 0 {
        set_output(1 as std::ffi::c_int);
        if edit_first() == 0 as std::ffi::c_int {
            loop {
                cat_file();
                if !(edit_next(1 as std::ffi::c_int) == 0 as std::ffi::c_int) {
                    break;
                }
            }
        }
        quit(0 as std::ffi::c_int);
    }
    if missing_cap as std::ffi::c_uint != 0 && know_dumb == 0 {
        error(
            b"WARNING: terminal is not fully functional\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
    }
    open_getchr();
    raw_mode(1 as std::ffi::c_int);
    init_signals(1 as std::ffi::c_int);
    less_start_time = get_time();
    if !tagoption.is_null()
        || strcmp(tags, b"-\0" as *const u8 as *const std::ffi::c_char) == 0 as std::ffi::c_int
    {
        if nifile() > 0 as std::ffi::c_int {
            error(
                b"No filenames allowed with -t option\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
            quit(1 as std::ffi::c_int);
        }
        findtag(tagoption);
        if edit_tagfile() != 0 {
            quit(1 as std::ffi::c_int);
        }
        initial_scrpos.pos = tagsearch();
        if initial_scrpos.pos == -(1 as std::ffi::c_int) as POSITION {
            quit(1 as std::ffi::c_int);
        }
        initial_scrpos.ln = jump_sline;
    } else {
        if edit_first() != 0 {
            quit(1 as std::ffi::c_int);
        }
        if quit_if_one_screen != 0 {
            if nifile() > 1 as std::ffi::c_int {
                quit_if_one_screen = LFALSE as std::ffi::c_int;
            } else if no_init == 0 {
                one_screen = get_one_screen() as std::ffi::c_int;
            }
        }
    }
    if !init_header.is_null() {
        opt_header(2 as std::ffi::c_int, init_header);
        free(init_header as *mut std::ffi::c_void);
        init_header = 0 as *mut std::ffi::c_char;
    }
    if errmsgs > 0 as std::ffi::c_int {
        less_printf(
            b"Press RETURN to continue \0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        get_return();
        putchr('\n' as i32);
    }
    set_output(1 as std::ffi::c_int);
    init();
    commands();
    quit(0 as std::ffi::c_int);
    return 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn saven(
    mut s: *const std::ffi::c_char,
    mut n: size_t,
) -> *mut std::ffi::c_char {
    let mut p: *mut std::ffi::c_char = ecalloc(
        n.wrapping_add(1 as std::ffi::c_int as size_t),
        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_char;
    strncpy(p, s, n);
    *p.offset(n as isize) = '\0' as i32 as std::ffi::c_char;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn save(mut s: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    return saven(s, strlen(s));
}
#[no_mangle]
pub unsafe extern "C" fn out_of_memory() {
    error(
        b"Cannot allocate memory\0" as *const u8 as *const std::ffi::c_char,
        0 as *mut std::ffi::c_void as *mut PARG,
    );
    quit(1 as std::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ecalloc(mut count: size_t, mut size: size_t) -> *mut std::ffi::c_void {
    let mut p: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    p = calloc(count, size);
    if p.is_null() {
        out_of_memory();
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn skipsp(mut s: *mut std::ffi::c_char) -> *mut std::ffi::c_char {
    while *s as std::ffi::c_int == ' ' as i32 || *s as std::ffi::c_int == '\t' as i32 {
        s = s.offset(1);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn skipspc(mut s: *const std::ffi::c_char) -> *const std::ffi::c_char {
    while *s as std::ffi::c_int == ' ' as i32 || *s as std::ffi::c_int == '\t' as i32 {
        s = s.offset(1);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn sprefix(
    mut ps: *const std::ffi::c_char,
    mut s: *const std::ffi::c_char,
    mut uppercase: std::ffi::c_int,
) -> size_t {
    let mut c: std::ffi::c_char = 0;
    let mut sc: std::ffi::c_char = 0;
    let mut len: size_t = 0 as std::ffi::c_int as size_t;
    while *s as std::ffi::c_int != '\0' as i32 {
        c = *ps;
        if uppercase != 0 {
            if len == 0 as std::ffi::c_int as size_t
                && (c as std::ffi::c_int >= 'a' as i32 && c as std::ffi::c_int <= 'z' as i32)
            {
                return 0 as std::ffi::c_int as size_t;
            }
            if c as std::ffi::c_int >= 'A' as i32 && c as std::ffi::c_int <= 'Z' as i32 {
                c = (c as std::ffi::c_int - 'A' as i32 + 'a' as i32) as std::ffi::c_char;
            }
        }
        sc = *s;
        if len > 0 as std::ffi::c_int as size_t
            && (sc as std::ffi::c_int >= 'A' as i32 && sc as std::ffi::c_int <= 'Z' as i32)
        {
            sc = (sc as std::ffi::c_int - 'A' as i32 + 'a' as i32) as std::ffi::c_char;
        }
        if c as std::ffi::c_int != sc as std::ffi::c_int {
            break;
        }
        len = len.wrapping_add(1);
        s = s.offset(1);
        ps = ps.offset(1);
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn quit(mut status: std::ffi::c_int) {
    static mut save_status: std::ffi::c_int = 0;
    if status < 0 as std::ffi::c_int {
        status = save_status;
    } else {
        save_status = status;
    }
    quitting = LTRUE;
    check_altpipe_error();
    if interactive() != 0 {
        clear_bot();
    }
    deinit();
    flush();
    if redraw_on_quit != 0 && term_init_done != 0 {
        first_time = LTRUE;
        repaint();
        flush();
    }
    edit(0 as *mut std::ffi::c_void as *mut std::ffi::c_char);
    save_cmdhist();
    raw_mode(0 as std::ffi::c_int);
    close_getchr();
    exit(status);
}
#[no_mangle]
pub unsafe extern "C" fn secure_allow(mut features: std::ffi::c_int) -> std::ffi::c_int {
    return (secure_allow_features & features == features) as std::ffi::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn main() {
    unsafe { ::std::process::exit(main_0()) }
}
