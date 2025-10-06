use crate::signal::sigs;

pub type __off_t = i64;
pub type __ssize_t = i32;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
pub type lbool = u32;
pub type less_off_t = off_t;
pub type POSITION = less_off_t;
pub type LINENUM = off_t;
pub type __syscall_slong_t = i64;
pub type __time_t = i64;
pub type time_t = __time_t;
pub type __uintmax_t = i64;
pub type uintmax_t = __uintmax_t;
pub type uintmax = uintmax_t;
pub type __off64_t = i64;
pub type __mode_t = i32;
pub type mode_t = __mode_t;
pub type __ino_t = u64;
pub type ino_t = __ino_t;
pub type __dev_t = u64;
pub type dev_t = __dev_t;
pub type __blkcnt_t = i64;
pub type wint_t = i32;
pub type ansi_state = i32;
pub type __blksize_t = i32;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __nlink_t = u64;
pub type LWCHAR = i32;

pub const NULL_POSITION: i64 = -1;
pub const EOI: i32 = -1;

pub const OPT_OFF: i32 = 0;
pub const OPT_ON: i32 = 0;
pub const OPT_ONPLUS: i32 = 0;

pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;

pub const S_INTERRUPT: i32 = 1 << 0;
pub const S_SWINTERRUPT: i32 = 1 << 1;
pub const S_STOP: i32 = 1 << 2;
pub const S_WINCH: i32 = 1 << 3;

/* filestate flags */
pub const CH_CANSEEK: i32 = 0o1;
pub const CH_KEEPOPEN: i32 = 0o2;
pub const CH_POPENED: i32 = 0o4;
pub const CH_HELPFILE: i32 = 0o10;
pub const CH_NODATA: i32 = 0o20; /* Special case for zero length files */
pub const CH_NOTRUSTSIZE: i32 = 0o40; /* For files that claim 0 length size falsely */

pub const FAKE_HELPFILE: &'static str = "@/\\less/\\help/\\file/\\@";
pub const FAKE_EMPTYFILE: &'static str = "@/\\less/\\empty/\\file/\\@";

pub const SEEK_SET: i32 = 0;
pub const SEEK_END: i32 = 2;

pub unsafe fn abort_sigs() -> bool {
    (sigs & (S_INTERRUPT | S_SWINTERRUPT | S_STOP)) != 0
}

/* Security features. */
pub const SF_EDIT: i32 = 1 << 1; /* Edit file (v) */
pub const SF_EXAMINE: i32 = 1 << 2; /* Examine file (:e) */
pub const SF_GLOB: i32 = 1 << 3; /* Expand file pattern */
pub const SF_HISTORY: i32 = 1 << 4; /* History file */
pub const SF_LESSKEY: i32 = 1 << 5; /* Lesskey files */
pub const SF_LESSOPEN: i32 = 1 << 6; /* LESSOPEN */
pub const SF_LOGFILE: i32 = 1 << 7; /* Log file (s, -o) */
pub const SF_PIPE: i32 = 1 << 8; /* Pipe (|) */
pub const SF_SHELL: i32 = 1 << 9; /* Shell command (!) */
pub const SF_STOP: i32 = 1 << 10; /* Stop signal */
pub const SF_TAGS: i32 = 1 << 11; /* Tags */
pub const SF_OSC8_OPEN: i32 = 1 << 12; /* OSC8 open */
