# `async`/`await!` Primer

`async`/`await!` is Rust's built-in tool for writing asynchronous functions
that look like synchronous code. `async` transforms a block of code into a
state machine that implements a trait called `Future`. This block can be
run to completion using an futures executor:

```rust
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let future = hello_world(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
}
```

Inside an `async fn`, you can use `await!` to wait for the completion of
another type that implements the `Future` trait, such as the output of
another `async fn`:

```rust
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

async fn async_main() {
    await!(hello_world());
    await!(hello_world());
}

fn main() {
    // Run the future returned by `async_main`, causing "hello, world!"
    // to be printed twice.
    block_on(async_main());
}
```

Unlike `block_on`, `await!` doesn't block the current thread, but instead
asynchronously waits for the future to complete, allowing other tasks to
run if the future is currently unable to make progress. For example,
imagine that we have three `async fn`: `learn_song`, `sing_song`, and
`dance`:

```rust
async fn learn_song() -> Song { ... }
async fn sing_song(song: Song) { ... }
async fn dance() { ... }
```

One way to do learn, sing, and dance would be to block on each of these
individually:

```rust
fn main() {
  let song = block_on(learn_song());
  block_on(sing_song(song));
  block_on(dance);
}
```

However, we're not giving the best performance possible this way-- we're
only ever doing one thing at once! Clearly we have to learn the song before
we can sing it, but it's possible to dance at the same time as learning and
singing the song. To do this, we can create two separate `async fn` which
can be run concurrently:

```rust
async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `await!` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = await!(learn_song());
    await!(sing_song(song));
}

async fn async_main() {
    let f1 = learn_and_sing(); 
    let f2 = dance();

    // `join!` is like `await!` but can wait for multiple futures concurrently
    join!(f1, f2) 
}

fn main() {
    block_on(async_main());
}
```

In this example, learning the song must happen before singing the song, but
both learning and singing can happen at the same time as dancing. If we used
`block_on(learn_song())` rather than `await!(learn_song())` in `learn_and_sing`,
the execution would block until learning the song completed, making it
impossible to dance at the same time. By `await!`ing learning the song, we
allow other tasks to run concurrently.

Now that you've learned the basics of `async`/`await!`, let's try out an
example.
