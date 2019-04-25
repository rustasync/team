- Feature Name: N/A
- Start Date: (fill me in with today's date, 2019-03-27)
- RFC PR: [rust-lang/rfcs#0000](https://github.com/rust-lang/rfcs/pull/0000)
- RustAsync Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)

# Summary
[summary]: #summary

The roadmap for the first 6 week for the Rustasync ecosystem work group.

This RFC is intended to provide the general outline and define the goals for the first sprint. The 6 week sprint provides a block time long enough to take a feature to completion, while at the same time being short enough that things do not stagnate.

Since this will be the first sprint for the async ecosystems wg, the goal is to build a good foundation on which future work can proceed.

* Start date: *28 March*
* End date: *2 May*

# Detailed description
[detailed-description]: #detailed-description

# Tide
[tide]: #tide

The project has seen a lot of discussion happen in [issues][issues] as well as the discord channel. The general conversation currently revolves around getting the core framework stabilized in order to build features on top of that. To this end, the currently [open PR by @aturon][context-pr] works to refactor the way data is passed along to endpoint functions. This change is a major change allowing data extraction to be more explicit.

 Once this feature lands, here are some of the issues that work can begin on:

 * [Authentication in Tide][issues-99]
 * [Serving static files][issues-63]
 * [URL generation][issues-24]

There has been some discussion on the [Sprint 1 goals issue][sprint-goals], and a really good overview for the features to work on has been been [outlined by @gruberb][goals-outline]. This outline a great basis to set up the roadmap for Tide. It consists of the following broad goals:

* [ ] Stabilize tide core.
* [ ] Session management
* [ ] Authentication

## Merge Context PR
[merge-context-pr]: #merge-context-pr

Currently [the PR][context-pr] is open, with most of the core changes done. There is ongoing discussion regarding the change, and you can follow with the progress directly on the PR.


### Goals
[stabilize-core-goals]: #stabilize-core-goals

* [ ] Merge PR
* [ ] Resolve design questions.

## Session management
[session-management]: #session-management

Current discussion around session management in tide is centered around the [design issue][issues-9] for the same. @tomhoule has got a working generic session implementation written against the new tide core changes. You can check the [project out here][session-project]. This provides types to define middleware and custom session storage backends which hook directly into the `Context` object provided to the endpoint function.

The current discussion is centered on providing a simple in memory session storage with Cookies as the default in the framework, and provide external crates to hook into external data stores.

This change is currently blocked by the tide core changes.

### Goals

* [ ] Stable API.
* [ ] In memory session backend

## Authentication
[Authentication]: #authentication

Currently rust web frameworks have disparate ways of authentication. One of the goals of the Async ecosystem WG is to provide a common set of crates which can be used to build higher level abstractions. In terms of Tide, there currently isn't an authentication story present. @tomhoule has built cookie based authentication into the session management middleware as a POC. Therefore the goals for this sprint would be to sketch out the design of the authentication API for tide.

### Goals

* [ ] Design authentication API in Tide.

[issues]: https://github.com/rustasync/tide/issues/
[context-pr]: https://github.com/rustasync/tide/pull/156
[issues-9]: https://github.com/rustasync/tide/issues/9
[issues-99]: https://github.com/rustasync/tide/issues/99
[issues-63]: https://github.com/rustasync/tide/issues/63
[issues-24]: https://github.com/rustasync/tide/issues/24

[sprint-goals]: https://github.com/rustasync/team/issues/96
[goals-outline]: https://github.com/rustasync/team/issues/96#issuecomment-471552499
[session-project]: https://github.com/tomhoule/tide-cookie-session

# Websites
[websites]: #websites

There are two primary websites for keeping tack of async await progress and web development progress:

* [Areweasyncyet][areweasyncyet]
* [Arewewebyet][Arewewebyet]

There isn't a lot that has been discussed around these two as they are in maintenance mode and work will mostly be around updating the content as and when appropriate.

[areweasyncyet]: https://areweasyncyet.rs/
[arewewebyet]: http://www.arewewebyet.org/

# Async book
[async-book]: #async-book

The current [Async book][async-book] is a great start at explaining the details of why `async/await` is important, but is light on the details. As mentioned in [this issue][lucio-issue], there are discussions on improving the contents and providing a starting point for people wanting to learn about how the feature works. There are also plans for adding sections for helping maintainers of existing libraries to migrate to std futures.

The book can also provide guides for building different kinds of software using `async/await` as well as provide guides for people wanting to help out with the documentation and migration effort, akin to [Tokio's doc-push][doc-push].


### Goals

Echoing the theme of the sprint, the goals for the book will be to provide a good foundation to build upon.

Some general goals are

* [ ] Outline chapters
* [ ] Move Async book to rustasync org
* [ ] Provide guides for writing chapters.

# Library ecosystem
[library-ecosystem]: #library-ecosystem

There are a lot of existing libraries which use Future 0.1 . As it stands there is no resource to refer to when a library maintainer would want to port their library to work with std futures. [The issue][lucio-issue] mentioned in the previous section goes into more detail, but the general theme is to provide material and help for library maintainers to migrate their libraries to std Futures.

### Goals

Some general goals for this sprint are:

* [ ] Reach out to crate maintainers and learn their requirements
* [ ] Migrate projects for experience reports.


[async-book]: https://rust-lang.github.io/async-book/
[lucio-issue]: https://github.com/rustasync/team/issues/102
[doc-push]: https://tokio.rs/blog/2018-10-doc-blitz/
