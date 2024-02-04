#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;
use core::fmt::{self, Write};

global_asm!(include_str!("boot.S"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

struct SerialOutput;

impl Write for SerialOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            unsafe {
                core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
            }
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    SerialOutput.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[no_mangle]
pub unsafe fn main(){
    let _x = 1;
    println!("Welcome to Rust World\n");
}