#![no_std]

#[no_mangle]
pub extern "C" fn rust_function() {

}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
