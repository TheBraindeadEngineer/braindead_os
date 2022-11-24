#![no_std]
#![no_main]

use bootloader::{BootInfo, entry_point};

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{}", info);
    loop {}
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> !{
    use braindead_os::memory::active_level_4_table;
    use x86_64::VirtAddr;

    braindead_os::init();

    println!("Welcome to Braindead OS!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let x14_table = unsafe { active_level_4_table(phys_mem_offset) };

    for(i, entry) in x14_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }

    braindead_os::hlt_loop();
}
