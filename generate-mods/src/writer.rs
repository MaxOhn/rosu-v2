use std::{fs::File, io, slice};

use crate::error::GenResult;

pub struct Writer {
    inner: io::BufWriter<File>,
}

impl Writer {
    pub fn new(file: File) -> Self {
        Self {
            inner: io::BufWriter::new(file),
        }
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub fn write(&mut self, content: impl Content) -> GenResult {
        let bytes = content.to_bytes();

        debug_assert!(
            bytes.windows(2).all(|window| window != [b' ', b' ']),
            "Found consecutive whitespace, forgot a backslash?"
        );

        self.write_raw(bytes)
    }

    pub fn write_raw(&mut self, bytes: &[u8]) -> GenResult {
        io::Write::write_all(&mut self.inner, bytes).map_err(From::from)
    }

    pub fn flush(&mut self) -> GenResult {
        io::Write::flush(&mut self.inner).map_err(From::from)
    }
}

pub trait Content {
    fn to_bytes(&self) -> &[u8];
}

impl Content for str {
    fn to_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Content for &str {
    fn to_bytes(&self) -> &[u8] {
        Content::to_bytes(*self)
    }
}

impl Content for &Box<str> {
    fn to_bytes(&self) -> &[u8] {
        Content::to_bytes(self.as_ref())
    }
}

impl Content for u8 {
    fn to_bytes(&self) -> &[u8] {
        slice::from_ref(self)
    }
}
