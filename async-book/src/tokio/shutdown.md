# Closing down a connection

In the synchronous world, you often don't have to worry too much about
flushing. You can freely write to a buffered I/O object and it will periodically
flush as you do so. And, most importantly, if at any point you *drop* the
object, the remaining content of the buffer is automatically flushed. Worst
case, there is an error trying to perform this flush, which gets swallowed; but
generally there's not much you could've done about such an error anyway.

In the async world, flushing is more critical, for a simple reason: **the model
does not afford us the ability to automatically flush on drop**. In particular,
*forcing* a flush means potentially blocking the calling thread until that flush
completes. Since async I/O objects are generally floating around within executor
tasks, this is a non-starter; blocking an executor can bring the whole system to
a halt.

Thus, it's critical to ensure all data is flushed before dropping an async I/O
object, using `AsyncWrite::flush`.
