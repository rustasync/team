# Example: `ReadExact`

Let's get started with a useful example. The standard library's `Read` trait
provides a convenient function, `read_exact`, which reads a specific number of
bytes from an I/O object (which may require issuing multiple calls to the `read`
method).

We want to "port" this functionality to the async world. Futures are a perfect
match: we can represent

```rust,no_run
struct ReadExactData<R> {
    reader: R,
    buf: Vec<u8>,
}

struct ReadExact<R> {
    data: Option<ReadExactData<R>>,
    from: usize,
    to: usize,
}

fn read_exact<R>(reader: R, len: usize) -> ReadExact<R> {
    ReadExact {
        data: Some(ReadExactData {
            reader,
            buf: vec![0; len],
        },
        from: 0,
        to: len,
    }
}
```

```rust,no_run
impl<R: AsyncRead> Future for ReadExact<R> {
    type Item = ReadExactData<R>;
    type Error = io::Error;

    fn get(&mut self) -> AsynIoResult<Self::Item> {
        use std::mem;

        while self.from < self.to {
            let data = self.data.as_mut().unwrap();
            let n = try_done!(data.read(&mut data.buf[data.from .. data.to]));
            data.from += n;
        }

        Ok(Async::Done(self.data.take().unwrap()))
    }
}
```
