# Futures

Up until now, we've been working with the task model and I/O events in a
"direct" way, writing code that manually juggles `Async` values and
`WakeHandle`s. In this chapter, we'll see how this kind of code can be fit into
a general abstraction, *futures*, that provides a number of tools for working at
a higher level.
