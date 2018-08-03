# The core definition

In Rust, a future is **an asynchronous computation that can be driven to produce
a value**. It represents a value that may become available in the future, but
which requires pushing along the computation to produce it.

We've [already seen](task-model/real/tasks.html) the core definitions of the
`Future` trait, describing such computations:

```rust,no_run
/// An asynchronous computation that completes with a value or an error.
trait Future {
    type Item;
    type Error;

    /// Attempt to complete the future, yielding `Ok(Async::WillWake)`
    /// if the future is blocked waiting for some other event to occur.
    fn get(&mut self) -> AsyncResult<Self::Item, Self::Error>;

    // ... and a large number of default methods that we'll meet shortly!
}

type AsyncResult<T, E> = Result<Async<T>, E>;

enum Async<T> {
    /// Work completed with a result of type `T`.
    Done(T),

    /// Work was blocked, and the task is set to be woken when ready
    /// to continue.
    WillWake,
}
```

Just calling `get` once does *not* guarantee that a final value will be
produced, and if the future is blocked waiting on some other event to occur, it
is not guaranteed to make progress until `get` is called again. The first part
of this chapter will focus on exactly *who* calls `get`, and *when*.

What makes futures interesting is that, by abstracting out the very general idea
of "computing something asychronously", we make it possible to combine such
computations in expressive and surprising ways. This also informs their
relationship to tasks: a task is generally made up of a number of smaller
futures that have been stitched together.
