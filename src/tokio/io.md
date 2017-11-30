# Reading and writing

```rust
type AsyncResult<T, E> = Result<Async<T>, E>;

trait AsyncRead {
    fn async_read(&mut self, buf: &mut [u8], wake: &WakeHandle) -> AsyncResult<usize, io::Error>;
}

trait AsyncWrite {
    fn async_write(&mut self, buf: &[u8], wake: &WakeHandle) -> AsyncResult<usize, io::Error>;
    fn async_flush(&mut self, wake: &WakeHandle) -> AsyncResult<(), io::Error>;
}

struct Bridge<T>(T);

impl<T: AsyncRead> AsyncRead for Bridge<T> { .. }
impl<T: AsyncWrite> AsyncWrite for Bridge<T> { .. }

impl<T: AsyncRead> Read for Bridge<T> { .. }
impl<T: AsyncWrite> Write for Bridge<T> { .. }
```
