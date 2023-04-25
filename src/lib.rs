#![cfg_attr(not(feature = "std"), no_std)]
#![feature(lang_items, core_intrinsics, start, error_in_core)]

#[macro_use]
extern crate alloc;

extern crate psp2_sys as psp2;

use core::{error::Error, fmt::Write, intrinsics, panic::PanicInfo, time::Duration};

use alloc::boxed::Box;
use deblockator::Deblockator;
use vitallocator::Vitallocator;

#[global_allocator]
static ALLOCATOR: Deblockator<Vitallocator> = Deblockator::new(Vitallocator::new());

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    intrinsics::abort()
}

#[no_mangle]
pub fn main(_argc: isize, _argv: *const *const u8) -> Result<(), Box<dyn Error>> {
    let a = vec![1, 2, 3];
    let mut screen = vita::debug::screen::DebugScreen::new(vita::debug::font::DEFAULT_FONT);
    writeln!(screen, "This bare-metal is starting to rust! {a:?}")?;
    vita::thread::sleep(Duration::from_secs(1)); // Wait for 1 second
    writeln!(screen, "See ? I told you !")?;
    vita::thread::sleep(Duration::from_secs(3));
    unsafe {
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
