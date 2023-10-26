// lib.rs
//! `tfon`: Bitmap font library  
//! ` ↖ ↙ `
#![forbid(unsafe_code)]

mod common;
pub mod ifntx;
pub mod tfon;

pub use common::{Bitmap, Error, Prop};
