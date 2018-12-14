# PercOS v2

PercOS v2 is the successor to [PercOS](https://github.com/theperkinrex/percos), a python3 virtual machine that just simulated a UNIX-like OS.

PercOS v2 is barebones x86-64 bit kernel written in rust.

For now it has a module to write to the VGA buffer implementing the `print!` and `println!` macros as well as a macro for panicking. It implements simple paging & very rough shell.

***

To set up the environment, you'll need vagrant, lldb (The rust-lldb version), make, rust & a lot of things.

To set up vagrant follow the instructions on the vagrant docs

To setup the lldb debugger, run the script `setup.sh`, you may need to run `chmod +x setup.sh` to be able to run the script.

If you're not on osx, you may need to install qemu & edit th makefile to make the `qemu` variable point to your installation
As well you can use gdb on linux, but follow this instructions to use it: https://os.phil-opp.com/set-up-gdb/

***

To use the environment, everything goes through the makefile.

To start, start the vagrant vm with `make up`  
To stop the vm: `make halt`   
To build everything: `make iso`  
To run it with qemu: `make run`
To start lldb & qemu: `make lldb`
To start lldb & qemu connecting to qemu from the start: `make lldb-d`
To start qemu and make it wait for a debugger to connect use: `make debug`