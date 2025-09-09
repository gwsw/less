use crate::compose_uni::compose_table;
use crate::decode::lgetenv;
use crate::fmt_uni::fmt_table;
use crate::ubin_uni::ubin_table;
use crate::wide_uni::wide_table;
use crate::xbuf::XBuffer;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt::Arguments;
use std::sync::LazyLock;

type wchar_range_table = Vec<wchar_range>;

extern "C" {
    fn snprintf(
        _: *mut std::ffi::c_char,
        _: std::ffi::c_ulong,
        _: *const std::ffi::c_char,
        _: ...
    ) -> std::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const std::ffi::c_ushort;
    fn strcpy(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strchr(_: *const std::ffi::c_char, _: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn strstr(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn lstrtoulc(
        _: *const std::ffi::c_char,
        _: *mut *const std::ffi::c_char,
        _: std::ffi::c_int,
    ) -> std::ffi::c_ulong;
    fn xbuf_init(xbuf: *mut xbuffer);
    fn xbuf_add_data(xbuf: *mut xbuffer, data: *const std::ffi::c_uchar, len: size_t);
    fn quit(status: std::ffi::c_int);
    fn isnullenv(s: *const std::ffi::c_char) -> lbool;
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn setlocale(
        __category: std::ffi::c_int,
        __locale: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_char;
    fn nl_langinfo(__item: nl_item) -> *mut std::ffi::c_char;
    static mut bs_mode: std::ffi::c_int;
}
pub type __off_t = std::ffi::c_long;
pub type off_t = __off_t;
pub type size_t = std::ffi::c_ulong;
pub type C2RustUnnamed = std::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type lbool = std::ffi::c_uint;
pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;
pub type LWCHAR = std::ffi::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wchar_range {
    pub first: char,
    pub last: char,
}

impl wchar_range {
    fn as_bytes(&self) -> Vec<u8> {
        [
            u32::to_ne_bytes(self.first as u32),
            u32::to_ne_bytes(self.last as u32),
        ]
        .concat()
    }
}

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
pub struct charset {
    pub name: &'static str,
    pub p_flag: Option<bool>,
    pub desc: &'static str,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cs_alias {
    pub name: &'static str,
    pub oname: &'static str,
}
pub const CODESET: C2RustUnnamed_0 = 14;
pub type nl_item = std::ffi::c_int;
pub type C2RustUnnamed_0 = std::ffi::c_uint;
pub const _NL_NUM: C2RustUnnamed_0 = 786449;
pub const _NL_NUM_LC_IDENTIFICATION: C2RustUnnamed_0 = 786448;
pub const _NL_IDENTIFICATION_CODESET: C2RustUnnamed_0 = 786447;
pub const _NL_IDENTIFICATION_CATEGORY: C2RustUnnamed_0 = 786446;
pub const _NL_IDENTIFICATION_DATE: C2RustUnnamed_0 = 786445;
pub const _NL_IDENTIFICATION_REVISION: C2RustUnnamed_0 = 786444;
pub const _NL_IDENTIFICATION_ABBREVIATION: C2RustUnnamed_0 = 786443;
pub const _NL_IDENTIFICATION_APPLICATION: C2RustUnnamed_0 = 786442;
pub const _NL_IDENTIFICATION_AUDIENCE: C2RustUnnamed_0 = 786441;
pub const _NL_IDENTIFICATION_TERRITORY: C2RustUnnamed_0 = 786440;
pub const _NL_IDENTIFICATION_LANGUAGE: C2RustUnnamed_0 = 786439;
pub const _NL_IDENTIFICATION_FAX: C2RustUnnamed_0 = 786438;
pub const _NL_IDENTIFICATION_TEL: C2RustUnnamed_0 = 786437;
pub const _NL_IDENTIFICATION_EMAIL: C2RustUnnamed_0 = 786436;
pub const _NL_IDENTIFICATION_CONTACT: C2RustUnnamed_0 = 786435;
pub const _NL_IDENTIFICATION_ADDRESS: C2RustUnnamed_0 = 786434;
pub const _NL_IDENTIFICATION_SOURCE: C2RustUnnamed_0 = 786433;
pub const _NL_IDENTIFICATION_TITLE: C2RustUnnamed_0 = 786432;
pub const _NL_NUM_LC_MEASUREMENT: C2RustUnnamed_0 = 720898;
pub const _NL_MEASUREMENT_CODESET: C2RustUnnamed_0 = 720897;
pub const _NL_MEASUREMENT_MEASUREMENT: C2RustUnnamed_0 = 720896;
pub const _NL_NUM_LC_TELEPHONE: C2RustUnnamed_0 = 655365;
pub const _NL_TELEPHONE_CODESET: C2RustUnnamed_0 = 655364;
pub const _NL_TELEPHONE_INT_PREFIX: C2RustUnnamed_0 = 655363;
pub const _NL_TELEPHONE_INT_SELECT: C2RustUnnamed_0 = 655362;
pub const _NL_TELEPHONE_TEL_DOM_FMT: C2RustUnnamed_0 = 655361;
pub const _NL_TELEPHONE_TEL_INT_FMT: C2RustUnnamed_0 = 655360;
pub const _NL_NUM_LC_ADDRESS: C2RustUnnamed_0 = 589837;
pub const _NL_ADDRESS_CODESET: C2RustUnnamed_0 = 589836;
pub const _NL_ADDRESS_LANG_LIB: C2RustUnnamed_0 = 589835;
pub const _NL_ADDRESS_LANG_TERM: C2RustUnnamed_0 = 589834;
pub const _NL_ADDRESS_LANG_AB: C2RustUnnamed_0 = 589833;
pub const _NL_ADDRESS_LANG_NAME: C2RustUnnamed_0 = 589832;
pub const _NL_ADDRESS_COUNTRY_ISBN: C2RustUnnamed_0 = 589831;
pub const _NL_ADDRESS_COUNTRY_NUM: C2RustUnnamed_0 = 589830;
pub const _NL_ADDRESS_COUNTRY_CAR: C2RustUnnamed_0 = 589829;
pub const _NL_ADDRESS_COUNTRY_AB3: C2RustUnnamed_0 = 589828;
pub const _NL_ADDRESS_COUNTRY_AB2: C2RustUnnamed_0 = 589827;
pub const _NL_ADDRESS_COUNTRY_POST: C2RustUnnamed_0 = 589826;
pub const _NL_ADDRESS_COUNTRY_NAME: C2RustUnnamed_0 = 589825;
pub const _NL_ADDRESS_POSTAL_FMT: C2RustUnnamed_0 = 589824;
pub const _NL_NUM_LC_NAME: C2RustUnnamed_0 = 524295;
pub const _NL_NAME_CODESET: C2RustUnnamed_0 = 524294;
pub const _NL_NAME_NAME_MS: C2RustUnnamed_0 = 524293;
pub const _NL_NAME_NAME_MISS: C2RustUnnamed_0 = 524292;
pub const _NL_NAME_NAME_MRS: C2RustUnnamed_0 = 524291;
pub const _NL_NAME_NAME_MR: C2RustUnnamed_0 = 524290;
pub const _NL_NAME_NAME_GEN: C2RustUnnamed_0 = 524289;
pub const _NL_NAME_NAME_FMT: C2RustUnnamed_0 = 524288;
pub const _NL_NUM_LC_PAPER: C2RustUnnamed_0 = 458755;
pub const _NL_PAPER_CODESET: C2RustUnnamed_0 = 458754;
pub const _NL_PAPER_WIDTH: C2RustUnnamed_0 = 458753;
pub const _NL_PAPER_HEIGHT: C2RustUnnamed_0 = 458752;
pub const _NL_NUM_LC_MESSAGES: C2RustUnnamed_0 = 327685;
pub const _NL_MESSAGES_CODESET: C2RustUnnamed_0 = 327684;
pub const __NOSTR: C2RustUnnamed_0 = 327683;
pub const __YESSTR: C2RustUnnamed_0 = 327682;
pub const __NOEXPR: C2RustUnnamed_0 = 327681;
pub const __YESEXPR: C2RustUnnamed_0 = 327680;
pub const _NL_NUM_LC_NUMERIC: C2RustUnnamed_0 = 65542;
pub const _NL_NUMERIC_CODESET: C2RustUnnamed_0 = 65541;
pub const _NL_NUMERIC_THOUSANDS_SEP_WC: C2RustUnnamed_0 = 65540;
pub const _NL_NUMERIC_DECIMAL_POINT_WC: C2RustUnnamed_0 = 65539;
pub const __GROUPING: C2RustUnnamed_0 = 65538;
pub const THOUSEP: C2RustUnnamed_0 = 65537;
pub const __THOUSANDS_SEP: C2RustUnnamed_0 = 65537;
pub const RADIXCHAR: C2RustUnnamed_0 = 65536;
pub const __DECIMAL_POINT: C2RustUnnamed_0 = 65536;
pub const _NL_NUM_LC_MONETARY: C2RustUnnamed_0 = 262190;
pub const _NL_MONETARY_CODESET: C2RustUnnamed_0 = 262189;
pub const _NL_MONETARY_THOUSANDS_SEP_WC: C2RustUnnamed_0 = 262188;
pub const _NL_MONETARY_DECIMAL_POINT_WC: C2RustUnnamed_0 = 262187;
pub const _NL_MONETARY_CONVERSION_RATE: C2RustUnnamed_0 = 262186;
pub const _NL_MONETARY_DUO_VALID_TO: C2RustUnnamed_0 = 262185;
pub const _NL_MONETARY_DUO_VALID_FROM: C2RustUnnamed_0 = 262184;
pub const _NL_MONETARY_UNO_VALID_TO: C2RustUnnamed_0 = 262183;
pub const _NL_MONETARY_UNO_VALID_FROM: C2RustUnnamed_0 = 262182;
pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: C2RustUnnamed_0 = 262181;
pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: C2RustUnnamed_0 = 262180;
pub const _NL_MONETARY_DUO_N_SIGN_POSN: C2RustUnnamed_0 = 262179;
pub const _NL_MONETARY_DUO_P_SIGN_POSN: C2RustUnnamed_0 = 262178;
pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: C2RustUnnamed_0 = 262177;
pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: C2RustUnnamed_0 = 262176;
pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: C2RustUnnamed_0 = 262175;
pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: C2RustUnnamed_0 = 262174;
pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: C2RustUnnamed_0 = 262173;
pub const _NL_MONETARY_DUO_N_CS_PRECEDES: C2RustUnnamed_0 = 262172;
pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: C2RustUnnamed_0 = 262171;
pub const _NL_MONETARY_DUO_P_CS_PRECEDES: C2RustUnnamed_0 = 262170;
pub const _NL_MONETARY_DUO_FRAC_DIGITS: C2RustUnnamed_0 = 262169;
pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: C2RustUnnamed_0 = 262168;
pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: C2RustUnnamed_0 = 262167;
pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: C2RustUnnamed_0 = 262166;
pub const __INT_N_SIGN_POSN: C2RustUnnamed_0 = 262165;
pub const __INT_P_SIGN_POSN: C2RustUnnamed_0 = 262164;
pub const __INT_N_SEP_BY_SPACE: C2RustUnnamed_0 = 262163;
pub const __INT_N_CS_PRECEDES: C2RustUnnamed_0 = 262162;
pub const __INT_P_SEP_BY_SPACE: C2RustUnnamed_0 = 262161;
pub const __INT_P_CS_PRECEDES: C2RustUnnamed_0 = 262160;
pub const _NL_MONETARY_CRNCYSTR: C2RustUnnamed_0 = 262159;
pub const __N_SIGN_POSN: C2RustUnnamed_0 = 262158;
pub const __P_SIGN_POSN: C2RustUnnamed_0 = 262157;
pub const __N_SEP_BY_SPACE: C2RustUnnamed_0 = 262156;
pub const __N_CS_PRECEDES: C2RustUnnamed_0 = 262155;
pub const __P_SEP_BY_SPACE: C2RustUnnamed_0 = 262154;
pub const __P_CS_PRECEDES: C2RustUnnamed_0 = 262153;
pub const __FRAC_DIGITS: C2RustUnnamed_0 = 262152;
pub const __INT_FRAC_DIGITS: C2RustUnnamed_0 = 262151;
pub const __NEGATIVE_SIGN: C2RustUnnamed_0 = 262150;
pub const __POSITIVE_SIGN: C2RustUnnamed_0 = 262149;
pub const __MON_GROUPING: C2RustUnnamed_0 = 262148;
pub const __MON_THOUSANDS_SEP: C2RustUnnamed_0 = 262147;
pub const __MON_DECIMAL_POINT: C2RustUnnamed_0 = 262146;
pub const __CURRENCY_SYMBOL: C2RustUnnamed_0 = 262145;
pub const __INT_CURR_SYMBOL: C2RustUnnamed_0 = 262144;
pub const _NL_NUM_LC_CTYPE: C2RustUnnamed_0 = 86;
pub const _NL_CTYPE_EXTRA_MAP_14: C2RustUnnamed_0 = 85;
pub const _NL_CTYPE_EXTRA_MAP_13: C2RustUnnamed_0 = 84;
pub const _NL_CTYPE_EXTRA_MAP_12: C2RustUnnamed_0 = 83;
pub const _NL_CTYPE_EXTRA_MAP_11: C2RustUnnamed_0 = 82;
pub const _NL_CTYPE_EXTRA_MAP_10: C2RustUnnamed_0 = 81;
pub const _NL_CTYPE_EXTRA_MAP_9: C2RustUnnamed_0 = 80;
pub const _NL_CTYPE_EXTRA_MAP_8: C2RustUnnamed_0 = 79;
pub const _NL_CTYPE_EXTRA_MAP_7: C2RustUnnamed_0 = 78;
pub const _NL_CTYPE_EXTRA_MAP_6: C2RustUnnamed_0 = 77;
pub const _NL_CTYPE_EXTRA_MAP_5: C2RustUnnamed_0 = 76;
pub const _NL_CTYPE_EXTRA_MAP_4: C2RustUnnamed_0 = 75;
pub const _NL_CTYPE_EXTRA_MAP_3: C2RustUnnamed_0 = 74;
pub const _NL_CTYPE_EXTRA_MAP_2: C2RustUnnamed_0 = 73;
pub const _NL_CTYPE_EXTRA_MAP_1: C2RustUnnamed_0 = 72;
pub const _NL_CTYPE_NONASCII_CASE: C2RustUnnamed_0 = 71;
pub const _NL_CTYPE_MAP_TO_NONASCII: C2RustUnnamed_0 = 70;
pub const _NL_CTYPE_TRANSLIT_IGNORE: C2RustUnnamed_0 = 69;
pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: C2RustUnnamed_0 = 68;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: C2RustUnnamed_0 = 67;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: C2RustUnnamed_0 = 66;
pub const _NL_CTYPE_TRANSLIT_TO_TBL: C2RustUnnamed_0 = 65;
pub const _NL_CTYPE_TRANSLIT_TO_IDX: C2RustUnnamed_0 = 64;
pub const _NL_CTYPE_TRANSLIT_FROM_TBL: C2RustUnnamed_0 = 63;
pub const _NL_CTYPE_TRANSLIT_FROM_IDX: C2RustUnnamed_0 = 62;
pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: C2RustUnnamed_0 = 61;
pub const _NL_CTYPE_OUTDIGIT9_WC: C2RustUnnamed_0 = 60;
pub const _NL_CTYPE_OUTDIGIT8_WC: C2RustUnnamed_0 = 59;
pub const _NL_CTYPE_OUTDIGIT7_WC: C2RustUnnamed_0 = 58;
pub const _NL_CTYPE_OUTDIGIT6_WC: C2RustUnnamed_0 = 57;
pub const _NL_CTYPE_OUTDIGIT5_WC: C2RustUnnamed_0 = 56;
pub const _NL_CTYPE_OUTDIGIT4_WC: C2RustUnnamed_0 = 55;
pub const _NL_CTYPE_OUTDIGIT3_WC: C2RustUnnamed_0 = 54;
pub const _NL_CTYPE_OUTDIGIT2_WC: C2RustUnnamed_0 = 53;
pub const _NL_CTYPE_OUTDIGIT1_WC: C2RustUnnamed_0 = 52;
pub const _NL_CTYPE_OUTDIGIT0_WC: C2RustUnnamed_0 = 51;
pub const _NL_CTYPE_OUTDIGIT9_MB: C2RustUnnamed_0 = 50;
pub const _NL_CTYPE_OUTDIGIT8_MB: C2RustUnnamed_0 = 49;
pub const _NL_CTYPE_OUTDIGIT7_MB: C2RustUnnamed_0 = 48;
pub const _NL_CTYPE_OUTDIGIT6_MB: C2RustUnnamed_0 = 47;
pub const _NL_CTYPE_OUTDIGIT5_MB: C2RustUnnamed_0 = 46;
pub const _NL_CTYPE_OUTDIGIT4_MB: C2RustUnnamed_0 = 45;
pub const _NL_CTYPE_OUTDIGIT3_MB: C2RustUnnamed_0 = 44;
pub const _NL_CTYPE_OUTDIGIT2_MB: C2RustUnnamed_0 = 43;
pub const _NL_CTYPE_OUTDIGIT1_MB: C2RustUnnamed_0 = 42;
pub const _NL_CTYPE_OUTDIGIT0_MB: C2RustUnnamed_0 = 41;
pub const _NL_CTYPE_INDIGITS9_WC: C2RustUnnamed_0 = 40;
pub const _NL_CTYPE_INDIGITS8_WC: C2RustUnnamed_0 = 39;
pub const _NL_CTYPE_INDIGITS7_WC: C2RustUnnamed_0 = 38;
pub const _NL_CTYPE_INDIGITS6_WC: C2RustUnnamed_0 = 37;
pub const _NL_CTYPE_INDIGITS5_WC: C2RustUnnamed_0 = 36;
pub const _NL_CTYPE_INDIGITS4_WC: C2RustUnnamed_0 = 35;
pub const _NL_CTYPE_INDIGITS3_WC: C2RustUnnamed_0 = 34;
pub const _NL_CTYPE_INDIGITS2_WC: C2RustUnnamed_0 = 33;
pub const _NL_CTYPE_INDIGITS1_WC: C2RustUnnamed_0 = 32;
pub const _NL_CTYPE_INDIGITS0_WC: C2RustUnnamed_0 = 31;
pub const _NL_CTYPE_INDIGITS_WC_LEN: C2RustUnnamed_0 = 30;
pub const _NL_CTYPE_INDIGITS9_MB: C2RustUnnamed_0 = 29;
pub const _NL_CTYPE_INDIGITS8_MB: C2RustUnnamed_0 = 28;
pub const _NL_CTYPE_INDIGITS7_MB: C2RustUnnamed_0 = 27;
pub const _NL_CTYPE_INDIGITS6_MB: C2RustUnnamed_0 = 26;
pub const _NL_CTYPE_INDIGITS5_MB: C2RustUnnamed_0 = 25;
pub const _NL_CTYPE_INDIGITS4_MB: C2RustUnnamed_0 = 24;
pub const _NL_CTYPE_INDIGITS3_MB: C2RustUnnamed_0 = 23;
pub const _NL_CTYPE_INDIGITS2_MB: C2RustUnnamed_0 = 22;
pub const _NL_CTYPE_INDIGITS1_MB: C2RustUnnamed_0 = 21;
pub const _NL_CTYPE_INDIGITS0_MB: C2RustUnnamed_0 = 20;
pub const _NL_CTYPE_INDIGITS_MB_LEN: C2RustUnnamed_0 = 19;
pub const _NL_CTYPE_MAP_OFFSET: C2RustUnnamed_0 = 18;
pub const _NL_CTYPE_CLASS_OFFSET: C2RustUnnamed_0 = 17;
pub const _NL_CTYPE_TOLOWER32: C2RustUnnamed_0 = 16;
pub const _NL_CTYPE_TOUPPER32: C2RustUnnamed_0 = 15;
pub const _NL_CTYPE_CODESET_NAME: C2RustUnnamed_0 = 14;
pub const _NL_CTYPE_MB_CUR_MAX: C2RustUnnamed_0 = 13;
pub const _NL_CTYPE_WIDTH: C2RustUnnamed_0 = 12;
pub const _NL_CTYPE_MAP_NAMES: C2RustUnnamed_0 = 11;
pub const _NL_CTYPE_CLASS_NAMES: C2RustUnnamed_0 = 10;
pub const _NL_CTYPE_GAP6: C2RustUnnamed_0 = 9;
pub const _NL_CTYPE_GAP5: C2RustUnnamed_0 = 8;
pub const _NL_CTYPE_GAP4: C2RustUnnamed_0 = 7;
pub const _NL_CTYPE_GAP3: C2RustUnnamed_0 = 6;
pub const _NL_CTYPE_CLASS32: C2RustUnnamed_0 = 5;
pub const _NL_CTYPE_GAP2: C2RustUnnamed_0 = 4;
pub const _NL_CTYPE_TOLOWER: C2RustUnnamed_0 = 3;
pub const _NL_CTYPE_GAP1: C2RustUnnamed_0 = 2;
pub const _NL_CTYPE_TOUPPER: C2RustUnnamed_0 = 1;
pub const _NL_CTYPE_CLASS: C2RustUnnamed_0 = 0;
pub const _NL_NUM_LC_COLLATE: C2RustUnnamed_0 = 196627;
pub const _NL_COLLATE_CODESET: C2RustUnnamed_0 = 196626;
pub const _NL_COLLATE_COLLSEQWC: C2RustUnnamed_0 = 196625;
pub const _NL_COLLATE_COLLSEQMB: C2RustUnnamed_0 = 196624;
pub const _NL_COLLATE_SYMB_EXTRAMB: C2RustUnnamed_0 = 196623;
pub const _NL_COLLATE_SYMB_TABLEMB: C2RustUnnamed_0 = 196622;
pub const _NL_COLLATE_SYMB_HASH_SIZEMB: C2RustUnnamed_0 = 196621;
pub const _NL_COLLATE_INDIRECTWC: C2RustUnnamed_0 = 196620;
pub const _NL_COLLATE_EXTRAWC: C2RustUnnamed_0 = 196619;
pub const _NL_COLLATE_WEIGHTWC: C2RustUnnamed_0 = 196618;
pub const _NL_COLLATE_TABLEWC: C2RustUnnamed_0 = 196617;
pub const _NL_COLLATE_GAP3: C2RustUnnamed_0 = 196616;
pub const _NL_COLLATE_GAP2: C2RustUnnamed_0 = 196615;
pub const _NL_COLLATE_GAP1: C2RustUnnamed_0 = 196614;
pub const _NL_COLLATE_INDIRECTMB: C2RustUnnamed_0 = 196613;
pub const _NL_COLLATE_EXTRAMB: C2RustUnnamed_0 = 196612;
pub const _NL_COLLATE_WEIGHTMB: C2RustUnnamed_0 = 196611;
pub const _NL_COLLATE_TABLEMB: C2RustUnnamed_0 = 196610;
pub const _NL_COLLATE_RULESETS: C2RustUnnamed_0 = 196609;
pub const _NL_COLLATE_NRULES: C2RustUnnamed_0 = 196608;
pub const _NL_NUM_LC_TIME: C2RustUnnamed_0 = 131231;
pub const _NL_WABALTMON_12: C2RustUnnamed_0 = 131230;
pub const _NL_WABALTMON_11: C2RustUnnamed_0 = 131229;
pub const _NL_WABALTMON_10: C2RustUnnamed_0 = 131228;
pub const _NL_WABALTMON_9: C2RustUnnamed_0 = 131227;
pub const _NL_WABALTMON_8: C2RustUnnamed_0 = 131226;
pub const _NL_WABALTMON_7: C2RustUnnamed_0 = 131225;
pub const _NL_WABALTMON_6: C2RustUnnamed_0 = 131224;
pub const _NL_WABALTMON_5: C2RustUnnamed_0 = 131223;
pub const _NL_WABALTMON_4: C2RustUnnamed_0 = 131222;
pub const _NL_WABALTMON_3: C2RustUnnamed_0 = 131221;
pub const _NL_WABALTMON_2: C2RustUnnamed_0 = 131220;
pub const _NL_WABALTMON_1: C2RustUnnamed_0 = 131219;
pub const _NL_ABALTMON_12: C2RustUnnamed_0 = 131218;
pub const _NL_ABALTMON_11: C2RustUnnamed_0 = 131217;
pub const _NL_ABALTMON_10: C2RustUnnamed_0 = 131216;
pub const _NL_ABALTMON_9: C2RustUnnamed_0 = 131215;
pub const _NL_ABALTMON_8: C2RustUnnamed_0 = 131214;
pub const _NL_ABALTMON_7: C2RustUnnamed_0 = 131213;
pub const _NL_ABALTMON_6: C2RustUnnamed_0 = 131212;
pub const _NL_ABALTMON_5: C2RustUnnamed_0 = 131211;
pub const _NL_ABALTMON_4: C2RustUnnamed_0 = 131210;
pub const _NL_ABALTMON_3: C2RustUnnamed_0 = 131209;
pub const _NL_ABALTMON_2: C2RustUnnamed_0 = 131208;
pub const _NL_ABALTMON_1: C2RustUnnamed_0 = 131207;
pub const _NL_WALTMON_12: C2RustUnnamed_0 = 131206;
pub const _NL_WALTMON_11: C2RustUnnamed_0 = 131205;
pub const _NL_WALTMON_10: C2RustUnnamed_0 = 131204;
pub const _NL_WALTMON_9: C2RustUnnamed_0 = 131203;
pub const _NL_WALTMON_8: C2RustUnnamed_0 = 131202;
pub const _NL_WALTMON_7: C2RustUnnamed_0 = 131201;
pub const _NL_WALTMON_6: C2RustUnnamed_0 = 131200;
pub const _NL_WALTMON_5: C2RustUnnamed_0 = 131199;
pub const _NL_WALTMON_4: C2RustUnnamed_0 = 131198;
pub const _NL_WALTMON_3: C2RustUnnamed_0 = 131197;
pub const _NL_WALTMON_2: C2RustUnnamed_0 = 131196;
pub const _NL_WALTMON_1: C2RustUnnamed_0 = 131195;
pub const __ALTMON_12: C2RustUnnamed_0 = 131194;
pub const __ALTMON_11: C2RustUnnamed_0 = 131193;
pub const __ALTMON_10: C2RustUnnamed_0 = 131192;
pub const __ALTMON_9: C2RustUnnamed_0 = 131191;
pub const __ALTMON_8: C2RustUnnamed_0 = 131190;
pub const __ALTMON_7: C2RustUnnamed_0 = 131189;
pub const __ALTMON_6: C2RustUnnamed_0 = 131188;
pub const __ALTMON_5: C2RustUnnamed_0 = 131187;
pub const __ALTMON_4: C2RustUnnamed_0 = 131186;
pub const __ALTMON_3: C2RustUnnamed_0 = 131185;
pub const __ALTMON_2: C2RustUnnamed_0 = 131184;
pub const __ALTMON_1: C2RustUnnamed_0 = 131183;
pub const _NL_TIME_CODESET: C2RustUnnamed_0 = 131182;
pub const _NL_W_DATE_FMT: C2RustUnnamed_0 = 131181;
pub const _DATE_FMT: C2RustUnnamed_0 = 131180;
pub const _NL_TIME_TIMEZONE: C2RustUnnamed_0 = 131179;
pub const _NL_TIME_CAL_DIRECTION: C2RustUnnamed_0 = 131178;
pub const _NL_TIME_FIRST_WORKDAY: C2RustUnnamed_0 = 131177;
pub const _NL_TIME_FIRST_WEEKDAY: C2RustUnnamed_0 = 131176;
pub const _NL_TIME_WEEK_1STWEEK: C2RustUnnamed_0 = 131175;
pub const _NL_TIME_WEEK_1STDAY: C2RustUnnamed_0 = 131174;
pub const _NL_TIME_WEEK_NDAYS: C2RustUnnamed_0 = 131173;
pub const _NL_WERA_T_FMT: C2RustUnnamed_0 = 131172;
pub const _NL_WERA_D_T_FMT: C2RustUnnamed_0 = 131171;
pub const _NL_WALT_DIGITS: C2RustUnnamed_0 = 131170;
pub const _NL_WERA_D_FMT: C2RustUnnamed_0 = 131169;
pub const _NL_WERA_YEAR: C2RustUnnamed_0 = 131168;
pub const _NL_WT_FMT_AMPM: C2RustUnnamed_0 = 131167;
pub const _NL_WT_FMT: C2RustUnnamed_0 = 131166;
pub const _NL_WD_FMT: C2RustUnnamed_0 = 131165;
pub const _NL_WD_T_FMT: C2RustUnnamed_0 = 131164;
pub const _NL_WPM_STR: C2RustUnnamed_0 = 131163;
pub const _NL_WAM_STR: C2RustUnnamed_0 = 131162;
pub const _NL_WMON_12: C2RustUnnamed_0 = 131161;
pub const _NL_WMON_11: C2RustUnnamed_0 = 131160;
pub const _NL_WMON_10: C2RustUnnamed_0 = 131159;
pub const _NL_WMON_9: C2RustUnnamed_0 = 131158;
pub const _NL_WMON_8: C2RustUnnamed_0 = 131157;
pub const _NL_WMON_7: C2RustUnnamed_0 = 131156;
pub const _NL_WMON_6: C2RustUnnamed_0 = 131155;
pub const _NL_WMON_5: C2RustUnnamed_0 = 131154;
pub const _NL_WMON_4: C2RustUnnamed_0 = 131153;
pub const _NL_WMON_3: C2RustUnnamed_0 = 131152;
pub const _NL_WMON_2: C2RustUnnamed_0 = 131151;
pub const _NL_WMON_1: C2RustUnnamed_0 = 131150;
pub const _NL_WABMON_12: C2RustUnnamed_0 = 131149;
pub const _NL_WABMON_11: C2RustUnnamed_0 = 131148;
pub const _NL_WABMON_10: C2RustUnnamed_0 = 131147;
pub const _NL_WABMON_9: C2RustUnnamed_0 = 131146;
pub const _NL_WABMON_8: C2RustUnnamed_0 = 131145;
pub const _NL_WABMON_7: C2RustUnnamed_0 = 131144;
pub const _NL_WABMON_6: C2RustUnnamed_0 = 131143;
pub const _NL_WABMON_5: C2RustUnnamed_0 = 131142;
pub const _NL_WABMON_4: C2RustUnnamed_0 = 131141;
pub const _NL_WABMON_3: C2RustUnnamed_0 = 131140;
pub const _NL_WABMON_2: C2RustUnnamed_0 = 131139;
pub const _NL_WABMON_1: C2RustUnnamed_0 = 131138;
pub const _NL_WDAY_7: C2RustUnnamed_0 = 131137;
pub const _NL_WDAY_6: C2RustUnnamed_0 = 131136;
pub const _NL_WDAY_5: C2RustUnnamed_0 = 131135;
pub const _NL_WDAY_4: C2RustUnnamed_0 = 131134;
pub const _NL_WDAY_3: C2RustUnnamed_0 = 131133;
pub const _NL_WDAY_2: C2RustUnnamed_0 = 131132;
pub const _NL_WDAY_1: C2RustUnnamed_0 = 131131;
pub const _NL_WABDAY_7: C2RustUnnamed_0 = 131130;
pub const _NL_WABDAY_6: C2RustUnnamed_0 = 131129;
pub const _NL_WABDAY_5: C2RustUnnamed_0 = 131128;
pub const _NL_WABDAY_4: C2RustUnnamed_0 = 131127;
pub const _NL_WABDAY_3: C2RustUnnamed_0 = 131126;
pub const _NL_WABDAY_2: C2RustUnnamed_0 = 131125;
pub const _NL_WABDAY_1: C2RustUnnamed_0 = 131124;
pub const _NL_TIME_ERA_ENTRIES: C2RustUnnamed_0 = 131123;
pub const _NL_TIME_ERA_NUM_ENTRIES: C2RustUnnamed_0 = 131122;
pub const ERA_T_FMT: C2RustUnnamed_0 = 131121;
pub const ERA_D_T_FMT: C2RustUnnamed_0 = 131120;
pub const ALT_DIGITS: C2RustUnnamed_0 = 131119;
pub const ERA_D_FMT: C2RustUnnamed_0 = 131118;
pub const __ERA_YEAR: C2RustUnnamed_0 = 131117;
pub const ERA: C2RustUnnamed_0 = 131116;
pub const T_FMT_AMPM: C2RustUnnamed_0 = 131115;
pub const T_FMT: C2RustUnnamed_0 = 131114;
pub const D_FMT: C2RustUnnamed_0 = 131113;
pub const D_T_FMT: C2RustUnnamed_0 = 131112;
pub const PM_STR: C2RustUnnamed_0 = 131111;
pub const AM_STR: C2RustUnnamed_0 = 131110;
pub const MON_12: C2RustUnnamed_0 = 131109;
pub const MON_11: C2RustUnnamed_0 = 131108;
pub const MON_10: C2RustUnnamed_0 = 131107;
pub const MON_9: C2RustUnnamed_0 = 131106;
pub const MON_8: C2RustUnnamed_0 = 131105;
pub const MON_7: C2RustUnnamed_0 = 131104;
pub const MON_6: C2RustUnnamed_0 = 131103;
pub const MON_5: C2RustUnnamed_0 = 131102;
pub const MON_4: C2RustUnnamed_0 = 131101;
pub const MON_3: C2RustUnnamed_0 = 131100;
pub const MON_2: C2RustUnnamed_0 = 131099;
pub const MON_1: C2RustUnnamed_0 = 131098;
pub const ABMON_12: C2RustUnnamed_0 = 131097;
pub const ABMON_11: C2RustUnnamed_0 = 131096;
pub const ABMON_10: C2RustUnnamed_0 = 131095;
pub const ABMON_9: C2RustUnnamed_0 = 131094;
pub const ABMON_8: C2RustUnnamed_0 = 131093;
pub const ABMON_7: C2RustUnnamed_0 = 131092;
pub const ABMON_6: C2RustUnnamed_0 = 131091;
pub const ABMON_5: C2RustUnnamed_0 = 131090;
pub const ABMON_4: C2RustUnnamed_0 = 131089;
pub const ABMON_3: C2RustUnnamed_0 = 131088;
pub const ABMON_2: C2RustUnnamed_0 = 131087;
pub const ABMON_1: C2RustUnnamed_0 = 131086;
pub const DAY_7: C2RustUnnamed_0 = 131085;
pub const DAY_6: C2RustUnnamed_0 = 131084;
pub const DAY_5: C2RustUnnamed_0 = 131083;
pub const DAY_4: C2RustUnnamed_0 = 131082;
pub const DAY_3: C2RustUnnamed_0 = 131081;
pub const DAY_2: C2RustUnnamed_0 = 131080;
pub const DAY_1: C2RustUnnamed_0 = 131079;
pub const ABDAY_7: C2RustUnnamed_0 = 131078;
pub const ABDAY_6: C2RustUnnamed_0 = 131077;
pub const ABDAY_5: C2RustUnnamed_0 = 131076;
pub const ABDAY_4: C2RustUnnamed_0 = 131075;
pub const ABDAY_3: C2RustUnnamed_0 = 131074;
pub const ABDAY_2: C2RustUnnamed_0 = 131073;
pub const ABDAY_1: C2RustUnnamed_0 = 131072;

const ESC: u32 = '[' as u32 & 0o37;
const BS_CONTROL: i32 = 2; /* \b treated as control char; prints as ^H */

const IS_BINARY_CHAR: i32 = 1;
const IS_CONTROL_CHAR: i32 = 2;

/* Special char bit-flags used to tell put_line() to do something special */
const AT_NORMAL: i32 = 0;
const AT_UNDERLINE: i32 = 1 << 0;
const AT_BOLD: i32 = 1 << 1;
const AT_BLINK: i32 = 1 << 2;
const AT_STANDOUT: i32 = 1 << 3;
const AT_ANSI: i32 = 1 << 4; /* Content-supplied "ANSI" escape sequence */
const AT_BINARY: i32 = 1 << 5; /* LESS*BINFMT representation */
const AT_HILITE: i32 = 1 << 6; /* Internal highlights (e.g., for search) */

const AT_COLOR_SHIFT: i32 = 8;
const AT_NUM_COLORS: i32 = 16;
const AT_COLOR: i32 = (AT_NUM_COLORS - 1) << AT_COLOR_SHIFT;
const AT_COLOR_ATTN: i32 = 1 << AT_COLOR_SHIFT;
const AT_COLOR_BIN: i32 = 2 << AT_COLOR_SHIFT;
const AT_COLOR_CTRL: i32 = 3 << AT_COLOR_SHIFT;
const AT_COLOR_ERROR: i32 = 4 << AT_COLOR_SHIFT;
const AT_COLOR_LINENUM: i32 = 5 << AT_COLOR_SHIFT;
const AT_COLOR_MARK: i32 = 6 << AT_COLOR_SHIFT;
const AT_COLOR_PROMPT: i32 = 7 << AT_COLOR_SHIFT;
const AT_COLOR_RSCROLL: i32 = 8 << AT_COLOR_SHIFT;
const AT_COLOR_HEADER: i32 = 9 << AT_COLOR_SHIFT;
const AT_COLOR_SEARCH: i32 = 10 << AT_COLOR_SHIFT;

pub static mut utf_mode: bool = false;

#[rustfmt::skip]
pub static charsets: [charset; 20] = unsafe {
    [
        charset { name: "ascii",             p_flag: None,       desc: "8bcccbcc18b95.b" },
        charset { name: "utf-8",             p_flag: Some(utf_mode),  desc: "8bcccbcc18b95.b126.bb" },
        charset { name: "iso8859",           p_flag: None,       desc: "8bcccbcc18b95.33b." },
        charset { name: "latin3",            p_flag: None,       desc: "8bcccbcc18b95.33b5.b8.b15.b4.b12.b18.b12.b." },
        charset { name: "arabic",            p_flag: None,       desc: "8bcccbcc18b95.33b.3b.7b2.13b.3b.b26.5b19.b" },
        charset { name: "greek",             p_flag: None,       desc: "8bcccbcc18b95.33b4.2b4.b3.b35.b44.b" },
        charset { name: "greek2005",         p_flag: None,       desc: "8bcccbcc18b95.33b14.b35.b44.b" },
        charset { name: "hebrew",            p_flag: None,       desc: "8bcccbcc18b95.33b.b29.32b28.2b2.b" },
        charset { name: "koi8-r",            p_flag: None,       desc: "8bcccbcc18b95.b." },
        charset { name: "KOI8-T",            p_flag: None,       desc: "8bcccbcc18b95.b8.b6.b8.b.b.5b7.3b4.b4.b3.b.b.3b." },
        charset { name: "georgianps",        p_flag: None,       desc: "8bcccbcc18b95.3b11.4b12.2b." },
        charset { name: "tcvn",              p_flag: None,       desc: "b..b...bcccbccbbb7.8b95.b48.5b." },
        charset { name: "TIS-620",           p_flag: None,       desc: "8bcccbcc18b95.b.4b.11b7.8b." },
        charset { name: "next",              p_flag: None,       desc: "8bcccbcc18b95.bb125.bb" },
        charset { name: "dos",               p_flag: None,       desc: "8bcccbcc12bc5b95.b." },
        charset { name: "windows-1251",      p_flag: None,       desc: "8bcccbcc12bc5b95.b24.b." },
        charset { name: "windows-1252",      p_flag: None,       desc: "8bcccbcc12bc5b95.b.b11.b.2b12.b." },
        charset { name: "windows-1255",      p_flag: None,       desc: "8bcccbcc12bc5b95.b.b8.b.5b9.b.4b." },
        charset { name: "ebcdic",            p_flag: None,       desc: "5bc6bcc7bcc41b.9b7.9b5.b..8b6.10b6.b9.7b9.8b8.17b3.3b9.7b9.8b8.6b10.b.b.b." },
        charset { name: "IBM-1047",          p_flag: None,       desc: "4cbcbc3b9cbccbccbb4c6bcc5b3cbbc4bc4bccbc191.b" },
        ]
};

#[rustfmt::skip]
pub static cs_aliases: [cs_alias; 42] =
    [
	cs_alias { name: "UTF-8",              oname: "utf-8" },
	cs_alias { name: "utf8",               oname: "utf-8" },
	cs_alias { name: "UTF8",               oname: "utf-8" },
	cs_alias { name: "ANSI_X3.4-1968",     oname: "ascii" },
	cs_alias { name: "US-ASCII",           oname: "ascii" },
	cs_alias { name: "latin1",             oname: "iso8859" },
	cs_alias { name: "ISO-8859-1",         oname: "iso8859" },
	cs_alias { name: "latin9",             oname: "iso8859" },
	cs_alias { name: "ISO-8859-15",        oname: "iso8859" },
	cs_alias { name: "latin2",             oname: "iso8859" },
	cs_alias { name: "ISO-8859-2",         oname: "iso8859" },
	cs_alias { name: "ISO-8859-3",         oname: "latin3" },
	cs_alias { name: "latin4",             oname: "iso8859" },
	cs_alias { name: "ISO-8859-4",         oname: "iso8859" },
	cs_alias { name: "cyrillic",           oname: "iso8859" },
	cs_alias { name: "ISO-8859-5",         oname: "iso8859" },
	cs_alias { name: "ISO-8859-6",         oname: "arabic" },
	cs_alias { name: "ISO-8859-7",         oname: "greek" },
	cs_alias { name: "IBM9005",            oname: "greek2005" },
	cs_alias { name: "ISO-8859-8",         oname: "hebrew" },
	cs_alias { name: "latin5",             oname: "iso8859" },
	cs_alias { name: "ISO-8859-9",         oname: "iso8859" },
	cs_alias { name: "latin6",             oname: "iso8859" },
	cs_alias { name: "ISO-8859-10",        oname: "iso8859" },
	cs_alias { name: "latin7",             oname: "iso8859" },
	cs_alias { name: "ISO-8859-13",        oname: "iso8859" },
	cs_alias { name: "latin8",             oname: "iso8859" },
	cs_alias { name: "ISO-8859-14",        oname: "iso8859" },
	cs_alias { name: "latin10",            oname: "iso8859" },
	cs_alias { name: "ISO-8859-16",        oname: "iso8859" },
	cs_alias { name: "IBM437",             oname: "dos" },
	cs_alias { name: "EBCDIC-US",          oname: "ebcdic" },
	cs_alias { name: "IBM1047",            oname: "IBM-1047" },
	cs_alias { name: "KOI8-R",             oname: "koi8-r" },
	cs_alias { name: "KOI8-U",             oname: "koi8-r" },
	cs_alias { name: "GEORGIAN-PS",        oname: "georgianps" },
	cs_alias { name: "TCVN5712-1",         oname: "tcvn" },
	cs_alias { name: "NEXTSTEP",           oname: "next" },
	cs_alias { name: "windows",            oname: "windows-1252" }, /* backward compatibility */
	cs_alias { name: "CP1251",             oname: "windows-1251" },
	cs_alias { name: "CP1252",             oname: "windows-1252" },
	cs_alias { name: "CP1255",             oname: "windows-1255" },
    ];

static mut chardef: [u8; 256] = [0; 256];
static mut binfmt: String = String::new();
static mut utfbinfmt: String = String::new();

pub static mut binattr: i32 = AT_STANDOUT | AT_COLOR_BIN;

static mut user_wide_array: LazyLock<XBuffer> = LazyLock::new(|| XBuffer::new(16));
static mut user_ubin_array: LazyLock<XBuffer> = LazyLock::new(|| XBuffer::new(16));
static mut user_compose_array: LazyLock<XBuffer> = LazyLock::new(|| XBuffer::new(16));
static mut user_prt_array: LazyLock<XBuffer> = LazyLock::new(|| XBuffer::new(16));

static mut user_wide_table: wchar_range_table = Vec::new();
static mut user_ubin_table: wchar_range_table = Vec::new();
static mut user_compose_table: wchar_range_table = Vec::new();
static mut user_prt_table: wchar_range_table = Vec::new();

/*
 * Set a wchar_range_table to the table in an xbuffer.
 */
unsafe extern "C" fn wchar_range_table_set(arr: &mut XBuffer) -> wchar_range_table {
    arr.data
        .chunks_exact(4)
        .map(|chunk| char::from_u32(u32::from_le_bytes(chunk.try_into().unwrap())).unwrap())
        .collect::<Vec<char>>()
        .chunks_exact(2)
        .map(|chunk| wchar_range {
            first: chunk[0],
            last: chunk[1],
        })
        .collect()
}

/*
 * Skip over a "U" or "U+" prefix before a hex codepoint.
 */
unsafe extern "C" fn skip_uprefix(s: &[u8]) -> usize {
    let mut idx = 0;
    if s[0] == b'U' || s[0] == b'u' {
        if s[1] == b'+' {
            idx += 2;
        }
    }
    idx
}

fn parse_hex_prefix(input: &[u8]) -> Option<(char, usize)> {
    let mut end = 0;

    while end < input.len() && input[end].is_ascii_hexdigit() {
        end += 1;
    }

    if end == 0 {
        return None;
    }

    let number = u64::from_str_radix(std::str::from_utf8(&input[..end]).ok()?, 16).ok()?;

    if let Some(ch) = char::from_u32(number as u32) {
        return Some((ch, end));
    }
    None
}

/*
 * Parse a dash-separated range of hex values.
 */
unsafe extern "C" fn wchar_range_get(s: &[u8]) -> (wchar_range, usize) {
    let mut idx = skip_uprefix(s);
    let mut s = &s[idx..];
    let mut range = wchar_range {
        first: '\0',
        last: '\0',
    };
    if let Some((ch, i)) = parse_hex_prefix(&s) {
        range.first = ch;
        if s[i] == b'-' {
            idx = skip_uprefix(&s[i + 1..]);
            (range.last, idx) = parse_hex_prefix(&s[idx..]).unwrap();
        } else {
            range.last = range.first;
        }
    }
    (range, idx)
}

/*
 * Parse the LESSUTFCHARDEF variable.
 */
unsafe extern "C" fn ichardef_utf(r: Option<String>) {
    let mut idx = 0;
    if !r.is_none() {
        let s = r.unwrap();
        let s = s.as_str().as_bytes();
        while s[0] != b'\0' {
            let (mut range, mut idx) = wchar_range_get(s);
            if range.last as i32 == 0 {
                error(
                    b"invalid hex number(s) in LESSUTFCHARDEF\0" as *const u8
                        as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                quit(1);
            }
            if s[idx] == b':' {
                idx += 1;
                error(
                    b"missing colon in LESSUTFCHARDEF\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                quit(1);
            }
            let range = range.as_bytes();
            match s[idx] {
                b'b' => {
                    user_ubin_array.add_data(&range, range.len());
                }
                b'c' => {
                    user_compose_array.add_data(&range, range.len());
                }
                b'w' => {
                    user_wide_array.add_data(&range, range.len());
                    user_prt_array.add_data(&range, range.len());
                }
                b'p' | b'.' => {
                    user_prt_array.add_data(&range, range.len());
                }
                b'\0' => {
                    idx -= 1;
                }
                _ => { /* Ignore unknown character attribute. */ }
            }
            idx += 1;
            if s[idx] == b',' {
                idx += 1;
            }
        }
    }
    user_wide_table = wchar_range_table_set(&mut user_wide_array);
    user_ubin_table = wchar_range_table_set(&mut user_ubin_array);
    user_compose_table = wchar_range_table_set(&mut user_compose_array);
    user_prt_table = wchar_range_table_set(&mut user_prt_array);
}

/*
 * Define a charset, given a description string.
 * The string consists of 256 letters,
 * one for each character in the charset.
 * If the string is shorter than 256 letters, missing letters
 * are taken to be identical to the last one.
 * A decimal number followed by a letter is taken to be a
 * repetition of the letter.
 *
 * Each letter is one of:
 *      . normal character
 *      b binary character
 *      c control character
 */

pub unsafe fn ichardef(s: &str) {
    let mut cp = 0; // index into chardef
    let mut v: u8 = 0;
    let mut n: usize = 0;

    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '.' => {
                v = 0;
            }
            'c' => {
                v = IS_CONTROL_CHAR as u8;
            }
            'b' => {
                v = (IS_BINARY_CHAR | IS_CONTROL_CHAR) as u8;
            }
            '0'..='9' => {
                // build number safely
                let digit = (c as u8 - b'0') as usize;
                n = n
                    .checked_mul(10)
                    .and_then(|x| x.checked_add(digit))
                    .ok_or("overflow in chardef number")
                    .unwrap();

                // keep reading digits
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() {
                        let d = (chars.next().unwrap() as u8 - b'0') as usize;
                        n = n
                            .checked_mul(10)
                            .and_then(|x| x.checked_add(d))
                            .ok_or("overflow in chardef number")
                            .unwrap();
                    } else {
                        break;
                    }
                }
                continue;
            }
            _ => {
                error(
                    b"invalid chardef" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                quit(1 as std::ffi::c_int);
            }
        }

        // write v into chardef n times (at least once)
        let count = if n == 0 { 1 } else { n };
        if cp + count > chardef.len() {
            error(
                b"chardef longer than 256" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
            quit(1 as std::ffi::c_int);
        }
        for i in 0..count {
            chardef[cp + i] = v;
        }
        cp += count;
        n = 0;
    }

    // fill the rest with last v
    while cp < chardef.len() {
        chardef[cp] = v;
        cp += 1;
    }
}

/*
 * Define a charset, given a charset name.
 * The valid charset names are listed in the "charsets" array.
 */
unsafe extern "C" fn icharset(name: Option<String>, no_error: i32) -> i32 {
    let mut p = charset {
        name: "",
        p_flag: None,
        desc: "",
    };
    let mut a = cs_alias {
        name: "",
        oname: "",
    };

    if name.is_none() {
        return 0;
    }
    let name = name.unwrap();
    let mut name = name.as_str();
    if name.len() == 0 {
        return 0;
    }

    /* First see if the name is an alias. */
    for a in cs_aliases {
        if name == a.name {
            name = a.oname;
        }
    }

    for mut p in charsets {
        if name == p.name {
            ichardef(p.desc);
            if !p.p_flag.is_none() {
                p.p_flag = Some(true);
            }
            return 1;
        }
    }
    if no_error == 0 {
        error(
            b"invalid charset name\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        quit(1 as std::ffi::c_int);
    }
    0
}

/*
 * Define a charset, given a locale name.
 */
unsafe extern "C" fn ilocale() {
    for c in 0..255 {
        if (c as u8).is_ascii_graphic() || (c as u8).is_ascii_whitespace() {
            chardef[c] = 0;
        } else if (c as u8 as char).is_control() {
            chardef[c] = IS_CONTROL_CHAR as u8;
        } else {
            chardef[c] = (IS_BINARY_CHAR | IS_CONTROL_CHAR) as u8;
        }
    }
}

/*
 * Define the printing format for control (or binary utf) chars.
 */
pub fn setfmt<'a>(s: Option<String>, default_fmt: &'a str, for_printf: bool) -> (String, i32) {
    let mut attr = AT_NORMAL;
    let mut s = match &s {
        None => default_fmt,
        Some(st) if st == "" => default_fmt,
        Some(s) => {
            if for_printf {
                let bytes = s.as_bytes();
                if bytes.get(0) == Some(&b'*')
                    && (bytes.get(1) == None || bytes.get(2) == None && s[2..].contains('n'))
                {
                    default_fmt
                } else {
                    s
                }
            } else {
                s
            }
        }
    };

    /*
     * Select the attributes if it starts with "*".
     */
    if let Some(rest) = s.strip_prefix('*') {
        let mut chars = rest.chars();
        match chars.next() {
            Some('d') => {
                attr = AT_BOLD;
            }
            Some('k') => {
                attr = AT_BLINK;
            }
            Some('s') => {
                attr = AT_STANDOUT;
            }
            Some('u') => {
                attr = AT_UNDERLINE;
            }
            _ => {
                attr = AT_NORMAL;
            }
        }
        s = &rest[1..];
    }
    (s.to_string(), attr)
}

unsafe extern "C" fn set_charset() {
    ichardef_utf(lgetenv("LESSUTFCHARDEF").ok());

    /*
     * See if environment variable LESSCHARSET is defined.
     */
    let mut s = lgetenv("LESSCHARSET");
    if icharset(s.ok(), 0 as std::ffi::c_int) != 0 {
        return;
    }
    if let Ok(s) = lgetenv("LESSCHARDEF") {
        ichardef(&s);
        return;
    }
    s = Ok(CStr::from_ptr(nl_langinfo(CODESET as std::ffi::c_int))
        .to_str()
        .expect("invalid utf-8")
        .to_owned());
    if icharset(s.ok(), 1 as std::ffi::c_int) != 0 {
        return;
    }
    s = lgetenv("LC_ALL");
    if !s.is_err()
        || {
            s = lgetenv("LC_CTYPE");
            !s.is_err()
        }
        || {
            s = lgetenv("LANG");
            !s.is_err()
        }
    {
        let s = s.unwrap();
        if s.find("UTF-8").is_some()
            || s.find("utf-8").is_some()
            || s.find("UTF8").is_some()
            || s.find("utf8").is_some()
        {
            if icharset(Some("utf-8\0".into()), 1) != 0 {
                return;
            }
        }
    }
    ilocale();
}

pub unsafe extern "C" fn init_charset() {
    let mut s;
    // FIXME dfine LC_ALL (first param == 6 below) or use something else
    setlocale(
        6 as std::ffi::c_int,
        b"\0" as *const u8 as *const std::ffi::c_char,
    );
    set_charset();
    s = lgetenv("LESSBINFMT");
    (binfmt, binattr) = setfmt(s.ok(), "*s<%02X>\0", true);
    s = lgetenv("LESSUTFBINFMT");
    (utfbinfmt, binattr) = setfmt(s.ok(), "<U+%04lX>\0", true);
}

/*
 * Is a given character a "binary" character?
 */
pub unsafe extern "C" fn binary_char(c: char) -> bool {
    if utf_mode {
        return is_ubin_char(c);
    }
    if c as usize >= chardef.len() {
        return true;
    }
    chardef[c as usize] as i32 & IS_BINARY_CHAR != 0
}

/*
 * Is a given character a "control" character?
 */
pub unsafe extern "C" fn control_char(c: char) -> bool {
    if c as usize >= chardef.len() {
        return true;
    }
    (chardef[c as usize] as i32 & IS_CONTROL_CHAR) != 0
}

/*
 * Return the printable form of a character.
 * For example, in the "ascii" charset '\3' is printed as "^C".
 */
pub unsafe extern "C" fn prchar<'a>(c: char) -> String {
    let c = char::from_u32(c as u32 & 0o377).unwrap();
    let mut s = String::new();
    if (c < 128 as char || !utf_mode) && !c.is_control() {
        s = format!("^{}", c);
    } else if c as u32 == ESC {
        s = "ESC\0".into();
    } else if c < 128 as char && !char::from_u32((c as u32) ^ 0o100).unwrap().is_control() {
        s = format!("^{}", (c as u32) ^ 0o100);
    } else {
        // FIXME find a solution here
        //s = &format_args!(binfmt, c);
    }
    s
}

/*
 * Return the printable form of a UTF-8 character.
 */
pub unsafe extern "C" fn prutfchar(ch: char) -> &'static [u8] {
    static mut buf: [u8; 32] = [0; 32];
    let mut ch = ch;
    if ch as u32 == ESC {
        strcpy(
            CString::new(buf).unwrap().into_raw(),
            b"ESC\0" as *const u8 as *const std::ffi::c_char,
        );
    } else if ch < 128 as char && ch.is_control() {
        if !char::from_u32(ch as u32 ^ 0o100).unwrap().is_control() {
            snprintf(
                CString::new(buf).unwrap().into_raw(),
                ::core::mem::size_of::<[std::ffi::c_char; 32]>() as std::ffi::c_ulong,
                b"^%c\0" as *const u8 as *const std::ffi::c_char,
                ch as std::ffi::c_char as std::ffi::c_int ^ 0o100 as std::ffi::c_int,
            );
        } else {
            snprintf(
                CString::new(buf).unwrap().into_raw(),
                ::core::mem::size_of::<[std::ffi::c_char; 32]>() as std::ffi::c_ulong,
                binfmt.as_ptr() as *const i8,
                ch as std::ffi::c_char as std::ffi::c_int,
            );
        }
    } else if is_ubin_char(ch) {
        snprintf(
            CString::new(buf).unwrap().into_raw(),
            ::core::mem::size_of::<[std::ffi::c_char; 32]>() as std::ffi::c_ulong,
            utfbinfmt.as_ptr() as *const i8,
            ch,
        );
    } else {
        if ch as i64 >= 0x80000000 {
            ch = char::from_u32(0xfffd).unwrap();
        }
        let idx = put_wchar(&mut buf, ch);
        buf[idx] = b'\0';
    }
    &buf
}

pub unsafe extern "C" fn utf_len(ch: u8) -> usize {
    if ch & 0x80 == 0 {
        return 1;
    }
    if ch & 0xE0 == 0xC0 {
        return 2;
    }
    if ch & 0xF0 == 0xE0 {
        return 3;
    }
    if ch & 0xF8 == 0xF0 {
        return 4;
    }
    1
}

fn is_utf8_invalid(c: char) -> bool {
    c as i32 & 0xFE == 0xFE
}

/*
 * Does the parameter point to the lead byte of a well-formed UTF-8 character?
 */
pub unsafe extern "C" fn is_utf8_well_formed(ss: &[u8], slen: usize) -> bool {
    let mut i = 0;
    let mut len = 0;
    let mut s0 = ss[0];

    if is_utf8_invalid(s0 as char) {
        return false;
    }
    len = utf_len(ss[0]);
    if len > slen {
        return false;
    }
    if len == 1 {
        return true;
    }
    if len == 2 {
        if s0 < 0xc2 {
            return false;
        }
    } else {
        let mut mask = !(1 << (8 - len) - 1) as u8;
        if s0 == mask && ss[1] & mask == 0x80 {
            return false;
        }
    }
    for i in 1..len {
        if !is_utf8_trail(ss[i]) {
            return false;
        }
    }
    true
}
pub unsafe extern "C" fn utf_skip_to_lead(
    mut pp: *mut *const std::ffi::c_char,
    mut limit: *const std::ffi::c_char,
) {
    loop {
        *pp = (*pp).offset(1);
        if !(*pp < limit
            && !(*(*pp).offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                & 0o377 as std::ffi::c_int
                & 0xc0 as std::ffi::c_int
                == 0xc0 as std::ffi::c_int
                && !(*(*pp).offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                    & 0o377 as std::ffi::c_int
                    & 0xfe as std::ffi::c_int
                    == 0xfe as std::ffi::c_int))
            && !(*(*pp).offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                & 0x80 as std::ffi::c_int
                == 0 as std::ffi::c_int))
        {
            break;
        }
    }
}

/*
 * Get the value of a UTF-8 character.
 */
pub unsafe extern "C" fn get_wchar(sp: &[u8]) -> char {
    let mut idx = 0;

    match utf_len(sp[0]) {
        2 => {
            /* 110xxxxx 10xxxxxx */
            return char::from_u32((((sp[0] as u32) & 0x1F) << 6) | (sp[1] as u32 & 0x3F)).unwrap();
        }
        3 => {
            /* 1110xxxx 10xxxxxx 10xxxxxx */
            return char::from_u32(
                ((sp[0] as u32 & 0x0F) << 12)
                    | ((sp[1] as u32 & 0x3F) << 6)
                    | (sp[2] as u32 & 0x3F),
            )
            .unwrap();
        }
        4 => {
            /* 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx */
            return char::from_u32(
                ((sp[0] as u32 & 0x07) << 18)
                    | ((sp[1] as u32 & 0x3F) << 12)
                    | ((sp[2] as u32 & 0x3F) << 6)
                    | (sp[3] as u32 & 0x3F),
            )
            .unwrap();
        }
        1 | _ => {
            /* 0xxxxxxx */
            return (sp[0] & 0xFF) as char;
        }
    };
}

/*
 * Store a character into a UTF-8 string.
 */
pub unsafe extern "C" fn put_wchar(mut pp: &mut [u8], ch: char) -> usize {
    let mut idx = 0;
    if !utf_mode || (ch as i32) < 0x80 {
        /* 0xxxxxxx */
        pp[idx] = ch as u8;
        idx += 1;
    } else if (ch as i32) < 0x800 {
        /* 110xxxxx 10xxxxxx */
        pp[idx] = (0xC0 | ((ch as i32 >> 6) & 0x1F)) as u8;
        idx += 1;
        pp[idx] = (0x80 | (ch as i32 & 0x3F)) as u8;
        idx += 1;
    } else if (ch as i32) < 0x10000 {
        /* 1110xxxx 10xxxxxx 10xxxxxx */
        pp[idx] = (0xE0 | ((ch as i32 >> 12) & 0x0F)) as u8;
        idx += 1;
        pp[idx] = (0x80 | ((ch as i32 >> 6) & 0x3F)) as u8;
        idx += 1;
        pp[idx] = (0x80 | (ch as i32 & 0x3F)) as u8;
        idx += 1;
    } else if (ch as i32) < 0x200000 {
        /* 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx */
        pp[idx] = (0xF0 | ((ch as i32 >> 18) & 0x07)) as u8;
        idx += 1;
        pp[idx] = (0x80 | ((ch as i32 >> 12) & 0x3F)) as u8;
        idx += 1;
        pp[idx] = (0x80 | ((ch as i32 >> 6) & 0x3F)) as u8;
        idx += 1;
        pp[idx] = (0x80 | (ch as i32 & 0x3F)) as u8;
        idx += 1;
    }
    idx
}

fn is_utf8_trail(c: u8) -> bool {
    c & 0xC0 == 0x80
}

/*
 * Step forward or backward one character in a string.
 *
 * retruns the idenified wide char and the updated position
 */
pub unsafe extern "C" fn step_charc(
    s: &[u8],
    dir: i32,
    start: usize,
    limit: usize,
) -> (char, usize) {
    let mut ch = '\0';
    let mut len = 0;
    let mut s_idx = start;

    if !utf_mode {
        /* It's easy if chars are one byte. */
        if dir > 0 {
            ch = if s_idx < limit {
                s[s_idx] as char
            } else {
                '\0'
            };
            s_idx += 1;
        } else {
            ch = if s_idx > limit {
                s_idx -= 1;
                s[s_idx] as char
            } else {
                '\0'
            }
        }
    } else if dir > 0 {
        if s_idx >= limit {
            ch = '\0';
        } else {
            len = utf_len(s[s_idx]);
            if len as usize > limit || is_utf8_well_formed(s, len) as u64 == 0 {
                ch = s[s_idx] as char;
                s_idx += 1;
            } else {
                ch = get_wchar(&s);
                s_idx += len as usize;
            }
        }
    } else {
        while s_idx > limit && is_utf8_trail(s[s_idx - 1]) {
            s_idx -= 1;
        }
        if s_idx <= limit {
            ch = '\0';
        } else {
            s_idx -= 1;
            len = utf_len(s[s_idx]);
            if s_idx + len != start || !is_utf8_well_formed(&s[s_idx..], len) {
                s_idx = start - 1;
                ch = s[s_idx] as char;
            } else {
                ch = get_wchar(&s[s_idx..]);
            }
        }
    }
    return (ch, s_idx);
}

pub unsafe extern "C" fn step_char(
    p: &[u8],
    dir: i32,
    start: usize,
    limit: usize,
) -> (char, usize) {
    step_charc(p, dir, start, limit)
}

static mut comb_table: LazyLock<Vec<wchar_range>> = LazyLock::new(|| {
    vec![
        wchar_range {
            first: char::from_u32(0x644).unwrap(),
            last: char::from_u32(0x622).unwrap(),
        },
        wchar_range {
            first: char::from_u32(0x644).unwrap(),
            last: char::from_u32(0x623).unwrap(),
        },
        wchar_range {
            first: char::from_u32(0x644).unwrap(),
            last: char::from_u32(0x625).unwrap(),
        },
        wchar_range {
            first: char::from_u32(0x644).unwrap(),
            last: char::from_u32(0x627).unwrap(),
        },
    ]
});

unsafe extern "C" fn is_in_table(ch: char, table: &wchar_range_table) -> bool {
    let mut hi = 0;
    let mut lo = 0;
    if table.len() == 0 || ch < table[0].first {
        return false;
    }
    lo = 0;
    hi = table.len() - 1;
    while lo <= hi {
        let mut mid = (lo + hi) / 2;
        if ch > table[mid].last {
            lo = mid + 1;
        } else if ch < table[mid].first {
            hi = mid - 1;
        } else {
            return true;
        }
    }
    return false;
}

/*
 * Is a character a UTF-8 composing character?
 * If a composing character follows any char, the two combine into one glyph.
 */
pub unsafe extern "C" fn is_composing_char(ch: char) -> bool {
    if is_in_table(ch, &user_prt_table) {
        return false;
    }
    return is_in_table(ch, &user_compose_table)
        || is_in_table(ch, &compose_table)
        || (bs_mode != BS_CONTROL && is_in_table(ch, &fmt_table));
}

/*
 * Should this UTF-8 character be treated as binary?
 */
pub unsafe extern "C" fn is_ubin_char(ch: char) -> bool {
    if is_in_table(ch, &mut user_prt_table) {
        return false;
    }
    return is_in_table(ch, &mut user_ubin_table)
        || is_in_table(ch, &ubin_table)
        || bs_mode == BS_CONTROL && is_in_table(ch, &fmt_table);
}

/*
 * Is this a double width UTF-8 character?
 */
pub unsafe extern "C" fn is_wide_char(ch: char) -> bool {
    return is_in_table(ch, &user_wide_table) || is_in_table(ch, &wide_table);
}

/*
 * Is a character a UTF-8 combining character?
 * A combining char acts like an ordinary char, but if it follows
 * a specific char (not any char), the two combine into one glyph.
 */
pub unsafe extern "C" fn is_combining_char(mut ch1: char, mut ch2: char) -> bool {
    /* The table is small; use linear search. */
    for i in 0..comb_table.len() {
        if ch1 == comb_table[i].first && ch2 == comb_table[i].last {
            return true;
        }
    }
    return false;
}
