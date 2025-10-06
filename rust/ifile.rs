use std::any::Any;
use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};

/// Opaque handle (like the C `IFILE` pointer).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IFileHandle(pub usize);

/// Position type used by `scrpos` (matches C `POSITION`).
pub type Position = u64;
/// NULL_POSITION analogue.
pub const NULL_POSITION: Position = u64::MAX;

/// Saved screen position (mirrors `struct scrpos` from the C code).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrPos {
    pub pos: Position,
    pub ln: i32,
}

impl Default for ScrPos {
    fn default() -> Self {
        Self {
            pos: NULL_POSITION,
            ln: 0,
        }
    }
}

/// Rust representation of the C `struct ifile` (same field names where practical).
#[derive(Debug)]
pub struct IFile {
    /// Display filename (h_filename in C).
    pub h_filename: PathBuf,
    /// Canonical filename (h_rfilename in C); may be None if resolution failed.
    pub h_rfilename: Option<PathBuf>,
    /// File state (opaque pointer in C). We store an optional boxed Any value.
    pub h_filestate: Option<Box<dyn Any + Send + Sync>>,
    /// Index within the command-line list (h_index in C). 1-based like original C.
    pub h_index: i32,
    /// Hold count (h_hold in C).
    pub h_hold: i32,
    /// Has this ifile been opened? (h_opened in C)
    pub h_opened: bool,
    /// Saved position within the file (h_scrpos in C)
    pub h_scrpos: ScrPos,
    /// Alt pipe pointer (h_altpipe in C) stored opaquely
    pub h_altpipe: Option<Box<dyn Any + Send + Sync>>,
    /// Alt filename (h_altfilename in C)
    pub h_altfilename: Option<PathBuf>,
}

impl IFile {
    fn new(filename: impl AsRef<Path>) -> Self {
        let filename = filename.as_ref().to_path_buf();
        let r = fs::canonicalize(&filename).ok();
        Self {
            h_filename: filename,
            h_rfilename: r,
            h_filestate: None,
            h_index: 0,
            h_hold: 0,
            h_opened: false,
            h_scrpos: ScrPos::default(),
            h_altpipe: None,
            h_altfilename: None,
        }
    }
}

/// Manager storing the ordered list of IFiles using a `VecDeque`.
/// The ordering in the deque mirrors the linked-list order in the
/// original C implementation (front == first after the anchor).
pub struct IFileManager {
    files: VecDeque<IFile>,
}

impl IFileManager {
    pub fn new() -> Self {
        Self {
            files: VecDeque::new(),
        }
    }

    /// Helper: try to canonicalize (like `lrealpath` in the C code).
    fn canonicalize(path: &Path) -> Option<PathBuf> {
        fs::canonicalize(path).ok()
    }

    /// Get number of ifiles (like `nifile()` in C).
    pub fn nifile(&self) -> usize {
        self.files.len()
    }

    /// Recompute h_index for all entries so they match the C behaviour
    /// (1-based index where the first file after the anchor is index 1).
    fn recompute_indices(&mut self) {
        for (i, f) in self.files.iter_mut().enumerate() {
            f.h_index = (i as i32) + 1; // 1-based
        }
    }

    /// Insert a new ifile after the given handle. If `after` is None,
    /// insert at the beginning (after the conceptual anchor), matching
    /// the C `new_ifile(..., prev)` semantics where prev==NULL means
    /// insert at the head.
    pub fn link_after(
        &mut self,
        after: Option<IFileHandle>,
        filename: impl AsRef<Path>,
    ) -> IFileHandle {
        let slot = IFile::new(filename);
        let insert_pos = match after {
            None => 0, // insert at head (after anchor)
            Some(h) if h.0 < self.files.len() => h.0 + 1,
            _ => self.files.len(), // invalid handle -> append at end
        };
        self.files.insert(insert_pos, slot);
        self.recompute_indices();
        IFileHandle(insert_pos)
    }

    /// Allocate a new ifile and insert it after `prev` (mirrors `new_ifile`).
    /// `prev` may be None (insert at head).
    pub fn new_ifile(
        &mut self,
        filename: impl AsRef<Path>,
        prev: Option<IFileHandle>,
    ) -> IFileHandle {
        println!("---> new file: {:?}", filename.as_ref());
        self.link_after(prev, filename)
    }

    /// Delete an existing ifile. Returns true if deletion occurred.
    /// Mirrors the behaviour of `del_ifile` (note: callers should handle
    /// any relocation of "current file" like the C code does).
    pub fn del_ifile(&mut self, h: Option<IFileHandle>) -> bool {
        let h = match h {
            Some(hh) => hh,
            None => return false,
        };
        if h.0 >= self.files.len() {
            return false;
        }
        // Remove the slot
        let _removed = self.files.remove(h.0);
        self.recompute_indices();
        true
    }

    /// Find an ifile by filename. Attempts to canonicalize the argument
    /// and compare against stored `h_rfilename` (like `find_ifile`).
    pub fn find_ifile(&self, filename: impl AsRef<Path>) -> Option<IFileHandle> {
        let filename = filename.as_ref();
        let r = Self::canonicalize(filename);
        for (i, f) in self.files.iter().enumerate() {
            if let (Some(ref rf), Some(ref rf_arg)) = (&f.h_rfilename, &r) {
                if rf == rf_arg {
                    return Some(IFileHandle(i));
                }
            } else {
                // fallback: compare display names (not canonical)
                if f.h_filename == filename {
                    return Some(IFileHandle(i));
                }
            }
        }
        None
    }

    /// Get or create the ifile for `filename`. If not found, insert after `prev`.
    pub fn get_ifile(
        &mut self,
        filename: impl AsRef<Path>,
        prev: Option<IFileHandle>,
    ) -> IFileHandle {
        if let Some(h) = self.find_ifile(filename.as_ref()) {
            return h;
        }
        self.new_ifile(filename, prev)
    }

    /// Get the display filename associated with an ifile.
    pub fn get_filename(&self, h: Option<IFileHandle>) -> Option<&Path> {
        let h = h?;
        self.files.get(h.0).map(|f| f.h_filename.as_path())
    }

    /// Get the canonical (real) filename associated with an ifile.
    pub fn get_real_filename(&self, h: Option<IFileHandle>) -> Option<&Path> {
        let h = h?;
        self.files.get(h.0).and_then(|f| f.h_rfilename.as_deref())
    }

    /// Get the index of the file associated with an ifile (1-based).
    pub fn get_index(&self, h: Option<IFileHandle>) -> Option<i32> {
        let h = h?;
        self.files.get(h.0).map(|f| f.h_index)
    }

    /// Store the screen position for an ifile.
    pub fn store_pos(&mut self, h: Option<IFileHandle>, scrpos: ScrPos) {
        if let Some(hh) = h {
            if let Some(f) = self.files.get_mut(hh.0) {
                f.h_scrpos = scrpos;
            }
        }
    }

    /// Recall the screen position associated with an ifile.
    pub fn get_pos(&self, h: Option<IFileHandle>) -> Option<ScrPos> {
        let h = h?;
        self.files.get(h.0).map(|f| f.h_scrpos)
    }

    /// Mark the ifile as opened.
    pub fn set_open(&mut self, h: Option<IFileHandle>) {
        if let Some(hh) = h {
            if let Some(f) = self.files.get_mut(hh.0) {
                f.h_opened = true;
            }
        }
    }

    /// Whether the ifile has been opened previously.
    pub fn opened(&self, h: Option<IFileHandle>) -> bool {
        if let Some(hh) = h {
            self.files.get(hh.0).map(|f| f.h_opened).unwrap_or(false)
        } else {
            false
        }
    }

    /// Increment/decrement hold count (like `hold_ifile`).
    pub fn hold_ifile(&mut self, h: Option<IFileHandle>, incr: i32) {
        if let Some(hh) = h {
            if let Some(f) = self.files.get_mut(hh.0) {
                f.h_hold += incr;
            }
        }
    }

    /// Return hold count.
    pub fn held_ifile(&self, h: Option<IFileHandle>) -> Option<i32> {
        let h = h?;
        self.files.get(h.0).map(|f| f.h_hold)
    }

    /// Get/Set the opaque filestate pointer.
    pub fn get_filestate(&self, h: Option<IFileHandle>) -> Option<&Box<dyn Any + Send + Sync>> {
        let h = h?;
        self.files.get(h.0).and_then(|f| f.h_filestate.as_ref())
    }

    pub fn set_filestate(
        &mut self,
        h: Option<IFileHandle>,
        fs: Option<Box<dyn Any + Send + Sync>>,
    ) {
        if let Some(hh) = h {
            if let Some(f) = self.files.get_mut(hh.0) {
                f.h_filestate = fs;
            }
        }
    }

    /// Set/get alt pipe and alt filename (opaque in original C).
    pub fn set_altpipe(&mut self, h: Option<IFileHandle>, p: Option<Box<dyn Any + Send + Sync>>) {
        if let Some(hh) = h {
            if let Some(f) = self.files.get_mut(hh.0) {
                f.h_altpipe = p;
            }
        }
    }
    pub fn get_altpipe(&self, h: Option<IFileHandle>) -> Option<&Box<dyn Any + Send + Sync>> {
        let h = h?;
        self.files.get(h.0).and_then(|f| f.h_altpipe.as_ref())
    }

    pub fn set_altfilename(&mut self, h: Option<IFileHandle>, alt: Option<impl AsRef<Path>>) {
        println!("set: {:?}", h);
        if let Some(hh) = h {
            if let Some(f) = self.files.get_mut(hh.0) {
                f.h_altfilename = alt.map(|p| p.as_ref().to_path_buf());
            }
        }
    }

    pub fn get_altfilename(&self, h: Option<IFileHandle>) -> Option<&Path> {
        let h = h?;
        self.files.get(h.0).and_then(|f| f.h_altfilename.as_deref())
    }

    /// Return the ifile after a given one in the list (NULL_IFILE semantics supported).
    /// Mirrors `next_ifile` in C: if `h` is None, returns the first file; if we're at the end,
    /// returns None.
    pub fn next_ifile(&self, h: Option<IFileHandle>) -> Option<IFileHandle> {
        if self.files.is_empty() {
            return None;
        }
        let pos = match h {
            None => {
                return Some(IFileHandle(0));
            }
            Some(hh) => hh.0,
        };
        if pos + 1 >= self.files.len() {
            None
        } else {
            Some(IFileHandle(pos + 1))
        }
    }

    /// Return the ifile before a given one in the list (NULL_IFILE semantics supported).
    pub fn prev_ifile(&self, h: Option<IFileHandle>) -> Option<IFileHandle> {
        if self.files.is_empty() {
            return None;
        }
        let pos = match h {
            None => {
                return Some(IFileHandle(self.files.len() - 1));
            }
            Some(hh) => hh.0,
        };
        if pos == 0 {
            None
        } else {
            Some(IFileHandle(pos - 1))
        }
    }

    /// Return a different ifile from the given one, preferring prev then next (like `getoff_ifile`).
    pub fn getoff_ifile(&self, h: Option<IFileHandle>) -> Option<IFileHandle> {
        if let Some(p) = self.prev_ifile(h) {
            return Some(p);
        }
        if let Some(n) = self.next_ifile(h) {
            return Some(n);
        }
        None
    }
}

// ------------------
// Unit tests
// ------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_link_and_indices() {
        let mut mgr = IFileManager::new();
        let a = mgr.new_ifile("a.txt", None);
        let b = mgr.new_ifile("b.txt", Some(a));
        let c = mgr.new_ifile("c.txt", Some(b));

        assert_eq!(mgr.nifile(), 3);
        assert_eq!(mgr.get_index(Some(a)).unwrap(), 1);
        assert_eq!(mgr.get_index(Some(b)).unwrap(), 2);
        assert_eq!(mgr.get_index(Some(c)).unwrap(), 3);

        // insert x after a -> a,x,b,c
        let x = mgr.link_after(Some(a), "x.txt");
        assert_eq!(mgr.get_filename(Some(x)).unwrap(), Path::new("x.txt"));
        assert_eq!(mgr.get_index(Some(x)).unwrap(), 2);
        assert_eq!(mgr.get_index(Some(b)).unwrap(), 3);
    }

    #[test]
    fn find_and_get_ifile() {
        let mut mgr = IFileManager::new();
        let h = mgr.get_ifile("file.txt", None);
        assert_eq!(mgr.get_filename(Some(h)).unwrap(), Path::new("file.txt"));
        let found = mgr.find_ifile("file.txt").unwrap();
        assert_eq!(found, h);
    }
}
