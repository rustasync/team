# Rust 2019 Async Ecosystem Working Group

This repo is for coordinating the Rust Async Ecosystem Working Group.

- [Announcement of the 2019 working group reorganization][async-working-groups]
- [Background on the original WG-net 2018 working groups is here.][wg-net-working-groups]
- [Come chat with us on #wg-async-ecosystem! #wg-async-foundation to chat with the Async Foundations WG!][discord]

The [issue tracker] on this repo is a primary point of coordination. If you have an async-related topic you'd like to raise, please feel free to open an issue!

[async-working-groups]: https://blog.yoshuawuyts.com/async-ecosystem-wg/
[wg-net-working-groups]: https://internals.rust-lang.org/t/announcing-the-2018-domain-working-groups/6737
[discord]: https://discord.gg/rust-lang
[issue tracker]: https://github.com/rustasync/team/issues

# Goals and structure

- **Leads**: @aturon and @yoshuawuyts
- [Vision document and roster](https://rustasync.github.io/team/web-foundations)

The WG is focused on progressing the ecosystem around the async foundations in 2019. If you want to get involved in these efforts, hop on [Discord][discord] and say hello, or take a look at the [issue tracker]. Our goal is to improve the async library ecosystem in Rust by:

  - **Bolstering web components**, i.e. assessing the state of foundational crates for web programming (like `http` and `url`), and working to improve it by writing documentation and examples, making API improvements, standardizing interfaces, and in some cases writing whole new crates.
  - **Building _Tide_**, which is a combination of a simple, modular web framework built on the above components, and extensive documentation on what the components are, how to use them directly, and how they integrate into a framework. The name "Tide" refers to "a rising tide lifts all boats", conveying the intent that this work is aimed to improve sharing, compatibility, and improvements across *all* web development and frameworks in Rust.
  - **Experimenting** with projects such as the [Juliex][juliex] executor, the [Romio][romio] reactor, and the [Runtime][runtime] crate
  - **The [Asynchronous Programming in Rust book](https://github.com/rust-lang/async-book)** should have a complete draft, covering async/await, core futures concepts, Tokio, and enough of the ecosystem to give good examples and guidance. It should also explicitly talk about the stabilization situation, including how to bridge between stable 0.1 and unstable 0.3 worlds.

[juliex]: https://github.com/withoutboats/juliex
[romio]: https://github.com/withoutboats/romio
[runtime]: https://github.com/rustasync/runtime
