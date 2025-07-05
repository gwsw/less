use ::libc;
extern "C" {
    fn snprintf(
        _: *mut std::ffi::c_char,
        _: std::ffi::c_ulong,
        _: *const std::ffi::c_char,
        _: ...
    ) -> std::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const std::ffi::c_ushort;
    fn strcpy(
        _: *mut std::ffi::c_char,
        _: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_char;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strchr(_: *const std::ffi::c_char, _: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn strstr(
        _: *const std::ffi::c_char,
        _: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_char;
    fn lstrtoulc(
        _: *const std::ffi::c_char,
        _: *mut *const std::ffi::c_char,
        _: std::ffi::c_int,
    ) -> std::ffi::c_ulong;
    fn xbuf_init(xbuf: *mut xbuffer);
    fn xbuf_add_data(xbuf: *mut xbuffer, data: *const std::ffi::c_uchar, len: size_t);
    fn quit(status: std::ffi::c_int);
    fn lgetenv(var: *const std::ffi::c_char) -> *const std::ffi::c_char;
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
    pub first: LWCHAR,
    pub last: LWCHAR,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wchar_range_table {
    pub table: *mut wchar_range,
    pub count: std::ffi::c_uint,
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
    pub name: *mut std::ffi::c_char,
    pub p_flag: *mut std::ffi::c_int,
    pub desc: *mut std::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cs_alias {
    pub name: *mut std::ffi::c_char,
    pub oname: *mut std::ffi::c_char,
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
#[no_mangle]
pub static mut utf_mode: std::ffi::c_int = 0 as std::ffi::c_int;
#[no_mangle]
pub static mut charsets: [charset; 21] = unsafe {
    [
        {
            let mut init = charset {
                name: b"ascii\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"8bcccbcc18b95.b\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: b"utf-8\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: &utf_mode as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"8bcccbcc18b95.b126.bb\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"8bcccbcc18b95.33b.\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: b"latin3\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"8bcccbcc18b95.33b5.b8.b15.b4.b12.b18.b12.b.\0" as *const u8
                    as *const std::ffi::c_char as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: b"arabic\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"8bcccbcc18b95.33b.3b.7b2.13b.3b.b26.5b19.b\0" as *const u8
                    as *const std::ffi::c_char as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: b"greek\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"8bcccbcc18b95.33b4.2b4.b3.b35.b44.b\0" as *const u8
                    as *const std::ffi::c_char as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: b"greek2005\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"8bcccbcc18b95.33b14.b35.b44.b\0" as *const u8
                    as *const std::ffi::c_char as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: b"hebrew\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"8bcccbcc18b95.33b.b29.32b28.2b2.b\0" as *const u8
                    as *const std::ffi::c_char as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: b"koi8-r\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"8bcccbcc18b95.b.\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: b"KOI8-T\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"8bcccbcc18b95.b8.b6.b8.b.b.5b7.3b4.b4.b3.b.b.3b.\0" as *const u8
                    as *const std::ffi::c_char as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: b"georgianps\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"8bcccbcc18b95.3b11.4b12.2b.\0" as *const u8
                    as *const std::ffi::c_char as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: b"tcvn\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"b..b...bcccbccbbb7.8b95.b48.5b.\0" as *const u8
                    as *const std::ffi::c_char as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: b"TIS-620\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"8bcccbcc18b95.b.4b.11b7.8b.\0" as *const u8
                    as *const std::ffi::c_char as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: b"next\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"8bcccbcc18b95.bb125.bb\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: b"dos\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"8bcccbcc12bc5b95.b.\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: b"windows-1251\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"8bcccbcc12bc5b95.b24.b.\0" as *const u8
                    as *const std::ffi::c_char as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: b"windows-1252\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"8bcccbcc12bc5b95.b.b11.b.2b12.b.\0" as *const u8
                    as *const std::ffi::c_char as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: b"windows-1255\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"8bcccbcc12bc5b95.b.b8.b.5b9.b.4b.\0" as *const u8
                    as *const std::ffi::c_char as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: b"ebcdic\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"5bc6bcc7bcc41b.9b7.9b5.b..8b6.10b6.b9.7b9.8b8.17b3.3b9.7b9.8b8.6b10.b.b.b.\0"
                    as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: b"IBM-1047\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: b"4cbcbc3b9cbccbccbb4c6bcc5b3cbbc4bc4bccbc191.b\0" as *const u8
                    as *const std::ffi::c_char as *mut std::ffi::c_char,
            };
            init
        },
        {
            let mut init = charset {
                name: 0 as *const std::ffi::c_char as *mut std::ffi::c_char,
                p_flag: 0 as *const std::ffi::c_int as *mut std::ffi::c_int,
                desc: 0 as *const std::ffi::c_char as *mut std::ffi::c_char,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut cs_aliases: [cs_alias; 43] = [
    {
        let mut init = cs_alias {
            name: b"UTF-8\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"utf-8\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"utf8\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"utf-8\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"UTF8\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"utf-8\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"ANSI_X3.4-1968\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"ascii\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"US-ASCII\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"ascii\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"latin1\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"ISO-8859-1\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"latin9\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"ISO-8859-15\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"latin2\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"ISO-8859-2\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"ISO-8859-3\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"latin3\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"latin4\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"ISO-8859-4\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"cyrillic\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"ISO-8859-5\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"ISO-8859-6\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"arabic\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"ISO-8859-7\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"greek\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"IBM9005\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"greek2005\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"ISO-8859-8\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"hebrew\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"latin5\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"ISO-8859-9\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"latin6\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"ISO-8859-10\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"latin7\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"ISO-8859-13\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"latin8\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"ISO-8859-14\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"latin10\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"ISO-8859-16\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"iso8859\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"IBM437\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"dos\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"EBCDIC-US\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"ebcdic\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"IBM1047\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"IBM-1047\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"KOI8-R\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"koi8-r\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"KOI8-U\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"koi8-r\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"GEORGIAN-PS\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"georgianps\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"TCVN5712-1\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"tcvn\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"NEXTSTEP\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"next\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"windows\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"windows-1252\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"CP1251\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"windows-1251\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"CP1252\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"windows-1252\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: b"CP1255\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
            oname: b"windows-1255\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = cs_alias {
            name: 0 as *const std::ffi::c_char as *mut std::ffi::c_char,
            oname: 0 as *const std::ffi::c_char as *mut std::ffi::c_char,
        };
        init
    },
];
static mut chardef: [std::ffi::c_char; 256] = [0; 256];
static mut binfmt: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
static mut utfbinfmt: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
#[no_mangle]
pub static mut binattr: std::ffi::c_int = (1 as std::ffi::c_int) << 3 as std::ffi::c_int
    | (2 as std::ffi::c_int) << 8 as std::ffi::c_int;
static mut user_wide_array: xbuffer = xbuffer {
    data: 0 as *const std::ffi::c_uchar as *mut std::ffi::c_uchar,
    end: 0,
    size: 0,
    init_size: 0,
};
static mut user_ubin_array: xbuffer = xbuffer {
    data: 0 as *const std::ffi::c_uchar as *mut std::ffi::c_uchar,
    end: 0,
    size: 0,
    init_size: 0,
};
static mut user_compose_array: xbuffer = xbuffer {
    data: 0 as *const std::ffi::c_uchar as *mut std::ffi::c_uchar,
    end: 0,
    size: 0,
    init_size: 0,
};
static mut user_prt_array: xbuffer = xbuffer {
    data: 0 as *const std::ffi::c_uchar as *mut std::ffi::c_uchar,
    end: 0,
    size: 0,
    init_size: 0,
};
static mut user_wide_table: wchar_range_table = wchar_range_table {
    table: 0 as *const wchar_range as *mut wchar_range,
    count: 0,
};
static mut user_ubin_table: wchar_range_table = wchar_range_table {
    table: 0 as *const wchar_range as *mut wchar_range,
    count: 0,
};
static mut user_compose_table: wchar_range_table = wchar_range_table {
    table: 0 as *const wchar_range as *mut wchar_range,
    count: 0,
};
static mut user_prt_table: wchar_range_table = wchar_range_table {
    table: 0 as *const wchar_range as *mut wchar_range,
    count: 0,
};
unsafe extern "C" fn wchar_range_table_set(
    mut tbl: *mut wchar_range_table,
    mut arr: *mut xbuffer,
) {
    (*tbl).table = (*arr).data as *mut wchar_range;
    (*tbl)
        .count = ((*arr).end)
        .wrapping_div(::core::mem::size_of::<wchar_range>() as std::ffi::c_ulong)
        as std::ffi::c_uint;
}
unsafe extern "C" fn skip_uprefix(
    mut s: *const std::ffi::c_char,
) -> *const std::ffi::c_char {
    if *s as std::ffi::c_int == 'U' as i32 || *s as std::ffi::c_int == 'u' as i32 {
        s = s.offset(1);
        if *s as std::ffi::c_int == '+' as i32 {
            s = s.offset(1);
            s;
        }
    }
    return s;
}
unsafe extern "C" fn wchar_range_get(
    mut ss: *mut *const std::ffi::c_char,
    mut range: *mut wchar_range,
) {
    let mut s: *const std::ffi::c_char = skip_uprefix(*ss);
    (*range).first = lstrtoulc(s, &mut s, 16 as std::ffi::c_int);
    if *s.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int == '-' as i32 {
        s = skip_uprefix(&*s.offset(1 as std::ffi::c_int as isize));
        (*range).last = lstrtoulc(s, &mut s, 16 as std::ffi::c_int);
    } else {
        (*range).last = (*range).first;
    }
    *ss = s;
}
unsafe extern "C" fn ichardef_utf(mut s: *const std::ffi::c_char) {
    xbuf_init(&mut user_wide_array);
    xbuf_init(&mut user_ubin_array);
    xbuf_init(&mut user_compose_array);
    xbuf_init(&mut user_prt_array);
    if !s.is_null() {
        while *s.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != '\0' as i32
        {
            let mut range: wchar_range = wchar_range { first: 0, last: 0 };
            wchar_range_get(&mut s, &mut range);
            if range.last == 0 as std::ffi::c_int as LWCHAR {
                error(
                    b"invalid hex number(s) in LESSUTFCHARDEF\0" as *const u8
                        as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                quit(1 as std::ffi::c_int);
            }
            let fresh0 = s;
            s = s.offset(1);
            if *fresh0 as std::ffi::c_int != ':' as i32 {
                error(
                    b"missing colon in LESSUTFCHARDEF\0" as *const u8
                        as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                quit(1 as std::ffi::c_int);
            }
            let fresh1 = s;
            s = s.offset(1);
            match *fresh1 as std::ffi::c_int {
                98 => {
                    xbuf_add_data(
                        &mut user_ubin_array,
                        &mut range as *mut wchar_range as *mut std::ffi::c_uchar,
                        ::core::mem::size_of::<wchar_range>() as std::ffi::c_ulong,
                    );
                }
                99 => {
                    xbuf_add_data(
                        &mut user_compose_array,
                        &mut range as *mut wchar_range as *mut std::ffi::c_uchar,
                        ::core::mem::size_of::<wchar_range>() as std::ffi::c_ulong,
                    );
                }
                119 => {
                    xbuf_add_data(
                        &mut user_wide_array,
                        &mut range as *mut wchar_range as *mut std::ffi::c_uchar,
                        ::core::mem::size_of::<wchar_range>() as std::ffi::c_ulong,
                    );
                    xbuf_add_data(
                        &mut user_prt_array,
                        &mut range as *mut wchar_range as *mut std::ffi::c_uchar,
                        ::core::mem::size_of::<wchar_range>() as std::ffi::c_ulong,
                    );
                }
                112 | 46 => {
                    xbuf_add_data(
                        &mut user_prt_array,
                        &mut range as *mut wchar_range as *mut std::ffi::c_uchar,
                        ::core::mem::size_of::<wchar_range>() as std::ffi::c_ulong,
                    );
                }
                0 => {
                    s = s.offset(-1);
                    s;
                }
                _ => {}
            }
            if *s.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int == ',' as i32
            {
                s = s.offset(1);
                s;
            }
        }
    }
    wchar_range_table_set(&mut user_wide_table, &mut user_wide_array);
    wchar_range_table_set(&mut user_ubin_table, &mut user_ubin_array);
    wchar_range_table_set(&mut user_compose_table, &mut user_compose_array);
    wchar_range_table_set(&mut user_prt_table, &mut user_prt_array);
}
unsafe extern "C" fn ichardef(mut s: *const std::ffi::c_char) {
    let mut cp: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut n: std::ffi::c_int = 0;
    let mut v: std::ffi::c_char = 0;
    n = 0 as std::ffi::c_int;
    v = 0 as std::ffi::c_int as std::ffi::c_char;
    cp = chardef.as_mut_ptr();
    let mut current_block_15: u64;
    while *s as std::ffi::c_int != '\0' as i32 {
        let fresh2 = s;
        s = s.offset(1);
        match *fresh2 as std::ffi::c_int {
            46 => {
                v = 0 as std::ffi::c_int as std::ffi::c_char;
                current_block_15 = 7149356873433890176;
            }
            99 => {
                v = 0o2 as std::ffi::c_int as std::ffi::c_char;
                current_block_15 = 7149356873433890176;
            }
            98 => {
                v = (0o1 as std::ffi::c_int | 0o2 as std::ffi::c_int)
                    as std::ffi::c_char;
                current_block_15 = 7149356873433890176;
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                let (fresh3, fresh4) = n.overflowing_mul(10 as std::ffi::c_int);
                *(&mut n as *mut std::ffi::c_int) = fresh3;
                if !(fresh4 as std::ffi::c_int != 0
                    || {
                        let (fresh5, fresh6) = n
                            .overflowing_add(
                                *s.offset(-(1 as std::ffi::c_int) as isize)
                                    as std::ffi::c_int - '0' as i32,
                            );
                        *(&mut n as *mut std::ffi::c_int) = fresh5;
                        fresh6 as std::ffi::c_int != 0
                    })
                {
                    continue;
                }
                current_block_15 = 5866350808638245930;
            }
            _ => {
                current_block_15 = 5866350808638245930;
            }
        }
        match current_block_15 {
            5866350808638245930 => {
                error(
                    b"invalid chardef\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                quit(1 as std::ffi::c_int);
            }
            _ => {}
        }
        loop {
            if cp
                >= chardef
                    .as_mut_ptr()
                    .offset(
                        ::core::mem::size_of::<[std::ffi::c_char; 256]>()
                            as std::ffi::c_ulong as isize,
                    )
            {
                error(
                    b"chardef longer than 256\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                quit(1 as std::ffi::c_int);
            }
            let fresh7 = cp;
            cp = cp.offset(1);
            *fresh7 = v;
            n -= 1;
            if !(n > 0 as std::ffi::c_int) {
                break;
            }
        }
        n = 0 as std::ffi::c_int;
    }
    while cp
        < chardef
            .as_mut_ptr()
            .offset(
                ::core::mem::size_of::<[std::ffi::c_char; 256]>() as std::ffi::c_ulong
                    as isize,
            )
    {
        let fresh8 = cp;
        cp = cp.offset(1);
        *fresh8 = v;
    }
}
unsafe extern "C" fn icharset(
    mut name: *const std::ffi::c_char,
    mut no_error: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut p: *mut charset = 0 as *mut charset;
    let mut a: *mut cs_alias = 0 as *mut cs_alias;
    if name.is_null() || *name as std::ffi::c_int == '\0' as i32 {
        return 0 as std::ffi::c_int;
    }
    a = cs_aliases.as_mut_ptr();
    while !((*a).name).is_null() {
        if strcmp(name, (*a).name) == 0 as std::ffi::c_int {
            name = (*a).oname;
            break;
        } else {
            a = a.offset(1);
            a;
        }
    }
    p = charsets.as_mut_ptr();
    while !((*p).name).is_null() {
        if strcmp(name, (*p).name) == 0 as std::ffi::c_int {
            ichardef((*p).desc);
            if !((*p).p_flag).is_null() {
                *(*p).p_flag = 1 as std::ffi::c_int;
            }
            return 1 as std::ffi::c_int;
        }
        p = p.offset(1);
        p;
    }
    if no_error == 0 {
        error(
            b"invalid charset name\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        quit(1 as std::ffi::c_int);
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn ilocale() {
    let mut c: std::ffi::c_int = 0;
    c = 0 as std::ffi::c_int;
    while c
        < ::core::mem::size_of::<[std::ffi::c_char; 256]>() as std::ffi::c_ulong
            as std::ffi::c_int
    {
        if *(*__ctype_b_loc()).offset(c as isize) as std::ffi::c_int
            & _ISprint as std::ffi::c_int as std::ffi::c_ushort as std::ffi::c_int != 0
        {
            chardef[c as usize] = 0 as std::ffi::c_int as std::ffi::c_char;
        } else if *(*__ctype_b_loc()).offset(c as isize) as std::ffi::c_int
            & _IScntrl as std::ffi::c_int as std::ffi::c_ushort as std::ffi::c_int != 0
        {
            chardef[c as usize] = 0o2 as std::ffi::c_int as std::ffi::c_char;
        } else {
            chardef[c
                as usize] = (0o1 as std::ffi::c_int | 0o2 as std::ffi::c_int)
                as std::ffi::c_char;
        }
        c += 1;
        c;
    }
}
#[no_mangle]
pub unsafe extern "C" fn setfmt(
    mut s: *const std::ffi::c_char,
    mut fmtvarptr: *mut *const std::ffi::c_char,
    mut attrptr: *mut std::ffi::c_int,
    mut default_fmt: *const std::ffi::c_char,
    mut for_printf: lbool,
) {
    if s.is_null() || *s as std::ffi::c_int == '\0' as i32 {
        s = default_fmt;
    } else if for_printf as std::ffi::c_uint != 0
        && (*s as std::ffi::c_int == '*' as i32
            && (*s.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                == '\0' as i32
                || *s.offset(2 as std::ffi::c_int as isize) as std::ffi::c_int
                    == '\0' as i32
                || !(strchr(s.offset(2 as std::ffi::c_int as isize), 'n' as i32))
                    .is_null())
            || *s as std::ffi::c_int != '*' as i32 && !(strchr(s, 'n' as i32)).is_null())
    {
        s = default_fmt;
    }
    if *s as std::ffi::c_int == '*' as i32
        && *s.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int != '\0' as i32
    {
        match *s.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int {
            100 => {
                *attrptr = (1 as std::ffi::c_int) << 1 as std::ffi::c_int;
            }
            107 => {
                *attrptr = (1 as std::ffi::c_int) << 2 as std::ffi::c_int;
            }
            115 => {
                *attrptr = (1 as std::ffi::c_int) << 3 as std::ffi::c_int;
            }
            117 => {
                *attrptr = (1 as std::ffi::c_int) << 0 as std::ffi::c_int;
            }
            _ => {
                *attrptr = 0 as std::ffi::c_int;
            }
        }
        s = s.offset(2 as std::ffi::c_int as isize);
    }
    *fmtvarptr = s;
}
unsafe extern "C" fn set_charset() {
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    ichardef_utf(lgetenv(b"LESSUTFCHARDEF\0" as *const u8 as *const std::ffi::c_char));
    s = lgetenv(b"LESSCHARSET\0" as *const u8 as *const std::ffi::c_char);
    if icharset(s, 0 as std::ffi::c_int) != 0 {
        return;
    }
    s = lgetenv(b"LESSCHARDEF\0" as *const u8 as *const std::ffi::c_char);
    if isnullenv(s) as u64 == 0 {
        ichardef(s);
        return;
    }
    s = nl_langinfo(CODESET as std::ffi::c_int);
    if icharset(s, 1 as std::ffi::c_int) != 0 {
        return;
    }
    s = lgetenv(b"LC_ALL\0" as *const u8 as *const std::ffi::c_char);
    if !s.is_null()
        || {
            s = lgetenv(b"LC_CTYPE\0" as *const u8 as *const std::ffi::c_char);
            !s.is_null()
        }
        || {
            s = lgetenv(b"LANG\0" as *const u8 as *const std::ffi::c_char);
            !s.is_null()
        }
    {
        if !(strstr(s, b"UTF-8\0" as *const u8 as *const std::ffi::c_char)).is_null()
            || !(strstr(s, b"utf-8\0" as *const u8 as *const std::ffi::c_char)).is_null()
            || !(strstr(s, b"UTF8\0" as *const u8 as *const std::ffi::c_char)).is_null()
            || !(strstr(s, b"utf8\0" as *const u8 as *const std::ffi::c_char)).is_null()
        {
            if icharset(
                b"utf-8\0" as *const u8 as *const std::ffi::c_char,
                1 as std::ffi::c_int,
            ) != 0
            {
                return;
            }
        }
    }
    ilocale();
}
#[no_mangle]
pub unsafe extern "C" fn init_charset() {
    let mut s: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    setlocale(6 as std::ffi::c_int, b"\0" as *const u8 as *const std::ffi::c_char);
    set_charset();
    s = lgetenv(b"LESSBINFMT\0" as *const u8 as *const std::ffi::c_char);
    setfmt(
        s,
        &mut binfmt,
        &mut binattr,
        b"*s<%02X>\0" as *const u8 as *const std::ffi::c_char,
        LTRUE,
    );
    s = lgetenv(b"LESSUTFBINFMT\0" as *const u8 as *const std::ffi::c_char);
    setfmt(
        s,
        &mut utfbinfmt,
        &mut binattr,
        b"<U+%04lX>\0" as *const u8 as *const std::ffi::c_char,
        LTRUE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binary_char(mut c: LWCHAR) -> lbool {
    if utf_mode != 0 {
        return is_ubin_char(c);
    }
    if c >= ::core::mem::size_of::<[std::ffi::c_char; 256]>() as std::ffi::c_ulong {
        return LTRUE;
    }
    return (chardef[c as usize] as std::ffi::c_int & 0o1 as std::ffi::c_int
        != 0 as std::ffi::c_int) as std::ffi::c_int as lbool;
}
#[no_mangle]
pub unsafe extern "C" fn control_char(mut c: LWCHAR) -> lbool {
    if c >= ::core::mem::size_of::<[std::ffi::c_char; 256]>() as std::ffi::c_ulong {
        return LTRUE;
    }
    return (chardef[c as usize] as std::ffi::c_int & 0o2 as std::ffi::c_int) as lbool;
}
#[no_mangle]
pub unsafe extern "C" fn prchar(mut c: LWCHAR) -> *const std::ffi::c_char {
    static mut buf: [std::ffi::c_char; 32] = [0; 32];
    c &= 0o377 as std::ffi::c_int as LWCHAR;
    if (c < 128 as std::ffi::c_int as LWCHAR || utf_mode == 0)
        && control_char(c) as u64 == 0
    {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[std::ffi::c_char; 32]>() as std::ffi::c_ulong,
            b"%c\0" as *const u8 as *const std::ffi::c_char,
            c as std::ffi::c_int,
        );
    } else if c == ('[' as i32 & 0o37 as std::ffi::c_int) as LWCHAR {
        strcpy(buf.as_mut_ptr(), b"ESC\0" as *const u8 as *const std::ffi::c_char);
    } else if c < 128 as std::ffi::c_int as LWCHAR
        && control_char(c ^ 0o100 as std::ffi::c_int as LWCHAR) as u64 == 0
    {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[std::ffi::c_char; 32]>() as std::ffi::c_ulong,
            b"^%c\0" as *const u8 as *const std::ffi::c_char,
            (c ^ 0o100 as std::ffi::c_int as LWCHAR) as std::ffi::c_int,
        );
    } else {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[std::ffi::c_char; 32]>() as std::ffi::c_ulong,
            binfmt,
            c,
        );
    }
    return buf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn prutfchar(mut ch: LWCHAR) -> *const std::ffi::c_char {
    static mut buf: [std::ffi::c_char; 32] = [0; 32];
    if ch == ('[' as i32 & 0o37 as std::ffi::c_int) as LWCHAR {
        strcpy(buf.as_mut_ptr(), b"ESC\0" as *const u8 as *const std::ffi::c_char);
    } else if ch < 128 as std::ffi::c_int as LWCHAR
        && control_char(ch) as std::ffi::c_uint != 0
    {
        if control_char(ch ^ 0o100 as std::ffi::c_int as LWCHAR) as u64 == 0 {
            snprintf(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[std::ffi::c_char; 32]>() as std::ffi::c_ulong,
                b"^%c\0" as *const u8 as *const std::ffi::c_char,
                ch as std::ffi::c_char as std::ffi::c_int ^ 0o100 as std::ffi::c_int,
            );
        } else {
            snprintf(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[std::ffi::c_char; 32]>() as std::ffi::c_ulong,
                binfmt,
                ch as std::ffi::c_char as std::ffi::c_int,
            );
        }
    } else if is_ubin_char(ch) as u64 != 0 {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[std::ffi::c_char; 32]>() as std::ffi::c_ulong,
            utfbinfmt,
            ch,
        );
    } else {
        let mut p: *mut std::ffi::c_char = buf.as_mut_ptr();
        if ch >= 0x80000000 as std::ffi::c_uint as LWCHAR {
            ch = 0xfffd as std::ffi::c_int as LWCHAR;
        }
        put_wchar(&mut p, ch);
        *p = '\0' as i32 as std::ffi::c_char;
    }
    return buf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn utf_len(mut ch: std::ffi::c_char) -> std::ffi::c_int {
    if ch as std::ffi::c_int & 0x80 as std::ffi::c_int == 0 as std::ffi::c_int {
        return 1 as std::ffi::c_int;
    }
    if ch as std::ffi::c_int & 0xe0 as std::ffi::c_int == 0xc0 as std::ffi::c_int {
        return 2 as std::ffi::c_int;
    }
    if ch as std::ffi::c_int & 0xf0 as std::ffi::c_int == 0xe0 as std::ffi::c_int {
        return 3 as std::ffi::c_int;
    }
    if ch as std::ffi::c_int & 0xf8 as std::ffi::c_int == 0xf0 as std::ffi::c_int {
        return 4 as std::ffi::c_int;
    }
    return 1 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn is_utf8_well_formed(
    mut ss: *const std::ffi::c_char,
    mut slen: std::ffi::c_int,
) -> lbool {
    let mut i: std::ffi::c_int = 0;
    let mut len: std::ffi::c_int = 0;
    let mut s0: std::ffi::c_uchar = *ss.offset(0 as std::ffi::c_int as isize)
        as std::ffi::c_uchar;
    if s0 as std::ffi::c_int & 0xfe as std::ffi::c_int == 0xfe as std::ffi::c_int {
        return LFALSE;
    }
    len = utf_len(*ss.offset(0 as std::ffi::c_int as isize));
    if len > slen {
        return LFALSE;
    }
    if len == 1 as std::ffi::c_int {
        return LTRUE;
    }
    if len == 2 as std::ffi::c_int {
        if (s0 as std::ffi::c_int) < 0xc2 as std::ffi::c_int {
            return LFALSE;
        }
    } else {
        let mut mask: std::ffi::c_uchar = !(((1 as std::ffi::c_int)
            << 8 as std::ffi::c_int - len) - 1 as std::ffi::c_int) as std::ffi::c_uchar;
        if s0 as std::ffi::c_int == mask as std::ffi::c_int
            && *ss.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                & mask as std::ffi::c_int == 0x80 as std::ffi::c_int
        {
            return LFALSE;
        }
    }
    i = 1 as std::ffi::c_int;
    while i < len {
        if !(*ss.offset(i as isize) as std::ffi::c_int & 0xc0 as std::ffi::c_int
            == 0x80 as std::ffi::c_int)
        {
            return LFALSE;
        }
        i += 1;
        i;
    }
    return LTRUE;
}
#[no_mangle]
pub unsafe extern "C" fn utf_skip_to_lead(
    mut pp: *mut *const std::ffi::c_char,
    mut limit: *const std::ffi::c_char,
) {
    loop {
        *pp = (*pp).offset(1);
        *pp;
        if !(*pp < limit
            && !(*(*pp).offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                & 0o377 as std::ffi::c_int & 0xc0 as std::ffi::c_int
                == 0xc0 as std::ffi::c_int
                && !(*(*pp).offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                    & 0o377 as std::ffi::c_int & 0xfe as std::ffi::c_int
                    == 0xfe as std::ffi::c_int))
            && !(*(*pp).offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                & 0x80 as std::ffi::c_int == 0 as std::ffi::c_int))
        {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_wchar(mut sp: *const std::ffi::c_char) -> LWCHAR {
    let mut p: *const std::ffi::c_uchar = sp as *const std::ffi::c_uchar;
    match utf_len(*sp.offset(0 as std::ffi::c_int as isize)) {
        2 => {
            return ((*p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                & 0x1f as std::ffi::c_int) << 6 as std::ffi::c_int
                | *p.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                    & 0x3f as std::ffi::c_int) as LWCHAR;
        }
        3 => {
            return ((*p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                & 0xf as std::ffi::c_int) << 12 as std::ffi::c_int
                | (*p.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                    & 0x3f as std::ffi::c_int) << 6 as std::ffi::c_int
                | *p.offset(2 as std::ffi::c_int as isize) as std::ffi::c_int
                    & 0x3f as std::ffi::c_int) as LWCHAR;
        }
        4 => {
            return ((*p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                & 0x7 as std::ffi::c_int) << 18 as std::ffi::c_int
                | (*p.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                    & 0x3f as std::ffi::c_int) << 12 as std::ffi::c_int
                | (*p.offset(2 as std::ffi::c_int as isize) as std::ffi::c_int
                    & 0x3f as std::ffi::c_int) << 6 as std::ffi::c_int
                | *p.offset(3 as std::ffi::c_int as isize) as std::ffi::c_int
                    & 0x3f as std::ffi::c_int) as LWCHAR;
        }
        1 | _ => {
            return (*p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                & 0xff as std::ffi::c_int) as LWCHAR;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn put_wchar(mut pp: *mut *mut std::ffi::c_char, mut ch: LWCHAR) {
    if utf_mode == 0 || ch < 0x80 as std::ffi::c_int as LWCHAR {
        let fresh9 = *pp;
        *pp = (*pp).offset(1);
        *fresh9 = ch as std::ffi::c_char;
    } else if ch < 0x800 as std::ffi::c_int as LWCHAR {
        let fresh10 = *pp;
        *pp = (*pp).offset(1);
        *fresh10 = (0xc0 as std::ffi::c_int as LWCHAR
            | ch >> 6 as std::ffi::c_int & 0x1f as std::ffi::c_int as LWCHAR)
            as std::ffi::c_char;
        let fresh11 = *pp;
        *pp = (*pp).offset(1);
        *fresh11 = (0x80 as std::ffi::c_int as LWCHAR
            | ch & 0x3f as std::ffi::c_int as LWCHAR) as std::ffi::c_char;
    } else if ch < 0x10000 as std::ffi::c_int as LWCHAR {
        let fresh12 = *pp;
        *pp = (*pp).offset(1);
        *fresh12 = (0xe0 as std::ffi::c_int as LWCHAR
            | ch >> 12 as std::ffi::c_int & 0xf as std::ffi::c_int as LWCHAR)
            as std::ffi::c_char;
        let fresh13 = *pp;
        *pp = (*pp).offset(1);
        *fresh13 = (0x80 as std::ffi::c_int as LWCHAR
            | ch >> 6 as std::ffi::c_int & 0x3f as std::ffi::c_int as LWCHAR)
            as std::ffi::c_char;
        let fresh14 = *pp;
        *pp = (*pp).offset(1);
        *fresh14 = (0x80 as std::ffi::c_int as LWCHAR
            | ch & 0x3f as std::ffi::c_int as LWCHAR) as std::ffi::c_char;
    } else if ch < 0x200000 as std::ffi::c_int as LWCHAR {
        let fresh15 = *pp;
        *pp = (*pp).offset(1);
        *fresh15 = (0xf0 as std::ffi::c_int as LWCHAR
            | ch >> 18 as std::ffi::c_int & 0x7 as std::ffi::c_int as LWCHAR)
            as std::ffi::c_char;
        let fresh16 = *pp;
        *pp = (*pp).offset(1);
        *fresh16 = (0x80 as std::ffi::c_int as LWCHAR
            | ch >> 12 as std::ffi::c_int & 0x3f as std::ffi::c_int as LWCHAR)
            as std::ffi::c_char;
        let fresh17 = *pp;
        *pp = (*pp).offset(1);
        *fresh17 = (0x80 as std::ffi::c_int as LWCHAR
            | ch >> 6 as std::ffi::c_int & 0x3f as std::ffi::c_int as LWCHAR)
            as std::ffi::c_char;
        let fresh18 = *pp;
        *pp = (*pp).offset(1);
        *fresh18 = (0x80 as std::ffi::c_int as LWCHAR
            | ch & 0x3f as std::ffi::c_int as LWCHAR) as std::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn step_charc(
    mut pp: *mut *const std::ffi::c_char,
    mut dir: std::ffi::c_int,
    mut limit: *const std::ffi::c_char,
) -> LWCHAR {
    let mut ch: LWCHAR = 0;
    let mut len: std::ffi::c_int = 0;
    let mut p: *const std::ffi::c_char = *pp;
    if utf_mode == 0 {
        if dir > 0 as std::ffi::c_int {
            ch = (if p < limit {
                let fresh19 = p;
                p = p.offset(1);
                *fresh19 as std::ffi::c_int
            } else {
                0 as std::ffi::c_int
            }) as std::ffi::c_uchar as LWCHAR;
        } else {
            ch = (if p > limit {
                p = p.offset(-1);
                *p as std::ffi::c_int
            } else {
                0 as std::ffi::c_int
            }) as std::ffi::c_uchar as LWCHAR;
        }
    } else if dir > 0 as std::ffi::c_int {
        if p >= limit {
            ch = 0 as std::ffi::c_int as LWCHAR;
        } else {
            len = utf_len(*p);
            if p.offset(len as isize) > limit || is_utf8_well_formed(p, len) as u64 == 0
            {
                let fresh20 = p;
                p = p.offset(1);
                ch = *fresh20 as std::ffi::c_uchar as LWCHAR;
            } else {
                ch = get_wchar(p);
                p = p.offset(len as isize);
            }
        }
    } else {
        while p > limit
            && *p.offset(-(1 as std::ffi::c_int) as isize) as std::ffi::c_int
                & 0xc0 as std::ffi::c_int == 0x80 as std::ffi::c_int
        {
            p = p.offset(-1);
            p;
        }
        if p <= limit {
            ch = 0 as std::ffi::c_int as LWCHAR;
        } else {
            p = p.offset(-1);
            len = utf_len(*p);
            if p.offset(len as isize) != *pp || is_utf8_well_formed(p, len) as u64 == 0 {
                p = (*pp).offset(-(1 as std::ffi::c_int as isize));
                ch = *p as std::ffi::c_uchar as LWCHAR;
            } else {
                ch = get_wchar(p);
            }
        }
    }
    *pp = p;
    return ch;
}
#[no_mangle]
pub unsafe extern "C" fn step_char(
    mut pp: *mut *mut std::ffi::c_char,
    mut dir: std::ffi::c_int,
    mut limit: *const std::ffi::c_char,
) -> LWCHAR {
    let mut p: *const std::ffi::c_char = *pp as *const std::ffi::c_char;
    let mut ch: LWCHAR = step_charc(&mut p, dir, limit);
    *pp = p as *mut std::ffi::c_char;
    return ch;
}
static mut compose_array: [wchar_range; 365] = [
    {
        let mut init = wchar_range {
            first: 0x300 as std::ffi::c_int as LWCHAR,
            last: 0x36f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x483 as std::ffi::c_int as LWCHAR,
            last: 0x487 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x488 as std::ffi::c_int as LWCHAR,
            last: 0x489 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x591 as std::ffi::c_int as LWCHAR,
            last: 0x5bd as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x5bf as std::ffi::c_int as LWCHAR,
            last: 0x5bf as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x5c1 as std::ffi::c_int as LWCHAR,
            last: 0x5c2 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x5c4 as std::ffi::c_int as LWCHAR,
            last: 0x5c5 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x5c7 as std::ffi::c_int as LWCHAR,
            last: 0x5c7 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x610 as std::ffi::c_int as LWCHAR,
            last: 0x61a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x64b as std::ffi::c_int as LWCHAR,
            last: 0x65f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x670 as std::ffi::c_int as LWCHAR,
            last: 0x670 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x6d6 as std::ffi::c_int as LWCHAR,
            last: 0x6dc as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x6df as std::ffi::c_int as LWCHAR,
            last: 0x6e4 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x6e7 as std::ffi::c_int as LWCHAR,
            last: 0x6e8 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x6ea as std::ffi::c_int as LWCHAR,
            last: 0x6ed as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x711 as std::ffi::c_int as LWCHAR,
            last: 0x711 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x730 as std::ffi::c_int as LWCHAR,
            last: 0x74a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x7a6 as std::ffi::c_int as LWCHAR,
            last: 0x7b0 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x7eb as std::ffi::c_int as LWCHAR,
            last: 0x7f3 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x7fd as std::ffi::c_int as LWCHAR,
            last: 0x7fd as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x816 as std::ffi::c_int as LWCHAR,
            last: 0x819 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x81b as std::ffi::c_int as LWCHAR,
            last: 0x823 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x825 as std::ffi::c_int as LWCHAR,
            last: 0x827 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x829 as std::ffi::c_int as LWCHAR,
            last: 0x82d as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x859 as std::ffi::c_int as LWCHAR,
            last: 0x85b as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x897 as std::ffi::c_int as LWCHAR,
            last: 0x89f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x8ca as std::ffi::c_int as LWCHAR,
            last: 0x8e1 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x8e3 as std::ffi::c_int as LWCHAR,
            last: 0x902 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x93a as std::ffi::c_int as LWCHAR,
            last: 0x93a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x93c as std::ffi::c_int as LWCHAR,
            last: 0x93c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x941 as std::ffi::c_int as LWCHAR,
            last: 0x948 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x94d as std::ffi::c_int as LWCHAR,
            last: 0x94d as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x951 as std::ffi::c_int as LWCHAR,
            last: 0x957 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x962 as std::ffi::c_int as LWCHAR,
            last: 0x963 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x981 as std::ffi::c_int as LWCHAR,
            last: 0x981 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x9bc as std::ffi::c_int as LWCHAR,
            last: 0x9bc as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x9c1 as std::ffi::c_int as LWCHAR,
            last: 0x9c4 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x9cd as std::ffi::c_int as LWCHAR,
            last: 0x9cd as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x9e2 as std::ffi::c_int as LWCHAR,
            last: 0x9e3 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x9fe as std::ffi::c_int as LWCHAR,
            last: 0x9fe as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa01 as std::ffi::c_int as LWCHAR,
            last: 0xa02 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa3c as std::ffi::c_int as LWCHAR,
            last: 0xa3c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa41 as std::ffi::c_int as LWCHAR,
            last: 0xa42 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa47 as std::ffi::c_int as LWCHAR,
            last: 0xa48 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa4b as std::ffi::c_int as LWCHAR,
            last: 0xa4d as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa51 as std::ffi::c_int as LWCHAR,
            last: 0xa51 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa70 as std::ffi::c_int as LWCHAR,
            last: 0xa71 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa75 as std::ffi::c_int as LWCHAR,
            last: 0xa75 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa81 as std::ffi::c_int as LWCHAR,
            last: 0xa82 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xabc as std::ffi::c_int as LWCHAR,
            last: 0xabc as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xac1 as std::ffi::c_int as LWCHAR,
            last: 0xac5 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xac7 as std::ffi::c_int as LWCHAR,
            last: 0xac8 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xacd as std::ffi::c_int as LWCHAR,
            last: 0xacd as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xae2 as std::ffi::c_int as LWCHAR,
            last: 0xae3 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xafa as std::ffi::c_int as LWCHAR,
            last: 0xaff as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xb01 as std::ffi::c_int as LWCHAR,
            last: 0xb01 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xb3c as std::ffi::c_int as LWCHAR,
            last: 0xb3c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xb3f as std::ffi::c_int as LWCHAR,
            last: 0xb3f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xb41 as std::ffi::c_int as LWCHAR,
            last: 0xb44 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xb4d as std::ffi::c_int as LWCHAR,
            last: 0xb4d as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xb55 as std::ffi::c_int as LWCHAR,
            last: 0xb56 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xb62 as std::ffi::c_int as LWCHAR,
            last: 0xb63 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xb82 as std::ffi::c_int as LWCHAR,
            last: 0xb82 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xbc0 as std::ffi::c_int as LWCHAR,
            last: 0xbc0 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xbcd as std::ffi::c_int as LWCHAR,
            last: 0xbcd as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xc00 as std::ffi::c_int as LWCHAR,
            last: 0xc00 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xc04 as std::ffi::c_int as LWCHAR,
            last: 0xc04 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xc3c as std::ffi::c_int as LWCHAR,
            last: 0xc3c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xc3e as std::ffi::c_int as LWCHAR,
            last: 0xc40 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xc46 as std::ffi::c_int as LWCHAR,
            last: 0xc48 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xc4a as std::ffi::c_int as LWCHAR,
            last: 0xc4d as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xc55 as std::ffi::c_int as LWCHAR,
            last: 0xc56 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xc62 as std::ffi::c_int as LWCHAR,
            last: 0xc63 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xc81 as std::ffi::c_int as LWCHAR,
            last: 0xc81 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xcbc as std::ffi::c_int as LWCHAR,
            last: 0xcbc as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xcbf as std::ffi::c_int as LWCHAR,
            last: 0xcbf as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xcc6 as std::ffi::c_int as LWCHAR,
            last: 0xcc6 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xccc as std::ffi::c_int as LWCHAR,
            last: 0xccd as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xce2 as std::ffi::c_int as LWCHAR,
            last: 0xce3 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xd00 as std::ffi::c_int as LWCHAR,
            last: 0xd01 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xd3b as std::ffi::c_int as LWCHAR,
            last: 0xd3c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xd41 as std::ffi::c_int as LWCHAR,
            last: 0xd44 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xd4d as std::ffi::c_int as LWCHAR,
            last: 0xd4d as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xd62 as std::ffi::c_int as LWCHAR,
            last: 0xd63 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xd81 as std::ffi::c_int as LWCHAR,
            last: 0xd81 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xdca as std::ffi::c_int as LWCHAR,
            last: 0xdca as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xdd2 as std::ffi::c_int as LWCHAR,
            last: 0xdd4 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xdd6 as std::ffi::c_int as LWCHAR,
            last: 0xdd6 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xe31 as std::ffi::c_int as LWCHAR,
            last: 0xe31 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xe34 as std::ffi::c_int as LWCHAR,
            last: 0xe3a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xe47 as std::ffi::c_int as LWCHAR,
            last: 0xe4e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xeb1 as std::ffi::c_int as LWCHAR,
            last: 0xeb1 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xeb4 as std::ffi::c_int as LWCHAR,
            last: 0xebc as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xec8 as std::ffi::c_int as LWCHAR,
            last: 0xece as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xf18 as std::ffi::c_int as LWCHAR,
            last: 0xf19 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xf35 as std::ffi::c_int as LWCHAR,
            last: 0xf35 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xf37 as std::ffi::c_int as LWCHAR,
            last: 0xf37 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xf39 as std::ffi::c_int as LWCHAR,
            last: 0xf39 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xf71 as std::ffi::c_int as LWCHAR,
            last: 0xf7e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xf80 as std::ffi::c_int as LWCHAR,
            last: 0xf84 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xf86 as std::ffi::c_int as LWCHAR,
            last: 0xf87 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xf8d as std::ffi::c_int as LWCHAR,
            last: 0xf97 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xf99 as std::ffi::c_int as LWCHAR,
            last: 0xfbc as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xfc6 as std::ffi::c_int as LWCHAR,
            last: 0xfc6 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x102d as std::ffi::c_int as LWCHAR,
            last: 0x1030 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1032 as std::ffi::c_int as LWCHAR,
            last: 0x1037 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1039 as std::ffi::c_int as LWCHAR,
            last: 0x103a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x103d as std::ffi::c_int as LWCHAR,
            last: 0x103e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1058 as std::ffi::c_int as LWCHAR,
            last: 0x1059 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x105e as std::ffi::c_int as LWCHAR,
            last: 0x1060 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1071 as std::ffi::c_int as LWCHAR,
            last: 0x1074 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1082 as std::ffi::c_int as LWCHAR,
            last: 0x1082 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1085 as std::ffi::c_int as LWCHAR,
            last: 0x1086 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x108d as std::ffi::c_int as LWCHAR,
            last: 0x108d as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x109d as std::ffi::c_int as LWCHAR,
            last: 0x109d as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1160 as std::ffi::c_int as LWCHAR,
            last: 0x11ff as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x135d as std::ffi::c_int as LWCHAR,
            last: 0x135f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1712 as std::ffi::c_int as LWCHAR,
            last: 0x1714 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1732 as std::ffi::c_int as LWCHAR,
            last: 0x1733 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1752 as std::ffi::c_int as LWCHAR,
            last: 0x1753 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1772 as std::ffi::c_int as LWCHAR,
            last: 0x1773 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x17b4 as std::ffi::c_int as LWCHAR,
            last: 0x17b5 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x17b7 as std::ffi::c_int as LWCHAR,
            last: 0x17bd as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x17c6 as std::ffi::c_int as LWCHAR,
            last: 0x17c6 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x17c9 as std::ffi::c_int as LWCHAR,
            last: 0x17d3 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x17dd as std::ffi::c_int as LWCHAR,
            last: 0x17dd as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x180b as std::ffi::c_int as LWCHAR,
            last: 0x180d as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x180f as std::ffi::c_int as LWCHAR,
            last: 0x180f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1885 as std::ffi::c_int as LWCHAR,
            last: 0x1886 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x18a9 as std::ffi::c_int as LWCHAR,
            last: 0x18a9 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1920 as std::ffi::c_int as LWCHAR,
            last: 0x1922 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1927 as std::ffi::c_int as LWCHAR,
            last: 0x1928 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1932 as std::ffi::c_int as LWCHAR,
            last: 0x1932 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1939 as std::ffi::c_int as LWCHAR,
            last: 0x193b as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1a17 as std::ffi::c_int as LWCHAR,
            last: 0x1a18 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1a1b as std::ffi::c_int as LWCHAR,
            last: 0x1a1b as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1a56 as std::ffi::c_int as LWCHAR,
            last: 0x1a56 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1a58 as std::ffi::c_int as LWCHAR,
            last: 0x1a5e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1a60 as std::ffi::c_int as LWCHAR,
            last: 0x1a60 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1a62 as std::ffi::c_int as LWCHAR,
            last: 0x1a62 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1a65 as std::ffi::c_int as LWCHAR,
            last: 0x1a6c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1a73 as std::ffi::c_int as LWCHAR,
            last: 0x1a7c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1a7f as std::ffi::c_int as LWCHAR,
            last: 0x1a7f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1ab0 as std::ffi::c_int as LWCHAR,
            last: 0x1abd as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1abe as std::ffi::c_int as LWCHAR,
            last: 0x1abe as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1abf as std::ffi::c_int as LWCHAR,
            last: 0x1ace as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1b00 as std::ffi::c_int as LWCHAR,
            last: 0x1b03 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1b34 as std::ffi::c_int as LWCHAR,
            last: 0x1b34 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1b36 as std::ffi::c_int as LWCHAR,
            last: 0x1b3a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1b3c as std::ffi::c_int as LWCHAR,
            last: 0x1b3c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1b42 as std::ffi::c_int as LWCHAR,
            last: 0x1b42 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1b6b as std::ffi::c_int as LWCHAR,
            last: 0x1b73 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1b80 as std::ffi::c_int as LWCHAR,
            last: 0x1b81 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1ba2 as std::ffi::c_int as LWCHAR,
            last: 0x1ba5 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1ba8 as std::ffi::c_int as LWCHAR,
            last: 0x1ba9 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1bab as std::ffi::c_int as LWCHAR,
            last: 0x1bad as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1be6 as std::ffi::c_int as LWCHAR,
            last: 0x1be6 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1be8 as std::ffi::c_int as LWCHAR,
            last: 0x1be9 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1bed as std::ffi::c_int as LWCHAR,
            last: 0x1bed as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1bef as std::ffi::c_int as LWCHAR,
            last: 0x1bf1 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1c2c as std::ffi::c_int as LWCHAR,
            last: 0x1c33 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1c36 as std::ffi::c_int as LWCHAR,
            last: 0x1c37 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1cd0 as std::ffi::c_int as LWCHAR,
            last: 0x1cd2 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1cd4 as std::ffi::c_int as LWCHAR,
            last: 0x1ce0 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1ce2 as std::ffi::c_int as LWCHAR,
            last: 0x1ce8 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1ced as std::ffi::c_int as LWCHAR,
            last: 0x1ced as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1cf4 as std::ffi::c_int as LWCHAR,
            last: 0x1cf4 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1cf8 as std::ffi::c_int as LWCHAR,
            last: 0x1cf9 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1dc0 as std::ffi::c_int as LWCHAR,
            last: 0x1dff as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x20d0 as std::ffi::c_int as LWCHAR,
            last: 0x20dc as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x20dd as std::ffi::c_int as LWCHAR,
            last: 0x20e0 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x20e1 as std::ffi::c_int as LWCHAR,
            last: 0x20e1 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x20e2 as std::ffi::c_int as LWCHAR,
            last: 0x20e4 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x20e5 as std::ffi::c_int as LWCHAR,
            last: 0x20f0 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2cef as std::ffi::c_int as LWCHAR,
            last: 0x2cf1 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2d7f as std::ffi::c_int as LWCHAR,
            last: 0x2d7f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2de0 as std::ffi::c_int as LWCHAR,
            last: 0x2dff as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x302a as std::ffi::c_int as LWCHAR,
            last: 0x302d as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x3099 as std::ffi::c_int as LWCHAR,
            last: 0x309a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa66f as std::ffi::c_int as LWCHAR,
            last: 0xa66f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa670 as std::ffi::c_int as LWCHAR,
            last: 0xa672 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa674 as std::ffi::c_int as LWCHAR,
            last: 0xa67d as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa69e as std::ffi::c_int as LWCHAR,
            last: 0xa69f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa6f0 as std::ffi::c_int as LWCHAR,
            last: 0xa6f1 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa802 as std::ffi::c_int as LWCHAR,
            last: 0xa802 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa806 as std::ffi::c_int as LWCHAR,
            last: 0xa806 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa80b as std::ffi::c_int as LWCHAR,
            last: 0xa80b as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa825 as std::ffi::c_int as LWCHAR,
            last: 0xa826 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa82c as std::ffi::c_int as LWCHAR,
            last: 0xa82c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa8c4 as std::ffi::c_int as LWCHAR,
            last: 0xa8c5 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa8e0 as std::ffi::c_int as LWCHAR,
            last: 0xa8f1 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa8ff as std::ffi::c_int as LWCHAR,
            last: 0xa8ff as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa926 as std::ffi::c_int as LWCHAR,
            last: 0xa92d as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa947 as std::ffi::c_int as LWCHAR,
            last: 0xa951 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa980 as std::ffi::c_int as LWCHAR,
            last: 0xa982 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa9b3 as std::ffi::c_int as LWCHAR,
            last: 0xa9b3 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa9b6 as std::ffi::c_int as LWCHAR,
            last: 0xa9b9 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa9bc as std::ffi::c_int as LWCHAR,
            last: 0xa9bd as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa9e5 as std::ffi::c_int as LWCHAR,
            last: 0xa9e5 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xaa29 as std::ffi::c_int as LWCHAR,
            last: 0xaa2e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xaa31 as std::ffi::c_int as LWCHAR,
            last: 0xaa32 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xaa35 as std::ffi::c_int as LWCHAR,
            last: 0xaa36 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xaa43 as std::ffi::c_int as LWCHAR,
            last: 0xaa43 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xaa4c as std::ffi::c_int as LWCHAR,
            last: 0xaa4c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xaa7c as std::ffi::c_int as LWCHAR,
            last: 0xaa7c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xaab0 as std::ffi::c_int as LWCHAR,
            last: 0xaab0 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xaab2 as std::ffi::c_int as LWCHAR,
            last: 0xaab4 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xaab7 as std::ffi::c_int as LWCHAR,
            last: 0xaab8 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xaabe as std::ffi::c_int as LWCHAR,
            last: 0xaabf as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xaac1 as std::ffi::c_int as LWCHAR,
            last: 0xaac1 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xaaec as std::ffi::c_int as LWCHAR,
            last: 0xaaed as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xaaf6 as std::ffi::c_int as LWCHAR,
            last: 0xaaf6 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xabe5 as std::ffi::c_int as LWCHAR,
            last: 0xabe5 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xabe8 as std::ffi::c_int as LWCHAR,
            last: 0xabe8 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xabed as std::ffi::c_int as LWCHAR,
            last: 0xabed as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xd7b0 as std::ffi::c_int as LWCHAR,
            last: 0xd7c6 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xd7cb as std::ffi::c_int as LWCHAR,
            last: 0xd7fb as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xfb1e as std::ffi::c_int as LWCHAR,
            last: 0xfb1e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xfe00 as std::ffi::c_int as LWCHAR,
            last: 0xfe0f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xfe20 as std::ffi::c_int as LWCHAR,
            last: 0xfe2f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x101fd as std::ffi::c_int as LWCHAR,
            last: 0x101fd as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x102e0 as std::ffi::c_int as LWCHAR,
            last: 0x102e0 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x10376 as std::ffi::c_int as LWCHAR,
            last: 0x1037a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x10a01 as std::ffi::c_int as LWCHAR,
            last: 0x10a03 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x10a05 as std::ffi::c_int as LWCHAR,
            last: 0x10a06 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x10a0c as std::ffi::c_int as LWCHAR,
            last: 0x10a0f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x10a38 as std::ffi::c_int as LWCHAR,
            last: 0x10a3a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x10a3f as std::ffi::c_int as LWCHAR,
            last: 0x10a3f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x10ae5 as std::ffi::c_int as LWCHAR,
            last: 0x10ae6 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x10d24 as std::ffi::c_int as LWCHAR,
            last: 0x10d27 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x10d69 as std::ffi::c_int as LWCHAR,
            last: 0x10d6d as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x10eab as std::ffi::c_int as LWCHAR,
            last: 0x10eac as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x10efc as std::ffi::c_int as LWCHAR,
            last: 0x10eff as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x10f46 as std::ffi::c_int as LWCHAR,
            last: 0x10f50 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x10f82 as std::ffi::c_int as LWCHAR,
            last: 0x10f85 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11001 as std::ffi::c_int as LWCHAR,
            last: 0x11001 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11038 as std::ffi::c_int as LWCHAR,
            last: 0x11046 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11070 as std::ffi::c_int as LWCHAR,
            last: 0x11070 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11073 as std::ffi::c_int as LWCHAR,
            last: 0x11074 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1107f as std::ffi::c_int as LWCHAR,
            last: 0x11081 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x110b3 as std::ffi::c_int as LWCHAR,
            last: 0x110b6 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x110b9 as std::ffi::c_int as LWCHAR,
            last: 0x110ba as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x110c2 as std::ffi::c_int as LWCHAR,
            last: 0x110c2 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11100 as std::ffi::c_int as LWCHAR,
            last: 0x11102 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11127 as std::ffi::c_int as LWCHAR,
            last: 0x1112b as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1112d as std::ffi::c_int as LWCHAR,
            last: 0x11134 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11173 as std::ffi::c_int as LWCHAR,
            last: 0x11173 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11180 as std::ffi::c_int as LWCHAR,
            last: 0x11181 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x111b6 as std::ffi::c_int as LWCHAR,
            last: 0x111be as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x111c9 as std::ffi::c_int as LWCHAR,
            last: 0x111cc as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x111cf as std::ffi::c_int as LWCHAR,
            last: 0x111cf as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1122f as std::ffi::c_int as LWCHAR,
            last: 0x11231 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11234 as std::ffi::c_int as LWCHAR,
            last: 0x11234 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11236 as std::ffi::c_int as LWCHAR,
            last: 0x11237 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1123e as std::ffi::c_int as LWCHAR,
            last: 0x1123e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11241 as std::ffi::c_int as LWCHAR,
            last: 0x11241 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x112df as std::ffi::c_int as LWCHAR,
            last: 0x112df as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x112e3 as std::ffi::c_int as LWCHAR,
            last: 0x112ea as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11300 as std::ffi::c_int as LWCHAR,
            last: 0x11301 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1133b as std::ffi::c_int as LWCHAR,
            last: 0x1133c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11340 as std::ffi::c_int as LWCHAR,
            last: 0x11340 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11366 as std::ffi::c_int as LWCHAR,
            last: 0x1136c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11370 as std::ffi::c_int as LWCHAR,
            last: 0x11374 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x113bb as std::ffi::c_int as LWCHAR,
            last: 0x113c0 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x113ce as std::ffi::c_int as LWCHAR,
            last: 0x113ce as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x113d0 as std::ffi::c_int as LWCHAR,
            last: 0x113d0 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x113d2 as std::ffi::c_int as LWCHAR,
            last: 0x113d2 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x113e1 as std::ffi::c_int as LWCHAR,
            last: 0x113e2 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11438 as std::ffi::c_int as LWCHAR,
            last: 0x1143f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11442 as std::ffi::c_int as LWCHAR,
            last: 0x11444 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11446 as std::ffi::c_int as LWCHAR,
            last: 0x11446 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1145e as std::ffi::c_int as LWCHAR,
            last: 0x1145e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x114b3 as std::ffi::c_int as LWCHAR,
            last: 0x114b8 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x114ba as std::ffi::c_int as LWCHAR,
            last: 0x114ba as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x114bf as std::ffi::c_int as LWCHAR,
            last: 0x114c0 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x114c2 as std::ffi::c_int as LWCHAR,
            last: 0x114c3 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x115b2 as std::ffi::c_int as LWCHAR,
            last: 0x115b5 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x115bc as std::ffi::c_int as LWCHAR,
            last: 0x115bd as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x115bf as std::ffi::c_int as LWCHAR,
            last: 0x115c0 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x115dc as std::ffi::c_int as LWCHAR,
            last: 0x115dd as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11633 as std::ffi::c_int as LWCHAR,
            last: 0x1163a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1163d as std::ffi::c_int as LWCHAR,
            last: 0x1163d as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1163f as std::ffi::c_int as LWCHAR,
            last: 0x11640 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x116ab as std::ffi::c_int as LWCHAR,
            last: 0x116ab as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x116ad as std::ffi::c_int as LWCHAR,
            last: 0x116ad as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x116b0 as std::ffi::c_int as LWCHAR,
            last: 0x116b5 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x116b7 as std::ffi::c_int as LWCHAR,
            last: 0x116b7 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1171d as std::ffi::c_int as LWCHAR,
            last: 0x1171d as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1171f as std::ffi::c_int as LWCHAR,
            last: 0x1171f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11722 as std::ffi::c_int as LWCHAR,
            last: 0x11725 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11727 as std::ffi::c_int as LWCHAR,
            last: 0x1172b as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1182f as std::ffi::c_int as LWCHAR,
            last: 0x11837 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11839 as std::ffi::c_int as LWCHAR,
            last: 0x1183a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1193b as std::ffi::c_int as LWCHAR,
            last: 0x1193c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1193e as std::ffi::c_int as LWCHAR,
            last: 0x1193e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11943 as std::ffi::c_int as LWCHAR,
            last: 0x11943 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x119d4 as std::ffi::c_int as LWCHAR,
            last: 0x119d7 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x119da as std::ffi::c_int as LWCHAR,
            last: 0x119db as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x119e0 as std::ffi::c_int as LWCHAR,
            last: 0x119e0 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11a01 as std::ffi::c_int as LWCHAR,
            last: 0x11a0a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11a33 as std::ffi::c_int as LWCHAR,
            last: 0x11a38 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11a3b as std::ffi::c_int as LWCHAR,
            last: 0x11a3e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11a47 as std::ffi::c_int as LWCHAR,
            last: 0x11a47 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11a51 as std::ffi::c_int as LWCHAR,
            last: 0x11a56 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11a59 as std::ffi::c_int as LWCHAR,
            last: 0x11a5b as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11a8a as std::ffi::c_int as LWCHAR,
            last: 0x11a96 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11a98 as std::ffi::c_int as LWCHAR,
            last: 0x11a99 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11c30 as std::ffi::c_int as LWCHAR,
            last: 0x11c36 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11c38 as std::ffi::c_int as LWCHAR,
            last: 0x11c3d as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11c3f as std::ffi::c_int as LWCHAR,
            last: 0x11c3f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11c92 as std::ffi::c_int as LWCHAR,
            last: 0x11ca7 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11caa as std::ffi::c_int as LWCHAR,
            last: 0x11cb0 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11cb2 as std::ffi::c_int as LWCHAR,
            last: 0x11cb3 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11cb5 as std::ffi::c_int as LWCHAR,
            last: 0x11cb6 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11d31 as std::ffi::c_int as LWCHAR,
            last: 0x11d36 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11d3a as std::ffi::c_int as LWCHAR,
            last: 0x11d3a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11d3c as std::ffi::c_int as LWCHAR,
            last: 0x11d3d as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11d3f as std::ffi::c_int as LWCHAR,
            last: 0x11d45 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11d47 as std::ffi::c_int as LWCHAR,
            last: 0x11d47 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11d90 as std::ffi::c_int as LWCHAR,
            last: 0x11d91 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11d95 as std::ffi::c_int as LWCHAR,
            last: 0x11d95 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11d97 as std::ffi::c_int as LWCHAR,
            last: 0x11d97 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11ef3 as std::ffi::c_int as LWCHAR,
            last: 0x11ef4 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11f00 as std::ffi::c_int as LWCHAR,
            last: 0x11f01 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11f36 as std::ffi::c_int as LWCHAR,
            last: 0x11f3a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11f40 as std::ffi::c_int as LWCHAR,
            last: 0x11f40 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11f42 as std::ffi::c_int as LWCHAR,
            last: 0x11f42 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x11f5a as std::ffi::c_int as LWCHAR,
            last: 0x11f5a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x13440 as std::ffi::c_int as LWCHAR,
            last: 0x13440 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x13447 as std::ffi::c_int as LWCHAR,
            last: 0x13455 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1611e as std::ffi::c_int as LWCHAR,
            last: 0x16129 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1612d as std::ffi::c_int as LWCHAR,
            last: 0x1612f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x16af0 as std::ffi::c_int as LWCHAR,
            last: 0x16af4 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x16b30 as std::ffi::c_int as LWCHAR,
            last: 0x16b36 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x16f4f as std::ffi::c_int as LWCHAR,
            last: 0x16f4f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x16f8f as std::ffi::c_int as LWCHAR,
            last: 0x16f92 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x16fe4 as std::ffi::c_int as LWCHAR,
            last: 0x16fe4 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1bc9d as std::ffi::c_int as LWCHAR,
            last: 0x1bc9e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1cf00 as std::ffi::c_int as LWCHAR,
            last: 0x1cf2d as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1cf30 as std::ffi::c_int as LWCHAR,
            last: 0x1cf46 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1d167 as std::ffi::c_int as LWCHAR,
            last: 0x1d169 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1d17b as std::ffi::c_int as LWCHAR,
            last: 0x1d182 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1d185 as std::ffi::c_int as LWCHAR,
            last: 0x1d18b as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1d1aa as std::ffi::c_int as LWCHAR,
            last: 0x1d1ad as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1d242 as std::ffi::c_int as LWCHAR,
            last: 0x1d244 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1da00 as std::ffi::c_int as LWCHAR,
            last: 0x1da36 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1da3b as std::ffi::c_int as LWCHAR,
            last: 0x1da6c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1da75 as std::ffi::c_int as LWCHAR,
            last: 0x1da75 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1da84 as std::ffi::c_int as LWCHAR,
            last: 0x1da84 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1da9b as std::ffi::c_int as LWCHAR,
            last: 0x1da9f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1daa1 as std::ffi::c_int as LWCHAR,
            last: 0x1daaf as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1e000 as std::ffi::c_int as LWCHAR,
            last: 0x1e006 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1e008 as std::ffi::c_int as LWCHAR,
            last: 0x1e018 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1e01b as std::ffi::c_int as LWCHAR,
            last: 0x1e021 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1e023 as std::ffi::c_int as LWCHAR,
            last: 0x1e024 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1e026 as std::ffi::c_int as LWCHAR,
            last: 0x1e02a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1e08f as std::ffi::c_int as LWCHAR,
            last: 0x1e08f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1e130 as std::ffi::c_int as LWCHAR,
            last: 0x1e136 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1e2ae as std::ffi::c_int as LWCHAR,
            last: 0x1e2ae as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1e2ec as std::ffi::c_int as LWCHAR,
            last: 0x1e2ef as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1e4ec as std::ffi::c_int as LWCHAR,
            last: 0x1e4ef as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1e5ee as std::ffi::c_int as LWCHAR,
            last: 0x1e5ef as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1e8d0 as std::ffi::c_int as LWCHAR,
            last: 0x1e8d6 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1e944 as std::ffi::c_int as LWCHAR,
            last: 0x1e94a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xe0100 as std::ffi::c_int as LWCHAR,
            last: 0xe01ef as std::ffi::c_int as LWCHAR,
        };
        init
    },
];
#[no_mangle]
pub static mut compose_table: wchar_range_table = wchar_range_table {
    table: 0 as *const wchar_range as *mut wchar_range,
    count: 0,
};
static mut ubin_array: [wchar_range; 10] = [
    {
        let mut init = wchar_range {
            first: 0 as std::ffi::c_int as LWCHAR,
            last: 0x7 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xb as std::ffi::c_int as LWCHAR,
            last: 0xb as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xe as std::ffi::c_int as LWCHAR,
            last: 0x1f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x7f as std::ffi::c_int as LWCHAR,
            last: 0x9f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2028 as std::ffi::c_int as LWCHAR,
            last: 0x2028 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2029 as std::ffi::c_int as LWCHAR,
            last: 0x2029 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xd800 as std::ffi::c_int as LWCHAR,
            last: 0xdfff as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xe000 as std::ffi::c_int as LWCHAR,
            last: 0xf8ff as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xf0000 as std::ffi::c_int as LWCHAR,
            last: 0xffffd as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x100000 as std::ffi::c_int as LWCHAR,
            last: 0x10fffd as std::ffi::c_int as LWCHAR,
        };
        init
    },
];
#[no_mangle]
pub static mut ubin_table: wchar_range_table = wchar_range_table {
    table: 0 as *const wchar_range as *mut wchar_range,
    count: 0,
};
static mut wide_array: [wchar_range; 124] = [
    {
        let mut init = wchar_range {
            first: 0x1100 as std::ffi::c_int as LWCHAR,
            last: 0x115f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x231a as std::ffi::c_int as LWCHAR,
            last: 0x231b as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2329 as std::ffi::c_int as LWCHAR,
            last: 0x232a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x23e9 as std::ffi::c_int as LWCHAR,
            last: 0x23ec as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x23f0 as std::ffi::c_int as LWCHAR,
            last: 0x23f0 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x23f3 as std::ffi::c_int as LWCHAR,
            last: 0x23f3 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x25fd as std::ffi::c_int as LWCHAR,
            last: 0x25fe as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2614 as std::ffi::c_int as LWCHAR,
            last: 0x2615 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2630 as std::ffi::c_int as LWCHAR,
            last: 0x2637 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2648 as std::ffi::c_int as LWCHAR,
            last: 0x2653 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x267f as std::ffi::c_int as LWCHAR,
            last: 0x267f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x268a as std::ffi::c_int as LWCHAR,
            last: 0x268f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2693 as std::ffi::c_int as LWCHAR,
            last: 0x2693 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x26a1 as std::ffi::c_int as LWCHAR,
            last: 0x26a1 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x26aa as std::ffi::c_int as LWCHAR,
            last: 0x26ab as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x26bd as std::ffi::c_int as LWCHAR,
            last: 0x26be as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x26c4 as std::ffi::c_int as LWCHAR,
            last: 0x26c5 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x26ce as std::ffi::c_int as LWCHAR,
            last: 0x26ce as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x26d4 as std::ffi::c_int as LWCHAR,
            last: 0x26d4 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x26ea as std::ffi::c_int as LWCHAR,
            last: 0x26ea as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x26f2 as std::ffi::c_int as LWCHAR,
            last: 0x26f3 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x26f5 as std::ffi::c_int as LWCHAR,
            last: 0x26f5 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x26fa as std::ffi::c_int as LWCHAR,
            last: 0x26fa as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x26fd as std::ffi::c_int as LWCHAR,
            last: 0x26fd as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2705 as std::ffi::c_int as LWCHAR,
            last: 0x2705 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x270a as std::ffi::c_int as LWCHAR,
            last: 0x270b as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2728 as std::ffi::c_int as LWCHAR,
            last: 0x2728 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x274c as std::ffi::c_int as LWCHAR,
            last: 0x274c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x274e as std::ffi::c_int as LWCHAR,
            last: 0x274e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2753 as std::ffi::c_int as LWCHAR,
            last: 0x2755 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2757 as std::ffi::c_int as LWCHAR,
            last: 0x2757 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2795 as std::ffi::c_int as LWCHAR,
            last: 0x2797 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x27b0 as std::ffi::c_int as LWCHAR,
            last: 0x27b0 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x27bf as std::ffi::c_int as LWCHAR,
            last: 0x27bf as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2b1b as std::ffi::c_int as LWCHAR,
            last: 0x2b1c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2b50 as std::ffi::c_int as LWCHAR,
            last: 0x2b50 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2b55 as std::ffi::c_int as LWCHAR,
            last: 0x2b55 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2e80 as std::ffi::c_int as LWCHAR,
            last: 0x2e99 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2e9b as std::ffi::c_int as LWCHAR,
            last: 0x2ef3 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2f00 as std::ffi::c_int as LWCHAR,
            last: 0x2fd5 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2ff0 as std::ffi::c_int as LWCHAR,
            last: 0x2fff as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x3000 as std::ffi::c_int as LWCHAR,
            last: 0x3000 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x3001 as std::ffi::c_int as LWCHAR,
            last: 0x303e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x3041 as std::ffi::c_int as LWCHAR,
            last: 0x3096 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x3099 as std::ffi::c_int as LWCHAR,
            last: 0x30ff as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x3105 as std::ffi::c_int as LWCHAR,
            last: 0x312f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x3131 as std::ffi::c_int as LWCHAR,
            last: 0x318e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x3190 as std::ffi::c_int as LWCHAR,
            last: 0x31e5 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x31ef as std::ffi::c_int as LWCHAR,
            last: 0x321e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x3220 as std::ffi::c_int as LWCHAR,
            last: 0x3247 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x3250 as std::ffi::c_int as LWCHAR,
            last: 0xa48c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa490 as std::ffi::c_int as LWCHAR,
            last: 0xa4c6 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xa960 as std::ffi::c_int as LWCHAR,
            last: 0xa97c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xac00 as std::ffi::c_int as LWCHAR,
            last: 0xd7a3 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xf900 as std::ffi::c_int as LWCHAR,
            last: 0xfaff as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xfe10 as std::ffi::c_int as LWCHAR,
            last: 0xfe19 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xfe30 as std::ffi::c_int as LWCHAR,
            last: 0xfe52 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xfe54 as std::ffi::c_int as LWCHAR,
            last: 0xfe66 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xfe68 as std::ffi::c_int as LWCHAR,
            last: 0xfe6b as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xff01 as std::ffi::c_int as LWCHAR,
            last: 0xff60 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xffe0 as std::ffi::c_int as LWCHAR,
            last: 0xffe6 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x16fe0 as std::ffi::c_int as LWCHAR,
            last: 0x16fe4 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x16ff0 as std::ffi::c_int as LWCHAR,
            last: 0x16ff1 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x17000 as std::ffi::c_int as LWCHAR,
            last: 0x187f7 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x18800 as std::ffi::c_int as LWCHAR,
            last: 0x18cd5 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x18cff as std::ffi::c_int as LWCHAR,
            last: 0x18d08 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1aff0 as std::ffi::c_int as LWCHAR,
            last: 0x1aff3 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1aff5 as std::ffi::c_int as LWCHAR,
            last: 0x1affb as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1affd as std::ffi::c_int as LWCHAR,
            last: 0x1affe as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1b000 as std::ffi::c_int as LWCHAR,
            last: 0x1b122 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1b132 as std::ffi::c_int as LWCHAR,
            last: 0x1b132 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1b150 as std::ffi::c_int as LWCHAR,
            last: 0x1b152 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1b155 as std::ffi::c_int as LWCHAR,
            last: 0x1b155 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1b164 as std::ffi::c_int as LWCHAR,
            last: 0x1b167 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1b170 as std::ffi::c_int as LWCHAR,
            last: 0x1b2fb as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1d300 as std::ffi::c_int as LWCHAR,
            last: 0x1d356 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1d360 as std::ffi::c_int as LWCHAR,
            last: 0x1d376 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f004 as std::ffi::c_int as LWCHAR,
            last: 0x1f004 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f0cf as std::ffi::c_int as LWCHAR,
            last: 0x1f0cf as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f18e as std::ffi::c_int as LWCHAR,
            last: 0x1f18e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f191 as std::ffi::c_int as LWCHAR,
            last: 0x1f19a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f200 as std::ffi::c_int as LWCHAR,
            last: 0x1f202 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f210 as std::ffi::c_int as LWCHAR,
            last: 0x1f23b as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f240 as std::ffi::c_int as LWCHAR,
            last: 0x1f248 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f250 as std::ffi::c_int as LWCHAR,
            last: 0x1f251 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f260 as std::ffi::c_int as LWCHAR,
            last: 0x1f265 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f300 as std::ffi::c_int as LWCHAR,
            last: 0x1f320 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f32d as std::ffi::c_int as LWCHAR,
            last: 0x1f335 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f337 as std::ffi::c_int as LWCHAR,
            last: 0x1f37c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f37e as std::ffi::c_int as LWCHAR,
            last: 0x1f393 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f3a0 as std::ffi::c_int as LWCHAR,
            last: 0x1f3ca as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f3cf as std::ffi::c_int as LWCHAR,
            last: 0x1f3d3 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f3e0 as std::ffi::c_int as LWCHAR,
            last: 0x1f3f0 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f3f4 as std::ffi::c_int as LWCHAR,
            last: 0x1f3f4 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f3f8 as std::ffi::c_int as LWCHAR,
            last: 0x1f43e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f440 as std::ffi::c_int as LWCHAR,
            last: 0x1f440 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f442 as std::ffi::c_int as LWCHAR,
            last: 0x1f4fc as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f4ff as std::ffi::c_int as LWCHAR,
            last: 0x1f53d as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f54b as std::ffi::c_int as LWCHAR,
            last: 0x1f54e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f550 as std::ffi::c_int as LWCHAR,
            last: 0x1f567 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f57a as std::ffi::c_int as LWCHAR,
            last: 0x1f57a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f595 as std::ffi::c_int as LWCHAR,
            last: 0x1f596 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f5a4 as std::ffi::c_int as LWCHAR,
            last: 0x1f5a4 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f5fb as std::ffi::c_int as LWCHAR,
            last: 0x1f64f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f680 as std::ffi::c_int as LWCHAR,
            last: 0x1f6c5 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f6cc as std::ffi::c_int as LWCHAR,
            last: 0x1f6cc as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f6d0 as std::ffi::c_int as LWCHAR,
            last: 0x1f6d2 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f6d5 as std::ffi::c_int as LWCHAR,
            last: 0x1f6d7 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f6dc as std::ffi::c_int as LWCHAR,
            last: 0x1f6df as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f6eb as std::ffi::c_int as LWCHAR,
            last: 0x1f6ec as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f6f4 as std::ffi::c_int as LWCHAR,
            last: 0x1f6fc as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f7e0 as std::ffi::c_int as LWCHAR,
            last: 0x1f7eb as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f7f0 as std::ffi::c_int as LWCHAR,
            last: 0x1f7f0 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f90c as std::ffi::c_int as LWCHAR,
            last: 0x1f93a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f93c as std::ffi::c_int as LWCHAR,
            last: 0x1f945 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1f947 as std::ffi::c_int as LWCHAR,
            last: 0x1f9ff as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1fa70 as std::ffi::c_int as LWCHAR,
            last: 0x1fa7c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1fa80 as std::ffi::c_int as LWCHAR,
            last: 0x1fa89 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1fa8f as std::ffi::c_int as LWCHAR,
            last: 0x1fac6 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1face as std::ffi::c_int as LWCHAR,
            last: 0x1fadc as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1fadf as std::ffi::c_int as LWCHAR,
            last: 0x1fae9 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1faf0 as std::ffi::c_int as LWCHAR,
            last: 0x1faf8 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x20000 as std::ffi::c_int as LWCHAR,
            last: 0x2fffd as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x30000 as std::ffi::c_int as LWCHAR,
            last: 0x3fffd as std::ffi::c_int as LWCHAR,
        };
        init
    },
];
#[no_mangle]
pub static mut wide_table: wchar_range_table = wchar_range_table {
    table: 0 as *const wchar_range as *mut wchar_range,
    count: 0,
};
static mut fmt_array: [wchar_range; 21] = [
    {
        let mut init = wchar_range {
            first: 0xad as std::ffi::c_int as LWCHAR,
            last: 0xad as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x600 as std::ffi::c_int as LWCHAR,
            last: 0x605 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x61c as std::ffi::c_int as LWCHAR,
            last: 0x61c as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x6dd as std::ffi::c_int as LWCHAR,
            last: 0x6dd as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x70f as std::ffi::c_int as LWCHAR,
            last: 0x70f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x890 as std::ffi::c_int as LWCHAR,
            last: 0x891 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x8e2 as std::ffi::c_int as LWCHAR,
            last: 0x8e2 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x180e as std::ffi::c_int as LWCHAR,
            last: 0x180e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x200b as std::ffi::c_int as LWCHAR,
            last: 0x200f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x202a as std::ffi::c_int as LWCHAR,
            last: 0x202e as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2060 as std::ffi::c_int as LWCHAR,
            last: 0x2064 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x2066 as std::ffi::c_int as LWCHAR,
            last: 0x206f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xfeff as std::ffi::c_int as LWCHAR,
            last: 0xfeff as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xfff9 as std::ffi::c_int as LWCHAR,
            last: 0xfffb as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x110bd as std::ffi::c_int as LWCHAR,
            last: 0x110bd as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x110cd as std::ffi::c_int as LWCHAR,
            last: 0x110cd as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x13430 as std::ffi::c_int as LWCHAR,
            last: 0x1343f as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1bca0 as std::ffi::c_int as LWCHAR,
            last: 0x1bca3 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x1d173 as std::ffi::c_int as LWCHAR,
            last: 0x1d17a as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xe0001 as std::ffi::c_int as LWCHAR,
            last: 0xe0001 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0xe0020 as std::ffi::c_int as LWCHAR,
            last: 0xe007f as std::ffi::c_int as LWCHAR,
        };
        init
    },
];
#[no_mangle]
pub static mut fmt_table: wchar_range_table = wchar_range_table {
    table: 0 as *const wchar_range as *mut wchar_range,
    count: 0,
};
static mut comb_table: [wchar_range; 4] = [
    {
        let mut init = wchar_range {
            first: 0x644 as std::ffi::c_int as LWCHAR,
            last: 0x622 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x644 as std::ffi::c_int as LWCHAR,
            last: 0x623 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x644 as std::ffi::c_int as LWCHAR,
            last: 0x625 as std::ffi::c_int as LWCHAR,
        };
        init
    },
    {
        let mut init = wchar_range {
            first: 0x644 as std::ffi::c_int as LWCHAR,
            last: 0x627 as std::ffi::c_int as LWCHAR,
        };
        init
    },
];
unsafe extern "C" fn is_in_table(
    mut ch: LWCHAR,
    mut table: *mut wchar_range_table,
) -> lbool {
    let mut hi: std::ffi::c_uint = 0;
    let mut lo: std::ffi::c_uint = 0;
    if ((*table).table).is_null()
        || (*table).count == 0 as std::ffi::c_int as std::ffi::c_uint
        || ch < (*((*table).table).offset(0 as std::ffi::c_int as isize)).first
    {
        return LFALSE;
    }
    lo = 0 as std::ffi::c_int as std::ffi::c_uint;
    hi = ((*table).count).wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint);
    while lo <= hi {
        let mut mid: std::ffi::c_uint = lo
            .wrapping_add(hi)
            .wrapping_div(2 as std::ffi::c_int as std::ffi::c_uint);
        if ch > (*((*table).table).offset(mid as isize)).last {
            lo = mid.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
        } else if ch < (*((*table).table).offset(mid as isize)).first {
            hi = mid.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint);
        } else {
            return LTRUE
        }
    }
    return LFALSE;
}
#[no_mangle]
pub unsafe extern "C" fn is_composing_char(mut ch: LWCHAR) -> lbool {
    if is_in_table(ch, &mut user_prt_table) as u64 != 0 {
        return LFALSE;
    }
    return (is_in_table(ch, &mut user_compose_table) as std::ffi::c_uint != 0
        || is_in_table(ch, &mut compose_table) as std::ffi::c_uint != 0
        || bs_mode != 2 as std::ffi::c_int
            && is_in_table(ch, &mut fmt_table) as std::ffi::c_uint != 0)
        as std::ffi::c_int as lbool;
}
#[no_mangle]
pub unsafe extern "C" fn is_ubin_char(mut ch: LWCHAR) -> lbool {
    if is_in_table(ch, &mut user_prt_table) as u64 != 0 {
        return LFALSE;
    }
    return (is_in_table(ch, &mut user_ubin_table) as std::ffi::c_uint != 0
        || is_in_table(ch, &mut ubin_table) as std::ffi::c_uint != 0
        || bs_mode == 2 as std::ffi::c_int
            && is_in_table(ch, &mut fmt_table) as std::ffi::c_uint != 0)
        as std::ffi::c_int as lbool;
}
#[no_mangle]
pub unsafe extern "C" fn is_wide_char(mut ch: LWCHAR) -> lbool {
    return (is_in_table(ch, &mut user_wide_table) as std::ffi::c_uint != 0
        || is_in_table(ch, &mut wide_table) as std::ffi::c_uint != 0) as std::ffi::c_int
        as lbool;
}
#[no_mangle]
pub unsafe extern "C" fn is_combining_char(mut ch1: LWCHAR, mut ch2: LWCHAR) -> lbool {
    let mut i: std::ffi::c_int = 0;
    i = 0 as std::ffi::c_int;
    while i
        < (::core::mem::size_of::<[wchar_range; 4]>() as std::ffi::c_ulong)
            .wrapping_div(::core::mem::size_of::<wchar_range>() as std::ffi::c_ulong)
            as std::ffi::c_int
    {
        if ch1 == comb_table[i as usize].first && ch2 == comb_table[i as usize].last {
            return LTRUE;
        }
        i += 1;
        i;
    }
    return LFALSE;
}
unsafe extern "C" fn run_static_initializers() {
    compose_table = {
        let mut init = wchar_range_table {
            table: compose_array.as_mut_ptr(),
            count: (::core::mem::size_of::<[wchar_range; 365]>() as std::ffi::c_ulong)
                .wrapping_div(::core::mem::size_of::<wchar_range>() as std::ffi::c_ulong)
                as std::ffi::c_int as std::ffi::c_uint,
        };
        init
    };
    ubin_table = {
        let mut init = wchar_range_table {
            table: ubin_array.as_mut_ptr(),
            count: (::core::mem::size_of::<[wchar_range; 10]>() as std::ffi::c_ulong)
                .wrapping_div(::core::mem::size_of::<wchar_range>() as std::ffi::c_ulong)
                as std::ffi::c_int as std::ffi::c_uint,
        };
        init
    };
    wide_table = {
        let mut init = wchar_range_table {
            table: wide_array.as_mut_ptr(),
            count: (::core::mem::size_of::<[wchar_range; 124]>() as std::ffi::c_ulong)
                .wrapping_div(::core::mem::size_of::<wchar_range>() as std::ffi::c_ulong)
                as std::ffi::c_int as std::ffi::c_uint,
        };
        init
    };
    fmt_table = {
        let mut init = wchar_range_table {
            table: fmt_array.as_mut_ptr(),
            count: (::core::mem::size_of::<[wchar_range; 21]>() as std::ffi::c_ulong)
                .wrapping_div(::core::mem::size_of::<wchar_range>() as std::ffi::c_ulong)
                as std::ffi::c_int as std::ffi::c_uint,
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
