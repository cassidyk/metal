
#![no_std]

#![feature(const_fn)]

extern crate x86_64;

mod port;

pub use port::{Port, PortPair};