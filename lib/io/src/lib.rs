
#![no_std]

#![feature(const_fn)]

extern crate x86_64;

use x86_64::instructions::port::*;

use core::marker::PhantomData;

trait InOut {
    unsafe fn port_in(port: u16) -> Self;
    unsafe fn port_out(port: u16, value: Self);
}

impl InOut for u8 {
    unsafe fn port_in(port: u16) -> u8 {
        inb(port)
    }
    unsafe fn port_out(port: u16, value: u8) {
        outb(port, value)
    }
}

impl InOut for u16 {
    unsafe fn port_in(port: u16) -> u16 {
        inw(port)
    }
    unsafe fn port_out(port: u16, value: u16) {
        outw(port, value)
    }
}

impl InOut for u32 {
    unsafe fn port_in(port: u16) -> u32 {
        inl(port)
    }
    unsafe fn port_out(port: u16, value: u32) {
        outl(port, value)
    }
}

pub struct Port<T> {
    port: u16,
    _phantom: PhantomData<T>,
}

impl<T: InOut> Port<T> {
    /// Creates a new port
    pub const fn new(port: u16) -> Port<T> {
        Port {
            port: port,
            _phantom: PhantomData,
        }
    }

    /// Reads from port
    pub unsafe fn read(&self) -> T {
        T::port_in(self.port)
    }

    /// Writes to port
    pub unsafe fn write(&mut self, value: T) {
        T::port_out(self.port, value);
    }
}