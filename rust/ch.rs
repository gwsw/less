//! Idiomatic Rust port of the C `ch_get` buffering logic.
//!
//! This is a self-contained, ergonomic Rust module that mirrors the original behavior
//! while using safe, idiomatic abstractions. It models:
//! - MRU buffer pool with fixed-size blocks (LBUFSIZE)
//! - Hash lookup by block id (simplified to a `HashMap<i64, usize>`)
//! - Seekable vs non-seekable inputs (pipes/FIFOs)
//! - Optional "help file" source (static memory)
//! - Ungot character support
//! - Follow mode & waiting-for-data with user hooks
//! - EOF handling similar to the C code
//!
//! Notes:
//! - Signals (`ABORT_SIGS`, `sigs`) and platform-specific errno branches are exposed via
//!   injected hooks/callbacks (`abort_sigs`, `on_wait`, etc.).
//! - The original returned magic integers (`EOI`, `'?'`); here we return `GetChar` to
//!   capture those semantics explicitly and type-safely.
//! - The MRU list is a `VecDeque`; we do a short linear search when looking up an
//!   existing block to keep the implementation dependency-free and clear.
//!   (You can swap in `linked_hash_map` or `lru` crate if desired.)

use crate::defs::*;
use crate::defs::*;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

extern "C" {
    fn lseek(__fd: std::ffi::c_int, __offset: __off_t, __whence: std::ffi::c_int) -> __off_t;
    fn close(__fd: std::ffi::c_int) -> std::ffi::c_int;
    fn write(__fd: std::ffi::c_int, __buf: *const std::ffi::c_void, __n: size_t) -> ssize_t;
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
    fn secure_allow(features: std::ffi::c_int) -> std::ffi::c_int;
    fn clear_eol();
    fn screen_trashed_num(trashed: std::ffi::c_int);
    fn filesize(f: std::ffi::c_int) -> POSITION;
    fn curr_ifile_changed() -> lbool;
    fn get_filestate(ifile: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn set_filestate(ifile: *mut std::ffi::c_void, filestate: *mut std::ffi::c_void);
    fn sleep_ms(ms: std::ffi::c_int);
    fn flush();
    fn putstr(s: *const std::ffi::c_char);
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn ierror(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn ixerror(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn wait_message() -> *const std::ffi::c_char;
    static mut autobuf: std::ffi::c_int;
    static mut sigs: std::ffi::c_int;
    static mut follow_mode: std::ffi::c_int;
    static mut waiting_for_data: bool;
    static helpdata: [std::ffi::c_char; 0];
    static size_helpdata: std::ffi::c_int;
    static mut curr_ifile: *mut std::ffi::c_void;
    static mut logfile: std::ffi::c_int;
    static mut namelogfile: *mut std::ffi::c_char;
}

/*
 * Low level character input from the input file.
 * We use these special purpose routines which optimize moving
 * both forward and backward from the current read pointer.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub union parg {
    pub p_string: *const std::ffi::c_char,
    pub p_int: std::ffi::c_int,
    pub p_linenum: LINENUM,
    pub p_char: std::ffi::c_char,
}
pub type PARG = parg;

const BUFHASH_SIZE: usize = 1024;
pub type BLOCKNUM = POSITION;

/*
 * Pool of buffers holding the most recently used blocks of the input file.
 * The buffer pool is kept as a doubly-linked circular list,
 * in order from most- to least-recently used.
 * The circular list is anchored by the file state "thisfile".
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bufnode {
    pub next: *mut bufnode,
    pub prev: *mut bufnode,
    pub hnext: *mut bufnode,
    pub hprev: *mut bufnode,
}

#[no_mangle]
pub static mut ignore_eoi: std::ffi::c_int = 0;
static mut ch_ungotchar: std::ffi::c_uchar = 0;
static mut ch_have_ungotchar: bool = false;
static mut maxbufs: std::ffi::c_int = -(1 as std::ffi::c_int);

pub const LBUFSIZE: usize = 8192;

/// Outcome of `ch_get`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GetChar {
    /// Successfully produced the next byte.
    Byte(u8),
    /// End-of-input (EOI) — nothing more available now.
    Eoi,
    /// Non-seekable source lost data (C code returned `'?'`).
    LostData,
}

/// Configurable behavior toggles, mirroring original flags.
pub struct Config {
    /// If true and input is non-seekable, allow auto-growing buffers (like `autobuf`).
    pub autobuf: bool,
    /// Max buffers allowed (None means unlimited, like `maxbufs < 0`).
    pub maxbufs: Option<usize>,
    /// Treat source as seekable (CH_CANSEEK) — set false for pipes/FIFOs.
    pub can_seek: bool,
    /// Treat source as help file (CH_HELPFILE) that reads from `helpdata`.
    pub is_helpfile: bool,
    /// Ignore EOI and wait for more data (tail -f behavior).
    pub ignore_eoi: bool,
    /// Follow mode; when `Name`, we'll call `curr_ifile_changed` hook.
    pub follow_mode: FollowMode,
    /// Optional static help data.
    pub helpdata: Option<&'static [u8]>,
    /// If present, all read bytes are also written here (logfile analogue).
    pub logfile: Option<Box<dyn Write + Send>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FollowMode {
    Off,
    Name,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            autobuf: false,
            maxbufs: None,
            can_seek: true,
            is_helpfile: false,
            ignore_eoi: false,
            follow_mode: FollowMode::Off,
            helpdata: None,
            logfile: None,
        }
    }
}

#[derive(Debug, Clone)]
struct Buffer {
    block: i64,
    datasize: usize,
    data: [u8; LBUFSIZE],
}

impl Buffer {
    fn empty_for(block: i64) -> Self {
        Self {
            block,
            datasize: 0,
            data: [0; LBUFSIZE],
        }
    }
}

/// Hooks to integrate platform-specific behavior (signals, UI messages, file changes).
pub struct Hooks {
    /// Return true to abort (maps to `ABORT_SIGS`).
    pub abort_sigs: Box<dyn Fn() -> bool + Send + Sync>,
    /// Called when we decide to show a waiting message (once per wait loop).
    pub on_wait: Box<dyn Fn(&str) + Send + Sync>,
    /// Sleep function (so callers can mock in tests).
    pub sleep_ms: Box<dyn Fn(u64) + Send + Sync>,
    /// Whether the current file (by name) changed while following.
    pub curr_ifile_changed: Box<dyn Fn() -> bool + Send + Sync>,
}

impl Default for Hooks {
    fn default() -> Self {
        Self {
            abort_sigs: Box::new(|| false),
            on_wait: Box::new(|_msg| {}),
            sleep_ms: Box::new(|ms| thread::sleep(Duration::from_millis(ms))),
            curr_ifile_changed: Box::new(|| false),
        }
    }
}

/// Main state, analogous to `struct filestate` + surrounding globals.
pub struct FileState<R: Read + Seek> {
    reader: R,
    pub cfg: Config,
    pub hooks: Hooks,

    /// MRU buffers (front = most recently used)
    bufs: VecDeque<Buffer>,
    /// Map block -> index in `bufs` (best-effort; updated on inserts/moves)
    by_block: HashMap<i64, usize>,

    /// Logical file position of next OS read.
    fpos: u64,
    /// Known file size (None for pipes until discovered).
    fsize: Option<u64>,

    /// Current logical position we want: (block, offset)
    block: i64,
    offset: usize,

    /// Ungot character support.
    ungot: Option<u8>,

    /// Internal: whether we've printed the wait message during a wait loop.
    waiting_for_data: bool,
}

impl<R: Read + Seek> FileState<R> {
    pub fn new(reader: R, cfg: Config) -> Self {
        Self {
            reader,
            cfg,
            hooks: Hooks::default(),
            bufs: VecDeque::new(),
            by_block: HashMap::new(),
            fpos: 0,
            fsize: None,
            block: 0,
            offset: 0,
            ungot: None,
            waiting_for_data: false,
        }
    }

    /// Set external hooks after construction.
    pub fn with_hooks(mut self, hooks: Hooks) -> Self {
        self.hooks = hooks;
        self
    }

    /// Update the target read pointer (block, offset).
    pub fn seek_logical(&mut self, block: i64, offset: usize) {
        self.block = block;
        self.offset = offset;
    }

    /// Equivalent of `ch_position(block, offset)`
    #[inline]
    fn logical_pos(block: i64, offset: usize) -> u64 {
        (block as u64) * (LBUFSIZE as u64) + (offset as u64)
    }

    /// Get current logical position.
    fn pos(&self) -> u64 {
        Self::logical_pos(self.block, self.offset)
    }

    /// Equivalent of `ch_length()`
    fn length(&self) -> Option<u64> {
        self.fsize
    }

    /// Equivalent of `ch_resize()` — refresh `fsize` if seekable.
    fn resize(&mut self) -> io::Result<()> {
        if !self.cfg.can_seek {
            return Ok(());
        }
        let cur = self.fpos;
        let end = self.reader.seek(SeekFrom::End(0))?;
        // Return to previous fpos
        self.reader.seek(SeekFrom::Start(cur))?;
        self.fsize = Some(end);
        Ok(())
    }

    /// Equivalent of `ch_set_eof()` for pipes: set "known size" to current `fpos`.
    fn set_eof(&mut self) {
        self.fsize = Some(self.fpos);
    }

    /// Add a new buffer (like `ch_addbuf()`); returns whether it succeeded.
    fn addbuf(&mut self) -> bool {
        if let Some(max) = self.cfg.maxbufs {
            if self.bufs.len() >= max {
                return false;
            }
        }
        self.bufs.push_front(Buffer::empty_for(-1));
        // Rebuild index map conservatively
        self.reindex_blocks();
        true
    }

    /// Rebuild the `by_block` map from current `bufs` order.
    fn reindex_blocks(&mut self) {
        self.by_block.clear();
        for (i, b) in self.bufs.iter().enumerate() {
            self.by_block.insert(b.block, i);
        }
    }

    /// Move buffer at index `i` to MRU head.
    fn move_to_head(&mut self, i: usize) {
        if i == 0 {
            return;
        }
        if let Some(buf) = self.bufs.remove(i) {
            self.bufs.push_front(buf);
            self.reindex_blocks();
        }
    }

    /// Find a buffer containing `self.block`.
    fn find_buffer_index(&self, blk: i64) -> Option<usize> {
        self.by_block.get(&blk).copied()
    }

    /// Read into the current buffer (`bp`) starting at its current datasize.
    fn read_into_buffer(&mut self, bidx: usize) -> io::Result<usize> {
        // 1) Ungot character takes precedence.
        if let Some(ch) = self.ungot.take() {
            if self.bufs[bidx].datasize < LBUFSIZE {
                let i = self.bufs[bidx].datasize;
                self.bufs[bidx].data[i] = ch;
                self.bufs[bidx].datasize += 1;
            }
            self.fpos += 1; // mirror C code advancing fpos
            return Ok(1);
        }
        // 2) Helpfile byte if configured.
        if self.cfg.is_helpfile {
            if let Some(h) = self.cfg.helpdata {
                let idx = self.fpos as usize;
                if idx < h.len() {
                    if self.bufs[bidx].datasize < LBUFSIZE {
                        let i = self.bufs[bidx].datasize;
                        self.bufs[bidx].data[i] = h[idx];
                        self.bufs[bidx].datasize += 1;
                    }
                    self.fpos += 1;
                    return Ok(1);
                } else {
                    return Ok(0); // EOF on helpdata
                }
            }
        }
        // 3) Actual I/O
        let mut tmp = [0u8; LBUFSIZE];
        let want = LBUFSIZE - self.bufs[bidx].datasize;
        let n = self.reader.read(&mut tmp[..want])?;
        if n > 0 {
            let range = self.bufs[bidx].datasize..self.bufs[bidx].datasize + n;
            self.bufs[bidx].data[range].copy_from_slice(&tmp[..n]);
            self.bufs[bidx].datasize += n;
            // Write to logfile if configured
            if let Some(ref mut log) = self.cfg.logfile.as_mut() {
                let _ = log.write_all(&tmp[..n]);
            }
        }
        self.fpos += n as u64;
        Ok(n)
    }

    pub fn ch_get(&mut self) -> GetChar {
        // Quick check: front buffer has our block and enough data.
        if let Some(front) = self.bufs.front() {
            if front.block == self.block && self.offset < front.datasize {
                return GetChar::Byte(front.data[self.offset]);
            }
        }

        // Look for a buffer holding the desired block.
        self.waiting_for_data = false;
        if let Some(i) = self.find_buffer_index(self.block) {
            let needs_more = {
                let bp = &self.bufs[i];
                self.offset >= bp.datasize
            };
            if !needs_more {
                // Move MRU and return
                self.move_to_head(i);
                let bp = &self.bufs[0];
                return GetChar::Byte(bp.data[self.offset]);
            }
            // else: fall through to read more into this buffer
            self.move_to_head(i);
        } else {
            // Block not present: choose LRU (or allocate) and assign desired block.
            let need_new = self.bufs.back().map(|b| b.block != -1).unwrap_or(true);
            if need_new {
                if (self.cfg.autobuf && !self.cfg.can_seek)
                    || self.cfg.maxbufs.map_or(true, |m| self.bufs.len() < m)
                {
                    let _ = self.addbuf();
                }
            }
            // Use tail if exists, else create one.
            if self.bufs.is_empty() {
                self.bufs.push_front(Buffer::empty_for(-1));
            }
            let mut tail = self.bufs.pop_back().unwrap();
            tail.block = self.block;
            tail.datasize = 0;
            self.bufs.push_front(tail);
            self.reindex_blocks();
        }

        // Now front buffer is for our block; read until we have `offset+1` or hit EOI.
        loop {
            // Are we past known EOF?
            let pos = Self::logical_pos(self.block, self.bufs[0].datasize);
            let mut read_pipe_at_eof = false;
            if let Some(len) = self.length() {
                if pos >= len {
                    // Double-check file size in case it changed.
                    let _ = self.resize();
                    if let Some(len2) = self.length() {
                        if pos >= len2 {
                            if self.cfg.can_seek || self.cfg.is_helpfile {
                                return GetChar::Eoi;
                            }
                            // Pipes: try to read once more to see if data appeared again.
                            read_pipe_at_eof = true;
                        }
                    }
                }
            }

            // Ensure OS file pos matches desired.
            if self.fpos != pos {
                if !self.cfg.can_seek {
                    return GetChar::LostData;
                }
                if self.reader.seek(SeekFrom::Start(pos)).is_err() {
                    return GetChar::Eoi;
                }
                self.fpos = pos;
            }

            // Read chunk
            let n = match self.read_into_buffer(0) {
                Ok(n) => n,
                Err(e) => {
                    // Interrupted/Again map to no data this round
                    match e.kind() {
                        io::ErrorKind::Interrupted | io::ErrorKind::WouldBlock => 0,
                        io::ErrorKind::BrokenPipe => 0,
                        _ => return GetChar::Eoi,
                    }
                }
            };

            if read_pipe_at_eof {
                self.set_eof();
            }

            if n == 0 {
                // EOF or temporarily no data
                if !self.cfg.ignore_eoi {
                    return GetChar::Eoi;
                }

                // Follow-mode special case
                if self.cfg.follow_mode == FollowMode::Name && (self.hooks.curr_ifile_changed)() {
                    // Signal to caller to reopen (mirror screen_trashed=2 -> EOI)
                    return GetChar::Eoi;
                }

                if (self.hooks.abort_sigs)() {
                    return GetChar::Eoi;
                }

                if read_pipe_at_eof {
                    return GetChar::Eoi;
                }

                if !self.waiting_for_data {
                    (self.hooks.on_wait)("waiting for data...");
                    self.waiting_for_data = true;
                }
                (self.hooks.sleep_ms)(50);
                continue; // try again
            }

            // We may have enough now
            if self.offset < self.bufs[0].datasize {
                break;
            }
            // else loop to read more
        }

        GetChar::Byte(self.bufs[0].data[self.offset])
    }

    pub fn ch_forw_get(&mut self) -> GetChar {
        let c = self.ch_get();
        match c {
            GetChar::Byte(_) => {
                if self.offset < LBUFSIZE - 1 {
                    self.offset += 1;
                } else {
                    self.block += 1;
                    self.offset = 0;
                }
                c
            }
            _ => c,
        }
    }

    /// Push back a single byte so it will be returned by the next `ch_get`.
    pub fn unget(&mut self, ch: u8) {
        self.ungot = Some(ch);
    }

    /// Advance the logical offset by one (call after consuming a returned Byte).
    pub fn advance(&mut self) {
        self.offset += 1;
        if self.offset >= LBUFSIZE {
            self.block += 1;
            self.offset = 0;
        }
    }

    /// Expose file size (if known).
    pub fn known_size(&self) -> Option<u64> {
        self.fsize
    }
}

// -------------------------------
// Example usage (doc test style)
// -------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn basic_read_seekable() {
        let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let reader = Cursor::new(data);
        let mut fs = FileState::new(
            reader,
            Config {
                can_seek: true,
                ..Default::default()
            },
        );
        // Prepare position (block=0, offset=0)
        fs.seek_logical(0, 0);
        let mut out = Vec::new();
        for _ in 0..10 {
            match fs.ch_get() {
                GetChar::Byte(b) => {
                    out.push(b);
                    fs.advance();
                }
                other => panic!("unexpected: {:?}", other),
            }
        }
        assert_eq!(out, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn helpfile_source() {
        let reader = Cursor::new(Vec::<u8>::new());
        let help: &'static [u8] = b"hello";
        let mut fs = FileState::new(
            reader,
            Config {
                can_seek: true,
                is_helpfile: true,
                helpdata: Some(help),
                ..Default::default()
            },
        );
        fs.seek_logical(0, 0);
        let mut s = Vec::new();
        loop {
            match fs.ch_get() {
                GetChar::Byte(b) => {
                    s.push(b);
                    fs.advance();
                }
                GetChar::Eoi => break,
                _ => unreachable!(),
            }
        }
        assert_eq!(s, b"hello");
    }
}

/*
#[no_mangle]
pub unsafe extern "C" fn sync_logfile() {
    let mut bp: *mut buf = 0 as *mut buf;
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    let mut warned: lbool = LFALSE;
    let mut h: std::ffi::c_int = 0;
    let mut block: BLOCKNUM = 0;
    let mut nblocks: BLOCKNUM = 0;
    if logfile < 0 as std::ffi::c_int {
        return;
    }
    nblocks = ((*thisfile).fpos + 8192 as std::ffi::c_int as POSITION
        - 1 as std::ffi::c_int as POSITION)
        / 8192 as std::ffi::c_int as POSITION;
    block = 0 as std::ffi::c_int as BLOCKNUM;
    while block < nblocks {
        let mut wrote: lbool = LFALSE;
        h = (block & (1024 as std::ffi::c_int - 1 as std::ffi::c_int) as BLOCKNUM)
            as std::ffi::c_int;
        bn = (*thisfile).hashtbl[h as usize].hnext;
        while bn != &mut *((*thisfile).hashtbl).as_mut_ptr().offset(h as isize) as *mut bufnode {
            bp = bn as *mut buf;
            if (*bp).block == block {
                write(
                    logfile,
                    ((*bp).data).as_mut_ptr() as *const std::ffi::c_void,
                    (*bp).datasize,
                );
                wrote = LTRUE;
                break;
            } else {
                bn = (*bn).hnext;
            }
        }
        if wrote as u64 == 0 && warned as u64 == 0 {
            error(
                b"Warning: log file is incomplete\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
            warned = LTRUE;
        }
        block += 1;
    }
}
*/
/*
unsafe extern "C" fn buffered(mut block: BLOCKNUM) -> lbool {
    let mut bp: *mut buf = 0 as *mut buf;
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    let mut h: std::ffi::c_int = 0;
    h = (block & (1024 as std::ffi::c_int - 1 as std::ffi::c_int) as BLOCKNUM) as std::ffi::c_int;
    bn = (*thisfile).hashtbl[h as usize].hnext;
    while bn != &mut *((*thisfile).hashtbl).as_mut_ptr().offset(h as isize) as *mut bufnode {
        bp = bn as *mut buf;
        if (*bp).block == block {
            return LTRUE;
        }
        bn = (*bn).hnext;
    }
    return LFALSE;
}
#[no_mangle]
pub unsafe extern "C" fn ch_seek(mut pos: POSITION) -> std::ffi::c_int {
    let mut new_block: BLOCKNUM = 0;
    let mut len: POSITION = 0;
    if thisfile.is_null() {
        return 0 as std::ffi::c_int;
    }
    len = ch_length();
    if pos < 0 as std::ffi::c_int as POSITION
        || len != -(1 as std::ffi::c_int) as POSITION && pos > len
    {
        return 1 as std::ffi::c_int;
    }
    new_block = pos / 8192 as std::ffi::c_int as POSITION;
    if (*thisfile).flags & 0o1 as std::ffi::c_int == 0
        && pos != (*thisfile).fpos
        && buffered(new_block) as u64 == 0
    {
        if (*thisfile).fpos > pos {
            return 1 as std::ffi::c_int;
        }
        while (*thisfile).fpos < pos {
            if ch_forw_get() == -(1 as std::ffi::c_int) {
                return 1 as std::ffi::c_int;
            }
            if sigs
                & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                    | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
                != 0
            {
                return 1 as std::ffi::c_int;
            }
        }
        return 0 as std::ffi::c_int;
    }
    (*thisfile).block = new_block;
    (*thisfile).offset = (pos % 8192 as std::ffi::c_int as POSITION) as size_t;
    return 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ch_end_seek() -> std::ffi::c_int {
    let mut len: POSITION = 0;
    if thisfile.is_null() {
        return 0 as std::ffi::c_int;
    }
    if (*thisfile).flags & 0o1 as std::ffi::c_int != 0 {
        (*thisfile).fsize = filesize((*thisfile).file);
    }
    len = ch_length();
    if len != -(1 as std::ffi::c_int) as POSITION {
        return ch_seek(len);
    }
    while ch_forw_get() != -(1 as std::ffi::c_int) {
        if sigs
            & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
            != 0
        {
            return 1 as std::ffi::c_int;
        }
    }
    return 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ch_end_buffer_seek() -> std::ffi::c_int {
    let mut bp: *mut buf = 0 as *mut buf;
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    let mut buf_pos: POSITION = 0;
    let mut end_pos: POSITION = 0;
    if thisfile.is_null() || (*thisfile).flags & 0o1 as std::ffi::c_int != 0 {
        return ch_end_seek();
    }
    end_pos = 0 as std::ffi::c_int as POSITION;
    bn = (*thisfile).buflist.next;
    while bn != &mut (*thisfile).buflist as *mut bufnode {
        bp = bn as *mut buf;
        buf_pos = ch_position((*bp).block, (*bp).datasize);
        if buf_pos > end_pos {
            end_pos = buf_pos;
        }
        bn = (*bn).next;
    }
    return ch_seek(end_pos);
}
#[no_mangle]
pub unsafe extern "C" fn ch_beg_seek() -> std::ffi::c_int {
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    let mut firstbn: *mut bufnode = 0 as *mut bufnode;
    if ch_seek(0 as std::ffi::c_int as POSITION) == 0 as std::ffi::c_int {
        return 0 as std::ffi::c_int;
    }
    firstbn = (*thisfile).buflist.next;
    if firstbn == &mut (*thisfile).buflist as *mut bufnode {
        return 1 as std::ffi::c_int;
    }
    bn = (*thisfile).buflist.next;
    while bn != &mut (*thisfile).buflist as *mut bufnode {
        if (*(bn as *mut buf)).block < (*(firstbn as *mut buf)).block {
            firstbn = bn;
        }
        bn = (*bn).next;
    }
    (*thisfile).block = (*(firstbn as *mut buf)).block;
    (*thisfile).offset = 0 as std::ffi::c_int as size_t;
    return 0 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ch_length() -> POSITION {
    if thisfile.is_null() {
        return -(1 as std::ffi::c_int) as POSITION;
    }
    if ignore_eoi != 0 {
        return -(1 as std::ffi::c_int) as POSITION;
    }
    if (*thisfile).flags & 0o10 as std::ffi::c_int != 0 {
        return size_helpdata as POSITION;
    }
    if (*thisfile).flags & 0o20 as std::ffi::c_int != 0 {
        return 0 as std::ffi::c_int as POSITION;
    }
    return (*thisfile).fsize;
}
#[no_mangle]
pub unsafe extern "C" fn ch_resize() {
    let mut fsize: POSITION = 0;
    if (*thisfile).flags & 0o1 as std::ffi::c_int == 0 {
        return;
    }
    fsize = filesize((*thisfile).file);
    if fsize != -(1 as std::ffi::c_int) as POSITION {
        (*thisfile).fsize = fsize;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ch_tell() -> POSITION {
    if thisfile.is_null() {
        return -(1 as std::ffi::c_int) as POSITION;
    }
    return ch_position((*thisfile).block, (*thisfile).offset);
}
#[no_mangle]
pub unsafe extern "C" fn ch_forw_get() -> std::ffi::c_int {
    let mut c: std::ffi::c_int = 0;
    if thisfile.is_null() {
        return -(1 as std::ffi::c_int);
    }
    c = ch_get();
    if c == -(1 as std::ffi::c_int) {
        return -(1 as std::ffi::c_int);
    }
    if (*thisfile).offset < (8192 as std::ffi::c_int - 1 as std::ffi::c_int) as size_t {
        (*thisfile).offset = ((*thisfile).offset).wrapping_add(1);
        (*thisfile).offset;
    } else {
        (*thisfile).block += 1;
        (*thisfile).block;
        (*thisfile).offset = 0 as std::ffi::c_int as size_t;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn ch_back_get() -> std::ffi::c_int {
    if thisfile.is_null() {
        return -(1 as std::ffi::c_int);
    }
    if (*thisfile).offset > 0 as std::ffi::c_int as size_t {
        (*thisfile).offset = ((*thisfile).offset).wrapping_sub(1);
        (*thisfile).offset;
    } else {
        if (*thisfile).block <= 0 as std::ffi::c_int as BLOCKNUM {
            return -(1 as std::ffi::c_int);
        }
        if (*thisfile).flags & 0o1 as std::ffi::c_int == 0
            && buffered((*thisfile).block - 1 as std::ffi::c_int as BLOCKNUM) as u64 == 0
        {
            return -(1 as std::ffi::c_int);
        }
        (*thisfile).block -= 1;
        (*thisfile).block;
        (*thisfile).offset = (8192 as std::ffi::c_int - 1 as std::ffi::c_int) as size_t;
    }
    return ch_get();
}
#[no_mangle]
pub unsafe extern "C" fn ch_setbufspace(mut bufspace: ssize_t) {
    if bufspace < 0 as std::ffi::c_int as ssize_t {
        maxbufs = -(1 as std::ffi::c_int);
    } else {
        let mut lbufk: size_t = (8192 as std::ffi::c_int / 1024 as std::ffi::c_int) as size_t;
        maxbufs = (bufspace as size_t / lbufk).wrapping_add(
            (bufspace as size_t % lbufk != 0 as std::ffi::c_int as size_t) as std::ffi::c_int
                as size_t,
        ) as std::ffi::c_int;
        if maxbufs < 1 as std::ffi::c_int {
            maxbufs = 1 as std::ffi::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ch_flush() {
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    if thisfile.is_null() {
        return;
    }
    if (*thisfile).flags & 0o1 as std::ffi::c_int == 0 {
        (*thisfile).fsize = -(1 as std::ffi::c_int) as POSITION;
        return;
    }
    bn = (*thisfile).buflist.next;
    while bn != &mut (*thisfile).buflist as *mut bufnode {
        (*(bn as *mut buf)).block = -(1 as std::ffi::c_int) as BLOCKNUM;
        bn = (*bn).next;
    }
    (*thisfile).fpos = 0 as std::ffi::c_int as POSITION;
    (*thisfile).block = 0 as std::ffi::c_int as BLOCKNUM;
    (*thisfile).offset = 0 as std::ffi::c_int as size_t;
    if (*thisfile).flags & 0o40 as std::ffi::c_int != 0 {
        (*thisfile).fsize = -(1 as std::ffi::c_int) as POSITION;
        (*thisfile).flags &= !(0o1 as std::ffi::c_int);
    } else {
        (*thisfile).fsize = if (*thisfile).flags & 0o10 as std::ffi::c_int != 0 {
            size_helpdata as POSITION
        } else {
            filesize((*thisfile).file)
        };
    }
    if lseek(
        (*thisfile).file,
        0 as std::ffi::c_int as less_off_t,
        0 as std::ffi::c_int,
    ) == -(1 as std::ffi::c_int) as off_t
    {
        error(
            b"seek error to 0\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
    }
}
unsafe extern "C" fn ch_addbuf() -> std::ffi::c_int {
    let mut bp: *mut buf = 0 as *mut buf;
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    bp = calloc(
        1 as std::ffi::c_int as std::ffi::c_ulong,
        ::core::mem::size_of::<buf>() as std::ffi::c_ulong,
    ) as *mut buf;
    if bp.is_null() {
        return 1 as std::ffi::c_int;
    }
    (*thisfile).nbufs += 1;
    (*thisfile).nbufs;
    (*bp).block = -(1 as std::ffi::c_int) as BLOCKNUM;
    bn = &mut (*bp).node;
    (*bn).next = &mut (*thisfile).buflist;
    (*bn).prev = (*thisfile).buflist.prev;
    (*(*thisfile).buflist.prev).next = bn;
    (*thisfile).buflist.prev = bn;
    (*bn).hnext = (*thisfile).hashtbl[0 as std::ffi::c_int as usize].hnext;
    (*bn).hprev = &mut *((*thisfile).hashtbl)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize) as *mut bufnode;
    (*(*thisfile).hashtbl[0 as std::ffi::c_int as usize].hnext).hprev = bn;
    (*thisfile).hashtbl[0 as std::ffi::c_int as usize].hnext = bn;
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn init_hashtbl() {
    let mut h: std::ffi::c_int = 0;
    h = 0 as std::ffi::c_int;
    while h < 1024 as std::ffi::c_int {
        (*thisfile).hashtbl[h as usize].hnext =
            &mut *((*thisfile).hashtbl).as_mut_ptr().offset(h as isize) as *mut bufnode;
        (*thisfile).hashtbl[h as usize].hprev =
            &mut *((*thisfile).hashtbl).as_mut_ptr().offset(h as isize) as *mut bufnode;
        h += 1;
    }
}
unsafe extern "C" fn ch_delbufs() {
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    while (*thisfile).buflist.next != &mut (*thisfile).buflist as *mut bufnode {
        bn = (*thisfile).buflist.next;
        (*(*bn).next).prev = (*bn).prev;
        (*(*bn).prev).next = (*bn).next;
        free(bn as *mut buf as *mut std::ffi::c_void);
    }
    (*thisfile).nbufs = 0 as std::ffi::c_int;
    init_hashtbl();
}
#[no_mangle]
pub unsafe extern "C" fn seekable(mut f: std::ffi::c_int) -> std::ffi::c_int {
    return (lseek(f, 1 as std::ffi::c_int as less_off_t, 0 as std::ffi::c_int)
        != -(1 as std::ffi::c_int) as off_t) as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ch_set_eof() {
    if (*thisfile).fsize != -(1 as std::ffi::c_int) as POSITION
        && (*thisfile).fsize < (*thisfile).fpos
    {
        (*thisfile).fsize = (*thisfile).fpos;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ch_init(
    mut f: std::ffi::c_int,
    mut flags: std::ffi::c_int,
    mut nread: ssize_t,
) {
    thisfile = get_filestate(curr_ifile) as *mut filestate;
    if thisfile.is_null() {
        thisfile = ecalloc(
            1 as std::ffi::c_int as size_t,
            ::core::mem::size_of::<filestate>() as std::ffi::c_ulong,
        ) as *mut filestate;
        (*thisfile).buflist.prev = &mut (*thisfile).buflist;
        (*thisfile).buflist.next = (*thisfile).buflist.prev;
        (*thisfile).nbufs = 0 as std::ffi::c_int;
        (*thisfile).flags = flags;
        (*thisfile).fpos = 0 as std::ffi::c_int as POSITION;
        (*thisfile).block = 0 as std::ffi::c_int as BLOCKNUM;
        (*thisfile).offset = 0 as std::ffi::c_int as size_t;
        (*thisfile).file = -(1 as std::ffi::c_int);
        (*thisfile).fsize = -(1 as std::ffi::c_int) as POSITION;
        init_hashtbl();
        if flags & 0o1 as std::ffi::c_int != 0 && seekable(f) == 0 {
            (*thisfile).flags &= !(0o1 as std::ffi::c_int);
        }
        set_filestate(curr_ifile, thisfile as *mut std::ffi::c_void);
    }
    if (*thisfile).file == -(1 as std::ffi::c_int) {
        (*thisfile).file = f;
    }
    (*thisfile).fsize = if flags & 0o10 as std::ffi::c_int != 0 {
        size_helpdata as POSITION
    } else {
        filesize((*thisfile).file)
    };
    if (*thisfile).fsize == 0 as std::ffi::c_int as POSITION
        && nread > 0 as std::ffi::c_int as ssize_t
    {
        (*thisfile).flags |= 0o40 as std::ffi::c_int;
    }
    ch_flush();
}
#[no_mangle]
pub unsafe extern "C" fn ch_close() {
    let mut keepstate: lbool = LFALSE;
    if thisfile.is_null() {
        return;
    }
    if (*thisfile).flags
        & (0o1 as std::ffi::c_int | 0o4 as std::ffi::c_int | 0o10 as std::ffi::c_int)
        != 0
        && (*thisfile).flags & 0o2 as std::ffi::c_int == 0
    {
        ch_delbufs();
    } else {
        keepstate = LTRUE;
    }
    if (*thisfile).flags & 0o2 as std::ffi::c_int == 0 {
        if (*thisfile).flags & (0o4 as std::ffi::c_int | 0o10 as std::ffi::c_int) == 0 {
            close((*thisfile).file);
        }
        (*thisfile).file = -(1 as std::ffi::c_int);
    } else {
        keepstate = LTRUE;
    }
    if keepstate as u64 == 0 {
        free(thisfile as *mut std::ffi::c_void);
        thisfile = 0 as *mut filestate;
        set_filestate(curr_ifile, 0 as *mut std::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ch_getflags() -> std::ffi::c_int {
    if thisfile.is_null() {
        return 0 as std::ffi::c_int;
    }
    return (*thisfile).flags;
}
*/
