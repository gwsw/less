pub type __off_t = i32;
pub type __ssize_t = i32;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
pub type lbool = u32;
pub type less_off_t = off_t;
pub type POSITION = less_off_t;
pub type LINENUM = off_t;

pub const NULL_POSITION: i32 = -1;
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

pub fn abort_sigs() -> bool {
    (sigs & (S_INTERRUPT | S_SWINTERRUPT | S_STOP)) != 0
}
