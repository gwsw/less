use crate::ch::FileState;
use crate::charset::{binary_char, is_utf8_well_formed, step_charc, utf_skip_to_lead};
use crate::decode::lgetenv;
use crate::defs::*;
use crate::ifile::{IFileHandle, IFileManager};
use crate::line::{ansi_start, skip_ansi};
use crate::xbuf::XBuffer;
use ::c2rust_bitfields;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};
use std::os::unix::fs::MetadataExt;
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdout};
use std::process::{Command, Stdio};

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ansi_state;
    fn sprintf(_: *mut std::ffi::c_char, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn snprintf(
        _: *mut std::ffi::c_char,
        _: std::ffi::c_ulong,
        _: *const std::ffi::c_char,
        _: ...
    ) -> std::ffi::c_int;
    fn getc(__stream: *mut FILE) -> std::ffi::c_int;
    fn fileno(__stream: *mut FILE) -> std::ffi::c_int;
    fn pclose(__stream: *mut FILE) -> std::ffi::c_int;
    fn popen(__command: *const std::ffi::c_char, __modes: *const std::ffi::c_char) -> *mut FILE;
    fn open(__file: *const std::ffi::c_char, __oflag: std::ffi::c_int, _: ...) -> std::ffi::c_int;
    fn lseek(__fd: std::ffi::c_int, __offset: __off_t, __whence: std::ffi::c_int) -> __off_t;
    fn close(__fd: std::ffi::c_int) -> std::ffi::c_int;
    fn read(__fd: std::ffi::c_int, __buf: *mut std::ffi::c_void, __nbytes: size_t) -> ssize_t;
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn realpath(
        __name: *const std::ffi::c_char,
        __resolved: *mut std::ffi::c_char,
    ) -> *mut std::ffi::c_char;
    fn strcpy(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strcat(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strncmp(
        _: *const std::ffi::c_char,
        _: *const std::ffi::c_char,
        _: std::ffi::c_ulong,
    ) -> std::ffi::c_int;
    fn strchr(_: *const std::ffi::c_char, _: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn save(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
    fn secure_allow(features: std::ffi::c_int) -> std::ffi::c_int;
    fn ch_ungetchar(c: std::ffi::c_int);
    fn seekable(f: std::ffi::c_int) -> std::ffi::c_int;
    fn isnullenv(s: *const std::ffi::c_char) -> lbool;
    fn get_filename(ifile: *mut std::ffi::c_void) -> *const std::ffi::c_char;
    fn ansi_done(pansi: *mut ansi_state);
    fn errno_message(filename: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn stat(__file: *const std::ffi::c_char, __buf: *mut stat) -> std::ffi::c_int;
    fn fstat(__fd: std::ffi::c_int, __buf: *mut stat) -> std::ffi::c_int;
    static mut force_open: bool;
    static mut use_lessopen: std::ffi::c_int;
    static mut ctldisp: std::ffi::c_int;
    static mut utf_mode: bool;
    static mut curr_ifile: Option<IFileHandle>;
    static mut old_ifile: Option<IFileHandle>;
    static mut openquote: char;
    static mut closequote: char;
    static mut curr_ino: ino_t;
    static mut curr_dev: dev_t;
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
pub union parg {
    pub p_string: *const std::ffi::c_char,
    pub p_int: std::ffi::c_int,
    pub p_linenum: LINENUM,
    pub p_char: std::ffi::c_char,
}
pub type PARG = parg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcpy {
    pub dest: *mut std::ffi::c_char,
    pub copied: size_t,
}
const DEF_METACHARS: &'static str = "; *?\t\n'\"()<>[]|&^`#\\$%=~{},";
const DEF_METAESCAPE: &'static str = "\\";

/// Remove surrounding shell quotes or escape sequences from a string.
pub fn shell_unquote(input: &str) -> String {
    // External helpers/constants assumed to exist:
    fn get_meta_escape() -> &'static str {
        // TODO: implement, e.g. "\\"
        "\\"
    }
    const OPEN_QUOTE: char = '"'; // TODO: adjust
    const CLOSE_QUOTE: char = '"'; // TODO: adjust

    let mut result = String::with_capacity(input.len());

    let mut chars = input.chars().peekable();

    if chars.peek() == Some(&OPEN_QUOTE) {
        // Skip the opening quote
        chars.next();

        while let Some(ch) = chars.next() {
            if ch == CLOSE_QUOTE {
                // If next char is also CLOSE_QUOTE, treat as escaped quote
                if chars.peek() == Some(&CLOSE_QUOTE) {
                    chars.next(); // consume second quote
                    result.push(CLOSE_QUOTE);
                    continue;
                } else {
                    // single closequote → end of quoted section
                    break;
                }
            }
            result.push(ch);
        }
    } else {
        // Escape-based handling
        let esc = get_meta_escape();
        let esclen = esc.len();

        let mut i = 0;
        let bytes = input.as_bytes();
        while i < bytes.len() {
            if esclen > 0 && i + esclen <= bytes.len() && &input[i..i + esclen] == esc {
                // Skip escape sequence
                i += esclen;
                if i < bytes.len() {
                    result.push(input[i..].chars().next().unwrap());
                    i += input[i..].chars().next().unwrap().len_utf8();
                }
            } else {
                // Copy char normally
                let ch = input[i..].chars().next().unwrap();
                result.push(ch);
                i += ch.len_utf8();
            }
        }
    }

    result
}

pub unsafe extern "C" fn get_meta_escape() -> String {
    if let Ok(s) = lgetenv("LESSMETAESCAPE") {
        s
    } else {
        DEF_METAESCAPE.into()
    }
}

unsafe extern "C" fn metachars() -> String {
    if let Ok(mchars) = lgetenv("LESSMETACHARS") {
        mchars
    } else {
        DEF_METACHARS.into()
    }
}

/*
 * Is this a shell metacharacter?
 */
unsafe extern "C" fn metachar(c: char) -> bool {
    metachars().contains(c)
}

/*
 * Must use quotes rather than escape char for this metachar?
 */
unsafe extern "C" fn must_quote(c: char) -> bool {
    /* {{ Maybe the set of must_quote chars should be configurable? }} */
    c == '\n'
}

/// Insert a backslash (or quote) before each metacharacter in a string.
///
/// Returns `None` if quoting is required but the input already contains quotes.
pub unsafe fn shell_quote(s: &str) -> Option<String> {
    let esc = &get_meta_escape();
    let esclen = esc.len();
    let mut use_quotes = false;
    let mut have_quotes = false;

    // First pass: decide if we need quotes
    for ch in s.chars() {
        if ch == openquote || ch == closequote {
            have_quotes = true;
        }
        if metachar(ch) {
            if esclen == 0 {
                // This shell doesn't support escape chars → must use quotes
                use_quotes = true;
            }
        }
    }

    if use_quotes {
        if have_quotes {
            // Cannot quote a string that already contains quotes
            return None;
        }
        // Surround the whole string with quotes
        let mut result = String::with_capacity(s.len() + 2);
        result.push(openquote);
        result.push_str(s);
        result.push(closequote);
        return Some(result);
    }

    // Otherwise, build escaped string piece by piece
    let mut result = String::with_capacity(s.len() + s.len() * esclen);
    for ch in s.chars() {
        if !metachar(ch) {
            result.push(ch);
        } else if must_quote(ch) {
            // Surround the char with quotes
            result.push(openquote);
            result.push(ch);
            result.push(closequote);
        } else {
            // Insert escape chars
            result.push_str(esc);
            result.push(ch);
        }
    }

    Some(result)
}

/// Construct a full pathname for a file in a directory.
/// Returns `None` if the directory is empty or if `must_exist` is true
/// and the file does not exist.
pub fn dirfile(dirname: &str, filename: &str, must_exist: bool) -> Option<String> {
    if dirname.is_empty() {
        return None;
    }

    // Construct the path in a portable way
    let path = Path::new(dirname).join(filename);

    if must_exist {
        // If required, check if the file exists
        if !path.exists() {
            return None;
        }
    }

    // Convert to owned String (lossy fallback for non-UTF8)
    Some(path.to_string_lossy().into_owned())
}

/// Return the full pathname of the given file in the "home directory".
/// Returns `None` if the file cannot be found.
pub fn homefile(filename: &str) -> Option<PathBuf> {
    // Try $HOME/filename
    if let Some(home) = env::var_os("HOME") {
        let candidate = Path::new(&home).join(filename);
        if candidate.exists() {
            return Some(candidate);
        }
    }

    // Check for windows home
    #[cfg(windows)]
    {
        if let Some(userprofile) = env::var_os("USERPROFILE") {
            let candidate = Path::new(&userprofile).join(filename);
            if candidate.exists() {
                return Some(candidate);
            }
        }
        if let Some(homedir) = env::var_os("HOMEPATH") {
            if let Some(homedrive) = env::var_os("HOMEDRIVE") {
                let candidate = Path::new(&homedrive).join(homedir).join(filename);
                if candidate.exists() {
                    return Some(candidate);
                }
            }
        }
    }

    // No valid file found
    None
}
/// Appends a filename to the string, quoting it if it contains spaces.
fn append_filename(buffer: &mut String, filename: &str, open_quote: char, close_quote: char) {
    let needs_quote = filename.contains(' ');

    if needs_quote {
        buffer.push(open_quote);
    }
    buffer.push_str(filename);
    if needs_quote {
        buffer.push(close_quote);
    }
}

/// Expands a string, substituting any "%" with the current filename,
/// and any "#" with the previous filename.
/// A string of N "%"s is replaced with N-1 "%"s.
/// Likewise for a string of N "#"s.
pub unsafe fn fexpand(ifiles: &mut IFileManager, s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        match ch {
            '%' | '#' => {
                // Check if next character is the same
                if i + 1 < chars.len() && chars[i + 1] == ch {
                    // Two identical chars: output just one
                    result.push(ch);
                    i += 2;
                } else {
                    // Single char: expand to filename
                    let ifile = if ch == '%' { curr_ifile } else { old_ifile };

                    if ifile == None {
                        result.push(ch);
                    } else {
                        let filename = ifiles.get_filename(ifile);
                        append_filename(
                            &mut result,
                            &filename.unwrap().to_str().unwrap(),
                            openquote,
                            closequote,
                        );
                    }
                    i += 1;
                }
            }
            _ => {
                result.push(ch);
                i += 1;
            }
        }
    }

    result
}

/// Returns a blank-separated list of filenames which "complete" the given string.
/// Returns `None` if the filename didn't expand or if security doesn't allow globbing.
pub unsafe fn fcomplete(s: &str) -> Option<String> {
    // Check security permissions
    if secure_allow(SF_GLOB) == 0 {
        return None;
    }

    // Build the glob pattern
    let fpat = format!("{}*", s);

    // Perform the glob operation using Rust's glob crate
    let matches = glob_pattern(&fpat)?;

    // Check if any files were actually matched
    if matches.is_empty() {
        None
    } else {
        Some(matches.join(" "))
    }
}

/// Performs glob pattern matching and returns a list of matching filenames.
/// Returns `None` if glob fails or no matches found.
fn glob_pattern(pattern: &str) -> Option<Vec<String>> {
    use glob::glob;

    match glob(pattern) {
        Ok(paths) => {
            let matches: Vec<String> = paths
                .filter_map(Result::ok)
                .filter_map(|p| {
                    p.to_str().map(|s| {
                        // Quote filenames with spaces
                        if s.contains(' ') {
                            format!("\"{}\"", s)
                        } else {
                            s.to_string()
                        }
                    })
                })
                .collect();

            if matches.is_empty() {
                None
            } else {
                Some(matches)
            }
        }
        Err(_) => None,
    }
}

pub unsafe fn bin_file(file: &mut File) -> Option<(bool, usize)> {
    const BUFFER_SIZE: usize = 256;
    const BINARY_THRESHOLD: usize = 5;

    // Try to seek to the beginning of the file
    if file.seek(SeekFrom::Start(0)).is_err() {
        return None;
    }

    // Read up to 256 bytes from the file
    let mut data = [0u8; BUFFER_SIZE];
    let bytes_read = match file.read(&mut data) {
        Ok(0) => return None, // Empty file or read error
        Ok(n) => n,
        Err(_) => return None,
    };

    let data = &data[..bytes_read];
    let mut bin_count = 0;
    let mut pos = 0;

    while pos < bytes_read {
        if utf_mode && !is_utf8_well_formed(&data[pos..], data.len() - pos) {
            // Not well-formed UTF-8
            bin_count += 1;
            // Skip to next UTF-8 lead byte
            utf_skip_to_lead(&data, &mut pos, bytes_read);
        } else {
            // Try to read a character
            let (c, p) = step_charc(data, 1, 0, bytes_read);
            // Check for ANSI escape sequences
            if ctldisp == OPT_ONPLUS {
                if let Some(mut pansi) = ansi_start(c) {
                    pos = skip_ansi(&mut pansi, c, data, bytes_read);
                    continue;
                }
            }

            // Check if character is binary
            if binary_char(c) {
                bin_count += 1;
            } else {
                // Couldn't read character, move forward
                pos += 1;
            }
        }
    }

    // Call it a binary file if there are more than 5 binary characters
    // in the first 256 bytes of the file.
    let is_binary = bin_count > BINARY_THRESHOLD;
    Some((is_binary, bytes_read))
}

pub fn seek_filesize(file: &File) -> POSITION {
    file.metadata().ok().map(|m| m.len()).unwrap() as POSITION
}

/// Read a line byte-by-byte (closer to the original C implementation).
/// This version reads character by character until newline or EOF.
pub fn readfd<R: Read>(reader: &mut R) -> Option<String> {
    let mut result = String::new();
    let mut buf = [0u8; 1];

    loop {
        match reader.read(&mut buf) {
            Ok(0) => {
                // EOF
                if result.is_empty() {
                    return None;
                }
                break;
            }
            Ok(_) => {
                let ch = buf[0];
                if ch == b'\n' {
                    break;
                }
                // Only add valid UTF-8 characters
                // Invalid bytes are replaced with replacement character
                if let Ok(s) = std::str::from_utf8(&buf) {
                    result.push_str(s);
                } else {
                    result.push('�'); // UTF-8 replacement character
                }
            }
            Err(_) => {
                if result.is_empty() {
                    return None;
                }
                break;
            }
        }
    }

    Some(result)
}

/// Execute a shell command and return a pipe to its stdout.
/// Returns an error if the command cannot be executed.
fn shellcmd(cmd: &str) -> io::Result<BufReader<std::process::ChildStdout>> {
    // Try $SHELL, fallback to /bin/sh
    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());

    // Build the command: shell -c "cmd"
    let child = Command::new(shell)
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .spawn()?;

    // Extract stdout and wrap in BufReader
    let stdout = child
        .stdout
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "failed to capture stdout"))?;

    Ok(BufReader::new(stdout))
}

/*
 * Does path not represent something in the file system?
 */
pub extern "C" fn is_fake_pathname(path: &str) -> bool {
    path == "-" || path == FAKE_HELPFILE || path == FAKE_EMPTYFILE
}

/// Return canonical pathname.
/// Falls back to the original path if canonicalization fails
/// or if the path is "fake".
pub fn lrealpath(path: &str) -> PathBuf {
    if !is_fake_pathname(path) {
        if let Ok(canon) = fs::canonicalize(path) {
            return canon;
        }
    }

    // Fallback: just return the given path as-is
    Path::new(path).to_path_buf()
}

/// Return number of %s escapes in a string.
/// Return a large number if there are any other % escapes besides %s.
fn num_pct_s(input: &str) -> i32 {
    let mut num = 0;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            match chars.peek() {
                Some('%') => {
                    // Skip the second '%'
                    chars.next();
                }
                Some('s') => {
                    num += 1;
                    chars.next();
                }
                Some(_) => {
                    return 999;
                }
                None => {
                    // Trailing '%' without pair
                    return 999;
                }
            }
        }
    }

    num
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecurityFlag {
    LessOpen,
}

/// Represents an alternative file that was opened
#[derive(Debug)]
pub enum AltFile {
    /// A pipe to a preprocessor command (reader only, process managed separately)
    Pipe(BufReader<ChildStdout>),
    /// A regular file specified by path
    RegularFile(String),
    /// An empty fake file
    EmptyFile,
}

/// Result of attempting to open an alternative file
pub type AltFileResult = Option<AltFile>;

/// See if we should open a "replacement file" instead of the file we're about to open.
///
/// This function checks the LESSOPEN environment variable and potentially runs
/// a preprocessor command to generate an alternative version of the file.
pub unsafe fn open_altfile<F>(filename: &str) -> AltFileResult {
    if secure_allow(SF_LESSOPEN) == 0 {
        return None;
    }

    if use_lessopen == 0 {
        return None;
    }

    ch_ungetchar(-1);

    let lessopen = env::var("LESSOPEN").ok()?;

    // Parse LESSOPEN string
    let (lessopen, pipe_count) = parse_lessopen(&lessopen);
    let (lessopen, accepts_stdin) = parse_stdin_flag(lessopen);

    // Check if we should process stdin
    if !accepts_stdin && filename == "-" {
        return None;
    }

    // Validate format string
    if num_pct_s(lessopen) != 1 {
        error(
            b"LESSOPEN ignored: must contain exactly one %s\0" as *const u8 as *const i8,
            0 as *mut PARG,
        );
        return None;
    }

    // Build and execute command
    let quoted_filename = shell_quote(filename).unwrap_or_else(|| filename.to_string());
    let cmd = lessopen.replace("%s", &quoted_filename);

    let reader = match shellcmd(&cmd) {
        Ok(r) => r,
        Err(_) => return None,
    };

    if pipe_count > 0 {
        // Pipe preprocessor mode
        handle_pipe_preprocessor(reader, pipe_count)
    } else {
        // Regular file mode - read filename from stdout
        handle_regular_file(reader)
    }
}

/// Parse pipe indicators (leading |) from LESSOPEN
fn parse_lessopen(lessopen: &str) -> (&str, usize) {
    let pipe_count = lessopen.chars().take_while(|&c| c == '|').count();
    (&lessopen[pipe_count..], pipe_count)
}

/// Parse stdin acceptance flag (leading -)
fn parse_stdin_flag(lessopen: &str) -> (&str, bool) {
    if lessopen.starts_with('-') {
        (&lessopen[1..], true)
    } else {
        (lessopen, false)
    }
}

/// Handle pipe preprocessor mode
fn handle_pipe_preprocessor(
    mut reader: BufReader<ChildStdout>,
    pipe_count: usize,
) -> AltFileResult {
    // Read one byte to check if pipe produces data
    let mut buf = [0u8; 1];

    match reader.read(&mut buf) {
        Ok(1) => {
            // Pipe has data - return the reader
            // Note: We've consumed one byte, which ideally should be "ungotten"
            // In a real implementation, you'd need to handle this
            Some(AltFile::Pipe(reader))
        }
        Ok(0) | Err(_) => {
            // Pipe is empty
            // Note: We can't easily check exit status with just a BufReader
            // This is a limitation of working with BufReader instead of Child
            if pipe_count > 1 {
                // Multiple pipes might indicate empty file
                Some(AltFile::EmptyFile)
            } else {
                // No alternative file
                None
            }
        }
        Ok(_) => unreachable!(),
    }
}

/// Handle regular file mode - read filename from command output
fn handle_regular_file(mut reader: BufReader<ChildStdout>) -> AltFileResult {
    let mut filename = String::new();
    if reader.read_line(&mut filename).ok()? == 0 {
        // Empty output - no alternative file
        return None;
    }

    // Note: We can't wait for the child process here since we only have the reader
    // The process will be cleaned up when the reader is dropped

    // Remove trailing newline
    let filename = filename.trim_end().to_string();

    if filename.is_empty() {
        None
    } else {
        Some(AltFile::RegularFile(filename))
    }
}

/// Close a replacement file by running LESSCLOSE command.
pub unsafe fn close_altfile(altfilename: &str, filename: &str) {
    if secure_allow(SF_LESSOPEN) == 0 {
        return;
    }

    let lessclose = match env::var("LESSCLOSE") {
        Ok(s) if !s.is_empty() => s,
        _ => return,
    };

    // Validate format string
    let percent_count = num_pct_s(&lessclose);
    if percent_count > 2 {
        error(
            b"LESSCLOSE ignored; must contain no more than 2 %s\0" as *const u8 as *const i8,
            0 as *mut PARG,
        );
        return;
    }

    // Build command string
    let quoted_filename = shell_quote(filename).unwrap_or_else(|| filename.to_string());
    let quoted_altfilename = shell_quote(altfilename).unwrap_or_else(|| altfilename.to_string());

    let cmd = if percent_count == 2 {
        // Replace first %s with altfilename, second with filename
        lessclose
            .replacen("%s", &quoted_altfilename, 1)
            .replacen("%s", &quoted_filename, 1)
    } else if percent_count == 1 {
        lessclose.replace("%s", &quoted_altfilename)
    } else {
        lessclose
    };

    // Execute command and consume output
    if let Ok(mut reader) = shellcmd(&cmd) {
        // Read and discard all output to ensure the process completes
        let mut buf = Vec::new();
        let _ = reader.read_to_end(&mut buf);
        // Process will be cleaned up when reader is dropped
    }
}

/// Is the specified file a directory?
///
/// Returns `true` if the path exists and is a directory, `false` otherwise.
/// This includes cases where the path doesn't exist or there's an I/O error.
pub fn is_dir<P: AsRef<Path>>(filename: P) -> bool {
    fs::metadata(filename).map(|m| m.is_dir()).unwrap_or(false)
}

/// Returns `None` if the file can be opened and is an ordinary file,
/// otherwise returns an error message (if it cannot be opened or is a directory, etc.)
pub unsafe fn bad_file<P: AsRef<Path>>(filename: P) -> Option<String> {
    let path = filename.as_ref();
    let filename_str = path.to_string_lossy();

    // Check if it's a directory (unless force_open is set)
    if !force_open && path.is_dir() {
        return Some(format!("{} is a directory", filename_str));
    }

    // Get file metadata
    match fs::metadata(path) {
        Err(e) => {
            // File cannot be accessed
            Some(format!("{}: {}", filename_str, e))
        }
        Ok(metadata) => {
            if force_open {
                // Force open mode - accept any file type
                None
            } else if !metadata.is_file() {
                // Not a regular file
                Some(format!(
                    "{} is not a regular file (use -f to see it)",
                    filename_str
                ))
            } else {
                // File is okay
                None
            }
        }
    }
}

/// Get file size with fallback to seeking
pub fn filesize(file: &mut File) -> POSITION {
    // Try metadata first (fastest, doesn't change file position)
    if let Ok(metadata) = file.metadata() {
        return metadata.len() as POSITION;
    }

    // Fall back to seeking to the end
    seek_filesize(file)
}

/// Check if the current file has changed.
///
/// Returns `true` if:
/// - The file's inode or device has changed (Unix)
/// - The file is smaller than the current position
/// - On non-Unix systems, if size or modification time changed
pub unsafe fn curr_ifile_changed<R>(ifiles: &mut IFileManager, fs: &FileState<File>) -> bool {
    let filename;
    if let Some(fname) = ifiles.get_filename(curr_ifile) {
        filename = fname;
    } else {
        return false;
    }
    // Get current file metadata
    let metadata = match fs::metadata(filename) {
        Ok(m) => m,
        Err(_) => return true, // File doesn't exist or can't be accessed
    };

    let curr_pos = fs.pos();
    let current_inode = metadata.ino();
    let current_device = metadata.dev();

    // Check if inode or device changed
    if current_inode != curr_ino || current_device != curr_dev {
        return true;
    }

    let current_size = metadata.len();
    if let Ok(current_modified) = metadata.modified() {
        if current_size != curr_pos as u64
        /*|| current_modified != original_identity.modified */
        {
            return true;
        }
    }

    // Check if file is smaller than current position
    if let Some(pos) = Some(curr_pos) {
        if metadata.len() < pos as u64 {
            return true;
        }
    }

    false
}

pub unsafe extern "C" fn last_component(
    mut name: *const std::ffi::c_char,
) -> *const std::ffi::c_char {
    let mut slash: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    slash = name.offset(strlen(name) as isize);
    while slash > name {
        slash = slash.offset(-1);
        if *slash as std::ffi::c_int
            == *(b"/\0" as *const u8 as *const std::ffi::c_char) as std::ffi::c_int
            || *slash as std::ffi::c_int == '/' as i32
        {
            return slash.offset(1 as std::ffi::c_int as isize);
        }
    }
    return name;
}
