#![feature(non_ascii_idents)]

extern crate tokio;
extern crate futures;

use tokio::prelude::*;
use tokio::prelude::Future;

#[macro_use]
mod instruction;

mod potato;

fn main() {
    println!("Hello, world!");
}
