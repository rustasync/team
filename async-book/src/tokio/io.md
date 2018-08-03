# Reading and writing

The `futures` crate contains an `io` module, which is the async counterpart to
`std::io`. That module defines, in particular, the core primitives for doing
async reading, writing, and flushing:

```rust,no_run
trait AsyncRead {
    fn read(&mut self, buf: &mut [u8]) -> AsyncIoResult<usize>;
}

trait AsyncWrite {
    fn write(&mut self, buf: &[u8]) -> AsyncIoResult<usize>;
    fn flush(&mut self) -> AsyncIoResult<()>;
}
```

These methods work exactly like their counterparts in `std`, except that if the
underlying I/O object is not ready to perform the requested action, they return
`Ok(Async::WillWake)`, and stash the given `wake` to be used once I/O is
ready. Once more, the fact that their result type involves `Async` is the clear
signal that they plug into the async task system.

## Example: echoing input

While the `AsyncRead` and `AsyncWrite` traits are simple enough, there are some
significant differences in *using* them, compared to the synchronous
versions. Most importantly, async tasks generally have an explicit *overall
state* associated with them (which plays the role usually played by the stack in
synchronous programming). To see this concretely, let's write a task for echoing
everything sent on a socket. First, the basic setup:

```rust,no_run
use tokio::net::TcpStream;

// The task structure -- echoing on a *single* connection
struct Echo {
    // The connection
    io: TcpStream,

    // Buffered data to be echoed back
    buf: Vec<u8>,

    // The current state of the "echo state machine"
    state: EchoState,
}

enum EchoState {
    // The next step is reading into `buf`, from the front
    Reading,

    // The next step is echoing back out `buf`, from the
    // given start and end points.
    Writing(usize, usize),
}

impl Echo {
    fn new(io: TcpStream) -> Echo {
        Echo {
            io,
            state: EchoState::Reading,
            buf: vec![0; 4096],
        }
    }
}
```

The idea then is that the `Echo` task alternates between reading and writing. If
at any point it is unable to perform that task, it returns `Async::WillWake`,
having been enrolled to be woken when the needed I/O is available. Such
state-machine tasks almost always have an outer `loop` that continuously moves
through the states until an obstruction is reached:

```rust,no_run
impl Future for Echo {
    type Item = ();
    type Error = io::Error;

    fn complete(&mut self) -> AsyncIoResult<()> {
        loop {
            self.state = match self.state {
                EchoState::Reading => {
                    match self.io.read(&mut self.buf)? {
                        Async::WillWake => return Ok(Async::WillWake),
                        Async::Done(len) => EchoState::Writing(0, len),
                    }
                }
                EchoState::Writing(from, to) if from >= to => {
                    EchoState::Reading
                }
                EchoState::Writing(from, to) => {
                    match self.io.write(&self.buf[from..to])? {
                        Async::WillWake => return Ok(Async::WillWake),
                        Async::Done(n) => EchoState::Writing(from + n, to),
                    }
                }
            };
        }
    }
}
```

It's important to note that we can freely "bubble up" `WillWake`, because if a
function like `read`, returns it, that function has already *guaranteed* to wake
up our task when `read`ing is possible. In particular, the `tokio` crate takes
care of stashing the `WakeHandle` as necessary whenever we attempt an
`AsyncRead::read`, and so on. All we have to do is bubble out the `WillWake`
result.

While the code here is not *so* complicated, it's a bit noisy for something so
simple. Much of the rest of this book will cover higher-level abstractions that
cut down on the noise. For this kind of low-level programming, though, the
futures crate provides a `try_done` macro that works much like the `?` operator,
except that it *also* bubbles out `Async::WillWake`. Using that macro, we can
rewrite the code as follows:

```rust,no_run
impl Future for Echo {
    type Item = ();
    type Error = io::Error;

    fn complete(&mut self) -> AsyncIoResult<()> {
        loop {
            self.state = match self.state {
                EchoState::Reading => {
                    let let = try_done!(self.io.read(&mut self.buf));
                    EchoState::Writing(0, len)
                }
                EchoState::Writing(from, to) if from >= to => {
                    EchoState::Reading
                }
                EchoState::Writing(from, to) => {
                    let n = try_done!(self.io.write(&self.buf[from..to]))
                    EchoState::Writing(from + n, to)
                }
            };
        }
    }
}
```

As we'll see in the [Futures](futures/_chapter.html) chapter, though, we'll
ultimately do better than this, by avoid writing a state machine at all.

## Exercises

- What would happen if we did not include the outer `loop`?
- Use the `CurrentThread` executor and `TcpListener` to turn the above code into
  a complete, working server.

https://gist.github.com/alexcrichton/da80683060f405d6be0e06b426588886
