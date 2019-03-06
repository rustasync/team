---
layout: post
title:  "Rust Web Survey 2018"
author: Bhargav Voleti
date:   2018-11-28
---
# Introduction

We recently sent out a survey regarding the state of the current Rust web ecosystem and we got over a 1000 responses! We really appreciate the feedback from the community. This will help us continue to improve upon the state of the Rust web ecosystem. Today, we would like to go over the responses and understand the results.


# The present
  Rust is a relatively new language and our users reflect that, with more than 60% of them having only used it for the last two years. Coming to web development, the two most popular frameworks for building web applications are [Rocket](https://rocket.rs/) followed by [Actix](https://actix.rs) at 27% and 24% respectively. Some of the many other popular web frameworks currently being used to build web applications are [Iron](http://ironframework.io/), [Gotham](https://gotham.rs), [Warp](https://github.com/seanmonstar/warp/) and [Tower-web](https://github.com/carllerche/tower-web). As we can see in the figure, 20% of people choose to not use any web frameworks and instead opt to build on top of the standard library or [Hyper](http://hyper.rs) when building their services.


<img src="/team/assets/images/wg-net-survey-frameworks.png" alt="Frameworks pie chart" width="100%"/>



# The issues

Coming to what people feel is missing from the Rust networking/web ecosystem, 65% of users find the lack of examples to be the biggest issue currently with the ecosystem followed by the lack of documentation at 53%. This is something that is actively being addressed by initiatives such as the [Tokio Doc Push](https://tokio.rs/blog/2018-10-doc-blitz/) and the [Rust async book](https://rust-lang.github.io/async-book/).

There is also users talking about the lack of a One True Framework a la [Django](https://www.djangoproject.com/) in Python and [Rails](https://rubyonrails.org/) in ruby. This is something which was considered by the networking work group and to this end, work has begun on [Tide](https://github.com/rust-net-web/tide/) a framework meant to provide a good starting point for people to get started with building web applications in Rust. Another goal for Tide is to serve as documentation for people wanting to dig deeper and learn how to write such services in Rust.

The next issue which users mentioned is the lack of bindings for frameworks and services used when building applications. This includes bindings for applications like [Cassandra](https://cassandra.apache.org/) a fast NoSQL database, support for running on [Kubernetes](https://kubernetes.io/), a container orchestration framework  and [LDAP](https://en.wikipedia.org/wiki/Lightweight_Directory_Access_Protocol) an  authentication protocol. A few people also touch on the lack of asynchronous database access support and no idiomatic example of how to go about doing it.

Rust aims to make writing systems software on various platforms easier. This is not always possible though, and is shows as our users report that using OpenSSL is the biggest platform specific pain faced by them.

92% of our respondents have written a web application in a different language and provide information regarding what Rust can learn from those languages. People find that they miss the vast batteries included standard library present in programming languages such as Go and Python which let you write performant web applications and services straight out of the box.

But all is not so bleak! The Rust language continues to interest people as the most popular reason why people chose to use Rust for their web service/application was the language itself at 90%. On top of the features of the Rust languages, the next reason was the runtime performance Rust offered at 70%.


# Conclusion

After going through all the great responses, these are some of the common themes that we found:

- There is a lot of excitement for the future of the Rust networking and web ecosystem.
- Most of the users are really excited for the Rust language itself.
- The lack of documentation and examples is the major limiting factor for people to build services and applications in Rust today.
- The basic building blocks for building robust web services are starting to stabilize, now the ecosystem built on top of these is what people are missing.
