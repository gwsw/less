use std::ffi::CStr;
use std::str::Utf8Error;

#[no_mangle]
pub unsafe extern "C" fn ptr_to_str<'a>(
    ptr: *const std::ffi::c_char,
) -> Result<&'a str, Utf8Error> {
    CStr::from_ptr(ptr).to_str()
}
