pub type __off_t = std::ffi::c_long;
pub type off_t = __off_t;
pub type lbool = std::ffi::c_uint;
pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;
pub type less_off_t = off_t;
pub type POSITION = i32;

pub const NULL_POSITION: i32 = -1;
pub const EOI: i32 = -1;

pub const OPT_OFF: i32 = 0;
pub const OPT_ON: i32 = 0;
pub const OPT_ONPLUS: i32 = 0;
