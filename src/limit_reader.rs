use std::io;
use std::cmp;

/// [Original impl](https://github.com/rust-lang/rust/blob/17bc7d8d5be3be9674d702ccad2fa88c487d23b0/src/libstd/old_io/util.rs#L20)
///
/// The LimitReader from the `std` just stops to read when reaches a limit, but we don't want
/// to return partially readed body to the client code because it is useless. This modified LimitReader
/// returns `IoError` with `IoErrorKind::InvalidInput` when it reaches the limit.
#[derive(Debug)]
pub struct LimitReader<R> {
    limit: usize,
    inner: R
}

impl<R: io::Read> LimitReader<R> {
    pub fn new(r: R, limit: usize) -> LimitReader<R> {
        LimitReader { limit: limit, inner: r }
    }

    pub fn into_inner(self) -> R { self.inner }
    pub fn limit(&self) -> usize { self.limit }
}

impl<R: io::Read> io::Read for LimitReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.limit == 0 {
            // Changed code is here
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput, "Body is too big"))
        }

        let len = cmp::min(self.limit, buf.len());
        let res = self.inner.read(&mut buf[..len]);
        match res {
            Ok(len) => self.limit -= len,
            _ => {}
        }
        res
    }
}