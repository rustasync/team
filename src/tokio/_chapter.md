# Tokio: async network I/O

The `tokio` crate complements the `futures` crate by providing a low-level,
cross-platform way to do asynchronous network I/O. The crate's API is modeled
after `std::net` and provides async versions of the same core functionality,
with strong cross-platform support.

This chapter covers both the primary `tokio` networking APIs as well as some
important tools in the `futures` crate for doing async I/O in general. It closes
by building a proxy server using `tokio` directly that aims for low overhead by
minimizing the number of in-flight buffers needed at any time.
