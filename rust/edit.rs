use crate::defs::*;
use crate::ifile::{IFileHandle, IFileManager, ScrPos};
use crate::line::forw_raw_line;
use ::c2rust_bitfields;
use glob::glob;
use std::any::Any;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type mlist;
    fn pclose(__stream: *mut FILE) -> std::ffi::c_int;
    fn open(__file: *const std::ffi::c_char, __oflag: std::ffi::c_int, _: ...) -> std::ffi::c_int;
    fn creat(__file: *const std::ffi::c_char, __mode: mode_t) -> std::ffi::c_int;
    fn lseek(__fd: std::ffi::c_int, __offset: __off_t, __whence: std::ffi::c_int) -> __off_t;
    fn close(__fd: std::ffi::c_int) -> std::ffi::c_int;
    fn isatty(__fd: std::ffi::c_int) -> std::ffi::c_int;
    fn free(_: *mut std::ffi::c_void);
    fn quit(status: std::ffi::c_int);
    fn end_logfile();
    fn ch_length() -> POSITION;
    fn ch_forw_get() -> std::ffi::c_int;
    fn ch_init(f: std::ffi::c_int, flags: std::ffi::c_int, nread: ssize_t);
    fn ch_close();
    fn ch_getflags() -> std::ffi::c_int;
    fn cmd_addhist(mlist: *mut mlist, cmd: *const std::ffi::c_char, modified: lbool);
    fn ungetcc_end_command();
    fn ungetsc(s: *const std::ffi::c_char);
    fn shell_unquote(str: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn get_meta_escape() -> *const std::ffi::c_char;
    fn shell_quote(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn bin_file(f: std::ffi::c_int, n: *mut ssize_t) -> std::ffi::c_int;
    fn lglob(afilename: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn open_altfile(
        filename: *const std::ffi::c_char,
        pf: *mut std::ffi::c_int,
        pfd: *mut *mut std::ffi::c_void,
    ) -> *mut std::ffi::c_char;
    fn close_altfile(altfilename: *const std::ffi::c_char, filename: *const std::ffi::c_char);
    fn bad_file(filename: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn store_pos(ifile: *mut std::ffi::c_void, scrpos: *mut scrpos);
    fn set_open(ifile: *mut std::ffi::c_void);
    fn hold_ifile(ifile: *mut std::ffi::c_void, incr: std::ffi::c_int);
    fn clr_linenum();
    fn scan_eof();
    fn lastmark();
    fn set_tabs(s: *const std::ffi::c_char, len: size_t);
    fn iopen(filename: *const std::ffi::c_char, flags: std::ffi::c_int) -> std::ffi::c_int;
    fn errno_message(filename: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn signal_message(sig: std::ffi::c_int) -> *const std::ffi::c_char;
    fn flush();
    fn putchr(ch: std::ffi::c_int) -> std::ffi::c_int;
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn query(fmt: *const std::ffi::c_char, parg: *mut PARG) -> std::ffi::c_int;
    fn pos_clear();
    fn get_scrpos(scrpos: *mut scrpos, where_0: std::ffi::c_int);
    fn undo_osc8();
    fn clr_hilite();
    fn set_header(pos: POSITION);
    fn stat(__file: *const std::ffi::c_char, __buf: *mut stat) -> std::ffi::c_int;
    static mut new_file: bool;
    static mut every_first_cmd: *mut std::ffi::c_char;
    static mut force_open: std::ffi::c_int;
    static mut is_tty: std::ffi::c_int;
    static mut sigs: std::ffi::c_int;
    static mut hshift: std::ffi::c_int;
    static mut want_filesize: std::ffi::c_int;
    static mut consecutive_nulls: std::ffi::c_int;
    static mut modelines: std::ffi::c_int;
    static mut show_preproc_error: std::ffi::c_int;
    static mut curr_ifile: Option<IFileHandle>;
    static mut old_ifile: Option<IFileHandle>;
    static mut initial_scrpos: scrpos;
    static mut ml_examine: *mut std::ffi::c_void;
    static mut soft_eof: POSITION;
    static mut openquote: std::ffi::c_char;
    static mut closequote: std::ffi::c_char;
    static mut logfile: std::ffi::c_int;
    static mut force_logfile: std::ffi::c_int;
    static mut namelogfile: Option<String>;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: std::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scrpos {
    pub pos: POSITION,
    pub ln: std::ffi::c_int,
}
#[derive(Clone)]
pub enum Parg {
    String(String),
    Int(i32),
    LineNum(LINENUM),
    Char(char),
    Null,
}
pub type PARG = Parg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct textlist {
    pub string: *mut std::ffi::c_char,
    pub endstring: *mut std::ffi::c_char,
}
#[derive(Clone)]
pub struct MlOption {
    pub opt_name: String,
    pub opt_func: fn(&str, usize) -> (),
}

pub static mut fd0: std::ffi::c_int = 0 as std::ffi::c_int;
pub static mut curr_dev: dev_t = 0;
pub static mut curr_ino: ino_t = 0;

#[derive(Debug, Clone)]
pub struct TextList {
    strings: Vec<String>,
}

impl TextList {
    fn new(strings: Vec<String>) -> Self {
        TextList { strings }
    }
}

//FIXME stub function until set_tabs from optfunc is properly implemented
fn set_tabs_dummy(s: &str, len: usize) {}

/*
 * Textlist functions deal with a list of words separated by spaces.
 * init_textlist sets up a textlist structure.
 * forw_textlist uses that structure to iterate thru the list of
 * words, returning each one as a standard null-terminated string.
 * back_textlist does the same, but runs thru the list backwards.
 */
fn init_textlist(s: &str) -> TextList {
    let mut delim_quoted = false;
    let mut s = s.trim_start();
    let open_quote = '"';
    let close_quote = '"';
    let mut result = Vec::new();

    let mut current = String::new();
    for c in s.chars() {
        if delim_quoted {
            if c == close_quote {
                delim_quoted = false;
            } else {
                current.push(c);
            }
        } else {
            if c == open_quote {
                delim_quoted = true;
            } else if c == ' ' {
                result.push(current.clone());
                current.clear();
            } else {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        result.push(current);
    }
    TextList::new(result)
}

pub unsafe extern "C" fn forw_textlist<'a>(
    tlist: &'a TextList,
    prev: Option<&String>,
) -> Option<&'a String> {
    /*
     * prev is None means return the first word in the list.
     * Otherwise, return the word after "prev".
     */
    if prev.is_none() {
        return Some(&tlist.strings[0]);
    }
    let len = tlist.strings.len();
    for (i, s) in tlist.strings.iter().enumerate() {
        if Some(s) == prev {
            if i + 1 > len {
                return None;
            } else {
                return Some(&tlist.strings[i + 1]);
            }
        }
    }
    None
}

pub unsafe extern "C" fn back_textlist<'a>(
    tlist: &'a TextList,
    prev: Option<&String>,
) -> Option<&'a String> {
    /*
     * prev is None means return the last word in the list.
     * Otherwise, return the word before "prev".
     */
    if prev.is_none() {
        return tlist.strings.last();
    }
    let len = tlist.strings.len();
    for (i, s) in tlist.strings.iter().enumerate() {
        if Some(s) == prev {
            if i > 0 {
                return Some(&tlist.strings[i - 1]);
            } else {
                return None;
            }
        }
    }
    None
}

/*
 * Parse a single option setting in a modeline.
 */
unsafe extern "C" fn modeline_option(s: &str, opt_len: usize) {
    let mut options: [MlOption; 2] = [
        MlOption {
            opt_name: "ts=".into(),
            opt_func: set_tabs_dummy,
        },
        MlOption {
            opt_name: "tabstop=".into(),
            opt_func: set_tabs_dummy,
        },
    ];
    for opt in options {
        let name_len = opt.opt_name.len();
        if opt_len > name_len && s.starts_with(&opt.opt_name) {
            (opt.opt_func)(&s[name_len..], opt_len - name_len);
        }
    }
}

/*
 * String length, terminated by option separator (space or colon).
 * Space/colon can be escaped with backspace.
 */
unsafe extern "C" fn modeline_option_len(s: &str) -> usize {
    let mut prev = ' ';
    for (i, c) in s.char_indices() {
        if prev != '\\' && (c == ' ' || c == ':') {
            return i;
        }
        prev = c;
    }
    0
}

/*
 * Parse colon- or space-separated option settings in a modeline.
 */
unsafe extern "C" fn modeline_options(s: &str, end_char: char) {
    let mut s = s;
    loop {
        let mut opt_len = 0;
        s = s.trim_start();
        if s.len() == 0 || s.chars().next() == Some(end_char) {
            break;
        }
        opt_len = modeline_option_len(s);
        modeline_option(s, opt_len);
        s = &s[opt_len..];
        if s.len() > 0 {
            s = &s[1..]; // skip past the separator
        }
    }
}

/*
 * See if there is a modeline string in a line.
 */
unsafe extern "C" fn check_modeline(line: &str) {
    let pgms: [&str; 4] = ["less:", "vim:", "vi:", "ex:"];
    let mut line = line;
    for (i, pgm) in pgms.iter().enumerate() {
        if let Some(idx) = line.find(pgm) {
            let s = line[idx + pgm.len()..].trim_start();
            if &line[idx..] == line || line.chars().nth(idx - 1) == Some(' ') {
                if s == "set" {
                    modeline_options(&s[4..], ':');
                } else if i != 0 {
                    // "less:" requires "set"
                    modeline_options(s, '\0');
                }
                break;
            }
            /* Continue searching the rest of the line. */
            line = s;
        } else {
            break;
        }
    }
}

/*
 * See if there is a modeline string in a line.
 */
unsafe extern "C" fn check_modelines() {
    let mut pos = 0;
    for i in 0..modelines {
        if abort_sigs() {
            return;
        }
        let (pos, line, _) = forw_raw_line(pos);
        if pos == NULL_POSITION {
            break;
        }
        check_modeline(std::str::from_utf8_unchecked(line));
    }
}

/*
 * Close a pipe opened via popen.
 */
// FIXME - add a proper implementation
unsafe extern "C" fn close_pipe(mut pipefd: *mut FILE) {
    let mut status: std::ffi::c_int = 0;
    let mut p: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut parg = Parg::Null;
    if pipefd.is_null() {
        return;
    }
    status = pclose(pipefd);
    if status == -(1 as std::ffi::c_int) {
        p = errno_message(b"pclose\0" as *const u8 as *const std::ffi::c_char);
        parg = Parg::String(CStr::from_ptr(p).to_string_lossy().into_owned());
        error(b"%s\0" as *const u8 as *const std::ffi::c_char, &mut parg);
        free(p as *mut std::ffi::c_void);
        return;
    }
    if show_preproc_error == 0 {
        return;
    }
    if status & 0x7f as std::ffi::c_int == 0 as std::ffi::c_int {
        let mut s: std::ffi::c_int = (status & 0xff00 as std::ffi::c_int) >> 8 as std::ffi::c_int;
        if s != 0 as std::ffi::c_int {
            parg = Parg::Int(s);
            error(
                b"Input preprocessor failed (status %d)\0" as *const u8 as *const std::ffi::c_char,
                &mut parg,
            );
        }
        return;
    }
    if ((status & 0x7f as std::ffi::c_int) + 1 as std::ffi::c_int) as std::ffi::c_schar
        as std::ffi::c_int
        >> 1 as std::ffi::c_int
        > 0 as std::ffi::c_int
    {
        let mut sig: std::ffi::c_int = status & 0x7f as std::ffi::c_int;
        if sig != 13 || ch_length() != -(1 as std::ffi::c_int) as POSITION {
            parg = Parg::String(
                CStr::from_ptr(signal_message(sig))
                    .to_string_lossy()
                    .into_owned(),
            );
            error(
                b"Input preprocessor terminated: %s\0" as *const u8 as *const std::ffi::c_char,
                &mut parg,
            );
        }
        return;
    }
    if status != 0 {
        parg = Parg::Int(status);
        error(
            b"Input preprocessor exited with status %x\0" as *const u8 as *const std::ffi::c_char,
            &mut parg,
        );
    }
}

/*
 * Drain and close an input pipe if needed.
 */
pub unsafe extern "C" fn close_altpipe(ifiles: &mut IFileManager, ifile: Option<IFileHandle>) {
    let altpipe = ifiles.get_altpipe(ifile);
    if !altpipe.is_none() && ch_getflags() & CH_KEEPOPEN != 0 {
        // FIXME  replace with proper implementation
        //close_pipe(altpipe);
        ifiles.set_altpipe(ifile, None);
    }
}

/*
 * Check for error status from the current altpipe.
 * May or may not close the pipe.
 */
pub unsafe extern "C" fn check_altpipe_error(ifiles: &mut IFileManager) {
    if show_preproc_error == 0 {
        return;
    }
    if !curr_ifile.is_none() && !(ifiles.get_altfilename(curr_ifile)).is_none() {
        close_altpipe(ifiles, curr_ifile);
    }
}

/*
 * Close the current input file.
 */
unsafe extern "C" fn close_file(ifiles: &mut IFileManager) {
    let mut scr_pos = ScrPos { pos: 0, ln: 0 };
    let mut altfilename: Option<&Path>;
    if curr_ifile.is_none() {
        return;
    }
    /*
     * Save the current position so that we can return to
     * the same position if we edit this file again.
     */
    let mut scrp = scrpos { pos: 0, ln: 0 };
    get_scrpos(&mut scrp, 0);
    if scrp.pos != NULL_POSITION {
        scr_pos.pos = scrp.pos as u64;
        scr_pos.ln = scrp.ln;
        ifiles.store_pos(curr_ifile, scr_pos);
        lastmark();
    }
    /*
     * Close the file descriptor, unless it is a pipe.
     */
    ch_close();
    /*
     * If we opened a file using an alternate name,
     * do special stuff to close it.
     */
    if let Some(altfilename) = ifiles.get_altfilename(curr_ifile).map(|p| p.to_path_buf()) {
        let filename = ifiles.get_filename(curr_ifile).unwrap().to_path_buf();
        close_altpipe(ifiles, curr_ifile);
        close_altfile(
            altfilename.as_os_str().as_bytes().as_ptr() as *const i8,
            filename.as_os_str().as_bytes().as_ptr() as *const i8,
        );
        ifiles.set_altfilename(curr_ifile, None::<&Path>);
    }
    curr_ifile = None;
    curr_dev = 0;
    curr_ino = curr_dev;
}

/*
 * Edit a new file (given its name).
 * Filename == "-" means standard input.
 * Filename == NULL means just close the current file.
 */
pub unsafe extern "C" fn edit(
    ifiles: &mut IFileManager,
    filename: Option<impl AsRef<Path>>,
) -> i32 {
    if filename.is_none() {
        return edit_ifile(ifiles, None);
    }
    let h = ifiles.get_ifile(filename.unwrap(), curr_ifile);
    return edit_ifile(ifiles, Some(h));
}

/*
 * Clean up what edit_ifile did before error return.
 */
unsafe extern "C" fn edit_error(
    ifiles: &mut IFileManager,
    filename: Option<impl AsRef<Path>>,
    alt_filename: Option<impl AsRef<Path>>,
    altpipe: Option<Box<dyn Any + Send + Sync>>,
    ifile: Option<IFileHandle>,
) -> i32 {
    if !alt_filename.is_none() {
        // FIXME  replace with proper implementation
        //close_pipe(altpipe as *mut FILE);
        close_altfile(
            alt_filename
                .unwrap()
                .as_ref()
                .as_os_str()
                .as_bytes()
                .as_ptr() as *const i8,
            filename.unwrap().as_ref().as_os_str().as_bytes().as_ptr() as *const i8,
        );
        //free(alt_filename as *mut std::ffi::c_char as *mut std::ffi::c_void);
    }
    ifiles.del_ifile(ifile);
    /*
     * Re-open the current file.
     */
    if curr_ifile == ifile {
        /*
         * Whoops.  The "current" ifile is the one we just deleted.
         * Just give up.
         */
        quit(1);
    }
    return 1;
}

/*
 * Edit a new file (given its IFILE).
 * ifile == NULL means just close the current file.
 */
pub unsafe extern "C" fn edit_ifile(ifiles: &mut IFileManager, ifile: Option<IFileHandle>) -> i32 {
    let mut f = 0;
    let mut answer = 0;
    let mut chflags = 0;
    let mut filename: Option<PathBuf> = None;
    let mut open_filename: Option<PathBuf> = None;
    let mut alt_filename: Option<PathBuf> = None;
    let mut altpipe: Option<&Box<dyn Any + Send + Sync>> = None;
    let mut was_curr_ifile: Option<IFileHandle> = None;
    let mut p: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut parg: PARG = Parg::Null;
    let mut nread = 0;
    if ifile == curr_ifile {
        /*
         * Already have the correct file open.
         */
        return 0;
    }
    new_file = true;
    if ifile.is_some() {
        /*
         * See if LESSOPEN specifies an "alternate" file to open.
         */
        filename = ifiles.get_filename(ifile).map(|p| p.to_path_buf());
        if let Some(_) = ifiles.get_altpipe(ifile) {
            /*
             * File is already open.
             * chflags and f are not used by ch_init if ifile has
             * filestate which should be the case if we're here.
             * Set them here to avoid uninitialized variable warnings.
             */
            chflags = 0;
            f = -1;
            alt_filename = ifiles.get_altfilename(ifile).map(|p| p.to_path_buf());
            open_filename = if !alt_filename.is_none() {
                alt_filename.clone()
            } else {
                filename.clone()
            };
        } else {
            if filename == Some(PathBuf::from(FAKE_HELPFILE))
                || filename == Some(PathBuf::from(FAKE_EMPTYFILE))
            {
                alt_filename = None;
            } else {
                // FIXME
                /*
                alt_filename = open_altfile(
                        CString.new(filename.unwrap()).unwrap().as_ptr(),
                        &mut f,
                        &mut Some(*altpipe.unwrap()),
                    ))
                    .to_string_lossy()
                    .into_owned(),
                );
                */
            }
            open_filename = if !alt_filename.is_none() {
                alt_filename.clone()
            } else {
                filename.clone()
            };

            chflags = 0;
            if !altpipe.is_none() {
                /*
                 * The alternate "file" is actually a pipe.
                 * f has already been set to the file descriptor of the pipe
                 * in the call to open_altfile above.
                 * Keep the file descriptor open because it was opened
                 * via popen(), and pclose() wants to close it.
                 */
                chflags |= CH_POPENED;
                if filename == Some(PathBuf::from("-")) {
                    chflags |= CH_KEEPOPEN;
                }
            } else if filename == Some(PathBuf::from("-")) {
                /*
                 * Use standard input.
                 * Keep the file descriptor open because we can't reopen it.
                 */
                f = fd0;
                chflags |= CH_KEEPOPEN;
                /*
                 * Must switch stdin to BINARY mode.
                 */
                // FIXME switch to binary mode
                //SET_BINARY(f);
            } else if open_filename == Some(PathBuf::from(FAKE_EMPTYFILE)) {
                f = -1;
                chflags |= CH_NODATA;
            } else if open_filename == Some(PathBuf::from(FAKE_HELPFILE)) {
                f = -1;
                chflags |= CH_HELPFILE;
            } else {
                p = bad_file(
                    CString::new(
                        open_filename
                            .clone()
                            .unwrap()
                            .to_string_lossy()
                            .into_owned(),
                    )
                    .unwrap()
                    .as_ptr(),
                );
                if !p.is_null() {
                    /*
                     * It looks like a bad file.  Don't try to open it.
                     */
                    parg = Parg::String(CStr::from_ptr(p).to_string_lossy().into_owned());
                    error(b"%s\0" as *const u8 as *const std::ffi::c_char, &mut parg);
                    free(p as *mut std::ffi::c_void);
                    return edit_error(
                        ifiles,
                        filename,
                        alt_filename,
                        Some(Box::new(altpipe.clone().unwrap())),
                        ifile,
                    );
                } else {
                    f = iopen(
                        CString::new(
                            open_filename
                                .clone()
                                .unwrap()
                                .to_string_lossy()
                                .into_owned(),
                        )
                        .unwrap()
                        .as_ptr(),
                        0,
                    );
                    if f < 0 {
                        /*
                         * Got an error trying to open it.
                         */
                        let mut p_0: *mut std::ffi::c_char = errno_message(
                            CString::new(filename.clone().unwrap().to_string_lossy().into_owned())
                                .unwrap()
                                .as_ptr(),
                        );
                        parg = Parg::String(CStr::from_ptr(p_0).to_string_lossy().into_owned());
                        error(b"%s\0" as *const u8 as *const std::ffi::c_char, &mut parg);
                        free(p_0 as *mut std::ffi::c_void);
                        return edit_error(
                            ifiles,
                            filename.clone(),
                            alt_filename,
                            Some(Box::new(altpipe.clone().unwrap())),
                            ifile,
                        );
                    } else {
                        chflags |= CH_CANSEEK;
                        if bin_file(f, &mut nread) != 0 && force_open == 0 && !ifiles.opened(ifile)
                        {
                            parg = Parg::String(
                                filename.clone().unwrap().to_string_lossy().into_owned(),
                            );
                            answer = query(
                                b"\"%s\" may be a binary file.  See it anyway? \0" as *const u8
                                    as *const std::ffi::c_char,
                                &mut parg,
                            );
                            if answer != 'y' as i32 && answer != 'Y' as i32 {
                                close(f);
                                return edit_error(
                                    ifiles,
                                    filename,
                                    alt_filename,
                                    Some(Box::new(altpipe.clone().unwrap())),
                                    ifile,
                                );
                            }
                        }
                    }
                }
            }
        }
        if force_open == 0 && f >= 0 && isatty(f) != 0 {
            let mut parg_0 = Parg::String(filename.clone().unwrap().to_string_lossy().into_owned());
            error(
                b"%s is a terminal (use -f to open it)\0" as *const u8 as *const std::ffi::c_char,
                &mut parg_0,
            );
            return edit_error(
                ifiles,
                filename.clone(),
                alt_filename,
                Some(Box::new(altpipe.clone().unwrap())),
                ifile,
            );
        }
    }
    end_logfile();
    was_curr_ifile = save_curr_ifile(ifiles);
    if curr_ifile.is_some() {
        let mut was_helpfile = ch_getflags() & CH_HELPFILE;
        close_file(ifiles);
        if was_helpfile != 0 && ifiles.held_ifile(was_curr_ifile).unwrap() <= 1 {
            /*
             * Don't keep the help file in the ifile list.
             */
            ifiles.del_ifile(was_curr_ifile);
            was_curr_ifile = None;
        }
    }
    unsave_ifile(ifiles, was_curr_ifile);
    if ifile.is_none() {
        /*
         * No new file to open.
         * (Don't set old_ifile, because if you call edit_ifile(NULL),
         *  you're supposed to have saved curr_ifile yourself,
         *  and you'll restore it if necessary.)
         */
        return 0;
    }

    /*
     * Set up the new ifile.
     * Get the saved position for the file.
     */
    curr_ifile = ifile;
    soft_eof = NULL_POSITION;
    println!("altfilename: {:?}", alt_filename);
    ifiles.set_altfilename(curr_ifile, alt_filename);
    println!("altpipe: {:?}", altpipe);
    println!("curr_ifile: {:?}", curr_ifile);
    if altpipe.is_none() {
        ifiles.set_altpipe(curr_ifile, None);
    } else {
        ifiles.set_altpipe(curr_ifile, Some(Box::new(altpipe.clone().unwrap())));
    }
    ifiles.set_open(curr_ifile);
    //let scr_pos = ifiles.get_pos(curr_ifile).unwrap();
    if let Some(scr_pos) = ifiles.get_pos(curr_ifile) {
        initial_scrpos.ln = scr_pos.ln;
        initial_scrpos.pos = scr_pos.pos as i64;
    } else {
        println!("cannot get pos!");
    }
    ch_init(f, chflags, nread);
    consecutive_nulls = 0;
    check_modelines();
    if chflags & CH_HELPFILE == 0 {
        if was_curr_ifile.is_some() {
            old_ifile = was_curr_ifile;
        }
        if !namelogfile.is_none() && is_tty != 0 {
            use_logfile(&namelogfile.clone().unwrap())
        }
        if open_filename != Some(PathBuf::from("-")) {
            let mut statbuf: stat = stat {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_mtim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_ctim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                __glibc_reserved: [0; 3],
            };
            let mut r = stat(
                CString::new(open_filename.unwrap().to_string_lossy().into_owned())
                    .unwrap()
                    .as_ptr(),
                &mut statbuf,
            );
            if r == 0 {
                curr_ino = statbuf.st_ino;
                curr_dev = statbuf.st_dev;
            }
        }
        if !every_first_cmd.is_null() {
            ungetsc(every_first_cmd);
            ungetcc_end_command();
        }
    }
    flush();
    if is_tty != 0 {
        /*
         * Output is to a real tty.
         */

        /*
         * Indicate there is nothing displayed yet.
         */
        pos_clear();
        clr_linenum();
        clr_hilite();
        undo_osc8();
        hshift = 0 as std::ffi::c_int;
        if filename != Some(PathBuf::from(FAKE_HELPFILE))
            && filename != Some(PathBuf::from(FAKE_EMPTYFILE))
        {
            let mut qfilename: *mut std::ffi::c_char = shell_quote(
                CString::new(filename.unwrap().to_string_lossy().into_owned())
                    .unwrap()
                    .as_ptr(),
            );
            cmd_addhist(ml_examine as *mut mlist, qfilename, LTRUE);
            free(qfilename as *mut std::ffi::c_void);
        }
        if want_filesize != 0 {
            scan_eof();
        }
        set_header(0);
    }
    0
}

/*
 * Edit a space-separated list of files.
 * For each filename in the list, enter it into the ifile list.
 * Then edit the first one.
 */
pub unsafe extern "C" fn edit_list(ifiles: &mut IFileManager, filelist: &str) -> i32 {
    let mut save_ifile: Option<IFileHandle> = None;
    let mut good_filename: Option<PathBuf> = None;
    let mut filename: Option<&String> = None;
    let mut gfilename: Option<&String> = None;
    let mut qfilename: Option<String> = None;
    save_ifile = save_curr_ifile(ifiles);
    /*
     * Run thru each filename in the list.
     * Try to glob the filename.
     * If it doesn't expand, just try to open the filename.
     * If it does expand, try to open each name in that list.
     */
    let tl_files = init_textlist(filelist);
    loop {
        filename = forw_textlist(&tl_files, filename);
        if filename.is_none() {
            break;
        }
        let gfilelist = if let Ok(gfl) = glob(filename.unwrap()) {
            gfl.filter(|p| p.is_ok())
                .map(|p| p.unwrap())
                .map(|p| p.as_path().to_string_lossy().to_owned().into_owned())
                .collect()
        } else {
            Vec::new()
        };
        let mut tl_gfiles = TextList::new(gfilelist);
        loop {
            if let Some(gfname) = forw_textlist(&tl_gfiles, gfilename.as_deref()) {
                let c_gfilename = CString::new(gfname.clone()).unwrap().as_ptr();
                qfilename = Some(
                    CStr::from_ptr(shell_unquote(c_gfilename))
                        .to_string_lossy()
                        .into_owned(),
                );
                if edit(ifiles, qfilename) == 0 && good_filename.is_none() {
                    if let Some(path) = ifiles.get_filename(curr_ifile) {
                        good_filename = Some(path.to_path_buf());
                    }
                }
            } else {
                break;
            }
        }
    }
    if good_filename.is_none() {
        unsave_ifile(ifiles, save_ifile);
        return 1;
    }
    if Some(ifiles.get_ifile(&good_filename.as_ref().unwrap(), curr_ifile)) == curr_ifile {
        unsave_ifile(ifiles, save_ifile);
        return 0;
    }
    reedit_ifile(ifiles, save_ifile);
    return edit(ifiles, Some(&good_filename.unwrap()));
}

/*
 * Edit the first file in the command line (ifile) list.
 */
pub unsafe extern "C" fn edit_first(ifiles: &mut IFileManager) -> i32 {
    if ifiles.nifile() == 0 {
        return edit_stdin(ifiles);
    }
    curr_ifile = None;
    edit_next(ifiles, 1)
}

/*
 * Edit the last file in the command line (ifile) list.
 */
pub unsafe extern "C" fn edit_last(ifiles: &mut IFileManager) -> std::ffi::c_int {
    curr_ifile = None;
    return edit_prev(ifiles, 1);
}

/*
 * Edit the n-th next or previous file in the command line (ifile) list.
 */
unsafe extern "C" fn edit_istep(
    ifiles: &mut IFileManager,
    h: Option<IFileHandle>,
    n: i32,
    dir: i32,
) -> i32 {
    let mut next: Option<IFileHandle> = None;
    let mut n = n;
    let mut h = h;

    /*
     * Skip n filenames, then try to edit each filename.
     */
    loop {
        next = if dir > 0 {
            ifiles.next_ifile(h)
        } else {
            ifiles.prev_ifile(h)
        };
        n -= 1;
        if n < 0 {
            if edit_ifile(ifiles, h) == 0 {
                break;
            }
        }
        if next.is_none() {
            /*
             * Reached end of the ifile list.
             */
            return 1;
        }
        if abort_sigs() {
            /*
             * Interrupt breaks out, if we're in a long
             * list of files that can't be opened.
             */
            return 1;
        }
        h = next;
    }
    /*
     * Found a file that we can edit.
     */
    0
}

unsafe extern "C" fn edit_inext(ifiles: &mut IFileManager, h: Option<IFileHandle>, n: i32) -> i32 {
    return edit_istep(ifiles, h, n, 1);
}

pub unsafe extern "C" fn edit_next(ifiles: &mut IFileManager, n: i32) -> i32 {
    return edit_istep(ifiles, curr_ifile, n, 1);
}

unsafe extern "C" fn edit_iprev(ifiles: &mut IFileManager, h: Option<IFileHandle>, n: i32) -> i32 {
    return edit_istep(ifiles, h, n, -(1 as std::ffi::c_int));
}

pub unsafe extern "C" fn edit_prev(ifiles: &mut IFileManager, n: i32) -> i32 {
    return edit_istep(ifiles, curr_ifile, n, -(1 as std::ffi::c_int));
}

/*
 * Edit a specific file in the command line (ifile) list.
 */
pub unsafe extern "C" fn edit_index(ifiles: &mut IFileManager, n: i32) -> i32 {
    let mut h: Option<IFileHandle> = None;
    loop {
        h = ifiles.next_ifile(h);
        if h.is_none() {
            return 1;
        }
        if !(ifiles.get_index(h) != Some(n)) {
            break;
        }
    }
    return edit_ifile(ifiles, h);
}

pub unsafe extern "C" fn save_curr_ifile(ifiles: &mut IFileManager) -> Option<IFileHandle> {
    if curr_ifile.is_some() {
        ifiles.hold_ifile(curr_ifile, 1);
    }
    curr_ifile
}

pub unsafe extern "C" fn unsave_ifile(ifiles: &mut IFileManager, save_ifile: Option<IFileHandle>) {
    if save_ifile.is_some() {
        ifiles.hold_ifile(save_ifile, -(1 as std::ffi::c_int));
    }
}

/*
 * Reedit the ifile which was previously open.
 */
pub unsafe extern "C" fn reedit_ifile(ifiles: &mut IFileManager, save_ifile: Option<IFileHandle>) {
    let next = ifiles.next_ifile(save_ifile);
    let prev = ifiles.prev_ifile(save_ifile);
    /*
     * Try to reopen the ifile.
     * Note that opening it may fail (maybe the file was removed),
     * in which case the ifile will be deleted from the list.
     * So save the next and prev ifiles first.
     */
    unsave_ifile(ifiles, save_ifile);
    if edit_ifile(ifiles, save_ifile) == 0 {
        return;
    }
    /*
     * If can't reopen it, open the next input file in the list.
     */
    if next.is_some() && edit_inext(ifiles, next, 0) == 0 {
        return;
    }
    /*
     * If can't open THAT one, open the previous input file in the list.
     */
    if prev.is_some() && edit_iprev(ifiles, prev, 0) == 0 {
        return;
    }

    /*
     * If can't even open that, we're stuck.  Just quit.
     */
    quit(1);
}

pub unsafe extern "C" fn reopen_curr_ifile(ifiles: &mut IFileManager) {
    let save_ifile = save_curr_ifile(ifiles);
    close_file(ifiles);
    reedit_ifile(ifiles, save_ifile);
}

/*
 * Edit standard input.
 */
pub unsafe extern "C" fn edit_stdin(ifiles: &mut IFileManager) -> i32 {
    if isatty(fd0) != 0 {
        error(
            b"Missing filename (\"less --help\" for help)\0" as *const u8
                as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        quit(0);
    }
    return edit(ifiles, Some("-"));
}

/*
 * Copy a file directly to standard output.
 * Used if standard output is not a tty.
 */
pub unsafe extern "C" fn cat_file() {
    let mut c = 0;
    loop {
        c = ch_forw_get();
        if !(c != EOI) {
            break;
        }
        putchr(c);
    }
    flush();
}

/*
 * If the user asked for a log file and our input file
 * is standard input, create the log file.
 * We take care not to blindly overwrite an existing file.
 */
pub unsafe extern "C" fn use_logfile(filename: &str) {
    let mut exists = false;
    let mut path = Path::new(filename);
    let mut answer = 0;
    let mut parg = Parg::Null;
    let mut file = OpenOptions::new();
    let mut log_file;
    if ch_getflags() & CH_CANSEEK != 0 {
        /*
         * Can't currently use a log file on a file that can seek.
         */
        return;
    }
    if let Ok(e) = path.try_exists() {
        exists = e;
    }

    /*
     * Decide whether to overwrite the log file or append to it.
     * If it doesn't exist we "overwrite" it.
     */
    if !exists || force_logfile != 0 {
        /*
         * Overwrite (or create) the log file.
         */
        answer = 'O' as i32;
    } else {
        /*
         * Ask user what to do.
         */
        parg = Parg::String(filename.into());
        answer = query(
            b"Warning: \"%s\" exists; Overwrite, Append, Don't log, or Quit? \0" as *const u8
                as *const std::ffi::c_char,
            &mut parg,
        );
    }
    loop {
        match char::from_u32(answer as u32) {
            Some('O') | Some('o') => {
                /*
                 * Overwrite: create the file.
                 */
                log_file = file.read(true).write(true).create(true).open(path);
                break;
            }
            Some('A') | Some('a') => {
                /*
                 * Append: open the file and seek to the end.
                 */
                log_file = file.append(true).open(path);
            }
            Some('D') | Some('d') => return,
            _ => {
                answer = query(
                    b"Overwrite, Append, Don't log, or Quit? (Type \"O\", \"A\", \"D\" or \"Q\") \0"
                        as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
            }
        }
    }
    if logfile < 0 {
        /*
         * Error in opening logfile.
         */
        parg = Parg::String(filename.into());
        error(
            b"Cannot write to \"%s\"\0" as *const u8 as *const std::ffi::c_char,
            &mut parg,
        );
        return;
    }
    //FIXME
    // SET_BINARY(logfile);
}
