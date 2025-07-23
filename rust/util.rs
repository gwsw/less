use std::ffi::CStr;

#[no_mangle]
pub unsafe extern "C" fn ptr_to_str<'a>(ptr: *const std::ffi::c_char) -> &'a str {
    CStr::from_ptr(ptr).to_str().unwrap()
}
