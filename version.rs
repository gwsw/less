use ::libc;
#[no_mangle]
pub static mut version: [std::ffi::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &mut [std::ffi::c_char; 5]>(b"681x\0")
};
