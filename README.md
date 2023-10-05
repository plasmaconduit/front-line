# Front Line

[![crates.io][crates-badge]][crates-link]
[![docs.rs][docs-badge]][docs-link]

[crates-badge]: https://img.shields.io/crates/v/front-line
[crates-link]: https://crates.io/crates/front-line
[docs-badge]: https://img.shields.io/docsrs/front-line
[docs-link]: https://docs.rs/front-line/latest/front_line/

**A declarative, zero-copy, proc-macro based HTTP router for Rust.**

The `front-line` crate offers utilities to route HTTP requests based on their 
method and path. With declarative macro-driven approach, it's suitable for 
both complex and straightforward use cases.

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**

- [Features](#features)
- [Basic Usage](#basic-usage)
- [Testing](#testing)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Features

- **Declarative:** With the help of proc-macro attributes, you can easily 
  define routes as enums.
- **Zero-copy capture:** Dynamic segments from paths, such as `/users/{id}`,
  can be captured without unnecessary data copying, ensuring efficient memory usage.
- **Dispatch free:** The crate focuses purely on route resolution based on the 
  path, leaving dispatch methods up to the user's discretion.

## Basic Usage

```rust
use front_line::{FrontLine, HttpVersion, RouterResult, Router};

#[derive(FrontLine)]
enum MarketingRoutes {
    #[get("/")]
    RenderIndex,
    #[get("/sign-up")]
    RenderSignUp,
    #[post("/sign-up")]
    ProcessSignUp,
    #[get("/log-in")]
    RenderLogIn,
    #[post("/log-in")]
    ProcessLogIn,
    #[get("/portal")]
    RenderPortal,
}

#[derive(FrontLine)]
#[prefix("/api")]
enum ApiRoutes<'a> {
    #[get("/users")]
    GetAllUsers,
    #[post("/users")]
    CreateUser,
    #[get("/users/{id}")]
    GetUser { id: u32 },
    #[get("/users/{id}/roles/{role}")]
    GetUserRole { id: u32, role: &'a str },
    #[put("/users/{id}/roles/{role}")]
    UpdateUserRole { id: u32, role: &'a str },
}

#[derive(FrontLine)]
enum AllRoutes<'a> {
    #[flatten]
    Marketing(MarketingRoutes),
    #[flatten]
    Api(ApiRoutes<'a>),
}

// Construct an example http request, typically read from a socket.
let request = b"GET /api/users/42?a=b HTTP/1.1\r\n\r\nContent-Length: 12\r\n\r\nHello World!";

// Parse and resolve the route
let route = AllRoutes::resolve(request);

// For demonstration purposes, assert the resolved route matches expectations
assert!(matches!(route, Ok(RouterResult {
  route: Some(AllRoutes::Api(ApiRoutes::GetUser { id: 42 })),
  query: "a=b",
  version: HttpVersion::OneOne,
  head_and_body: b"Content-Length: 12\r\n\r\nHello World!",
})));
```

## Testing

Tests should run fine with the standard `cargo test`.

However, for consistency, we recommend using the dockerized test environment.
To use the dockerized test environment the only requirements are `make` and
`docker` (you don't even need rust installed locally). Simply run the
following command.

```
make test
```
