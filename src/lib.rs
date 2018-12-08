#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(panic_implementation)]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![feature(panic_info_message)]
//#![feature(unique)]
//#![feature(const_unique_new)]
#![allow(dead_code)]
#![feature(unsize)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
extern crate cpuio;
#[macro_use]
extern crate bitflags;

#[macro_use]
mod vga_buffer;
mod memory;
mod keyboard;
use keyboard::*;

use memory::FrameAllocator;
use memory::Frame;
use core::panic::PanicInfo;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {

    vga_buffer::clear_screen();
    println!("PercOS v2");
    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");
    
    

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}",
            area.base_addr, area.length);
    }

    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf-sections tag required");
    
    println!("kernel sections:");
    for section in elf_sections_tag.sections() {
        println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}, allocated: {}",
            section.addr, section.size, section.flags, section.is_allocated());
    }

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("kernel start: {}, kernel end: {}", kernel_start, kernel_end);
    println!("multiboot start: {}, multiboot end: {}", multiboot_start, multiboot_end);

    let mut frame_allocator = memory::AreaFrameAllocator::new(kernel_start as usize, kernel_end as usize, multiboot_start, multiboot_end, memory_map_tag.memory_areas());
    for i in 0.. {
        if None == frame_allocator.allocate_frame() {
            println!("allocated {} frames", i);
            println!("Next free frame:  number: {}, address: {}, lenght: {} ", frame_allocator.next_free_frame.number, frame_allocator.next_free_frame.address, frame_allocator.next_free_frame.length);
            break;
        }
    }

    // Create a port pointing at 0x60, the address of the PS/2 keyboard
    // port on x86 hardware.  This is an unsafe operation because many
    // ports can be used to reconfigure your underlying hardware, and
    // it's the responsiblity of the port creator to make sure it's
    // used safely.
    let mut key_handler = keyboard::KeyHandler::new();
    
    loop {
        let key_handle = key_handler.update();
        if key_handle.is_some() {
            let key_out = key_handle.unwrap();
            print!("{}", key_out.clone().get_stdin());
            if key_out.get_flag(KEY_ESC) {
                clear!();
            }
        }
    }

    //warnln!("Test warning: panicking");
    //panic!("last wanrning");

    //loop{} // so that the assembly doesn't get to printing okay in the screen
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}
#[panic_handler]
#[no_mangle]
pub extern fn panic_fmt(pi: &PanicInfo) -> !{
    if pi.message().is_some() && pi.location().is_some(){
        panic_print!("A PercOS error occured because: {}\nat {}", pi.message().unwrap(), pi.location().unwrap());
    }else if pi.message().is_some() {
        panic_print!("A PercOS error occured because: {}", pi.message().unwrap());
    }else if pi.location().is_some() {
        panic_print!("A PercOS error occured at {}", pi.location().unwrap());
    }else{
        panic_print!("A PercOS error occured");
    }
    loop{}
}
