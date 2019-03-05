#![feature(async_await, await_macro, futures_api)]

extern crate futures;

use futures::io::{AsyncRead, AsyncWrite, Error};
use std::io::{Read, Write};
use std::task::{Poll, Poll::Ready, Waker};

/// # Examples
/// ```
/// #![feature(async_await, await_macro, futures_api)]
/// use std::io::Cursor;
/// use futures::{
///     io::AsyncReadExt,
///     io::AsyncWriteExt,
///     future::FutureExt,
///     executor::block_on,
/// };
///
/// use pseudo_async_io;
///
/// let mut w = pseudo_async_io::wrap(Cursor::new(vec![1; 6]));
/// block_on(w.write_all(&[1, 2, 3]));
/// assert_eq!(w.inner.get_ref(), &[1, 2, 3, 1, 1, 1]);
/// let mut tmp = [0; 4];
/// block_on(w.read(&mut tmp));
/// assert_eq!(&tmp, &[1, 1, 1, 0]);
///
///
/// ```
pub fn wrap<I>(i: I) -> PseudoAsyncIo<I> {
    PseudoAsyncIo::new(i)
}

pub struct PseudoAsyncIo<I> {
    pub inner: I
}

impl<I> PseudoAsyncIo<I> {
    pub fn new(i: I) -> PseudoAsyncIo<I> {
        PseudoAsyncIo { inner: i }
    }
}

impl<R: Read> AsyncRead for PseudoAsyncIo<R> {
    fn poll_read(&mut self, _: &Waker, buf: &mut [u8]) -> Poll<Result<usize, Error>> {
        Ready(self.inner.read(buf))
    }
}


impl<W: Write> AsyncWrite for PseudoAsyncIo<W> {
    fn poll_write(&mut self, _: &Waker, b: &[u8]) -> Poll<Result<usize, Error>> {
        Ready(self.inner.write(b))
    }

    fn poll_flush(&mut self, _: &Waker) -> Poll<Result<(), Error>> {
        Ready(Ok(self.inner.flush()?))
    }

    fn poll_close(&mut self, _: &Waker) -> Poll<Result<(), Error>> {
        Ready(Ok(()))
    }
}

/// # Examples
/// ```
/// #![feature(async_await, await_macro, futures_api)]
/// use std::io::Cursor;
/// use futures::{
///     io::AsyncReadExt,
///     io::AsyncWriteExt,
///     future::FutureExt,
///     executor::block_on,
/// };
/// use pseudo_async_io;
/// let mut rc = Cursor::new(vec![5, 4, 3, 2, 1, 0]);
/// let mut wc = Cursor::new(vec![0; 6]);
/// let mut w = pseudo_async_io::wrap_pair(rc, wc);
/// block_on(w.write_all(&[1, 2, 3]));
/// assert_eq!(w.writer.get_ref(), &[1, 2, 3, 0, 0, 0]);
/// let mut tmp = [0; 3];
/// block_on(w.read(&mut tmp));
/// assert_eq!(&tmp, &[5, 4, 3]);
/// ```
pub fn wrap_pair<R: Read, W: Write>(r: R, w: W) -> PseudoAsyncIoPair<R, W> {
    PseudoAsyncIoPair::new(r, w)
}

pub struct PseudoAsyncIoPair<R: Read, W: Write> {
    pub reader: R,
    pub writer: W,
}

impl<R: Read, W: Write> PseudoAsyncIoPair<R, W> {
    pub fn new(r: R, w: W) -> PseudoAsyncIoPair<R, W> {
        PseudoAsyncIoPair { reader: r, writer: w }
    }
}

impl<R: Read, W: Write> AsyncRead for PseudoAsyncIoPair<R, W> {
    fn poll_read(&mut self, _: &Waker, b: &mut [u8]) -> Poll<Result<usize, Error>> {
        Ready(self.reader.read(b))
    }
}

impl<R: Read, W: Write> AsyncWrite for PseudoAsyncIoPair<R, W> {
    fn poll_write(&mut self, _: &Waker, b: &[u8]) -> Poll<Result<usize, Error>> {
        Ready(self.writer.write(b))
    }

    fn poll_flush(&mut self, _: &Waker) -> Poll<Result<(), Error>> {
        Ready(Ok(self.writer.flush()?))
    }

    fn poll_close(&mut self, _: &Waker) -> Poll<Result<(), Error>> {
        Ready(Ok(()))
    }
}
