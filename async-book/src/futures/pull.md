# Push and pull: futures and tasks



This description is hopefully not surprising, given the previous few
chapters. However, it's important to realize that this setup is *drastically*
different from futures (aka promises) in other languages:

- Rust futures follow a *pull* model, where, once a future is blocked by an
  event, it must be retried through a call to `get` (i.e., by repeatedly trying
  to "pull" a value out).

- Other futures libraries follow a *push* model, where the completion of one
  event *automatically* triggers a cascade of follow-up work, *pushing* the
  computation forward. That follow-up work is usually given by registering
  callbacks with a future.



## Tasks: where push meets pull
