use crate::charset::wchar_range;
use std::sync::LazyLock;

#[rustfmt::skip]
pub static ubin_table: LazyLock<Vec<wchar_range>> = LazyLock::new(|| {
    vec![
        wchar_range { first: char::from_u32(0x0000).unwrap(), last: char::from_u32(0x0007).unwrap() }, /* Cc */
        wchar_range { first: char::from_u32(0x000b).unwrap(), last: char::from_u32(0x000b).unwrap() }, /* Cc */
        wchar_range { first: char::from_u32(0x000e).unwrap(), last: char::from_u32(0x001f).unwrap() }, /* Cc */
        wchar_range { first: char::from_u32(0x007f).unwrap(), last: char::from_u32(0x009f).unwrap() }, /* Cc */
        wchar_range { first: char::from_u32(0x2028).unwrap(), last: char::from_u32(0x2028).unwrap() }, /* Zl */
        wchar_range { first: char::from_u32(0x2029).unwrap(), last: char::from_u32(0x2029).unwrap() }, /* Zp */
        wchar_range { first: char::from_u32(0xd800).unwrap(), last: char::from_u32(0xdfff).unwrap() }, /* Cs */
        wchar_range { first: char::from_u32(0xe000).unwrap(), last: char::from_u32(0xf8ff).unwrap() }, /* Co */
        wchar_range { first: char::from_u32(0xf0000).unwrap(), last: char::from_u32(0xffffd).unwrap() }, /* Co */
        wchar_range { first: char::from_u32(0x100000).unwrap(), last: char::from_u32(0x10fffd).unwrap() }, /* Co */
    ]
});
