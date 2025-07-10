use ::libc;
extern "C" {
    fn free(_: *mut std::ffi::c_void);
    fn memcpy(
        _: *mut std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn out_of_memory();
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
}
pub type size_t = std::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xbuffer {
    pub data: *mut std::ffi::c_uchar,
    pub end: size_t,
    pub size: size_t,
    pub init_size: size_t,
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_init(mut xbuf: *mut xbuffer) {
    xbuf_init_size(xbuf, 16 as std::ffi::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_init_size(mut xbuf: *mut xbuffer, mut init_size: size_t) {
    (*xbuf).data = 0 as *mut std::ffi::c_uchar;
    (*xbuf).end = 0 as std::ffi::c_int as size_t;
    (*xbuf).size = (*xbuf).end;
    (*xbuf).init_size = init_size;
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_deinit(mut xbuf: *mut xbuffer) {
    if !((*xbuf).data).is_null() {
        free((*xbuf).data as *mut std::ffi::c_void);
    }
    xbuf_init(xbuf);
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_reset(mut xbuf: *mut xbuffer) {
    (*xbuf).end = 0 as std::ffi::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_add_byte(mut xbuf: *mut xbuffer, mut b: std::ffi::c_uchar) {
    if (*xbuf).end >= (*xbuf).size {
        let mut data: *mut std::ffi::c_uchar = 0 as *mut std::ffi::c_uchar;
        let (fresh0, fresh1) = ((*xbuf).size).overflowing_add(if (*xbuf).size != 0 {
            (*xbuf).size
        } else {
            (*xbuf).init_size
        });
        *(&mut (*xbuf).size as *mut size_t) = fresh0;
        if fresh1 {
            out_of_memory();
        }
        data = ecalloc(
            (*xbuf).size,
            ::core::mem::size_of::<std::ffi::c_uchar>() as std::ffi::c_ulong,
        ) as *mut std::ffi::c_uchar;
        if !((*xbuf).data).is_null() {
            memcpy(
                data as *mut std::ffi::c_void,
                (*xbuf).data as *const std::ffi::c_void,
                (*xbuf).end,
            );
            free((*xbuf).data as *mut std::ffi::c_void);
        }
        (*xbuf).data = data;
    }
    let fresh2 = (*xbuf).end;
    (*xbuf).end = ((*xbuf).end).wrapping_add(1);
    *((*xbuf).data).offset(fresh2 as isize) = b;
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_add_char(mut xbuf: *mut xbuffer, mut c: std::ffi::c_char) {
    xbuf_add_byte(xbuf, c as std::ffi::c_uchar);
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_add_data(
    mut xbuf: *mut xbuffer,
    mut data: *const std::ffi::c_uchar,
    mut len: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as std::ffi::c_int as size_t;
    while i < len {
        xbuf_add_byte(xbuf, *data.offset(i as isize));
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_pop(mut buf: *mut xbuffer) -> std::ffi::c_int {
    if (*buf).end == 0 as std::ffi::c_int as size_t {
        return -(1 as std::ffi::c_int);
    }
    (*buf).end = ((*buf).end).wrapping_sub(1);
    return *((*buf).data).offset((*buf).end as isize) as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_set(mut dst: *mut xbuffer, mut src: *mut xbuffer) {
    xbuf_reset(dst);
    xbuf_add_data(dst, (*src).data, (*src).end);
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_char_data(mut xbuf: *const xbuffer) -> *const std::ffi::c_char {
    return (*xbuf).data as *const std::ffi::c_char;
}
