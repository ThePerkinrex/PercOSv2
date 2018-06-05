#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(panic_implementation)]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![feature(unique)]
#![feature(const_unique_new)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() {

    vga_buffer::clear_screen();
    println!("PercOS v2");
    println!("Hola Luis");
    println!("Este es mi sistema operativo");
    //println!("{}", { println!("inner"); "outer" });
    loop{} // so that the assembly doesn't get to printing okay in the screen
}

//use core::intrinsics;
use core::panic::PanicInfo;

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[panic_implementation] #[no_mangle] pub extern fn panic_fmt(_info: &PanicInfo) -> ! {loop{}}
