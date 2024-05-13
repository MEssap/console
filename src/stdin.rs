pub struct Stdin {
    cache: InputCache,
}

pub struct InputCache {
    buf: [u8; 512],
}

impl InputCache {
    pub fn new() -> Self {
        Self { buf: [0u8; 512] }
    }
}
