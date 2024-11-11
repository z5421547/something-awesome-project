#![warn(clippy::all, rust_2018_idioms)]
#![allow(missing_docs)]

mod app;
pub mod clap;
pub use app::ConsoleDemo;
pub mod tools;
pub mod file;
pub mod journal;
pub mod scenario;
pub mod helpers;
pub mod sections;
// pub mod quiz;