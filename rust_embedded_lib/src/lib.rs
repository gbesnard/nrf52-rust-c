#![no_std]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

/// cbindgen:ignore
pub mod bindings;

fn state_machine(foo: &mut bindings::foo_struct_t) {
  match foo.state{
    bindings::state_t_STATE_INIT    => foo.state = bindings::state_t_STATE_STARTED,
    bindings::state_t_STATE_STARTED => foo.state = bindings::state_t_STATE_STOPPED,
    _                               => foo.state = bindings::state_t_STATE_STOPPED
  }
}

#[no_mangle]
pub extern "C" fn rust_function(foo: *mut bindings::foo_struct_t) -> u32 {
    // called from C!

    // call back a C function, go back to the unsafe world!
    unsafe {
        bindings::rust_function_cb();
    }

    // unreference unsafe C pointer
    if !foo.is_null() {
        unsafe {
            let safe_foo = &mut(*foo);
            safe_foo.n = safe_foo.n + 1;
            state_machine(safe_foo);

            // returning something that matters
            if safe_foo.state == bindings::state_t_STATE_STOPPED {
                return bindings::SUCCESS_CODE;
            }
        }
    }

    // returning something useless
    return bindings::CONTINUE_CODE;
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
