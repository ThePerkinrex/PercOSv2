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
extern crate x86_64;

#[macro_use]
mod vga_buffer;
mod memory;
mod shell;
use shell::PShell;
mod keyboard;
use keyboard::*;


use memory::FrameAllocator;
use core::panic::PanicInfo;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {

    vga_buffer::clear_screen();
    println!("PercOS v2");
    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
    

    let elf_sections_tag = boot_info.elf_sections_tag().expect("Elf-sections tag required");

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size).max().unwrap();

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);
    let mut frame_allocator = memory::AreaFrameAllocator::new(kernel_start as usize, kernel_end as usize, multiboot_start, multiboot_end, memory_map_tag.memory_areas());
    memory::test_paging(&mut frame_allocator);

    // Create a port pointing at 0x60, the address of the PS/2 keyboard
    // port on x86 hardware.  This is an unsafe operation because many
    // ports can be used to reconfigure your underlying hardware, and
    // it's the responsiblity of the port creator to make sure it's
    // used safely.
    

    let mut key_handler = keyboard::KeyHandler::new();
    let new_shell = &mut PShell::new(multiboot_information_address);
    let shell_manager = &mut shell::ShellManager::new(new_shell);
    
    loop {
        let key_handle = key_handler.update();
        if key_handle.is_some() {
            let key_out = key_handle.unwrap();
            if key_out.clone().get_flag(KEY_ESC) {
                clear!();
            }
            let shell_flags = shell_manager.update(key_out);
            if (shell_flags & shell::EXIT_FLAG) == 1 {
                break;
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
