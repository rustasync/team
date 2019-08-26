---
layout: page
title: Web Foundations
permalink: /web-foundations/
---

Our goal is to improve web programming in Rust by:

- **Bolstering web components**, i.e. assessing the state of foundational crates for web programming (like `http` and `url`), and working to improve it by writing documentation and examples, making API improvements, standardizing interfaces, and in some cases writing whole new crates.
- **Building _Tide_**, which is a combination of a modular web framework built on the above components, and extensive documentation on what the components are, how to use them directly, and how they integrate into a framework. The name "Tide" refers to "a rising tide lifts all boats", conveying the intent that this work is aimed to improve sharing, compatibility, and improvements across *all* web development and frameworks in Rust.

## Roster

| **Discord handle**    | **Github handle** | **Email**                   |
| --------------------- | ----------------- | --------------------------- |
| **aturon** (co-lead)      | aturon            | aturon@mozilla.com          |
| **yoshuawuyts** (co-lead) | yoshuawuyts       | yoshuawuyts@gmail.com       |
| gruberb               | gruberb           | gruberbastian@me.com        |
| bIgBV                 | bIgBV             | bhargav.voleti93@gmail.com  |
| tinaun                | tinaun            | tinagma@gmail.com           |
| coleman               | anxiousmodernman  | coleman.mcfarland@gmail.com |
| rolf                  | rolftimmermans    | rolf@zxcv.nl                |
| kimsnj                | kimsnj            | karim.snj@gmail.com         |
| Spartan-S63           | ELD               | eric@dattore.me             |
| aknudsen              | aknuds1           | arve.knudsen@gmail.com      |
| Yarn | Yarn | yarnnd@gmail.com |
| sgrif | sgrif | sean@seantheprogrammer.com |
| xenith | xenith | xenith@xenith.org |
| jbcden | jbcden | jbcden@gmail.com |
| secretfader | secretfader | nyoung@uptime.ventures |
| grey | fairingrey | fairingrey@gmail.com |
| vorot93               | vorot93           | artem@vorotnikov.me         |
| taiki-e               | taiki-e           | te316e89@gmail.com          |
| nasa                  | k-nasa            | mail@k-nasa.me              |

[Add yourself!](https://github.com/rustasync/team/blob/gh-pages/{{ page.path }})

# Detailed vision

## Bolstering web components (lead: @yoshuawuyts)

Rust’s ecosystem already contains a number of components that one expects when doing web service development. However, these components are not necessarily discoverable, interoperable, or well-documented. We want to assess and improve the state of the web ecosystem, focused primarily on foundational components rather than the top-level frameworks that bring them together.

In the long run, many Rust users will ultimately use web frameworks, rather than working directly with components. However, those frameworks will ideally be built on shared components, and differ primarily in how an application is organized and expressed. We want it to be easy to start building new frameworks in Rust. We want to encourage experimentation, and we think the best way to achieve that is by creating a stable foundation of shared components that frameworks can build upon.

**Approach**
A priority should be to establish what people are commonly using, and where people are struggling. We will do this in two ways:


- Send out a survey to gather information on how Rust is being used to build services. Ask about frameworks, TLS libraries, how difficult are things to do, which libraries do people like a lot, where do they deploy, incompatibilities, and so on.
- Describe a few example use cases people might want to build, and work together to implement them, taking careful notes of pain points along the way. (For example: “create a reverse proxy”, “create & authenticate users”, “save TODOs to SQLite”).

Once we’ve gathered this information, we will analyze and disseminate it, likely in the form of a series of posts, with each focusing on a particular sub-area. The analysis will inform further work for the group, including:


- Improvements to existing crates
  - Involve crate authors in the conversation
  - Help steer people to issues on the crates marked “help wanted” / “good first issue”
  - Help improve documentation, and contribute examples
- Creation of new crates providing components or interfaces
- The ecosystem guide (see below)

Finally, a litmus test for the foundational components is the ability to easily build a simple but non-toy web framework in Rust; see below.

**Non-goals**
It’s probably worth mentioning explicitly that “building a framework” is meant as a way of process, but isn’t a goal in and of itself. We want to help grow the ecosystem, and pinpoint which parts might need help. Example of help are writing new modules, asking clear questions, and helping document pieces that are missing documentation. We think the best way of figuring out where to help is by encouraging people to start writing their own applications and frameworks, and to document that process with some strong examples.

## Tide: a modular framework and an ecosystem guide (lead: @aturon)

We want to have a comprehensive overview of Rust’s web ecosystem. People should feel comfortable writing Web applications with Rust, and we want to help push toward that goal.

In the long run, we expect most web development in Rust to take place within the context of a framework (large or small), and we expect there to be an ecosystem of frameworks to choose from. At the moment, however, the ecosystem for *async* frameworks is in its infancy, in part due to the churn around async foundations. And we expect it to take many experiments for the community to find the “best” ways of structuring web apps in Rust.

Thus, we will create a project, *Tide* (as in “a rising tide raises all boats”), which is *simultaneously*:

- A guide to the *components* of the web ecosystem (as opposed to frameworks). The components are things like the `http` and `url` crates — libraries that underpin any web work.
- A guide to integrating these components into a coherent framework.
- A real, modular framework that is extremely well-documented thanks to the above.

In a way, this is replaying something like the [Flask](http://flask.pocoo.org/) origin story, but for Rust: we will build Tide as a modular framework, and then write a detailed guide that documents the various components and how they come together in the framework.

This work will have three distinct audiences and ways of consuming:

- You can read the guide about the individual components to use them directly in a from-scratch project.
- You can read the guide as a way to understand and use the Tide framework as a foundation for your own projects.
- You can use the guide as a way to empower yourself to extend or modify the Tide framework, or to build your own framework.

**Non Goals**
We don’t mind being slightly opinionated about the choices we present. The overarching goal is to provide people a good sense on how to do asynchronous web programming in Rust, not to provide a complete list of web-related packages.

Similarly there are always going to be different approaches possible. For the guide we’ll pick a relatively simple framework design, and go with it.

**Approach**
This work will proceed in parallel with the ecosystem assessment and improvement in the first goal. Initially the focus will be on core framework mechanisms (plumbing and routing), and then on incorporating components identified in the first line of work and building examples with them.

The “framework” itself should be a very thin layer that just brings together a large set of components, making it easy for people to build their own mixture of components instead.
