#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点

use core::panic::PanicInfo;

const UART0: *mut u8 = 0x09000000 as *mut u8; // ARM 架构上 UART 串行端口的内存地址

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let hello = b"Hello World!";
    for &byte in hello {
        unsafe {
            *UART0 = byte; // 向 UART0 地址写入字节，通过串行端口输出
        }
    }

    loop {}
}

/// 在发生 panic 时会调用该函数
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}