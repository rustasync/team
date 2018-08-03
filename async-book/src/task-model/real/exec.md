# Executors

First off, in the `futures` crate, executors are objects that can spawn a
`Future<Item = (), Error = !>` as a new task. There are two key executors to be
familiar with.

## The `ThreadPool` executor

The simplest executor is `futures::executor::ThreadPool`, which schedules tasks
onto a fixed pool of OS threads. Splitting the tasks across multiple OS threads
means that even if a particular `tick` invocation takes a long time, other tasks
can continue to make progress on other threads.

Setup and usage is very straightforward:

```rust,no_run
// Set up the thread pool, which spins up worker threads behind the scenes.
let exec = ThreadPool::new();

// Spawn tasks onto the thread pool.
exec.spawn(my_task);
exec.spawn(other_task);
```

We'll see later on a variety of ways to communicate with spawned tasks.

Note that, because the task will be executed on arbitrary threads, it is
required to be `Send` and `'static`.

## The `CurrentThread` executor

The `futures` crate also provides a single-threaded executor called
`CurrentThread`, similar in spirit to the one that we built. The key difference
from the `ThreadPool` executor is that `CurrentThread` can execute non-`Send`
and non-`'static` tasks. This is possible because the executor is run via an
explicit call from the current thread:

```rust,no_run
// start up the CurrentThread executor, which by default runs until all spawned
// tasks are complete:
CurrentThread::run(|_| {
    CurrentThread::spawn(my_task);
    CurrentThread::spawn(other_task);
})
```

The tradeoffs between `ThreadPool` and `CurrentThread` are explained in more
detail [later in the book](async-in-practice/concurrency.html).

## Spurious wakeups

In general, executors guarantee that they will call `get` any time a task is
awoken. However, they may *also* call `get` at arbitrary other times. Thus,
tasks cannot assume that a call to `get` means progress is possible; they should
always re-attempt the operation that previously blocked them, and be prepared to
wait again.

## Exercises

- Rewrite the example from the previous section to use the `ThreadPool` executor.
- Rewrite the example from the previous section to use the `CurrentThread` executor.
- What are the tradeoffs between using these two executors for the timer example?
