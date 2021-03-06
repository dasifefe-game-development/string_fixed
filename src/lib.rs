use core::str;

pub struct StringFixed {
    data: [u8; 64],
}

impl StringFixed {
    pub fn clear(&mut self) {
        for index in 0..self.data.len() {
            self.data[index] = 0;
        }
    }
    pub fn from_string(&mut self, string: &str) {
        let string_array_byte = string.as_bytes();
        for index in 0..self.data.len() {
            if index < string_array_byte.len() {
                self.data[index] = string_array_byte[index];
            } else {
                self.data[index] = 0
            }
        }
    }
    pub fn to_string(&self) -> &str {
        let result_string = str::from_utf8(&self.data);
        match result_string {
            Ok(string) => {
                string
            },
            Err(_error) => {
                ""
            }
        }
    }
}
