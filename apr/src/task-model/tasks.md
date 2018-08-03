# Taming async with tasks

Rust provides asynchrony through *tasks*, which are:

- Pieces of work that run independently (i.e., possibly concurrently), much like
  OS threads.
- Lightweight, in that they do not require a full OS thread. Instead, a single
  OS thread can juggle any number of independent tasks. This setup is sometimes
  known as "M:N threading" or "user space threads".

**The key idea is that, any time a task would [block](task-model/intro.html)
waiting for some external event to occur, it instead returns control to the
thread that was executing it (the "executor"), which can make progress on
another task instead.**

To see how these ideas work, over the course of this chapter we will build a
*toy* version of the task and executor system from the `futures` crate. At the
end of the chapter, we'll then connect these toy versions to the more
sophisticated abstractions in the actual crate.

We'll start by defining a simple task trait. Here, a task encompasses some
(possibly ongoing) work; you can ask the task to try to complete its work by
invoking `poll`:

```rust
/// An independent, non-blocking computation
trait ToyTask {
    /// Attempt to finish executing the task, returning `Async::Pending`
    /// if the task needs to wait for an event before it can complete.
    fn poll(&mut self, waker: &Waker) -> Async<()>;
}
```

At that point, the task will do as much as it can, but it may encounter the need
to wait for an event, e.g. for data to become available on a socket. Rather than
blocking at that point, it should return `Async::Pending`:

```rust
enum Async<T> {
    /// Work completed with a result of type `T`.
    Ready(T),

    /// Work was blocked, and the task is set to be woken when ready
    /// to continue.
    Pending,
}
```

The fact that the task *returns* instead of blocking is what gives the
underlying thread an opportunity to go do other useful work (like calling `poll`
on a *different* task). But how will we know when to try the original task's
`poll` method again?

If you look back at the `ToyTask::poll` method, you may notice that there's an
argument, `waker`, that we glossed over. This value is an instance of the
`Waker` type, which provides a way to wake up a task:

```rust
#[derive(Clone)]
struct Waker { .. }

impl Waker {
    /// Signals that the associated task is ready to be `poll`ed again.
    pub fn wake(&self) { ... }
}
```

So, **whenever you ask a task to execute, you also give it a handle for waking
itself back up**. If the task is unable to proceed because it's waiting for data
on a socket, it can associate that `waker` handle with the socket, so that when data
becomes available, the `waker` call is run.

The `Waker` type is essentially just a trait object for the `Wake` trait, which
allows different executors to use different wakeup strategies:

```rust
trait Wake: Send + Sync {
    /// Signals that the associated task is ready to be `poll`ed again.;
    fn wake(arced: &Arc<Self>)
}

impl<T: Wake + 'static> From<Arc<T>> for Waker {
    fn from(wake: Arc<T>) -> Waker { ... }
}
```

But this is all pretty abstract. Let's walk through the whole story concretely,
building:

- A simple *task executor* which can run any number of tasks on a single OS thread;
- A simple *timer event loop* which can wake up tasks based on timer events;
- A simple example that plugs them together.

Understanding these mechanics will give you a strong foundation for everything
else in the book.
