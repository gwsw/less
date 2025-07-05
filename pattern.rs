use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    fn free(_: *mut std::ffi::c_void);
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn regcomp(
        __preg: *mut regex_t,
        __pattern: *const std::ffi::c_char,
        __cflags: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn regexec(
        __preg: *const regex_t,
        __String: *const std::ffi::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn regfree(__preg: *mut regex_t);
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
    fn cvt_length(len: size_t, ops: std::ffi::c_int) -> size_t;
    fn cvt_text(
        odst: *mut std::ffi::c_char,
        osrc: *const std::ffi::c_char,
        chpos: *mut std::ffi::c_int,
        lenp: *mut size_t,
        ops: std::ffi::c_int,
    );
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    static mut caseless: std::ffi::c_int;
    static mut is_caseless: std::ffi::c_int;
}
pub type __off_t = std::ffi::c_long;
pub type off_t = __off_t;
pub type size_t = std::ffi::c_ulong;
pub type lbool = std::ffi::c_uint;
pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;
pub type LINENUM = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union parg {
    pub p_string: *const std::ffi::c_char,
    pub p_int: std::ffi::c_int,
    pub p_linenum: LINENUM,
    pub p_char: std::ffi::c_char,
}
pub type PARG = parg;
pub type __re_long_size_t = std::ffi::c_ulong;
pub type reg_syntax_t = std::ffi::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub __buffer: *mut re_dfa_t,
    pub __allocated: __re_long_size_t,
    pub __used: __re_long_size_t,
    pub __syntax: reg_syntax_t,
    pub __fastmap: *mut std::ffi::c_char,
    pub __translate: *mut std::ffi::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "__can_be_null", ty = "std::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "__regs_allocated", ty = "std::ffi::c_uint", bits = "1..=2")]
    #[bitfield(name = "__fastmap_accurate", ty = "std::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "__no_sub", ty = "std::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "__not_bol", ty = "std::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "__not_eol", ty = "std::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "__newline_anchor", ty = "std::ffi::c_uint", bits = "7..=7")]
    pub __can_be_null___regs_allocated___fastmap_accurate___no_sub___not_bol___not_eol___newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = std::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
unsafe extern "C" fn compile_pattern2(
    mut pattern: *const std::ffi::c_char,
    mut search_type: std::ffi::c_int,
    mut comp_pattern: *mut *mut regex_t,
    mut show_error: std::ffi::c_int,
) -> std::ffi::c_int {
    if search_type & (1 as std::ffi::c_int) << 12 as std::ffi::c_int != 0 {
        return 0 as std::ffi::c_int;
    }
    let mut comp: *mut regex_t = ecalloc(
        1 as std::ffi::c_int as size_t,
        ::core::mem::size_of::<regex_t>() as std::ffi::c_ulong,
    ) as *mut regex_t;
    if regcomp(
        comp,
        pattern,
        1 as std::ffi::c_int
            | (if is_caseless != 0 {
                (1 as std::ffi::c_int) << 1 as std::ffi::c_int
            } else {
                0 as std::ffi::c_int
            }),
    ) != 0
    {
        free(comp as *mut std::ffi::c_void);
        if show_error != 0 {
            error(
                b"Invalid pattern\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
        }
        return -(1 as std::ffi::c_int);
    }
    if !(*comp_pattern).is_null() {
        regfree(*comp_pattern);
        free(*comp_pattern as *mut std::ffi::c_void);
    }
    *comp_pattern = comp;
    return 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn compile_pattern(
    mut pattern: *const std::ffi::c_char,
    mut search_type: std::ffi::c_int,
    mut show_error: std::ffi::c_int,
    mut comp_pattern: *mut *mut regex_t,
) -> std::ffi::c_int {
    let mut result: std::ffi::c_int = 0;
    if caseless != 2 as std::ffi::c_int
        || LTRUE as std::ffi::c_int != 0
            && search_type & (1 as std::ffi::c_int) << 12 as std::ffi::c_int == 0
    {
        result = compile_pattern2(pattern, search_type, comp_pattern, show_error);
    } else {
        let mut cvt_pattern: *mut std::ffi::c_char = ecalloc(
            1 as std::ffi::c_int as size_t,
            cvt_length(strlen(pattern), 0o1 as std::ffi::c_int),
        ) as *mut std::ffi::c_char;
        cvt_text(
            cvt_pattern,
            pattern,
            0 as *mut std::ffi::c_int,
            0 as *mut size_t,
            0o1 as std::ffi::c_int,
        );
        result = compile_pattern2(cvt_pattern, search_type, comp_pattern, show_error);
        free(cvt_pattern as *mut std::ffi::c_void);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn uncompile_pattern(mut pattern: *mut *mut regex_t) {
    if !(*pattern).is_null() {
        regfree(*pattern);
        free(*pattern as *mut std::ffi::c_void);
    }
    *pattern = 0 as *mut regex_t;
}
#[no_mangle]
pub unsafe extern "C" fn is_null_pattern(mut pattern: *mut regex_t) -> lbool {
    return (pattern == 0 as *mut std::ffi::c_void as *mut regex_t) as std::ffi::c_int
        as lbool;
}
unsafe extern "C" fn match_0(
    mut pattern: *const std::ffi::c_char,
    mut pattern_len: size_t,
    mut buf: *const std::ffi::c_char,
    mut buf_len: std::ffi::c_int,
    mut sp: *mut *mut *const std::ffi::c_char,
    mut ep: *mut *mut *const std::ffi::c_char,
    mut nsubs: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut pp: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut lp: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut pattern_end: *const std::ffi::c_char = pattern.offset(pattern_len as isize);
    let mut buf_end: *const std::ffi::c_char = buf.offset(buf_len as isize);
    while buf < buf_end {
        pp = pattern;
        lp = buf;
        loop {
            let mut cp: std::ffi::c_char = *pp;
            let mut cl: std::ffi::c_char = *lp;
            if caseless == 2 as std::ffi::c_int
                && (cp as std::ffi::c_int >= 'A' as i32
                    && cp as std::ffi::c_int <= 'Z' as i32)
            {
                cp = (cp as std::ffi::c_int - 'A' as i32 + 'a' as i32)
                    as std::ffi::c_char;
            }
            if cp as std::ffi::c_int != cl as std::ffi::c_int {
                break;
            }
            if pp == pattern_end || lp == buf_end {
                break;
            }
            pp = pp.offset(1);
            pp;
            lp = lp.offset(1);
            lp;
        }
        if pp == pattern_end {
            let fresh0 = *sp;
            *sp = (*sp).offset(1);
            *fresh0 = buf;
            let fresh1 = *ep;
            *ep = (*ep).offset(1);
            *fresh1 = lp;
            return 1 as std::ffi::c_int;
        }
        buf = buf.offset(1);
        buf;
    }
    **ep = 0 as *const std::ffi::c_char;
    **sp = **ep;
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn match_pattern1(
    mut pattern: *mut regex_t,
    mut tpattern: *const std::ffi::c_char,
    mut line: *const std::ffi::c_char,
    mut aline_len: size_t,
    mut line_off: size_t,
    mut sp: *mut *const std::ffi::c_char,
    mut ep: *mut *const std::ffi::c_char,
    mut nsp: std::ffi::c_int,
    mut notbol: std::ffi::c_int,
    mut search_type: std::ffi::c_int,
) -> lbool {
    let mut matched: std::ffi::c_int = 0;
    let mut line_len: std::ffi::c_int = aline_len as std::ffi::c_int;
    if search_type & (1 as std::ffi::c_int) << 12 as std::ffi::c_int != 0 {
        matched = match_0(
            tpattern,
            strlen(tpattern),
            line.offset(line_off as isize),
            (line_len as size_t).wrapping_sub(line_off) as std::ffi::c_int,
            &mut sp,
            &mut ep,
            nsp,
        );
    } else {
        let mut rm: [regmatch_t; 7] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 7];
        let mut flags: std::ffi::c_int = if notbol != 0 {
            1 as std::ffi::c_int
        } else {
            0 as std::ffi::c_int
        };
        flags |= (1 as std::ffi::c_int) << 2 as std::ffi::c_int;
        rm[0 as std::ffi::c_int as usize].rm_so = line_off as regoff_t;
        rm[0 as std::ffi::c_int as usize].rm_eo = line_len;
        matched = (regexec(
            pattern,
            line,
            (16 as std::ffi::c_int - 10 as std::ffi::c_int - 1 as std::ffi::c_int
                + 2 as std::ffi::c_int) as size_t,
            rm.as_mut_ptr(),
            flags,
        ) == 0) as std::ffi::c_int;
        if matched != 0 {
            let mut i: std::ffi::c_int = 0;
            let mut ecount: std::ffi::c_int = 0;
            ecount = 16 as std::ffi::c_int - 10 as std::ffi::c_int - 1 as std::ffi::c_int
                + 2 as std::ffi::c_int;
            while ecount > 0 as std::ffi::c_int {
                if rm[(ecount - 1 as std::ffi::c_int) as usize].rm_so
                    >= 0 as std::ffi::c_int
                {
                    break;
                }
                ecount -= 1;
                ecount;
            }
            if ecount >= nsp {
                ecount = nsp - 1 as std::ffi::c_int;
            }
            i = 0 as std::ffi::c_int;
            while i < ecount {
                if rm[i as usize].rm_so < 0 as std::ffi::c_int {
                    let fresh2 = ep;
                    ep = ep.offset(1);
                    *fresh2 = line;
                    let fresh3 = sp;
                    sp = sp.offset(1);
                    *fresh3 = *fresh2;
                } else {
                    let fresh4 = sp;
                    sp = sp.offset(1);
                    *fresh4 = line.offset(rm[i as usize].rm_so as isize);
                    let fresh5 = ep;
                    ep = ep.offset(1);
                    *fresh5 = line.offset(rm[i as usize].rm_eo as isize);
                }
                i += 1;
                i;
            }
        }
    }
    *ep = 0 as *const std::ffi::c_char;
    *sp = *ep;
    matched = (search_type & (1 as std::ffi::c_int) << 8 as std::ffi::c_int == 0
        && matched != 0
        || search_type & (1 as std::ffi::c_int) << 8 as std::ffi::c_int != 0
            && matched == 0) as std::ffi::c_int;
    return (matched != 0 as std::ffi::c_int) as std::ffi::c_int as lbool;
}
unsafe extern "C" fn subsearch_ok(
    mut sp: *mut *const std::ffi::c_char,
    mut ep: *mut *const std::ffi::c_char,
    mut search_type: std::ffi::c_int,
) -> lbool {
    let mut i: std::ffi::c_int = 0;
    i = 1 as std::ffi::c_int;
    while i <= 16 as std::ffi::c_int - 10 as std::ffi::c_int - 1 as std::ffi::c_int {
        if search_type & (1 as std::ffi::c_int) << 17 as std::ffi::c_int + i != 0
            && *ep.offset(i as isize) == *sp.offset(i as isize)
        {
            return LFALSE;
        }
        i += 1;
        i;
    }
    return LTRUE;
}
#[no_mangle]
pub unsafe extern "C" fn match_pattern(
    mut pattern: *mut regex_t,
    mut tpattern: *const std::ffi::c_char,
    mut line: *const std::ffi::c_char,
    mut line_len: size_t,
    mut line_off: size_t,
    mut sp: *mut *const std::ffi::c_char,
    mut ep: *mut *const std::ffi::c_char,
    mut nsp: std::ffi::c_int,
    mut notbol: std::ffi::c_int,
    mut search_type: std::ffi::c_int,
) -> lbool {
    loop {
        let mut mlen: size_t = 0;
        let mut matched: lbool = match_pattern1(
            pattern,
            tpattern,
            line,
            line_len,
            line_off,
            sp,
            ep,
            nsp,
            notbol,
            search_type,
        );
        if matched as u64 == 0
            || subsearch_ok(sp, ep, search_type) as std::ffi::c_uint != 0
        {
            return matched;
        }
        mlen = (*ep.offset(0 as std::ffi::c_int as isize)).offset_from(line)
            as std::ffi::c_long as size_t;
        if mlen == 0 as std::ffi::c_int as size_t {
            return LFALSE;
        }
        line = line.offset(mlen as isize);
        line_len = line_len.wrapping_sub(mlen);
        notbol = 1 as std::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn pattern_lib_name() -> *const std::ffi::c_char {
    return b"POSIX\0" as *const u8 as *const std::ffi::c_char;
}
