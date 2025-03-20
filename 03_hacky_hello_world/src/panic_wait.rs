// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2023 Andre Richter <andre.o.richter@gmail.com>

//! A panic handler that infinitely waits.
use crate::{cpu, println};
use core::panic::PanicInfo;

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------


fn panic_prevent_reenter(){
    use core::sync::atomic::{AtomicBool, Ordering};

    #[cfg(not(target_arch = "aarch64"))]
    compile_error!("Add the target_arch to above's check if the following code is safe to use");
    println!("Some log message");
    static PANIC_IN_PROGRESS: AtomicBool = AtomicBool::new(false);

    if  !PANIC_IN_PROGRESS.load(Ordering::Relaxed) {
        PANIC_IN_PROGRESS.store(true, Ordering::Relaxed);
        //panic!("Panic reenter prevention");
        return;
    }

    cpu::wait_forever();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic_prevent_reenter();

    let (location, line, column) = match info.location() {
        Some(loc) => (loc.file(), loc.line(), loc.column()),
        _ => ("???", 0, 0),
    };

    println!(
        "Kernel panic!\n\n\
        Panic location:\n      File '{}', line {}, column {}\n\n\
        {:?}",
        location,
        line,
        column,
        info.message(),  // 直接打印 `PanicMessage`
    );

    cpu::wait_forever()
}


