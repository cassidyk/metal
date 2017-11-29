Metal needs to be rewritten to conform to project "requirements"

Current ideas of how it'll work

Processes
- extremely lightweight
- fast to create and destroy
- no dynamic code execution
    - to execute new code, a process must create a child process
- starts with a small, but resizable heap
- very fast to switch between processes for execution

IPC
- uses a single shared heap
- must be extremely fast
- no copying
- a process calls `ipc_connect`<sup>(name pending)</sup> which returns an area of memory that refers to an object in the shared heap
- process can write to that object
- once sent, if the process attempts to write to object, the process and its children will be immediately terminated
- **TODO**: *finish IPC definition*

Threading
- Not sure if threading should be implemented and if so, how

Microkernel
- all drivers, applications, etc run in "appmode".
- Applications provide a manifest of unsafe things they need to do
    - access raw memory
    - handle interrupts
    - etc

**This is a working document
Feel free to comment suggestions**
