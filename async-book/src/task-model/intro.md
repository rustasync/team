# Background: sync vs. async

It's often easiest to understand asynchronous programming *by contrast* with
synchronous programming. So let's start with a simple sync example:

```rust
// reads 4096 bytes into `my_vec`
socket.read_exact(&mut my_vec[..4096]);
```

This code is using [`read_exact`] from the standard library to read from
`socket`. Let's see what the docs say:

[`read_exact`]: https://static.rust-lang.org/doc/master/std/io/trait.Read.html#method.read_exact

> Read the exact number of bytes required to fill the given buffer.

So, if this method returns successfully, we're guaranteed that `my_vec` is
filled, meaning we've read 4k from `socket`. Great!

But what if the data isn't available yet? What if it hasn't even been sent from
the other side of the socket?

**To fulfill its guarantee, the `read_exact` method must *wait*. That's where the
term "synchrony" comes from: `read_exact` is *sychronizing* with the
availability of the needed data.**

To be more precise, `read_exact` "blocks" the thread that calls it, meaning that
the thread cannot make further progress until the data has been received.  The
problem is that a thread, in general, is a weighty thing to waste. And while
this thread is blocked, it's doing no useful work; all of the action is at the
OS level, until the data becomes available and the thread is unblocked.

More broadly, if we want to handle a number of connections while using methods
like `read_exact`, we're going to need something like a thread per connection;
otherwise, the handling of one connection could be blocked waiting for activity
on another connection. Even with a lot of tuning, the overhead of threads will
limit scalability.

## Asynchrony

To achieve better scalability, we need to avoid tying up an entire thread every
time we're waiting for some resource to become available. Most operating systems
provide a "non-blocking", i.e. *asychronous* mode for interacting with objects
like sockets. In this mode, operations that are not immediately ready return
with an error, allowing you to continue doing other work on the thread.

Working *manually* with resources in this way, though, can be quite painful. You
have to figure out how to juggle all of the "in flight" operations within a
single thread, even though most of the time those operations are going to
arise from completely independent activities (like two separate connections).

Fortunately, Rust provides a way of doing async programming that *feels* in many
respects like using threads, but under the hood accesses resources
asynchronously, and automatically juggles the in-flight operations for you. The
core ingredient is *tasks*, which you can think of as "lightweight threads"
(akin to [goroutines]) that can be automatically juggled onto a smaller number
of OS-level threads.

[goroutines]: https://tour.golang.org/concurrency/1

So let's take a look at the task model!
