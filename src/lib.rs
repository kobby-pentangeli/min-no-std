//! A minimal setup for a `no_std` library with `wee_alloc` as allocator

// Set up for no-std.
#![no_std]
// Currently, no_std requires the nightly compiler due to the crates that it uses.
#![feature(alloc_error_handler, core_intrinsics, lang_items)]

extern crate alloc;
extern crate wee_alloc;

// Set up the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Set up error handler
#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    core::intrinsics::abort();
}

// Set up panic handler
#[panic_handler]
#[lang = "panic_impl"]
extern "C" fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort();
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
