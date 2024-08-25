//! Internal utilities

/// Utility for no-alloc str writing to a buffer via `core::fmt`
pub struct StrWriter<'a> {
    pub buf: &'a mut [u8],
    pub written: usize,
}

impl<'a> StrWriter<'a> {
    #[inline]
    pub fn new(buf: &'a mut [u8]) -> Self {
        let written = 0;
        Self { buf, written }
    }
}

impl<'a> core::fmt::Write for StrWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let remaining = self.buf.len() - self.written;
        let write_len = remaining.min(s.len());
        let write_bytes = &s.as_bytes()[..write_len];
        // infallible truncating write
        self.buf[self.written..][..write_len].copy_from_slice(write_bytes);
        self.written += write_len;
        Ok(())
    }
}
