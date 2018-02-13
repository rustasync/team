# The task model

To effectively write async code in Rust, you need to have a good grasp on its
foundation: the task model. Fortunately, that model is made up of only a few
simple pieces.

This chapter walks through a high level introduction of the task
concept, then illustrates how the system fits together by building a working
task executor and an event loop, and plugging them together.
