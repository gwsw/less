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

    pub fn add_data(&mut self, data: &[u8]) {
        for b in data {
            self.data.push(*b);
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        self.data.pop()
    }

    pub fn set(&mut self, src: &XBuffer) {
        self.reset();
        self.add_data(&src.data);
    }
}
