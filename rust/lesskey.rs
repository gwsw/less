use ::c2rust_bitfields;
use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> std::ffi::c_int;
    fn fopen(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn fwrite(
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
        _: std::ffi::c_ulong,
        _: *mut FILE,
    ) -> std::ffi::c_ulong;
    fn perror(__s: *const std::ffi::c_char);
    fn strcpy(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strcat(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strncmp(
        _: *const std::ffi::c_char,
        _: *const std::ffi::c_char,
        _: std::ffi::c_ulong,
    ) -> std::ffi::c_int;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn strtol(
        _: *const std::ffi::c_char,
        _: *mut *mut std::ffi::c_char,
        _: std::ffi::c_int,
    ) -> std::ffi::c_long;
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn exit(_: std::ffi::c_int) -> !;
    fn getenv(__name: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn xbuf_char_data(xbuf: *const xbuffer) -> *const std::ffi::c_char;
    fn parse_lesskey(
        infile_0: *const std::ffi::c_char,
        tables: *mut lesskey_tables,
    ) -> std::ffi::c_int;
    static mut version: [std::ffi::c_char; 0];
}
pub type size_t = std::ffi::c_ulong;
pub type __off_t = std::ffi::c_long;
pub type __off64_t = std::ffi::c_long;
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
pub struct xbuffer {
    pub data: *mut std::ffi::c_uchar,
    pub end: size_t,
    pub size: size_t,
    pub init_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lesskey_cmdname {
    pub cn_name: *const std::ffi::c_char,
    pub cn_action: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lesskey_table {
    pub names: *const lesskey_cmdname,
    pub buf: xbuffer,
    pub is_var: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lesskey_tables {
    pub currtable: *mut lesskey_table,
    pub cmdtable: lesskey_table,
    pub edittable: lesskey_table,
    pub vartable: lesskey_table,
}
#[no_mangle]
pub static mut fileheader: [std::ffi::c_char; 4] = [
    '\0' as i32 as std::ffi::c_char,
    'M' as i32 as std::ffi::c_char,
    '+' as i32 as std::ffi::c_char,
    'G' as i32 as std::ffi::c_char,
];
#[no_mangle]
pub static mut filetrailer: [std::ffi::c_char; 3] = [
    'E' as i32 as std::ffi::c_char,
    'n' as i32 as std::ffi::c_char,
    'd' as i32 as std::ffi::c_char,
];
#[no_mangle]
pub static mut cmdsection: [std::ffi::c_char; 1] = ['c' as i32 as std::ffi::c_char];
#[no_mangle]
pub static mut editsection: [std::ffi::c_char; 1] = ['e' as i32 as std::ffi::c_char];
#[no_mangle]
pub static mut varsection: [std::ffi::c_char; 1] = ['v' as i32 as std::ffi::c_char];
#[no_mangle]
pub static mut endsection: [std::ffi::c_char; 1] = ['x' as i32 as std::ffi::c_char];
#[no_mangle]
pub static mut infile: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
#[no_mangle]
pub static mut outfile: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
unsafe extern "C" fn usage() {
    fprintf(
        stderr,
        b"usage: lesskey [-o output] [input]\n\0" as *const u8 as *const std::ffi::c_char,
    );
    exit(1 as std::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn lesskey_parse_error(mut s: *const std::ffi::c_char) {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const std::ffi::c_char, s);
}
#[no_mangle]
pub unsafe extern "C" fn lstrtoi(
    mut buf: *const std::ffi::c_char,
    mut ebuf: *mut *const std::ffi::c_char,
    mut radix: std::ffi::c_int,
) -> std::ffi::c_int {
    return strtol(buf, ebuf as *mut *mut std::ffi::c_char, radix) as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn out_of_memory() {
    fprintf(
        stderr,
        b"lesskey: cannot allocate memory\n\0" as *const u8 as *const std::ffi::c_char,
    );
    exit(1 as std::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ecalloc(mut count: size_t, mut size: size_t) -> *mut std::ffi::c_void {
    let mut p: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    p = calloc(count, size);
    if p.is_null() {
        out_of_memory();
    }
    return p;
}
unsafe extern "C" fn mkpathname(
    mut dirname: *const std::ffi::c_char,
    mut filename: *const std::ffi::c_char,
) -> *mut std::ffi::c_char {
    let mut pathname: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    pathname = ecalloc(
        (strlen(dirname))
            .wrapping_add(strlen(filename))
            .wrapping_add(2 as std::ffi::c_int as std::ffi::c_ulong),
        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
    ) as *mut std::ffi::c_char;
    strcpy(pathname, dirname);
    strcat(pathname, b"/\0" as *const u8 as *const std::ffi::c_char);
    strcat(pathname, filename);
    return pathname;
}
#[no_mangle]
pub unsafe extern "C" fn homefile(mut filename: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    let mut p: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut pathname: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    p = getenv(b"HOME\0" as *const u8 as *const std::ffi::c_char);
    if !p.is_null() && *p as std::ffi::c_int != '\0' as i32 {
        pathname = mkpathname(p, filename);
    } else {
        fprintf(
            stderr,
            b"cannot find $HOME - using current directory\n\0" as *const u8
                as *const std::ffi::c_char,
        );
        pathname = mkpathname(b".\0" as *const u8 as *const std::ffi::c_char, filename);
    }
    return pathname;
}
unsafe extern "C" fn parse_args(mut argc: std::ffi::c_int, mut argv: *mut *const std::ffi::c_char) {
    let mut arg: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    outfile = 0 as *const std::ffi::c_char;
    loop {
        argc -= 1;
        if !(argc > 0 as std::ffi::c_int) {
            break;
        }
        argv = argv.offset(1);
        arg = *argv;
        if *arg.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != '-' as i32 {
            break;
        } else {
            if *arg.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int == '\0' as i32 {
                break;
            }
            if *arg.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int == '-' as i32
                && *arg.offset(2 as std::ffi::c_int as isize) as std::ffi::c_int == '\0' as i32
            {
                argc -= 1;
                argc;
                argv = argv.offset(1);
                argv;
                break;
            } else {
                let mut current_block_18: u64;
                match *arg.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int {
                    45 => {
                        if strncmp(
                            arg,
                            b"--output\0" as *const u8 as *const std::ffi::c_char,
                            8 as std::ffi::c_int as std::ffi::c_ulong,
                        ) == 0 as std::ffi::c_int
                        {
                            if *arg.offset(8 as std::ffi::c_int as isize) as std::ffi::c_int
                                == '\0' as i32
                            {
                                outfile = &*arg.offset(8 as std::ffi::c_int as isize)
                                    as *const std::ffi::c_char;
                            } else if *arg.offset(8 as std::ffi::c_int as isize) as std::ffi::c_int
                                == '=' as i32
                            {
                                outfile = &*arg.offset(9 as std::ffi::c_int as isize)
                                    as *const std::ffi::c_char;
                            } else {
                                usage();
                            }
                            current_block_18 = 8375071636958269651;
                        } else if strcmp(
                            arg,
                            b"--version\0" as *const u8 as *const std::ffi::c_char,
                        ) == 0 as std::ffi::c_int
                        {
                            current_block_18 = 10137977975373869605;
                        } else {
                            usage();
                            current_block_18 = 9828876828309294594;
                        }
                    }
                    111 => {
                        outfile = &*(*argv.offset(0 as std::ffi::c_int as isize))
                            .offset(2 as std::ffi::c_int as isize)
                            as *const std::ffi::c_char;
                        current_block_18 = 8375071636958269651;
                    }
                    86 => {
                        current_block_18 = 10137977975373869605;
                    }
                    _ => {
                        usage();
                        current_block_18 = 9828876828309294594;
                    }
                }
                match current_block_18 {
                    10137977975373869605 => {
                        printf(
                            b"lesskey  version %s\n\0" as *const u8 as *const std::ffi::c_char,
                            version.as_mut_ptr(),
                        );
                        exit(0 as std::ffi::c_int);
                    }
                    8375071636958269651 => {
                        if *outfile as std::ffi::c_int == '\0' as i32 {
                            argc -= 1;
                            if argc <= 0 as std::ffi::c_int {
                                usage();
                            }
                            argv = argv.offset(1);
                            outfile = *argv;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    if argc > 1 as std::ffi::c_int {
        usage();
    }
    if argc > 0 as std::ffi::c_int {
        infile = *argv;
    }
}
unsafe extern "C" fn fputbytes(
    mut fd: *mut FILE,
    mut buf: *const std::ffi::c_char,
    mut len: size_t,
) {
    loop {
        let fresh0 = len;
        len = len.wrapping_sub(1);
        if !(fresh0 > 0 as std::ffi::c_int as size_t) {
            break;
        }
        fwrite(
            buf as *const std::ffi::c_void,
            ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            1 as std::ffi::c_int as std::ffi::c_ulong,
            fd,
        );
        buf = buf.offset(1);
        buf;
    }
}
unsafe extern "C" fn fputint(mut fd: *mut FILE, mut val: size_t) {
    let mut c1: std::ffi::c_char = 0;
    let mut c2: std::ffi::c_char = 0;
    if val >= (64 as std::ffi::c_int * 64 as std::ffi::c_int) as size_t {
        fprintf(
            stderr,
            b"error: cannot write %ld, max %ld\n\0" as *const u8 as *const std::ffi::c_char,
            val as std::ffi::c_long,
            (64 as std::ffi::c_int * 64 as std::ffi::c_int) as std::ffi::c_long,
        );
        exit(1 as std::ffi::c_int);
    }
    c1 = (val % 64 as std::ffi::c_int as size_t) as std::ffi::c_char;
    val = val / 64 as std::ffi::c_int as size_t;
    c2 = (val % 64 as std::ffi::c_int as size_t) as std::ffi::c_char;
    val = val / 64 as std::ffi::c_int as size_t;
    if val != 0 as std::ffi::c_int as size_t {
        fprintf(
            stderr,
            b"error: %ld exceeds max integer size (%ld)\n\0" as *const u8
                as *const std::ffi::c_char,
            val as std::ffi::c_long,
            (64 as std::ffi::c_int * 64 as std::ffi::c_int) as std::ffi::c_long,
        );
        exit(1 as std::ffi::c_int);
    }
    fwrite(
        &mut c1 as *mut std::ffi::c_char as *const std::ffi::c_void,
        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
        1 as std::ffi::c_int as std::ffi::c_ulong,
        fd,
    );
    fwrite(
        &mut c2 as *mut std::ffi::c_char as *const std::ffi::c_void,
        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
        1 as std::ffi::c_int as std::ffi::c_ulong,
        fd,
    );
}
unsafe fn main_0(
    mut argc: std::ffi::c_int,
    mut argv: *mut *const std::ffi::c_char,
) -> std::ffi::c_int {
    let mut tables: lesskey_tables = lesskey_tables {
        currtable: 0 as *mut lesskey_table,
        cmdtable: lesskey_table {
            names: 0 as *const lesskey_cmdname,
            buf: xbuffer {
                data: 0 as *mut std::ffi::c_uchar,
                end: 0,
                size: 0,
                init_size: 0,
            },
            is_var: 0,
        },
        edittable: lesskey_table {
            names: 0 as *const lesskey_cmdname,
            buf: xbuffer {
                data: 0 as *mut std::ffi::c_uchar,
                end: 0,
                size: 0,
                init_size: 0,
            },
            is_var: 0,
        },
        vartable: lesskey_table {
            names: 0 as *const lesskey_cmdname,
            buf: xbuffer {
                data: 0 as *mut std::ffi::c_uchar,
                end: 0,
                size: 0,
                init_size: 0,
            },
            is_var: 0,
        },
    };
    let mut out: *mut FILE = 0 as *mut FILE;
    let mut errors: std::ffi::c_int = 0;
    parse_args(argc, argv);
    errors = parse_lesskey(infile, &mut tables);
    if errors != 0 {
        fprintf(
            stderr,
            b"%d errors; no output produced\n\0" as *const u8 as *const std::ffi::c_char,
            errors,
        );
        return 1 as std::ffi::c_int;
    }
    fprintf(
        stderr,
        b"NOTE: lesskey is deprecated.\n      It is no longer necessary to run lesskey,\n      when using less version 582 and later.\n\0"
            as *const u8 as *const std::ffi::c_char,
    );
    if outfile.is_null() {
        outfile = getenv(b"LESSKEY\0" as *const u8 as *const std::ffi::c_char);
    }
    if outfile.is_null() {
        outfile = homefile(b".less\0" as *const u8 as *const std::ffi::c_char);
    }
    out = fopen(outfile, b"wb\0" as *const u8 as *const std::ffi::c_char);
    if out.is_null() {
        perror(outfile);
        return 1 as std::ffi::c_int;
    }
    fputbytes(
        out,
        fileheader.as_ptr(),
        ::core::mem::size_of::<[std::ffi::c_char; 4]>() as std::ffi::c_ulong,
    );
    fputbytes(
        out,
        cmdsection.as_ptr(),
        ::core::mem::size_of::<[std::ffi::c_char; 1]>() as std::ffi::c_ulong,
    );
    fputint(out, tables.cmdtable.buf.end);
    fputbytes(
        out,
        xbuf_char_data(&mut tables.cmdtable.buf),
        tables.cmdtable.buf.end,
    );
    fputbytes(
        out,
        editsection.as_ptr(),
        ::core::mem::size_of::<[std::ffi::c_char; 1]>() as std::ffi::c_ulong,
    );
    fputint(out, tables.edittable.buf.end);
    fputbytes(
        out,
        xbuf_char_data(&mut tables.edittable.buf),
        tables.edittable.buf.end,
    );
    fputbytes(
        out,
        varsection.as_ptr(),
        ::core::mem::size_of::<[std::ffi::c_char; 1]>() as std::ffi::c_ulong,
    );
    fputint(out, tables.vartable.buf.end);
    fputbytes(
        out,
        xbuf_char_data(&mut tables.vartable.buf),
        tables.vartable.buf.end,
    );
    fputbytes(
        out,
        endsection.as_ptr(),
        ::core::mem::size_of::<[std::ffi::c_char; 1]>() as std::ffi::c_ulong,
    );
    fputbytes(
        out,
        filetrailer.as_ptr(),
        ::core::mem::size_of::<[std::ffi::c_char; 3]>() as std::ffi::c_ulong,
    );
    fclose(out);
    return 0 as std::ffi::c_int;
}
pub fn main() {
    let mut args: Vec<*mut std::ffi::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as std::ffi::c_int,
            args.as_mut_ptr() as *mut *const std::ffi::c_char,
        ) as i32)
    }
}
