//! # What is this?
//!
//! `lttcore` is a set of common functionality for `LibTableTop` games and tools
//!
//! ## Who is this for
//!
//! `lttcore` has a number of modules for different use cases
//!
//! ### I want to build a game compatible with the `LibTableTop` tooling
//!
//! * Look through the [`examples`] directory which contains simple but complete
//! [`Play`](play::Play) compatible games for inspiration
//! * Look through the [`common`]/[`utilities`] modules to see if there are any off the shelf pieces you can reuse
//! * Look at the [`play`] documentation to learn the various parts of the [`Play`](play::Play) trait your game
//! must implement
//! * Look at [`bots`] to learn how to provide bot wrappers for your game
//!
//! ### I want to build a new runtime/analyzer/etc for `LibTableTop`
//!
//! * [`pov`] has various state machines to run [`Play`](play::Play) compatible games and analyze them from the
//! perspective of an [`observer`](pov::observer)/[`player`](pov::player)
//! * [`encoding`]/[`id`] have things to make it easier to interact with other existing tooling

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::redundant_closure_for_method_calls,
    clippy::default_trait_access,
    clippy::map_unwrap_or
)]

#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate lazy_static;

#[cfg(feature = "encoding")]
pub mod encoding;

#[cfg(feature = "examples")]
pub mod examples;

pub mod bots;
pub mod common;
pub mod id;
pub mod play;
pub mod pov;
pub mod utilities;
