# Acquiring a socket

Accepting TCP connections with Tokio is much like doing so with `std::net`,
except that it works asynchronously. In particular, `tokio::net` contains a
`TcpListener` with an API similar to the `std::net` version:

```rust,no_run
type AsyncIoResult<T> = AsyncResult<T, io::Error>;

impl TcpListener {
    fn bind(addr: &SocketAddr) -> io::Result<TcpListener>;
    fn accept(&mut self) -> AsyncIoResult<(TcpStream, SocketAddr)>;
}
```

Just like the occurrence of `Result` in a signature tells you that a function
may fail with an error, the occurrence of `Async` or `AsyncResult` tells you
that the function is intended to be used within the async task system. Thus,
looking at the two functions above, we can see that `bind` is a a standard
synchronous function, while `accept` is an asynchronous method.

To quickly see these APIs in action, let's build a future that will accept
connections asynchronously, record the peer address, and then close the
connection:

```rust,no_run
use tokio::net::TcpListener;

struct LogAndDrop {
    listener: TcpListener,
}

impl LogAndDrop {
    fn new(addr: &SocketAddr) -> io::Result<LogAndDrop> {
        LogAndDrop {
            listener: TcpListener::bind(addr)?
        }
    }
}

impl Future for LogAndDrop {
    type Item = ();
    type Error = io::Error;

    fn complete(&mut self) -> AsyncIoResult<()> {
        loop {
            match self.listener.accept(wake) {
                Ok(Async::Done((_, peer))) => {
                    println!("Connected to peer {:?}", peer);
                }
                Ok(Async::WillWake) => {
                    return Ok(Async::WillWake);
                }
                Err(e) => {
                    println!("Error: {:?}; shutting down", e);
                    return Err(e);
                }
            }
        }
    }
}
```

Note that, in the case that we succeed in accepting a connection, after logging
it we immediately loop and try to take another. This is typical for async tasks:
the task code does as much work as it possibly can, stopping only when
encountering an obstruction (such as the listener returning `WillWake`), at
which point it will be descheduled and woken up later, when the obstruction has
been cleared.
