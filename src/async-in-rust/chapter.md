# Async in Rust: what you need to know

There are two things that make asynchronous programming attractive.

First, it allows you to **do more with less**. You can use a single OS-level
thread to field any number of simultaneous interactions; a single-threaded
async server can scale to handle millions of connections.

Now, some operating systems make it possible to use a large number of OS threads
*relatively* cheaply. But there is still overhead. And this leads to the second
benefit of async programming: **by making "tasks" essentially free, it enables
highly expressive programming patterns** that would be impractical in a
synchronous setting.

In other words, the efficiency gains are so stark that they unlock a powerful
new style of programming.

So what's the catch?

Threads are treated in a first class way at the OS level, but if you want to
juggle different activities within the same thread, it's all on
you. Fortunately, Rust is expressive enough that we can build shared, zero-cost
abstractions that make "task-level" programming a first-class concept as well.

That said, there remain some important differences between sync and async
programming in Rust—especially on stable Rust. The purpose of this book is, in
part, to help guide you through these differences, teaching you a set of
patterns for effective async programming.

Finally, it's worth remembering that traditional *synchronous* programming
remains quite effective, outside of the highest-scale servers. In particular,
Rust's advantages around memory footprint and predictability mean that you can
get much farther with synchronous services than in many other languages. As with
any other architectural choice, it's important to consider whether your
application would be better served by using the simpler synchronous model.

## Where async Rust is today, and where it's headed

**Rust has a strong foundation for async programming** with the [`futures`] and
[`tokio`] crates, which cover the core abstractions for async, and primitives
for async I/O, respectively. On top of these crates there's an ecosystem for
interacting with various protocols and services, including HTTP, SSL, DNS,
WebSockets, and more. This book covers much of this ecosystem, recommending
production-quality libraries in each space.

[`futures`]: https://github.com/alexcrichton/futures-rs/
[`tokio`]: https://github.com/tokio-rs/tokio/

In addition, **there is ongoing work to improve the ergonomics** via
`async`/`await` notation. This work is currently only available on nightly Rust,
and is expected to provide more flexible borrowing support in the
future. Nevertheless, if you're willing to work with nightly today, the library
itself is stable and helpful. This book also covers its usage.

As we'll see shortly, there are easy ways to "bridge" between async and sync
code. In the long run, as async becomes a more first-class part of the language
itself, the expectation is that core libraries will be written in an async
fashion, but be easily consumed in either style. We'll see examples of that
later in the book.

## Who this book is for

This book aims to be a comprehensive, up-to-date guide on the async story in
Rust, appropriate for beginners and old hands alike:

- The early chapters provide a gentle introduction to async programming in
general, and to Rust's particular take on it.

- The middle chapters provide more powerful tools, best practices, and larger
  examples to help you work with async in a more serious way.

- The later chapters cover a cross section of the broader async ecosystem and
  the most advanced features of the core libraries, and more extensive case
  studies.

*Let's dive in!*
