---
layout: post
title: "Async in Rust, circa 2018"
date: 2018-12-13
author: Aaron Turon
---

Rust 2018 has shipped, and we're closing in on the end of the year. While we
didn't manage to ship async/await as part of the edition itself, the
community has made quite a lot of progress toward that goal. This post
summarizes the state of play, and announces the publication of several crates
intended to facilitate use of async/await on the nightly ecosystem.

# Why async/await

Before delving into the current status, it's worth taking a moment to recap
the core motivations for async/await, and its special importance for Rust.

Async/await notation is a way of making asynchronous programming more
closely resemble synchronous programming. To see how this works, consider
`Read::read` in `std::io`:

```rust
fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error>
```

This synchronous method *blocks the current thread* until data has been read
into `buf`, then says how many bytes were read. We can build on this method
to implement `read_exact`, a method that *continues* reading until the buffer
is filled:

```rust
fn read_exact<T: Read>(input: &mut T, buf: &mut [u8]) -> Result<(), std::io::Error> {
    let mut cursor = 0;
    while cursor < buf.len() {
        cursor += input.read(&mut buf[cursor..])?;
    }
}
```

In the asynchronous world, we want to perform similar operations, but rather
than blocking the current thread we want to leave it free to do other work
while the I/O operations complete asychronously. But actually programming
*directly* in that way is incredibly difficult. What we want is to program
*as if* I/O operations will block the current thread, but have the
compiler transform this code into more efficient asynchronous execution.

In short, our goal is to be able to write the following:

```rust
async fn read_exact<T: AsyncRead>(input: &mut T, buf: &mut [u8]) -> Result<(), std::io::Error> {
    let mut cursor = 0;
    while cursor < buf.len() {
        cursor += await!(input.read(&mut buf[cursor..]))?;
    }
}
```

Comparing the two snippets, there are three changes here:

- We write `async` before `fn`, to signal that the function should be
asynchronously executed on its parent thread. Async functions return
their result within a `Future`, representing a value that must be asynchronously
computed.

- We use the [`AsyncRead` trait](https://docs.rs/futures-preview/0.3.0-alpha.10/futures/io/trait.AsyncRead.html) (from `futures::io`), rather than the
`Read` trait. This makes an asynchronous version of the `read` method
available (currently provided via the [`AsyncReadExt` extension trait](https://docs.rs/futures-preview/0.3.0-alpha.10/futures/io/trait.AsyncReadExt.html)).

- We enclose the call to `read` with `await!`, signaling that we want to
*simulate* blocking on the operation to complete.

And that's all.

This approach to asynchrony has proven itself in many other languages already.
But there's an extra element in the Rust
version: borrowing. For `read_exact`, we are able to hold the borrows
of `input` and `buf` while we use `await!` (which may actually clear
the stack and run completely unrelated code). The validity of the
borrowing is still checked, and it works largely similarly to borrowing
within synchronous code. (The main difference: the way elision works in
`fn` signatures.)

If you've programmed with futures in Rust before, you'll know this is a
game changer: manual futures code generally must be restricted to `'static`
data, which (in addition to its verbosity) takes it far away from
idiomatic Rust, forcing you to program with `Arc` and `Mutex` far more
frequently than usual.

Below, we'll check in on the status of various aspects of the transition to
this new world.

# The book

Part of the work around async/await this year has been writing a new
book, covering the syntax, the underlying `Future` API, and ultimately
various programming patterns that emerge. @cramertj has written [an early
draft][apr] (repo [here](https://github.com/rust-lang/async-book), 
which is already useful for understanding these concepts.

[apr]: https://rust-lang.github.io/async-book/

# The syntax

The async/await syntax itself has had an implementation on nightly for
several months now, and is being used at a fairly large scale in Google's
Fuchsia project. You can find more detail about that usage [here][fuchsia].

[fuchsia]: https://github.com/rust-lang/rfcs/pull/2592#issuecomment-438894347

While there are a few remaining limitations in the implementation of the syntax,
the main issue that remains to be resolved prior to any stabilization is the
await side of the syntax. @withoutboats recently wrote a [blog post][boats-syntax]
describing the issues there in detail.

Today, `async` can be used in code blocks, for free functions (`async fn`),
and for inherent methods. Ultimately it will be usable in trait method signatures
as well, but this is effectively blocked by [existential types], another
feature on track to stabilization relatively soon. In the meantime, there are
several forward-compatible ways to continue using `async` blocks within trait
implementations, most simply by placing the `async` block into a `Box` that
will be removable later, once existential types are stable.

[boats-syntax]: https://boats.gitlab.io/blog/post/await-syntax/
[existential types]: https://github.com/rust-lang/rfcs/pull/2071

# The Supporting APIs

Like many other language features, async/await also requires some support in the
standard library: shipping the `Future` trait (and associated machinery) in `std`.
That work has been a major thrust this year, and is nearing completion.

## Core futures APIs

There's currently an [open RFC][futures-rfc] proposing stabilization of the futures
APIs, and includes a [fairly detailed writeup][futures-history] of the history of
those APIs. The pull request contains a checklist of current blockers; the most
signifcant one at the moment is finalizing the `Waker` APIs.

[futures-history]: https://github.com/aturon/rfcs/blob/future/text/0000-futures.md#historical-context
[futures-rfc]: https://github.com/rust-lang/rfcs/pull/2592

## The `Pin` API

One of the underlying mechanisms supporting the futures API is the `Pin` type,
which is also how we enable borrowing in `async` blocks. This API, too, has seen
significant iteration over the course of the year. @withoutboats's [blog post][pin-final-api]
from a few months ago covers the final design, which has also been
[proposed for stabilization][pin-stabilization]. The only remaining sticking point
is around type and trait naming.

[pin-final-api]: https://boats.gitlab.io/blog/post/rethinking-pin/
[pin-stabilization]: https://github.com/rust-lang/rust/issues/55766

## Compatibility with futures 0.1

The design of the futures API had to change in breaking ways in order to support
async/await. However, there's a large existing ecosystem of code that uses the earlier
futures 0.1 API. Luckily, we're able to provide a rather ergonomic compatibility layer
that makes it possible to move between the two APIs easily, and hence support
incremental migration. A [recent blog post from @jsdw][futures-compat] does an excellent
job of laying out how this compatibility story works.

[futures-compat]: https://jsdw.me/posts/rust-asyncawait-preview/

# Some new crates

In addition to the compatibility layer the Networking Working Group has also put effort
into building crates *directly* using the new futures API, in order to more fully
vet that API, to provide a smoother experience for others wanting to build code using
async/await, and to lay out a clear vision of what the new ecosystem might look like.

- **[Romio][Romio-repo]**, a minimal fork of Tokio based directly on the new futures API. While
Tokio proper aims to provide a comprehensive and opinionated story for the lowest-levels
of async networking code, Romio covers just the essentials: an API surface very similar
to `std::net`, but supporting async/await directly. The crate includes a good bit of
documentation and examples, and @withoutboats has written a [blog post][romio-blog]
detailing lessons learned through this port.

[romio-repo]: https://github.com/withoutboats/romio
[romio-blog]: https://boats.gitlab.io/blog/post/romio/

- **[http-service]**, a tiny crate building on [bytes], [http], and the new futures API
to provide a common interface for http-based services using the new futures API. This crate
is partly based on the ongoing work on [Tide], where the goal is to seed the ecosystem
with numerous small, useful crates of this kind that many different frameworks and libraries
can build on. As such, the API is an extraction of the one initially used internally in Tide.

[bytes]: https://docs.rs/bytes/
[http]: https://docs.rs/http
[http-service]: https://docs.rs/http-service
[Tide]: https://github.com/rust-net-web/tide/

- **Tyger**, a small crate that builds on top of [Hyper] to provide a direct http-service
interface (and thus usable with async/await directly, without shims).
Ultimately Tyger is likely to grow some other higher-level amenities, to complement
Hyper's relatively low-level focus. As with http-service, the crate is an early extraction
from the Tide work and is intended to provide a small, community-driven building block that
can be used by many other crates. It will be published some time in the next few weeks.

[Hyper]: https://github.com/hyperium/hyper/

# The road ahead

We've come a long way toward async/await in 2018! With the futures and pin APIs on the cusp of stabilization, we should very soon be in a position to propose stabilization of async/await
proper, hopefully shipping in the first half of 2019. It will be crucial to continue to build
out the library ecosystem around these APIs in the coming year. If you want to get involved
in any of this exciting work, please drop by the "WG-Net" channels on the [Rust Discord]!

[Rust Discord]: https://discord.gg/rust-lang
