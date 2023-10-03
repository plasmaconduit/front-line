# front-line

**A declarative, zero-copy HTTP router for Rust.**

The `front-line` crate offers utilities to route HTTP requests based on their 
method and path. With declarative macro-driven approach, it's suitable for 
both complex and straightforward use cases.

## Features:

- **Declarative:** With the help of proc-macro attributes, you can easily 
  define routes as enums.
- **Zero-copy capture:** Dynamic segments from paths, such as `/users/{id}`,
  can be captured without unnecessary data copying, ensuring efficient memory usage.
- **Dispatch free:** The crate focuses purely on route resolution based on the 
  path, leaving dispatch methods up to the user's discretion.

## Basic Usage:

```rust
use front_line::FrontLine;
use front_line_router::{HttpVersion, RouterResult, Router};

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