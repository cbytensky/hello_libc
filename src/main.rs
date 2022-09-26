#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	loop {}
}

extern crate libc;

#[no_mangle]
pub unsafe extern fn _start() {
	libc::puts("Hello World!\0".as_ptr() as *const _);
	libc::exit(0);
}
