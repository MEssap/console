pub struct Stdout {}

pub struct OutputCache {
    buf: [u8; 512],
}

impl OutputCache {
    pub fn new() -> Self {
        Self { buf: [0u8; 512] }
    }

    /// flush the cache, return buffer length
    pub fn flush() -> i32 {
        todo!()
    }
}
