#![feature(lang_items)]
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
    println!("Hello Percot!");
    println!("Hello Perc{}","!");
    println!("{}", { println!("inner"); "outer" });
    loop{} // so that the assembly doesn't get to printing okay in the screen
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
