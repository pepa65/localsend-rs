#![allow(dead_code)]

pub mod scanner;
pub mod send;
pub mod server;
pub mod util;

mod error;
mod receive;
mod settings;

pub use {error::*, settings::Settings};

pub type Result<T> = std::result::Result<T, crate::localsend_lib::Error>;
