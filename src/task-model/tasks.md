# Taming async with tasks

Rust provides asynchrony through *tasks*, which are:

- Pieces of work that run independently (i.e., possibly concurrently), much like
  OS threads.
- Lightweight, in that they do not require a full OS thread. Instead, a single
  OS thread can juggle any number of independent tasks. This setup is sometimes
  known as "M:N threading" or "user space threads".

**The key idea is that, any time a task would [block](task-model/intro.html)
waiting for some external event to occur, it instead returns control to the
thread that was executing it.**

To see how these ideas work, over the course of this chapter we will build a
*toy* version of the task and executor system from the `futures` crate. At the
end of the chapter, we'll then connect these toy versions to the more
sophisticated abstractions in the actual crate.

We'll start by defining a simple task trait. Here, a task encompasses some
(possibly ongoing) work; you can ask the task to try to complete its work by
invoking `complete`:

```rust
/// An independent, non-blocking computation
trait ToyTask {
    /// Attempt to finish executing the task, returning `Async::WillWake`
    /// if the task needs to wait for an event before it can complete.
    fn complete(&mut self, wake: &WakeHandle) -> Async<()>;
}
```

At that point, the task will do as much as it can, but it may encounter the need
to wait for an event, e.g. for data to become available on a socket. Rather than
blocking at that point, it should return `Async::WillWake`:

```rust
enum Async<T> {
    /// Work completed with a result of type `T`.
    Done(T),

    /// Work was blocked, and the task is set to be woken when ready
    /// to continue.
    WillWake,
}
```

The fact that the task *returns* instead of blocking is what gives the
underlying thread an opportunity to go do other useful work (like calling `complete`
on a *different* task). But how will we know when to try the original task's
`complete` method again?

If you look back at the `complete` method, you may notice that there's an argument,
`wake`, that we glossed over. This argument is a trait object for the `Wake`
trait:

```rust
trait Wake: Send + Sync + 'static {
    fn wake(&self);
}

type WakeHandle = Arc<Wake>;
```

So, **whenever you ask a task to execute, you also give it a handle for waking
itself back up**. If the task is unable to proceed because it's waiting for data
on a socket, it can associate that `wake` handle with the socket, so that when data
becomes available, the `wake` call is run.

But this is all pretty abstract. Let's walk through the whole story concretely,
building:

- A simple *task executor* which can run any number of tasks on a single OS thread;
- A simple *timer event loop* which can wake up tasks based on timer events;
- A simple example that plugs them together.

Understanding these mechanics will give you a strong foundation for everything
else in the book.
