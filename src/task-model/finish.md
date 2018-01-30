# Putting it all together

At this point, we've built a simple executor for running many tasks on a single
thread, and a simple event loop for dispatching timer events, again from a
single thread. Now let's plug them together to build an app that can support an
arbitrary number of tasks periodically "dinging", using only two OS threads.

To do this, we'll create a task called `Periodic`:

```rust,no_run
struct Periodic {
    // a name for this task
    id: u64,

    // how often to "ding"
    period: Duration,

    // when the next "ding" is scheduled
    next: Instant,

    // a handle back to the timer event loop
    timer: Timer,
}

impl Periodic {
    fn new(id: u64, period: Duration, timer: Timer) -> Periodic {
        Periodic {
            id, period, timer, next: Instant::now() + period
        }
    }
}
```

The `period` field says how often the task should print a "ding" message. The
implementation is very straightforward; note that the task is intended to run
forever, continuously printing a message after each `period` has elapsed:

```rust
impl Task for Periodic {
    fn complete(&mut self, wake: &WakeHandle) -> Async<()> {
        // are we ready to ding yet?
        let now = Instant::now();
        if now >= self.next {
            self.next = now + self.period;
            println!("Task {} - ding", self.id);
        }

        // make sure we're registered to wake up at the next expected `ding`
        self.timer.register(self.next, wake);
        Async::WillWake
    }
}
```

And now, we hook it all together:

```rust,no_run
fn main() {
    let timer = ToyTimer::new();
    let exec = ToyExec::new();

    for i in 1..10 {
        exec.spawn(Periodic::new(i, Duration::from_millis(i * 500), timer.clone()));
    }

    exec.run()
}
```

The program generates output like:

```
Task 1 - ding
Task 2 - ding
Task 1 - ding
Task 3 - ding
Task 1 - ding
Task 4 - ding
Task 2 - ding
Task 1 - ding
Task 5 - ding
Task 1 - ding
Task 6 - ding
Task 2 - ding
Task 3 - ding
Task 1 - ding
Task 7 - ding
Task 1 - ding
...
```

Stepping back, what we've done here is a bit magical: the implementation of
`Task` for `Periodic` is written in a pretty straightforward way that says how a
*single task* should behave. But then we can interleave any number of such
tasks, using only two OS threads total! That's the power of asynchrony.

## Exercise: multi-enrollment

The timer event loop contains an unfortunate explicit panic: "Attempted to add
to registrations for the same instant".

- Is it possible to encounter this panic in the above program?
- What would happen if we simply removed the panic?
- How can the code be improved to avoid this issue altogether?

## Exercise: winding down

Both the `Periodic` task and the `SingleThreadedExec` are designed to run
without ever stopping.

- Modify `Periodic` so that each instance is set to ding only a fixed number of
  times, and then the task is shut down.
- Modify `SingleThreadedExec` to stop running when there are no more tasks.
- Test your solution!
