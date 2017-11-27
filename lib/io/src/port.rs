use x86_64::instructions::port::*;

use core::marker::PhantomData;

pub trait InOut {
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

pub unsafe fn port_out<T>(port: u16, value: T)
    where T: InOut
{
    T::port_out(port, value);
}

pub unsafe fn port_in<T>(port: u16) -> T
    where T: InOut
{
    T::port_in(port)
}

pub struct Port<T> {
    port: u16,
    _phantom: PhantomData<T>,
}

impl<T: InOut> Port<T> {
    /// Creates a new port
    pub const unsafe fn new(port: u16) -> Port<T> {
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

pub struct PortPair<T> {
    control: Port<T>,
    data: Port<T>,
}

impl<T: InOut> PortPair<T> {
    pub const unsafe fn new(control: u16, data: u16) -> PortPair<T> {
        PortPair {
            control: Port::new(control),
            data: Port::new(data),
        }
    }

    pub unsafe fn read(&mut self, control: T) -> T {
        self.control.write(control);
        self.data.read()
    }

    pub unsafe fn write(&mut self, control: T, data: T) {
        self.control.write(control);
        self.data.write(data);
    }

    pub unsafe fn write_data(&mut self, data: T) {
        self.data.write(data);
    }

    pub unsafe fn read_data(&self) -> T {
        self.data.read()
    }

    pub unsafe fn write_cmd(&mut self, control: T) {
        self.control.write(control);
    }

    pub unsafe fn read_cmd(&self) -> T {
        self.control.read()
    }
}