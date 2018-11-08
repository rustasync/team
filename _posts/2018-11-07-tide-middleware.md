---
layout: post
title: "Middleware in Tide"
date: 2018-11-07
author: Aaron Turon
---

After the [positive response][internals-old] to the routing and extraction proposal in Tide, I'm pleased to say that **[an initial implementation is available on GitHub!][repo]** As a testament to the strong foundation that Rust's ecosystem provides, the basic framework implementation took only about 1,000 lines of code.

The repository is populated with numerous issues, including quite a few marked as [good first issue]. At this point, enough of the skeleton of Tide is in place to turn continued development into a fully collaborative effort. This is a great time to get involved and shape what will eventually become the initial 0.1 release!

[internals-old]: https://internals.rust-lang.org/t/routing-and-extraction-in-tide-a-first-sketch/8587
[repo]: https://github.com/rust-net-web/tide
[good first issue]: https://github.com/rust-net-web/tide/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22

As part of this initial implementation, I've also provided a simple middleware design, as well as a new idea of *computed values*. The rest of this post will cover these two additions.

# Middleware

The proposed approach to middleware is pretty simple, and is drawn *directly* from actix-web:

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

In this design, middleware can:

- alter the request data before proceeding to the next middleware or the endpoint
- perform side-effects before or after endpoint processing
- abort further processing by directly producing a response
- transform the response on the way out

This functionality is provided via *asynchronous* functions. The use of `FutureObj` here reflects the current way to express boxed futures, which is expected to change to `Box<Future>` in the near future. While boxing the futures has some performance cost, it's expected that the cost is extremely minimal, and boxing allows us to avoid much more complicated type tracking (and associated lengthy compile times). It's a technique that has been proven out in frameworks like actix-web.

Note that the `Request` type is just the one from the `http` crate, which contains an `extensions` typemap that middleware can use to communicate arbitrary information to the endpoint or between its methods.

If you're familiar with actix-web, you might notice that there's no method corresponding to `finish`, i.e. one that runs after the response is transmitted. There's an [issue open](https://github.com/rust-net-web/tide/issues/11) explaining why; feel free to take a stab at it!

Middleware is added to an app in a similarly simple way:

```rust
impl<Data> App<Data> {
    pub fn middleware(&mut self, middleware: impl Middleware<Data> + 'static) -> &mut Self { ... }
}
```

Ultimately we'll want to support more fine-grained application of middleware, e.g. applying it only to a particular set of subroutes. There's an [issue for that](https://github.com/rust-net-web/tide/issues/4) as well.

# Computed values

While the above middleware story is simple and flexible, it's often overkill. One new idea in Tide is *computed values*, which are values that can be produced on demand from a request. For example, you often want to parse the query portion of a URL into components. This parsing might be needed in the endpoint, or in various middleware. Rather than writing middleware to perform the parsing and stash it in `extensions` (which requires carefully ordering the middleware, and mucking around with request state), we can use a computed value to lazily perform the parsing as soon as it's request. The computed value will then *cache* the parsed result for the request, and any further uses will get the cached value.

Concretely, we have a `Compute` trait for computed values, with the required `compute_fresh` function saying how to compute the value from scratch, and the provided `compute` method handling caching automatically:

```rust
/// A value that can be computed on-demand from a request.
trait Compute: 'static + Sync + Send + Clone + Sized {
    /// Compute the value directly from the given request.
    fn compute_fresh(req: &mut Request) -> Self;

    /// Compute the value, or return a copy if it has already been computed for this request.
    fn compute(req: &mut Request) -> Self { ... }
}
```

There's also a `Computed` extractor, which endpoints can use to request computed data:

```rust
struct Computed<T: Compute>(T);
```

So, going back to our earlier example, we might define:

```rust
struct ParsedQuery { .. }

impl Compute for ParsedQuery { .. }

async fn my_endpoint(query: Computed<ParsedQuery>) { ... }
```

and we're done! Similarly, middleware can call `ParsedQuery::compute` method directly on the request to get the value, potentially from cache.

The hope is that a lot of functionality that might traditionally end up in middleware can instead be expressed as computed values, which is both more ergonomic, *and* provides far more guarantees for reasoning about your code.

# What's next

At this point, further work on Tide will happen in its repository and issue tracker, and on the #wg-net-web channel on Discord. There are a ton of open issues, including some design questions, and I encourage you to open additional issues with your own questions and ideas. As development progresses, we'll regularly post additional blog posts laying out further design sketches and milestones.
