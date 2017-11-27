# Metal
A reimagining of the modern OS

By moving all code execution into the kernel (to avoid the overhead of interrupts) and only allowing WebAssembly to be executed, Metal will probably be fast and secure.

## Booting

### Preinstall
```
# Install cmdline utilities
$ apt-get install nasm xorriso 
# Install xargo
$ cargo install xargo
# Install rust-src
$ rustup component add rust-src
# Set top-level directory to default to rust-nightly
$ rustup override set nightly
```

### Building and Running
`$ make kernel` compiles and links the kernel binary

`$ make iso` makes the kernel and builds a bootable ISO image

`$ make run` builds the ISO and runs the image in QEMU

# License
This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Metal by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
