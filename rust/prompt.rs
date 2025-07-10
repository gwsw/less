use ::libc;
extern "C" {
    fn free(_: *mut std::ffi::c_void);
    fn strcpy(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn postoa(_: POSITION, _: *mut std::ffi::c_char, _: std::ffi::c_int);
    fn linenumtoa(_: LINENUM, _: *mut std::ffi::c_char, _: std::ffi::c_int);
    fn inttoa(_: std::ffi::c_int, _: *mut std::ffi::c_char, _: std::ffi::c_int);
    fn save(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn ch_length() -> POSITION;
    fn ch_getflags() -> std::ffi::c_int;
    fn prchar(c: LWCHAR) -> *const std::ffi::c_char;
    fn prutfchar(ch: LWCHAR) -> *const std::ffi::c_char;
    fn put_wchar(pp: *mut *mut std::ffi::c_char, ch: LWCHAR);
    fn step_charc(
        pp: *mut *const std::ffi::c_char,
        dir: std::ffi::c_int,
        limit: *const std::ffi::c_char,
    ) -> LWCHAR;
    fn shell_quote(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn last_component(name: *const std::ffi::c_char) -> *const std::ffi::c_char;
    fn eof_displayed(offset: lbool) -> lbool;
    fn next_ifile(h: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn nifile() -> std::ffi::c_int;
    fn get_filename(ifile: *mut std::ffi::c_void) -> *const std::ffi::c_char;
    fn get_index(ifile: *mut std::ffi::c_void) -> std::ffi::c_int;
    fn find_linenum(pos: POSITION) -> LINENUM;
    fn currline(where_0: std::ffi::c_int) -> LINENUM;
    fn vlinenum(linenum: LINENUM) -> LINENUM;
    fn percentage(num: POSITION, den: POSITION) -> std::ffi::c_int;
    fn position(sindex: std::ffi::c_int) -> POSITION;
    fn sindex_from_sline(sline: std::ffi::c_int) -> std::ffi::c_int;
    fn ntags() -> std::ffi::c_int;
    fn curr_tag() -> std::ffi::c_int;
    static mut pr_type: std::ffi::c_int;
    static mut new_file: lbool;
    static mut linenums: std::ffi::c_int;
    static mut hshift: std::ffi::c_int;
    static mut sc_height: std::ffi::c_int;
    static mut jump_sline: std::ffi::c_int;
    static mut less_is_more: std::ffi::c_int;
    static mut header_lines: std::ffi::c_int;
    static mut utf_mode: std::ffi::c_int;
    static mut curr_ifile: *mut std::ffi::c_void;
    static mut osc8_path: *mut std::ffi::c_char;
    static mut editor: *const std::ffi::c_char;
}
pub type __off_t = std::ffi::c_long;
pub type off_t = __off_t;
pub type size_t = std::ffi::c_ulong;
pub type lbool = std::ffi::c_uint;
pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;
pub type LWCHAR = std::ffi::c_ulong;
pub type less_off_t = off_t;
pub type POSITION = less_off_t;
pub type LINENUM = off_t;
static mut s_proto: [std::ffi::c_char; 52] = unsafe {
    *::core::mem::transmute::<&[u8; 52], &[std::ffi::c_char; 52]>(
        b"?n?f%f .?m(%T %i of %m) ..?e(END) ?x- Next\\: %x..%t\0",
    )
};
static mut m_proto: [std::ffi::c_char; 77] = unsafe {
    *::core::mem::transmute::<&[u8; 77], &[std::ffi::c_char; 77]>(
        b"?n?f%f .?m(%T %i of %m) ..?e(END) ?x- Next\\: %x.:?pB%pB\\%:byte %bB?s/%s...%t\0",
    )
};
static mut M_proto: [std::ffi::c_char; 102] = unsafe {
    *::core::mem::transmute::<
        &[u8; 102],
        &[std::ffi::c_char; 102],
    >(
        b"?f%f .?n?m(%T %i of %m) ..?ltlines %lt-%lb?L/%L. :byte %bB?s/%s. .?e(END) ?x- Next\\: %x.:?pB%pB\\%..%t\0",
    )
};
static mut e_proto: [std::ffi::c_char; 84] = unsafe {
    *::core::mem::transmute::<&[u8; 84], &[std::ffi::c_char; 84]>(
        b"?f%f .?m(%T %i of %m) .?ltlines %lt-%lb?L/%L. .byte %bB?s/%s. ?e(END) :?pB%pB\\%..%t\0",
    )
};
static mut h_proto: [std::ffi::c_char; 80] = unsafe {
    *::core::mem::transmute::<&[u8; 80], &[std::ffi::c_char; 80]>(
        b"HELP -- ?eEND -- Press g to see it again:Press RETURN for more., or q when done\0",
    )
};
static mut w_proto: [std::ffi::c_char; 17] =
    unsafe { *::core::mem::transmute::<&[u8; 17], &[std::ffi::c_char; 17]>(b"Waiting for data\0") };
static mut more_proto: [std::ffi::c_char; 59] = unsafe {
    *::core::mem::transmute::<&[u8; 59], &[std::ffi::c_char; 59]>(
        b"--More--(?eEND ?x- Next\\: %x.:?pB%pB\\%:byte %bB?s/%s...%t)\0",
    )
};
#[no_mangle]
pub static mut prproto: [*mut std::ffi::c_char; 3] =
    [0 as *const std::ffi::c_char as *mut std::ffi::c_char; 3];
#[no_mangle]
pub static mut eqproto: *const std::ffi::c_char = unsafe { e_proto.as_ptr() };
#[no_mangle]
pub static mut hproto: *const std::ffi::c_char = unsafe { h_proto.as_ptr() };
#[no_mangle]
pub static mut wproto: *const std::ffi::c_char = unsafe { w_proto.as_ptr() };
static mut message: [std::ffi::c_char; 2048] = [0; 2048];
static mut mp: *mut std::ffi::c_char = 0 as *const std::ffi::c_char as *mut std::ffi::c_char;
#[no_mangle]
pub unsafe extern "C" fn init_prompt() {
    prproto[0 as std::ffi::c_int as usize] = save(s_proto.as_ptr());
    prproto[1 as std::ffi::c_int as usize] = save(if less_is_more != 0 {
        more_proto.as_ptr()
    } else {
        m_proto.as_ptr()
    });
    prproto[2 as std::ffi::c_int as usize] = save(M_proto.as_ptr());
    eqproto = save(e_proto.as_ptr());
    hproto = save(h_proto.as_ptr());
    wproto = save(w_proto.as_ptr());
}
unsafe extern "C" fn ap_estr(mut s: *const std::ffi::c_char, mut nprt: lbool) {
    let mut es: *const std::ffi::c_char = s.offset(strlen(s) as isize);
    while *s as std::ffi::c_int != '\0' as i32 {
        let mut ch: LWCHAR = step_charc(&mut s, 1 as std::ffi::c_int, es);
        let mut ps: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
        let mut ubuf: [std::ffi::c_char; 7] = [0; 7];
        let mut plen: size_t = 0;
        if nprt as u64 != 0 {
            ps = if utf_mode != 0 {
                prutfchar(ch)
            } else {
                prchar(ch)
            };
        } else {
            let mut up: *mut std::ffi::c_char = ubuf.as_mut_ptr();
            put_wchar(&mut up, ch);
            *up = '\0' as i32 as std::ffi::c_char;
            ps = ubuf.as_mut_ptr();
        }
        plen = strlen(ps);
        if mp.offset(plen as isize)
            >= message
                .as_mut_ptr()
                .offset(2048 as std::ffi::c_int as isize)
        {
            break;
        }
        strcpy(mp, ps);
        mp = mp.offset(plen as isize);
    }
    *mp = '\0' as i32 as std::ffi::c_char;
}
unsafe extern "C" fn ap_str(mut s: *const std::ffi::c_char) {
    ap_estr(s, LFALSE);
}
unsafe extern "C" fn ap_char(mut c: std::ffi::c_char) {
    if mp.offset(1 as std::ffi::c_int as isize)
        >= message
            .as_mut_ptr()
            .offset(2048 as std::ffi::c_int as isize)
    {
        return;
    }
    let fresh0 = mp;
    mp = mp.offset(1);
    *fresh0 = c;
    *mp = '\0' as i32 as std::ffi::c_char;
}
unsafe extern "C" fn ap_pos(mut pos: POSITION) {
    let mut buf: [std::ffi::c_char; 23] = [0; 23];
    postoa(pos, buf.as_mut_ptr(), 10 as std::ffi::c_int);
    ap_str(buf.as_mut_ptr());
}
unsafe extern "C" fn ap_linenum(mut linenum: LINENUM) {
    let mut buf: [std::ffi::c_char; 23] = [0; 23];
    linenumtoa(linenum, buf.as_mut_ptr(), 10 as std::ffi::c_int);
    ap_str(buf.as_mut_ptr());
}
unsafe extern "C" fn ap_int(mut num: std::ffi::c_int) {
    let mut buf: [std::ffi::c_char; 13] = [0; 13];
    inttoa(num, buf.as_mut_ptr(), 10 as std::ffi::c_int);
    ap_str(buf.as_mut_ptr());
}
unsafe extern "C" fn ap_quest() {
    ap_str(b"?\0" as *const u8 as *const std::ffi::c_char);
}
unsafe extern "C" fn curr_byte(mut where_0: std::ffi::c_int) -> POSITION {
    let mut pos: POSITION = 0;
    pos = position(where_0);
    while pos == -(1 as std::ffi::c_int) as POSITION
        && where_0 >= 0 as std::ffi::c_int
        && where_0 < sc_height - 1 as std::ffi::c_int
    {
        where_0 += 1;
        pos = position(where_0);
    }
    if pos == -(1 as std::ffi::c_int) as POSITION {
        pos = ch_length();
    }
    return pos;
}
unsafe extern "C" fn cond(mut c: std::ffi::c_char, mut where_0: std::ffi::c_int) -> lbool {
    let mut len: POSITION = 0;
    match c as std::ffi::c_int {
        97 => return (mp > message.as_mut_ptr()) as std::ffi::c_int as lbool,
        98 => {
            return (curr_byte(where_0) != -(1 as std::ffi::c_int) as POSITION) as std::ffi::c_int
                as lbool;
        }
        99 => return (hshift != 0 as std::ffi::c_int) as std::ffi::c_int as lbool,
        101 => return eof_displayed(LFALSE),
        102 | 103 => {
            return (strcmp(
                get_filename(curr_ifile),
                b"-\0" as *const u8 as *const std::ffi::c_char,
            ) != 0 as std::ffi::c_int) as std::ffi::c_int as lbool;
        }
        108 | 100 => {
            if linenums == 0 {
                return LFALSE;
            }
            return (currline(where_0) != 0 as std::ffi::c_int as LINENUM) as std::ffi::c_int
                as lbool;
        }
        76 | 68 => {
            return (linenums != 0 && ch_length() != -(1 as std::ffi::c_int) as POSITION)
                as std::ffi::c_int as lbool;
        }
        109 => {
            return (if ntags() != 0 {
                (ntags() > 1 as std::ffi::c_int) as std::ffi::c_int
            } else {
                (nifile() > 1 as std::ffi::c_int) as std::ffi::c_int
            }) as lbool;
        }
        110 => {
            return (if ntags() != 0 {
                LTRUE as std::ffi::c_int
            } else if new_file as std::ffi::c_uint != 0 {
                LTRUE as std::ffi::c_int
            } else {
                LFALSE as std::ffi::c_int
            }) as lbool;
        }
        112 => {
            return (curr_byte(where_0) != -(1 as std::ffi::c_int) as POSITION
                && ch_length() > 0 as std::ffi::c_int as POSITION)
                as std::ffi::c_int as lbool;
        }
        80 => {
            return (currline(where_0) != 0 as std::ffi::c_int as LINENUM
                && {
                    len = ch_length();
                    len > 0 as std::ffi::c_int as POSITION
                }
                && find_linenum(len) != 0 as std::ffi::c_int as LINENUM)
                as std::ffi::c_int as lbool;
        }
        115 | 66 => {
            return (ch_length() != -(1 as std::ffi::c_int) as POSITION) as std::ffi::c_int
                as lbool;
        }
        120 => {
            if ntags() != 0 {
                return LFALSE;
            }
            return (next_ifile(curr_ifile) != 0 as *mut std::ffi::c_void) as std::ffi::c_int
                as lbool;
        }
        _ => {}
    }
    return LFALSE;
}
unsafe extern "C" fn protochar(mut c: std::ffi::c_char, mut where_0: std::ffi::c_int) {
    let mut pos: POSITION = 0;
    let mut len: POSITION = 0;
    let mut linenum: LINENUM = 0;
    let mut last_linenum: LINENUM = 0;
    let mut h: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    let mut s: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    match c as std::ffi::c_int {
        98 => {
            pos = curr_byte(where_0);
            if pos != -(1 as std::ffi::c_int) as POSITION {
                ap_pos(pos);
            } else {
                ap_quest();
            }
        }
        99 => {
            ap_int(hshift);
        }
        100 => {
            linenum = currline(where_0);
            if linenum > 0 as std::ffi::c_int as LINENUM
                && sc_height > header_lines + 1 as std::ffi::c_int
            {
                ap_linenum(
                    (linenum - 1 as std::ffi::c_int as LINENUM)
                        / (sc_height - header_lines - 1 as std::ffi::c_int) as LINENUM
                        + 1 as std::ffi::c_int as LINENUM,
                );
            } else {
                ap_quest();
            }
        }
        68 => {
            len = ch_length();
            if len == -(1 as std::ffi::c_int) as POSITION {
                ap_quest();
            } else if len == 0 as std::ffi::c_int as POSITION {
                ap_linenum(0 as std::ffi::c_int as LINENUM);
            } else {
                linenum = find_linenum(len - 1 as std::ffi::c_int as POSITION);
                if linenum <= 0 as std::ffi::c_int as LINENUM {
                    ap_quest();
                } else {
                    ap_linenum(
                        (linenum - 1 as std::ffi::c_int as LINENUM)
                            / (sc_height - header_lines - 1 as std::ffi::c_int) as LINENUM
                            + 1 as std::ffi::c_int as LINENUM,
                    );
                }
            }
        }
        69 => {
            ap_str(editor);
        }
        102 => {
            ap_estr(get_filename(curr_ifile), LTRUE);
        }
        70 => {
            ap_estr(last_component(get_filename(curr_ifile)), LTRUE);
        }
        103 => {
            s = shell_quote(get_filename(curr_ifile));
            ap_str(s);
            free(s as *mut std::ffi::c_void);
        }
        105 => {
            if ntags() != 0 {
                ap_int(curr_tag());
            } else {
                ap_int(get_index(curr_ifile));
            }
        }
        108 => {
            linenum = currline(where_0);
            if linenum != 0 as std::ffi::c_int as LINENUM {
                ap_linenum(vlinenum(linenum));
            } else {
                ap_quest();
            }
        }
        76 => {
            len = ch_length();
            if len == -(1 as std::ffi::c_int) as POSITION
                || len == 0 as std::ffi::c_int as POSITION
                || {
                    linenum = find_linenum(len);
                    linenum <= 0 as std::ffi::c_int as LINENUM
                }
            {
                ap_quest();
            } else {
                ap_linenum(vlinenum(linenum - 1 as std::ffi::c_int as LINENUM));
            }
        }
        109 => {
            let mut n: std::ffi::c_int = ntags();
            if n != 0 {
                ap_int(n);
            } else {
                ap_int(nifile());
            }
        }
        111 => {
            if !osc8_path.is_null() {
                ap_str(osc8_path);
            } else {
                ap_quest();
            }
        }
        112 => {
            pos = curr_byte(where_0);
            len = ch_length();
            if pos != -(1 as std::ffi::c_int) as POSITION && len > 0 as std::ffi::c_int as POSITION
            {
                ap_int(percentage(pos, len));
            } else {
                ap_quest();
            }
        }
        80 => {
            linenum = currline(where_0);
            if linenum == 0 as std::ffi::c_int as LINENUM
                || {
                    len = ch_length();
                    len == -(1 as std::ffi::c_int) as POSITION
                }
                || len == 0 as std::ffi::c_int as POSITION
                || {
                    last_linenum = find_linenum(len);
                    last_linenum <= 0 as std::ffi::c_int as LINENUM
                }
            {
                ap_quest();
            } else {
                ap_int(percentage(linenum, last_linenum));
            }
        }
        115 | 66 => {
            len = ch_length();
            if len != -(1 as std::ffi::c_int) as POSITION {
                ap_pos(len);
            } else {
                ap_quest();
            }
        }
        116 => {
            while mp > message.as_mut_ptr()
                && *mp.offset(-(1 as std::ffi::c_int) as isize) as std::ffi::c_int == ' ' as i32
            {
                mp = mp.offset(-1);
            }
            *mp = '\0' as i32 as std::ffi::c_char;
        }
        84 => {
            if ntags() != 0 {
                ap_str(b"tag\0" as *const u8 as *const std::ffi::c_char);
            } else {
                ap_str(b"file\0" as *const u8 as *const std::ffi::c_char);
            }
        }
        120 => {
            h = next_ifile(curr_ifile);
            if h != 0 as *mut std::ffi::c_void {
                ap_str(get_filename(h));
            } else {
                ap_quest();
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn skipcond(mut p: *const std::ffi::c_char) -> *const std::ffi::c_char {
    let mut iflevel: std::ffi::c_int = 0;
    iflevel = 1 as std::ffi::c_int;
    loop {
        p = p.offset(1);
        match *p as std::ffi::c_int {
            63 => {
                iflevel += 1;
            }
            58 => {
                if iflevel == 1 as std::ffi::c_int {
                    return p;
                }
            }
            46 => {
                iflevel -= 1;
                if iflevel == 0 as std::ffi::c_int {
                    return p;
                }
            }
            92 => {
                if *p.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int != '\0' as i32 {
                    p = p.offset(1);
                }
            }
            0 => return p.offset(-(1 as std::ffi::c_int as isize)),
            _ => {}
        }
    }
}
unsafe extern "C" fn wherechar(
    mut p: *const std::ffi::c_char,
    mut wp: *mut std::ffi::c_int,
) -> *const std::ffi::c_char {
    match *p as std::ffi::c_int {
        98 | 100 | 108 | 112 | 80 => {
            p = p.offset(1);
            match *p as std::ffi::c_int {
                116 => {
                    *wp = 0 as std::ffi::c_int;
                }
                109 => {
                    *wp = -(3 as std::ffi::c_int);
                }
                98 => {
                    *wp = -(1 as std::ffi::c_int);
                }
                66 => {
                    *wp = -(2 as std::ffi::c_int);
                }
                106 => {
                    *wp = sindex_from_sline(jump_sline);
                }
                _ => {
                    *wp = 0 as std::ffi::c_int;
                    p = p.offset(-1);
                }
            }
        }
        _ => {}
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn pr_expand(mut proto: *const std::ffi::c_char) -> *const std::ffi::c_char {
    let mut p: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut c: std::ffi::c_char = 0;
    let mut where_0: std::ffi::c_int = 0;
    mp = message.as_mut_ptr();
    if *proto as std::ffi::c_int == '\0' as i32 {
        return b"\0" as *const u8 as *const std::ffi::c_char;
    }
    p = proto;
    while *p as std::ffi::c_int != '\0' as i32 {
        match *p as std::ffi::c_int {
            92 => {
                if *p.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int != '\0' as i32 {
                    p = p.offset(1);
                    ap_char(*p);
                }
            }
            63 => {
                p = p.offset(1);
                c = *p;
                if c as std::ffi::c_int == '\0' as i32 {
                    p = p.offset(-1);
                } else {
                    where_0 = 0 as std::ffi::c_int;
                    p = wherechar(p, &mut where_0);
                    if cond(c, where_0) as u64 == 0 {
                        p = skipcond(p);
                    }
                }
            }
            58 => {
                p = skipcond(p);
            }
            46 => {}
            37 => {
                p = p.offset(1);
                c = *p;
                if c as std::ffi::c_int == '\0' as i32 {
                    p = p.offset(-1);
                } else {
                    where_0 = 0 as std::ffi::c_int;
                    p = wherechar(p, &mut where_0);
                    protochar(c, where_0);
                }
            }
            _ => {
                ap_char(*p);
            }
        }
        p = p.offset(1);
    }
    if mp == message.as_mut_ptr() {
        return b"\0" as *const u8 as *const std::ffi::c_char;
    }
    return message.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn eq_message() -> *const std::ffi::c_char {
    return pr_expand(eqproto);
}
#[no_mangle]
pub unsafe extern "C" fn pr_string() -> *const std::ffi::c_char {
    let mut prompt: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut type_0: std::ffi::c_int = 0;
    type_0 = if less_is_more == 0 {
        pr_type
    } else if pr_type != 0 {
        0 as std::ffi::c_int
    } else {
        1 as std::ffi::c_int
    };
    prompt = pr_expand(if ch_getflags() & 0o10 as std::ffi::c_int != 0 {
        hproto
    } else {
        prproto[type_0 as usize] as *const std::ffi::c_char
    });
    new_file = LFALSE;
    return prompt;
}
#[no_mangle]
pub unsafe extern "C" fn wait_message() -> *const std::ffi::c_char {
    return pr_expand(wproto);
}
