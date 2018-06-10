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
extern crate multiboot2;
extern crate cpuio;

#[macro_use]
mod vga_buffer;
mod memory;
mod keyboard;

use memory::FrameAllocator;
use core::panic::PanicInfo;
use cpuio::Port;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {

    vga_buffer::clear_screen();
    println!("PercOS v2");
    //println!("Hola Luis");
    //println!("Este es mi sistema operativo");
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
        println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
            section.addr, section.size, section.flags);
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
        if let None = frame_allocator.allocate_frame() {
            println!("allocated {} frames", i);
            break;
        }
    }

    // Create a port pointing at 0x60, the address of the PS/2 keyboard
    // port on x86 hardware.  This is an unsafe operation because many
    // ports can be used to reconfigure your underlying hardware, and
    // it's the responsiblity of the port creator to make sure it's
    // used safely.
    let mut keyboard_port: Port<u8> = unsafe { Port::new(0x60) };

    let mut last_code: u8 = 0;
    loop {

        let read = keyboard_port.read();
        println!("scancode: 0x{:x?}, {:?}", read, read);
        let released: bool = keyboard::check_release(last_code, read);
        println!("releasecode: 0x{:x?}, {:?}", read, read);
        if released {
            break;
        }
        last_code = read;
    }

    //warnln!("Test warning: panicking");
    //panic!("last wanrning");

    loop{} // so that the assembly doesn't get to printing okay in the screen
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[panic_implementation]
#[no_mangle]
pub extern fn panic_fmt(pi: &PanicInfo) -> !
{
    panic_print!("\n\nPercOS {}", pi); // prints: "PercOS panicked at 'reason', src/'file':'location'"
    loop{}
}
