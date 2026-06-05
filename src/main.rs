#![no_std]
#![no_main]

#![feature(abi_x86_interrupt)] 

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod vga_buffer;
mod interrupts;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    interrupts::init_idt();
    // x86_64::instructions::interrupts::int3(); 

    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };

    println!("It did not crash!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}