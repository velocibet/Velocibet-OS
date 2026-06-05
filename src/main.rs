#![no_std]
#![no_main]

#![feature(abi_x86_interrupt)] 

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;
mod interrupts;
pub mod gdt;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    // interrupts::init_idt();
    // x86_64::instructions::interrupts::int3(); 

    blog_os::init();

    unsafe {
        // 존재하지 않는 메모리 주소를 강제로 수정하여 패닉을 발생시킴
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