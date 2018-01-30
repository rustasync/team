# A toy event loop

Asynchronous programming is most often used for I/O, but there are many other
sources of events. In this section, we'll build a tiny *event loop* that allows
you to register tasks to be woken up at a specific time.

To do this, we'll spin up a dedicated timer event thread, whose sole job is to
wake up tasks at appropriate times. Consumers of the timer just send this thread
a message saying when they'd like to be woken, and how to wake them:

```rust,no_run
use std::collections::BTreeMap;
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::{Duration, Instant};

/// A handle to a timer, used for registering wakeups
#[derive(Clone)]
struct ToyTimer {
    tx: mpsc::Sender<Registration>,
}

/// A wakeup request
struct Registration {
    at: Instant,
    wake: Arc<Wake>,
}

/// State for the worker thread that processes timer events
struct Worker {
    rx: mpsc::Receiver<Registration>,
    active: BTreeMap<Instant, Arc<Wake>>
}

impl ToyTimer {
    fn new() -> ToyTimer {
        let (tx, rx) = mpsc::channel();
        let worker = Worker { rx, active: BTreeMap::new() };
        thread::spawn(|| worker.work());
        ToyTimer { tx }
    }

    // Register a new wakeup with this timer
    fn register(&self, at: Instant, wake: Arc<Wake>) {
        self.tx.send(Registration { at, wake }).unwrap();
    }
}

```

The event loop lives in the `Worker::work` method. The basic approach is
extremely simple: we keep a `BTreeMap` with the currently-registered wakeups,
and use a channel to make new registrations. The one bit of cleverness: if it's
not yet time to fire off any wakeups, but we do have some scheduled, we can use
`recv_timeout` on the channel to wait for *either* a new registration to come
in, *or* for it to be time to fire the first existing one off:

```rust,no_run
impl Worker {
    fn enroll(&mut self, item: Registration) {
        if self.active.insert(item.at, item.wake).is_some() {
            // this simple setup doesn't support multiple registrations for
            // the same instant; we'll revisit that in the next section.
            panic!("Attempted to add to registrations for the same instant")
        }
    }

    fn fire(&mut self, key: Instant) {
        self.active.remove(&key).unwrap().wake();
    }

    fn work(mut self) {
        loop {
            if let Some(first) = self.active.keys().next().cloned() {
                let now = Instant::now();
                if first <= now {
                    self.fire(first);
                } else {
                    // we're not ready to fire off `first` yet, so wait until we are
                    // (or until we get a new registration, which might be for an
                    // earlier time).
                    if let Ok(new_registration) = self.rx.recv_timeout(first - now) {
                        self.enroll(new_registration);
                    }
                }
            } else {
                // no existing registrations, so unconditionally block until
                // we receive one.
                let new_registration = self.rx.recv().unwrap();
                self.enroll(new_registration)
            }
        }
    }
}
```

That's it! This kind of "event loop" pattern, where a dedicated thread is
continually processing events and registrations (or else blocking until they are
available) is foundational to async programming. Fortunately, to *do* async
programming in Rust, you can use off-the-shelf event loops for events of
interest, as we'll see in the next chapter.

But before we go there, let's plug together the pieces we've built into a tiny,
working app.
