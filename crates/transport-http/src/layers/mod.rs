//! tower http like layer implementations that work over the http::Request type.
#![cfg(all(any(not(target_arch = "wasm32"), target_vendor = "wasmer"), feature = "hyper"))]

#[cfg(feature = "jwt-auth")]
mod auth;
#[cfg(feature = "jwt-auth")]
pub use auth::{AuthLayer, AuthService};
