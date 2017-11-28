# Ideas
Everything here will probably change. This is just how I have it planned out right now.

## Overhead View

Metal is a new type of Operating System that runs all code in a VM.

Similarly to SingularityOS, the core of the kernel will be written in native code (Rust in the case of Metal), but everything else will be "managed."

## Services

Drivers and code that is intended to be accessed by other code that's running on the system (e.g. Network, Filesystem, etc) will be run as a **service**.

Instead of system-calls (although IPC architecture is still up-for-grabs), communication between **processes** and the kernel itself will be faciliated through memory queues.

WebAssembly modules that need to talk to the kernel (most, if not all, will have to) import functions that enable them to send messages to registered **services**. It's unclear how the implementation will precisely work at this time, but my intent is for it to be as asynchronous as possible (that is, no blocking whatsoever).

## Software Isolated Processes

All processes will be, in short, stored contexts that the WebAssembly runtime will execute. Whether the multithreading implementation will be preemptive or cooperative is unclear at this time.

Since all code runs in a VM (or VMs in multicore systems), there is no need for hardware-isolated processes, and thus no need for syscalls and context-switching, which both have signifigant overhead. Therefore, as long as switching between WebAssembly contexts is optimized enough, **Metal** has the possibility of outperforming conventional kernels.