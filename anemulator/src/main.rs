#![feature(non_ascii_idents)]

extern crate tokio;
extern crate futures;

extern crate anemulator_hardware;

use tokio::prelude::*;
use tokio::prelude::Future;

use anemulator_hardware::*;

fn main() {
    println!("Hello, world!");
}
