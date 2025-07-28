use crate::decode::lgetenv_ext;
use crate::decode::Tables;
use crate::util::ptr_to_str;
use crate::xbuf::XBuffer;

#[derive(Copy, Clone)]
pub struct Replace {
    pub fm_idx: usize,
    pub to_idx: usize,
}

/*
 * Code to support expanding environment variables in text.
 */

/*
 * Skip to the next unescaped slash or right curly bracket in a string.
 */
fn skipsl(buf: &[u8], len: usize, mut e: usize) -> usize {
    let mut esc = false;
    while e < len && buf[e] != 0 && (esc || (buf[e] != b'/' && buf[e] != b'}')) {
        esc = !esc && buf[e] == b'\\' && buf[e + 1] != 0;
        e += 1
    }
    e
}

/*
 * Parse a replacement string: one or more instances of
 * (slash, pattern, slash, replacement), followed by right curly bracket.
 * Replacement may be empty in which case the second slash is optional.
 */
fn make_replaces(buf: &mut [u8], len: usize, pe: &mut usize, mut term: u8) -> Vec<Replace> {
    let mut e = *pe;
    let mut replaces = Vec::new();

    while term == b'/' {
        let fm = e;
        e = skipsl(buf, len, e);
        if e >= len {
            break;
        }
        if e == fm {
            // Missing from-string: skip to closing brace if any
            while e < len {
                if buf[e] == b'}' {
                    e += 1;
                    break;
                }
                e += 1;
            }
            break;
        }

        term = buf[e];
        buf[e] = 0; // null-terminate from string
        e += 1;

        let to = if term != b'/' {
            e - 1 // treat as empty "to"
        } else {
            let to_start = e;
            e = skipsl(buf, len, e);
            if e >= len {
                break;
            }
            term = buf[e];
            buf[e] = 0; // null-terminate to string
            e += 1;
            to_start
        };

        replaces.push(Replace {
            fm_idx: fm,
            to_idx: to,
        });
    }

    *pe = e;
    replaces
}

/*
 * See if the initial substring of a string matches a pattern.
 * Backslash escapes in the pattern are ignored.
 * Return the length of the matched substring, or 0 if no match.
 */
fn evar_match(string: &[u8], pat: &[u8]) -> usize {
    let mut len = 0;
    let mut i = 0;
    while pat[i] != 0 {
        if pat[i] == b'\\' {
            i += 1;
        }
        if string[i] != pat[i] {
            return 0;
        }
        i += 1;
        len += 1;
    }
    return len;
}

/*
 * Find the replacement for a string (&evar[*pv]),
 * given a list of replace structs.
 */
unsafe extern "C" fn find_replace(
    replace: &[Replace],
    buf: &[u8],
    evar: &[u8],
    pv: &mut usize,
) -> Option<usize> {
    for repl in replace {
        let len = evar_match(&evar[*pv..], &buf[repl.fm_idx..]);
        if len > 0 {
            *pv += len;
            return Some(repl.to_idx);
        }
    }
    None
}

/*
 * With buf[e] positioned just after NAME in "${NAME" and
 * term containing the character at that point, parse the rest
 * of the environment var string (including the final right curly bracket).
 * Write evar to xbuf, performing any specified text replacements.
 * Return the new value of e to point just after the final right curly bracket.
 */
unsafe extern "C" fn add_evar(
    xbuf: &mut XBuffer,
    buf: &mut [u8],
    len: usize,
    e: usize,
    evar: &[u8],
    term: u8,
) -> usize {
    let mut e = e;
    let replaces = make_replaces(buf, len, &mut e, term);

    let mut v = 0;
    while evar[v] != 0 {
        if let Some(r) = find_replace(&replaces, buf, evar, &mut v) {
            let repl = &buf[r..];
            let mut r = 0;
            while repl[r] != 0 {
                if repl[r] == b'\\' && repl[r + 1] != 0 {
                    r += 1;
                }
                xbuf.xbuf_add_char(repl[r] as i8);
                r += 1;
            }
        } else {
            xbuf.xbuf_add_char(evar[v] as i8);
            v += 1;
        }
        v += 1;
    }
    e
}

/*
 * Expand env variables in a string.
 * Writes expanded output to xbuf. Corrupts buf.
 */
#[no_mangle]
pub unsafe extern "C" fn expand_evars(
    tables: &mut Tables,
    buf: &mut [u8],
    len: usize,
    xbuf: &mut XBuffer,
) {
    let mut i = 0;
    while i < len {
        if i + 1 < len && buf[i] == b'$' && buf[i + 1] == b'{' {
            i += 2; /* skip "${" */
            let mut e = 0;
            while e < len {
                if buf[e] == 0 || buf[e] == b'}' || buf[e] == b'/' {
                    break;
                }
            }
            if e >= len || buf[e] == 0 {
                break; /* missing right curly bracket; ignore var */
            }
            let term = buf[e];
            buf[e] = 0;
            e += 1;
            let mut evar = "";
            let ev = lgetenv_ext(
                tables,
                str::from_utf8_unchecked(&buf[i..]),
                xbuf.data.as_mut_ptr(),
                xbuf.data.len() as u64,
            );
            if ev != 0 as *const std::ffi::c_char {
                evar = ptr_to_str(ev).unwrap();
            }
            i = add_evar(xbuf, buf, len, e, evar.as_bytes(), term);
        } else {
            xbuf.xbuf_add_char(buf[i] as i8);
            i += 1;
        }
    }
}
