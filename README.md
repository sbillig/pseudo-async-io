# pseudo-async-io
Simple pseudo-async AsyncRead/AsyncWrite wrappers around the Read and Write traits.

For futures-0.3; requires nightly rust.

```rust
#![feature(async_await, await_macro, futures_api)]

use std::io::Cursor;
use futures::{
    io::AsyncReadExt,
    io::AsyncWriteExt,
	io::Error,
    future::FutureExt,
    executor::block_on,
};
use pseudo_async_io;

async fn do_some_io() -> Result<Vec<u8>, Error> {
	let mut w = pseudo_async_io::wrap(Cursor::new(vec![1; 6]));
	await!(w.write_all(&[1, 2, 3]))?;

	let mut tmp = vec![0; 4];
	await!(w.read(&mut tmp))?;
	Ok(tmp)
}
```
