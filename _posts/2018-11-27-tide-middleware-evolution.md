---
layout: post
title: "Tide's evolving middleware approach"
date: 2018-11-27
author: Aaron Turon
---

Since the [last post] on Tide, there have been a number of excellent contributions
from a bunch of new contributors! In this post, I want to talk about the work
that [@tirr-c](http://github.com/tirr-c) has done to substantially improve the
middleware story.

*As always: if you find these topics interesting, we'd **love** to have your
help building Tide!* There's an active pipeline of open issues, including ones
marked as good [starter issues], and there's ongoing discussion for what we'd
like to see in a [0.1 release]. Getting involved now is the best opportunity
to help shape the direction of this community-built framework!

[last post]: https://rust-lang-nursery.github.io/wg-net/2018/11/07/tide-middleware.html
[starter issues]: https://github.com/rust-net-web/tide/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22
[0.1 release]: https://github.com/rust-net-web/tide/issues/60

# Improving the `Middleware` trait

In the [last post], we proposed before/after-style middleware, borrowing from the [actix-web] design:

[actix-web]: https://github.com/actix/actix-web

```rust
pub trait Middleware<Data>: Send + Sync {
    /// Asynchronously transform the incoming request, or abort further handling by immediately
    /// returning a response.
    fn request(
        &self,
        data: &mut Data,
        req: Request,
        params: &RouteMatch<'_>,
    ) -> FutureObj<'static, Result<Request, Response>>;

    /// Asynchronously transform the outgoing response.
    fn response(
        &self,
        data: &mut Data,
        head: &Head,
        resp: Response,
    ) -> FutureObj<'static, Response>;
}
```

Since then, however, @tirr-c recognized that there were substantial gains to be had by instead
using an "around" design for the core trait:

```rust
/// Middleware that wraps around remaining middleware chain.
pub trait Middleware<Data>: Send + Sync {
    /// Asynchronously handle the request, and return a response.
    fn handle<'a>(&'a self, ctx: RequestContext<'a, Data>) -> FutureObj<'a, Response>;
}
```

This new interface is built using a convenient type, `RequestContext`, that encapsulates
all of the information middleware has at its disposal:

```rust
pub struct RequestContext<'a, Data> {
    pub app_data: Data,
    pub req: Request,
    pub params: RouteMatch<'a>,
    // plus additional, private fields
}

impl<'a, Data: Clone + Send> RequestContext<'a, Data> {
    /// Consume this context, and run remaining middleware chain to completion.
    pub fn next(mut self) -> FutureObj<'a, Response>;
}
```

In this approach, each middleware is given complete control over the remaining request-handling
pipeline. Keep in mind, however, that middleware and endpoints run *strictly after routing*,
and so the pipeline *must* return a response. (There's an open issue for [internal redirects].)

Notably, it's simple to build before/after-style middleware constructors on top of this interface,
so we don't lose that convenience. But using "around" middleware as the *core* interface has some
key advantages:

- It's much simpler to communicate data between steps that take place before and after the rest
of the pipeline. With the original proposal, you would have to use the `Request::extensions`
typemap to inject information for later extraction, and that information would have to be `'static`.
With around middleware, all you need is a `let` binding, and the binding can contain borrows that
persist until after the rest of the pipeline has executed.

- The original approach forced an allocation (of a `FutureObj`) for every middleware on every request.
In the new interface, a new `FutureObj` only needs to be allocated when the middleware is performing
asynchronous work or steps that occur after the rest of the pipeline.

- The new interfae is arguably simpler and tidier.

Thanks to @tirr-c for working this all out!

[internal redirects]: https://github.com/rust-net-web/tide/issues/82

# Nested routers with customized middleware

In the [last post], middleware could only be applied at the top level, and hence all endpoints would
employ the exact same middleware. However, it can be useful to introduce middleware that applies
only to a subset of routes. Usually, such customization groups routes by their path structure, and
that's the approach we've taken in Tide as well.

To apply middleware to a subset of routes with a common prefix, you can use `nest`:

```rust
let mut app = App::new(your_data);

app.at("/some/prefix").nest(|r| {
    r.middleware(some_middleware);      // applies to everything under `/some/prefix`
    r.at("/").get(prefix_top_endpoint); // matches `/some/prefix`
    r.at("/foo").get(foo_endpoint);     //matches `/some/prefix/foo`
});

// no middleware is applied to this route
app.at("/").get(index_endpoint);

app.serve(address);
```

The `nest` method gives you mutable access to a *subrouter* nested under the prefix you chose:

```rust
impl<'a, Data> Resource<'a, Data> {
    /// "Nest" a subrouter to the path.
    ///
    /// This method will build a fresh `Router` and give a mutable reference to it to the builder
    /// function. Builder can set up a subrouter using the `Router`. All middleware applied inside
    /// the builder will be local to the subrouter and its descendents.
    pub fn nest(self, builder: impl FnOnce(&mut Router<Data>));
}
```

We expect that this same nesting setup will have many other uses over time, including [configuration].

[configuration]: https://github.com/rust-net-web/tide/issues/5

Thanks again go to @tirr-c for working through several iterations of this design!

# Computed values: an exmaple

Finally, we now have a full example of using computed values for cookie parsing,
which you can find [here][cookies]! A very important area of ongoing work will be
to see how much traditional middleware can be expressed as computed values instead.
Doing so makes reasoning much easier (since computed values are less powerful)
and eliminates some common middleware pitfalls (order dependence, amongst others).

We [plan to move this computed value into Tide proper][cookies-issue], and it would
be wonderful to accumulate many other such building blocks; PRs very welcome!

[cookies]: https://github.com/rust-net-web/tide/blob/master/examples/computed_values.rs
[cookies-issue]: https://github.com/rust-net-web/tide/issues/84

# What's next?

It's been an exciting couple of weeks since Tide's initial code went online, with
a growing, enthusiastic contributor base already pushing it forward faster than I'd
dared to hope! Building on this momentum, there's a lot more we want to tackle; here
are some highlights.

## Routing

- [#62]: Provide a "catch-all" routing mechanism, often expressed as `*` in routing syntax,
which will match any path with a given prefix.
- [#82]: Work out a design for "internal redirects", where middleware or endpoints can
abort the current request-handling pipeline in favor of a redirected request.
- [#24]: Design an API for programmatically generatinng URLs based on the routing table.

[#62]: https://github.com/rust-net-web/tide/issues/62
[#82]: https://github.com/rust-net-web/tide/issues/82
[#24]:  https://github.com/rust-net-web/tide/issues/24

## Middleware

- [#73]: Make a "middleware stack" more of a first-class concept, ultimately supporting
debugging and other hooks.
- [#61]: Provide some notion of "always-applied" middleware, which is used even if there
is no matching route.
- [#26]: Build middleware for compression.

[#73]: https://github.com/rust-net-web/tide/issues/73
[#61]: https://github.com/rust-net-web/tide/issues/61
[#26]: https://github.com/rust-net-web/tide/issues/26

## Configuration

- [#5]: Build a configuration system, including the ability to customize extractor behavior
at point in a router.

[#5]: https://github.com/rust-net-web/tide/issues/5

## Additional HTTP methods

- [#51]: Provide built-in support for `OPTIONS`.

[#51]: https://github.com/rust-net-web/tide/issues/51

## Testing

- [#83]: Explore app testing approaches like mocking.

[#83]: https://github.com/rust-net-web/tide/issues/83

## Documentation

- [#77]: Start writing a high-level guide for using Tide.
- [#20]: Build some larger example applications.
- [#19]: Document how endpoint signatures (and hence, extractors) work.

[#77]: https://github.com/rust-net-web/tide/issues/77
[#20]: https://github.com/rust-net-web/tide/issues/20
[#19]: https://github.com/rust-net-web/tide/issues/19

## A 0.1 release

# Thanks!

Finally, a shout out to the 20 people (!) who have already contributed to Tide:

- Aaron Turon
- Bhargav Voleti
- Chris Stinson
- David Tolnay
- Fuyang Liu
- Harikrishnan Menon
- Heiko Seeberger
- Hendrik Sollich
- Joe ST
- Jonas Nicklas
- Pradip Caulagi
- Simon Andersson
- Taylor Cramer
- Theodore Zilist
- Tirr
- Wonwoo Choi
- Yoshua Wuyts
- csmoe
- ibaryshnikov
- lixiaohui