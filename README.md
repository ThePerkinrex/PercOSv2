# PercOS v2

PercOS v2 is a the successor to [PercOS](https://github.com/theperkinrex/percos), a python3 virtual machine that just simulated a UNIX-like OS.

PercOS v2 is barebones x86-64 bit kernel written in rust.

For now it has a module to write to the VGA buffer implementing the `print!` and `println!` macros as well as a macro for panicking. It implements panic and simple paging.