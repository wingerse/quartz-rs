#[derive(Debug)]
pub struct NibbleArray {
    backend: Vec<u8>,
    len: usize,
}

impl NibbleArray {
    pub fn new(len: usize) -> NibbleArray {
        NibbleArray { backend: vec![0; (len + 1) / 2], len }
    }

    pub fn new_with_default(len: usize, default: u8) -> NibbleArray {
        let mut ret = NibbleArray { backend: Vec::with_capacity((len + 1) / 2), len };
        // will fill with defaults.
        unsafe { ret.backend.set_len((len + 1) / 2); }

        for i in &mut ret.backend {
            *i = default << 4 | (default & 0x0f);
        }

        ret
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, i: usize) -> u8 {
        if i >= self.len {
            panic!("index out of bounds: the len is {} but the index is {}", self.len, i);
        }

        if i % 2 == 0 {
            self.backend[i / 2] & 0x0f
        } else {
            self.backend[i / 2] >> 4
        }
    }

    pub fn set(&mut self, i: usize, nibble: u8) {
        if i >= self.len {
            panic!("index out of bounds: the len is {} but the index is {}", self.len, i);
        }

        let backend_index = &mut self.backend[i / 2];
        if i % 2 == 0 {
            *backend_index &= !(0x0f as u8);
            *backend_index |= nibble & 0x0f;
        } else {
            *backend_index &= !(0xf0 as u8);
            *backend_index |= nibble << 4;
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.backend
    }
}