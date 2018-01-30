# Tasks

The `futures` crate does not define a `Task` trait directly, but instead defines
the more general concept of *futures*, something we'll be diving into in much
more detail soon. For the moment, though, let's look at the core definition for future
(ignoring its many defaulted methods):

```rust,no_run
/// An asynchronous computation that completes with a value or an error.
trait Future {
    type Item;
    type Error;

    /// Attempt to complete the future, yielding `Ok(Async::WillWake)`
    /// if the future is blocked waiting for some other event to occur.
    fn get(&mut self) -> AsyncResult<Self::Item, Self::Error>;
}

type AsyncResult<T, E> = Result<Async<T>, E>;
```

Futures are much like tasks, except that they return a result (which allows them
to be composed). In other words, you can think of a *task* as any type that
implements `Future<Item = (), Error = !>`.

There is another difference, however: the lack of a `WakeHandle` argument. In
practice, this argument would almost always be passed down, unchanged, from the
executor all the way to the point of enqueueing the handle in an appropriate
place. Thus with the `futures` crate, executors provide a `WakeHandle` in
thread-local storage for convenience. You can get it using the `current_wake`
function in `futures::task`:

```rust,no_run
/// When called within a task being executed, returns the wakeup handle for
/// that task. Panics if called outside of task execution.
fn current_wake() -> WakeHandle;
```

## Explicitly relating `Future` and `ToyTask`

It can be helpful to see *precisely* how `Future` and `ToyTask` relate. To do
this, we'll introduce a wrapper type for converting a `ToyTask` to a `Future`:

```rust,no_run
use futures::task;
use futures::{Future, AsyncResult};

struct ToyTaskToFuture<T>(T);

impl<T: ToyTask> Future for ToyTaskToFuture<T> {
    type Item = ();
    type Error = !;

    fn get(&mut self) -> AsyncResult<(), !> {
        Ok(self.0.complete(task::current_wake()))
    }
}
```
