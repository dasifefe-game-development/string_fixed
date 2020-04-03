pub struct StringFixed {
    data: [u8; 64],
}

impl StringFixed {
    pub fn clear(&mut self) {
        for index in 0..self.data.len() {
            self.data[index] = 0;
        }
    }
}
