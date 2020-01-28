pub struct ByteBuffer(Vec<u8>);

impl ByteBuffer {
    pub fn new(real: Vec<u8>) -> ByteBuffer {
        ByteBuffer(real)
    }
}

impl ByteBuffer {
    pub fn get_length(&self) -> i64 {
        self.0.len() as i64
    }

    pub fn get_content(&self) -> *mut u8 {
        self.0.as_ptr() as *mut u8
    }

    pub fn from_string(string: String) -> ByteBuffer {
        ByteBuffer(Vec::<u8>::from(string))
    }
}
