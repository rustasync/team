---
layout: page
title: Async Foundations
permalink: /async-foundations/
---

Our goal is to bring async/await onto a path to stabilization and to provide documentation for asynchronous programming:

- **Futures 0.3 and async/await** should be vetted, well-documented, well-integrated, future-proof, and on a clear path to stabilization.
- **The futures-rs blog** aims to give regular updates on the latest changes to `async`/`await` and the futures library.
- **The** [**Asynchronous Programming in Rust book**](https://github.com/rust-lang-nursery/wg-net/blob/master/async-book/src/SUMMARY.md) ****should have a complete draft, covering async/await, core futures concepts, Tokio, and enough of the ecosystem to give good examples and guidance. It should also explicitly talk about the stabilization situation, including how to bridge between stable 0.1 and unstable 0.3 worlds.

## Roster

| **Discord handle** | **Email**                    | **Github handle** |
| ------------------ | ---------------------------- | ----------------- |
| **MajorBreakfast** (co-lead)     | mail@josefbrandl.de          | MajorBreakfast    |
| **cramertj** (co-lead)          | cramertj@google.com          | cramertj          |
| tomaka             | pierre.krieger1708@gmail.com | tomaka            |
| jkozlowski         | mail@jakub-kozlowski.com     | jkozlowski        |
| aajtodd            | aajtodd@gmail.com            | aajtodd           |
| tinaun             | tinagma@gmail.com            | tinaun            |
| rolf               | rolf@zxcv.nl                 | rolftimmermans    |
| Spartan-S63        | eric@dattore.me              | ELD               |
| aknudsen           | arve.knudsen@gmail.com       | aknuds1           |
| jsgf | jsgf@fb.com | jsgf |
| inejge | inejge@gmail.com | inejge |

[Add yourself!](https://github.com/rust-lang-nursery/wg-net/blob/gh-pages/{{ page.path }})

# Futures 0.3 and async/await

This section explains in further detail what is meant by the above short description for futures 0.3 and `async`/`await`.

## Vetted

`async`/`await` and `futures` 0.3 must be used in existing production deployments of Tokio and other asynchronous frameworks. Any non-essential runtime structure decisions imposed by `task::Context` (e.g. the structure of `Waker` and `LocalWaker`) must be thoroughly benchmarked against alternatives so that no optimization opportunities are being lost by stabilizing the interface.

The capabilities of `async`/`await` need to be validated by porting existing 0.1 applications and libraries (like `hyper`) to make use of it.

## Well-Documented

 `futures` 0.3 and the `std::task` module must be well-documented and comprehensible to a beginner without significant external resources. The “Asynchronous Programming in Rust” book must be ready for public consumption and must have gone through review by multiple Rust users new to asynchronous programming.

## Well-Integrated

`async`/`await` must be usable with existing `futures` 0.1-based libraries, and multiple existing 0.1 libraries (including `tokio` and `hyper`) must have successfully ported to use `futures` 0.3 under the hood without any loss of performance.

Existing `futures` 0.1-based applications must be able to gradually adopt `futures` 0.3 without significant interruption to their development cycle.

## Future-Proof Syntax

The `async`/`await` syntax should be vetted for consistency with existing and planned language features. In particular consistency with the planned generator function and async generator function syntaxes is desired. A plan for how these feature could look like with consistent syntax needs to be established before `async`/`await` can be stabilized.

## Clear Path Towards Stabilization

All of the remaining technical blockers towards `async`/`await` stabilization must be resolved:
Consensus must be achieved among stakeholders (major library authors, production users, and other significantly interested parties) that the APIs in `std::task` and `std::future` are high-quality, leave no performance on the table, and provide a high-quality `async`/`await`-style developer experience.

Major stakeholders must see a path to using the `async`/`await` programming style in their existing applications. Blockers to adoption (such as `async fn` in traits, named existentials, or other language or library features) must have clear plans and goal timelines for resolution.
