# Net WG Lead Meeting #1
**Maximum duration**: 60 minutes
**Date picker**: https://doodle.com/poll/xttrxdp3f8xukfad

# Objective

This will be the first meeting Net WG meeting since we kicked off the different sub-working groups. Let’s figure out ways we can keep momentum going, and effectively communicate with the wider Rust community.

# Agenda

*Feel free to add more items that you think are important.*


1. General Introduction - 5m (Yosh)
  1. Agree someone to take notes
  2. Check if any agenda items are missing
2. WG Status - 25m
  1. Web - 5m
    1. Progress
    2. Goals
    3. Issues
  2. Async - 5m
    1. Progress
    2. Goals
    3. Issues
  3. Embedded - 5m
    1. Progress
    2. Goals
    3. Issues
3. Blog - 10m
  1. Content planning?
4. Newsletter ([link](https://github.com/rust-lang-nursery/wg-net/issues/51)) - 10m
  1. Agree on date
  2. Content planning?
5. Schedule next meeting - 5m
  1. Have a doodle poll ready
# Minutes

Attendees: yoshuawuyts, lkurusa, MajorBreakfast, Nemo157, cramertj

- Brief overview from each WG (goals, progress and blockers; to have a shared sense of what’s going on)
  - WG-Web
    - Two goals since kickoff:
      - to build out a learning web framework (tide)
      - to investigate the web ecosystem and help out where pieces are missing
    - aturon primarily responsible for the former, yoshuawuyts for the latter
    - Blog posts for tide in progress, placeholder repo at https://github.com/rust-net-web/tide
    - Two issues for ecosystem investigation:
      - https://github.com/rust-lang-nursery/wg-net/issues/44, a call for example web projects to help inform what folks are running into
      - https://github.com/rust-lang-nursery/wg-net/issues/54, to help document missing parts of tokio
    - Inaugural WG-Web meeting next Thursday
  - WG-Async
    - New methodology for testing, the `futures-test` crate. Contains various helpers to make writing tests for 0.3 futures easier. Will be shipped as part of the upcoming alpha 4 of the `futures` crate.
    - Disruption with the sudden changes to the `std` `Pin` APIs. Came as a surprise, will also be fixed in the alpha 4 release.
    - Work on the compatibility layer, blog post is ready. `tokio-async-await` was released by the tokio team, good opportunity for some collaboration; maybe move all the compatibility stuff to this crate.
    - cramertj has started discussion about a `FusedFuture` trait that should make it possible to check whether a future has already completed and produced its value ([https://github.com/rust-lang-nursery/futures-rs/issues/1219](https://github.com/rust-lang-nursery/futures-rs/issues/1219)). PR ready.
      - Easy to support with `async` produced futures as they already track this state for memory safety,
      - Difficult to decide on the best implementation strategy, discuss on the issue.
    - boats has been reworking the pin types, but has hit some stumbling blocks in method resolution, so that’s stuck at the moment ([https://github.com/rust-lang/rust/issues/53843](https://github.com/rust-lang/rust/issues/53843)).
    - cramertj has been working on a PR for a new “scoped tasks” feature, similar to crossbeam’s, that would let you spawn futures that could reference your local stack.
    - cramertj has also been giving some thought on how to solve the issue of wanting to return early from a function that returns an async block; finding it quite common in fuchsia so been exploring options there
    - discussion on the wg-net repo about the async programming book
  WG-embedded
    - nemo mostly been working on https://github.com/nemo157/embrio-rs - a `no_std` compatible cross-target executor + futures targeted Hardware Abstraction Layer (HAL) for interacting with micro-controller peripherals.
    - nemo major blocker is lack of async/await in no-std environments.
    - nemo async/await is also needed for traits in HAL because it’s basically a big collection of traits.
    - nemo blog post + docs are in the works.
    - nemo been meaning to have more discussion about standardizing async IO traits (https://github.com/rust-lang-nursery/wg-net/issues/53) + traits for datagram protocol like UDP and bring up ideas around removing the `poll_*` layer and having traits that return futures directly.
    - lkurusa got hold of a bunch 16 ARM controllers - hoping to use them to pinpoint problems with rust.
    - lkurusa released cgroups crate — also been attending the embedded WG meetings.

Blog

  aturon emphasised that we do have a blog, and we should probably make more use of it
  discussion on possible blog posts/plan for content
    MajorBreakfast: what about encouraging framework authors to present their framework in a blog post, we could come up with some kind of limited length format that could be followed
      MB: it would also mean that the authors could influence directly what is written, instead of someone else describing their work it’d be the authors themselves
      lkurusa: could we agree on a set of questions/bulletpoints beforehand
      MB: create an issue for it and agree on a semi-flexible style for the posts
      MB: Limit length so that we don’t get tutorials, but instead a glance at what the project’s are about
      yoshuawuyts will open issue, MB is lacking in spare time to followup ATM
    yoshuawuyts: aturon mentioned wanting to blog about tide a bunch, but not sure where the posts are intended to go

Newsletter

  https://github.com/rust-lang-nursery/wg-net/issues/51
  similar to the embedded WG, regular newsletter with what’s been happening
  suggestions to post to internals and blog (for an easily accessible archive)
  target date for first issue: 2018-09-12
