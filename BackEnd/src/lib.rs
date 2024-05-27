#![feature(trait_alias)]
#![feature(int_roundings)]
#![feature(let_chains)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

mod controllers;
mod error;
pub mod orm;
pub mod utils;

pub use controllers::AppBuilder;
