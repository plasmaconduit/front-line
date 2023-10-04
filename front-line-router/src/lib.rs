//! Do not import or use this crate directly, import and use `front-line` instead.
//! See: [front-line](https://docs.rs/front-line/latest/front_line/)

mod from_route;
mod http_version;
mod method;
mod router;
mod router_result;

pub use from_route::FromRoute;
pub use http_version::HttpVersion;
pub use method::Method;
pub use router::Error;
pub use router::Router;
pub use router_result::RouterResult;

pub use memchr;
