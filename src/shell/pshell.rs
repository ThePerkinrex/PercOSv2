use super::Shell;
use multiboot2::BootInformation;

pub struct PShell<'a> {
    boot_info: &'a BootInformation,
    multiboot_information_address: usize,
}

impl<'a> Shell for PShell<'a> {
    fn exec_command(&mut self, command: &str) -> usize {
        let mut ret_val = 1;
        if command == "mem areas" {
            println!("MEMORY AREAS:");
            let memory_map_tag = self.boot_info.memory_map_tag()
                .expect("Memory map tag required");
            
            for area in memory_map_tag.memory_areas() {
                println!("    start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
            }
        }else if command == "mem kernel" {
            let elf_sections_tag = self.boot_info.elf_sections_tag()
                .expect("Elf-sections tag required");
            
            println!("kernel sections:");
            for section in elf_sections_tag.sections() {
                println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}, allocated: {}",
                    section.addr, section.size, section.flags, section.is_allocated());
            }
        }else if command == "mem loc" {
            let elf_sections_tag = self.boot_info.elf_sections_tag()
                .expect("Elf-sections tag required");
            let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
                .min().unwrap();
            let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
                .max().unwrap();

            let multiboot_start = self.multiboot_information_address;
            let multiboot_end = multiboot_start + (self.boot_info.total_size as usize);
            println!("kernel start: {}, kernel end: {}", kernel_start, kernel_end);
            println!("multiboot start: {}, multiboot end: {}", multiboot_start, multiboot_end);
        }else if command == "exit" || command == "logout"{
            ret_val = Self::EXIT_OS;
        }
        ret_val
    }
}

impl<'a> PShell<'a> {
    pub fn new(multiboot_information_address: usize) -> Self {
        PShell {
            boot_info: unsafe{ multiboot2::load(multiboot_information_address) },
            multiboot_information_address: multiboot_information_address,
        }
    }
}