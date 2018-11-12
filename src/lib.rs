#![doc(html_root_url = "https://docs.rs/tokio-uds/0.2.3")]
#![deny(missing_docs, warnings, missing_debug_implementations)]

//! Unix Domain Sockets for Tokio.
//!
//! This crate provides APIs for using Unix Domain Sockets with Tokio.

extern crate bytes;
#[macro_use]
extern crate futures;
extern crate iovec;
extern crate log;
extern crate mio;
extern crate mio_uds_windows;
extern crate tokio_io;
extern crate tokio_reactor;

mod incoming;
mod listener;
mod stream;

pub use incoming::Incoming;
pub use listener::UnixListener;
pub use stream::{UnixStream, ConnectFuture};
