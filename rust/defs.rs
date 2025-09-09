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
