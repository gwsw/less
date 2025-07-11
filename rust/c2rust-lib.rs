#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
#[path = "brac.rs"]
pub mod brac;
#[path = "ch.rs"]
pub mod ch;
#[path = "charset.rs"]
pub mod charset;
#[path = "cmdbuf.rs"]
pub mod cmdbuf;
#[path = "command.rs"]
pub mod command;
#[path = "cvt.rs"]
pub mod cvt;
#[path = "decode.rs"]
pub mod decode;
#[path = "edit.rs"]
pub mod edit;
#[path = "evar.rs"]
pub mod evar;
#[path = "filename.rs"]
pub mod filename;
#[path = "forwback.rs"]
pub mod forwback;
#[path = "help.rs"]
pub mod help;
#[path = "ifile.rs"]
pub mod ifile;
#[path = "input.rs"]
pub mod input;
#[path = "jump.rs"]
pub mod jump;
#[path = "lessecho.rs"]
pub mod lessecho;
/*
#[path = "lesskey.rs"]
pub mod lesskey;
*/
#[path = "lesskey_parse.rs"]
pub mod lesskey_parse;
#[path = "line.rs"]
pub mod line;
#[path = "linenum.rs"]
pub mod linenum;
#[path = "lsystem.rs"]
pub mod lsystem;
#[path = "main.rs"]
pub mod main;
#[path = "mark.rs"]
pub mod mark;
#[path = "optfunc.rs"]
pub mod optfunc;
#[path = "option.rs"]
pub mod option;
#[path = "opttbl.rs"]
pub mod opttbl;
#[path = "os.rs"]
pub mod os;
#[path = "output.rs"]
pub mod output;
#[path = "pattern.rs"]
pub mod pattern;
#[path = "position.rs"]
pub mod position;
#[path = "prompt.rs"]
pub mod prompt;
#[path = "screen.rs"]
pub mod screen;
#[path = "search.rs"]
pub mod search;
#[path = "signal.rs"]
pub mod signal;
#[path = "tags.rs"]
pub mod tags;
#[path = "ttyin.rs"]
pub mod ttyin;
#[path = "util.rs"]
pub mod util;
#[path = "version.rs"]
pub mod version;
#[path = "xbuf.rs"]
pub mod xbuf;
