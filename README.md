# empire
Placeholder.

empire is a basic implementation of MPI in Rust targetting WebAssembly.
MPI requires blocking I/O, so work cannot proceed until
[WebAssembly threads and atomics](https://github.com/WebAssembly/threads)
are available.
