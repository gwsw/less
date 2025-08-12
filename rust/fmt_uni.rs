use crate::charset::wchar_range;
use std::sync::LazyLock;

#[rustfmt::skip]
pub static fmt_table: LazyLock<Vec<wchar_range>> = LazyLock::new(|| {
    vec![
        wchar_range { first: char::from_u32(0x00ad).unwrap(), last: char::from_u32(0x00ad).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0x0600).unwrap(), last: char::from_u32(0x0605).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0x061c).unwrap(), last: char::from_u32(0x061c).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0x06dd).unwrap(), last: char::from_u32(0x06dd).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0x070f).unwrap(), last: char::from_u32(0x070f).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0x0890).unwrap(), last: char::from_u32(0x0891).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0x08e2).unwrap(), last: char::from_u32(0x08e2).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0x180e).unwrap(), last: char::from_u32(0x180e).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0x200b).unwrap(), last: char::from_u32(0x200f).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0x202a).unwrap(), last: char::from_u32(0x202e).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0x2060).unwrap(), last: char::from_u32(0x2064).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0x2066).unwrap(), last: char::from_u32(0x206f).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0xfeff).unwrap(), last: char::from_u32(0xfeff).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0xfff9).unwrap(), last: char::from_u32(0xfffb).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0x110bd).unwrap(), last: char::from_u32(0x110bd).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0x110cd).unwrap(), last: char::from_u32(0x110cd).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0x13430).unwrap(), last: char::from_u32(0x1343f).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0x1bca0).unwrap(), last: char::from_u32(0x1bca3).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0x1d173).unwrap(), last: char::from_u32(0x1d17a).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0xe0001).unwrap(), last: char::from_u32(0xe0001).unwrap() }, /* Cf */
        wchar_range { first: char::from_u32(0xe0020).unwrap(), last: char::from_u32(0xe007f).unwrap() }, /* Cf */
        ]
});
