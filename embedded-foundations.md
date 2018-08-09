---
layout: page
title: Embedded Foundations
permalink: /embedded-foundations/
---

Our goal is to support IoT development in Rust by:

- **Building reusable components for `no_std` asynchronous IO and hardware access**. While `std` based projects have the great `tokio` async IO stack to build off `no_std` projects don’t have much at this point. We should provide basic prototype implementations to kickstart this area of development, then see how production ready we can get stuff.
- **Prototyping an asynchronous IoT stack** from the hardware to the application, with a focus on ensuring portability and ease of integrating alternative layers. This should be examples of how to combine the components from the previous point into an actual IoT application, preferably based on easy to acquire development kits. The ultimate goal of this could be a guide to putting together the existing components and how to integrate your own custom components.

## Roster

| **Discord handle** | **Email**                | **Github handle** |
| ------------------ | ------------------------ | ----------------- |
| **Nemo157** (co-lead)  | wg-net@nemo157.com       | Nemo157           |
| **lkurusa** (co-lead)  | lkurusa@kernelstuff.org  | levex             |
| jkozlowski         | mail@jakub-kozlowski.com | jkozlowski        |
| degausser          | ricky@hosfelt.io         | deg4uss3r         |

# Detailed vision

## Reusable components for `no_std` async IO and hardware access

If possible this should reuse libraries developed by WG-embedded, but because the wake system requires plumbing at the very base level we’re unlikely to be able to use much more than the hardware definitions. There are fundamentally 4 major areas these components fall into:

- A `no_std` compatible executor to run futures on
- An async first hardware abstraction layer (HAL)
- Low-level networking libraries that bridge the gap between the hardware and what application level libraries expect
- Abstractions to allow integrating application level networking libraries with alternative network stacks

**A `no_std` compatible executor**
The most basic component that is necessary for async programming is to have an executor on which to schedule futures and waker system to re-schedule them when they can run. We should be able to provide a very basic `no_std` compatible executor that can run on major embedded targets like `thumbv6m-none-eabi`, this should be designed for portability to other targets.
By default the executor can be limited to running a single `Future` that can utilize internal concurrency. We can then experiment with different allocation strategies on top of this, e.g. simply utilizing `alloc` to be able to `Box` futures, or more complicated ones where the executor itself doesn’t own the allocation of the `Future` but is just responsible for polling it.

**An async first HAL**
Once we can run futures on our devices we then need to be able to interact with the hardware peripherals. It would be wonderful if we could reuse libraries from WG-embedded at this point, but integrating the waker system into them seems too invasive to be possible (we should still investigate this though). This will probably be a similar split to `embedded-hal`, a set of traits that libraries can build on top of and implementations for different hardware that an application can provide.

**Low-level networking libraries**
Most current async libraries in Rust are designed to be run on an OS, these all provide a native network stack that takes care of everything below the level of TCP/UDP for you. Some embedded devices may also provide this layer for you, but commonly you will be compiling your own network stack on top of an ethernet/radio transceiver/serial peripheral. Depending on what peripherals you have available you will need to provide more or less of the network stack, so being able to pull in just the parts you need is very important here.
We could potentially reuse existing C libraries such as `lwip` here, but integrating a C library that provides an async IO TCP/UDP stream while being built on top of an async IO UART/SPI stream sounds quite difficult.

**Integrating application level networking libraries**
Most current asynchronous networking libraries are built directly on top of `tokio`. To be able to instead run them on top of the alternative network stacks that embedded will need we have to be able to insert a layer of abstraction here.
We have the barest start of this with `futures-io` moving the core `AsyncRead`/`AsyncWrite` abstractions out of `tokio`, but we have far to go. Most services need the capability of binding a port and accepting a new connection coming in on it, most clients need to be able to initiate a new connection to a remote service.
There is also `std` provided functionality that will likely need to be moved to `core`, the entire `core::net` module is missing, so fundamental data types like `IpAddr` are not available.
There are many ways the actual abstraction could be accomplished, and we should investigate the pros and cons of these ways.

## **Prototyping an asynchronous IoT stack**

Once we have the pieces above in place we can provide example applications and documentation built on top of them. These should make it easy for existing embedded developers/existing Rust developers to get started in an IoT project based on Rust, and provide good pointers to how to develop and integrate their own libraries for missing functionality (there is such a wide array of hardware choices and network protocols that implementing everything as part of this WG is definitely non-viable).
