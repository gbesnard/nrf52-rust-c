#![no_std]

#[no_mangle]
pub extern "C" fn rust_function(number: u32) -> u32 {
  // doing something  really intelligent here
  let mut tmp: u32 = 40;
  tmp = tmp + number;
  return tmp;
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
