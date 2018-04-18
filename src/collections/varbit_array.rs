/// A variable bit array which uses u64 as a backend.
#[derive(Debug)]
pub struct VarbitArray {
    backend: Vec<u64>,
    bit_size: u8,
    mask: u64,
    len: usize,
}

impl VarbitArray {
    pub fn new(bit_size: u8, len: usize) -> VarbitArray {
        VarbitArray {
            backend: vec![0; ((bit_size as usize * len) as f64 / 64.0).ceil() as usize],
            bit_size,
            mask: (1 << bit_size) - 1,
            len,
        }
    }

    pub fn bit_size_needed(mut num: usize) -> u8 {
        let mut count = 0;
        while num != 0 {
            num >>= 1;
            count += 1;
        }
        count
    }

    pub fn change_bit_size(&mut self, new_bit_size: u8) {
        let mut new_array = VarbitArray::new(new_bit_size, self.len);

        for i in 0..self.len {
            let v = self.get(i);
            new_array.set(i, v);
        }

        *self = new_array;
    }

    pub fn bit_size(&self) -> u8 {
        self.bit_size
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, i: usize) -> u64 {
        if i >= self.len {
            panic!("index out of bounds: the len is {} but the index is {}", self.len, i);
        }

        let offset = i * self.bit_size as usize;
        let relative_offset = offset % 64;
        let end_offset = relative_offset + self.bit_size as usize;

        let long = self.backend[offset / 64];

        if end_offset > 64 {
            let mut num = long >> relative_offset;
            let next_long = self.backend[offset / 64 + 1];
            // (64 - relative_offset) is the length of number in first long.
            num |= next_long << (64 - relative_offset);

            num & self.mask
        } else {
            (long >> relative_offset) & self.mask
        }
    }

    pub fn set(&mut self, i: usize, mut v: u64) {
        if i >= self.len {
            panic!("index out of bounds: the len is {} but the index is {}", self.len, i);
        }

        v &= self.mask;

        let offset = i * self.bit_size as usize;
        let relative_offset = offset % 64;
        let end_offset = relative_offset + self.bit_size as usize;
        let long_index = offset / 64;

        let mut long = self.backend[long_index];
        // clear the part
        long &= !(self.mask << relative_offset);
        // set
        long |= v << relative_offset;
        self.backend[long_index] = long;

        // need to set extra bits to next long
        if end_offset > 64 {
            let mut next_long = self.backend[long_index + 1];
            let extra_bits = end_offset - 64;
            // clear the extra bits
            next_long >>= extra_bits;
            next_long <<= extra_bits;

            next_long |= v >> (64 - relative_offset);
            self.backend[long_index + 1] = next_long;
        }
    }

    pub fn as_longs(&self) -> &[u64] {
        &self.backend
    }
}