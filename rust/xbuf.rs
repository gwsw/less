#[derive(Clone)]
pub struct XBuffer {
    pub data: Vec<u8>,
    pub init_size: usize,
}

// Expandable text buffer.
impl XBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
            init_size: size,
        }
    }

    pub fn reset(&mut self) {
        self.data.clear();
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.data.push(byte);
    }

    pub fn xbuf_add_char(&mut self, ch: i8) {
        self.add_byte(ch as u8);
    }

    pub fn add_data(&mut self, data: &[u8], len: usize) {
        let mut rem = len;
        if rem == 0 {
            return;
        }
        for b in data {
            self.data.push(*b);
            rem -= 1;
            if rem == 0 {
                return;
            }
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        self.data.pop()
    }

    pub fn set(&mut self, src: &XBuffer) {
        self.reset();
        self.add_data(&src.data, src.data.len());
    }

    pub fn char_data(&self) -> &[i8] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const i8, self.data.len()) }
    }
}
