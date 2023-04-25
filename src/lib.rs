#![cfg_attr(not(feature = "std"), no_std)]
#![feature(lang_items, core_intrinsics, start)]

extern crate psp2_sys as psp2;

mod debug;

use core::{
    fmt::{Error, Write},
    intrinsics,
    panic::PanicInfo,
};

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    intrinsics::abort()
}

#[no_mangle]
pub fn main(_argc: isize, _argv: *const *const u8) -> Result<(), Error> {
    let mut screen = debug::screen::DebugScreen::new();
    writeln!(screen, "This bare-metal is starting to rust!\n")?;
    unsafe {
        psp2::kernel::threadmgr::sceKernelDelayThread(1_000_000); // Wait for 1 second
        writeln!(screen, "See ? I told you !\n")?;
        psp2::kernel::threadmgr::sceKernelDelayThread(3 * 1_000_000);
        psp2::kernel::processmgr::sceKernelExitProcess(0);
    }
    Ok(())
}

#[start]
#[no_mangle]
pub fn _start(argc: isize, argv: *const *const u8) -> isize {
    if let Ok(()) = main(argc, argv) {
        0
    } else {
        1
    }
}
