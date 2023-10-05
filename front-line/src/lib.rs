//! `front-line` - A declarative, zero-copy HTTP router for Rust.
//!
//! The `front-line` crate provides utilities to route HTTP requests based on their method and path,
//! offering both manual routing capabilities and a declarative macro-driven approach for simpler use cases.
//!
//! ## Features:
//!
//! - **Declarative**: Define routes as enums with proc-macro attributes.
//! - **Zero-copy capture**: Easily capture dynamic segments from paths (e.g., `/users/{id}`)
//!   with opt-in zero-copy capture to avoid unnecessary copying and allocations.
//! - **Dispatch free**: Only handles path based route resolution and allows the user to choose
//!   how to perform dispatch.
//!
//! ## Basic Usage:
//!
//! ```rust
//! use front_line::{FrontLine, HttpVersion, RouterResult, Router};
//!
//! #[derive(FrontLine)]
//! enum MarketingRoutes {
//!     #[get("/")]
//!     RenderIndex,
//!     #[get("/sign-up")]
//!     RenderSignUp,
//!     #[post("/sign-up")]
//!     ProcessSignUp,
//!     #[get("/log-in")]
//!     RenderLogIn,
//!     #[post("/log-in")]
//!     ProcessLogIn,
//!     #[get("/portal")]
//!     RenderPortal,
//! }
//!
//! #[derive(FrontLine)]
//! #[prefix("/api")]
//! enum ApiRoutes<'a> {
//!     #[get("/users")]
//!     GetAllUsers,
//!     #[post("/users")]
//!     CreateUser,
//!     #[get("/users/{id}")]
//!     GetUser { id: u32 },
//!     #[get("/users/{id}/roles/{role}")]
//!     GetUserRole { id: u32, role: &'a str },
//!     #[put("/users/{id}/roles/{role}")]
//!     UpdateUserRole { id: u32, role: &'a str },
//! }
//!
//! #[derive(FrontLine)]
//! enum AllRoutes<'a> {
//!     #[flatten]
//!     Marketing(MarketingRoutes),
//!     #[flatten]
//!     Api(ApiRoutes<'a>),
//! }
//!
//! // Construct an example http request, this would normally just be read off of a socket.
//! let request = b"GET /api/users/42?a=b HTTP/1.1\r\n\r\nContent-Length: 12\r\n\r\nHello World!";
//!
//! // Parse and and resolve the route
//! let route = AllRoutes::resolve(request);
//!
//! // For demonstration purposes, assert the resolved route is what we expect
//! assert!(matches!(route, Ok(RouterResult {
//!   route: Some(AllRoutes::Api(ApiRoutes::GetUser { id: 42 })),
//!   query: "a=b",
//!   version: HttpVersion::OneOne,
//!   head_and_body: b"Content-Length: 12\r\n\r\nHello World!",
//! })));
//!```
//!
//! For more advanced usage and examples, please refer to individual module documentation.

pub use front_line_derive::*;
pub use front_line_router::*;
