#![no_std]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");

fn state_machine(foo: &mut foo_struct_t) {
  // increment n counter
  foo.n = foo.n + 1;

  // switch state
  match foo.state{
    state_t_STATE_INIT    => foo.state = state_t_STATE_STARTED,
    state_t_STATE_STARTED => foo.state = state_t_STATE_STOPPED,
    _                     => foo.state = state_t_STATE_STOPPED
  }
}

#[no_mangle]
pub extern "C" fn rust_function(foo: *mut foo_struct_t) -> u32 {
    // called from C!

    // call back a C function, go back to the unsafe world!
    unsafe {
        rust_function_cb();
    }

    // unreference unsafe C pointer
    let safe_foo: &mut foo_struct_t;
    if !foo.is_null() {
        unsafe {
            safe_foo = &mut(*foo);
        }

        state_machine(safe_foo);

        // returning something that matters
        if safe_foo.state == state_t_STATE_STOPPED {
            return SUCCESS_CODE;
        }
    }

    // returning something useless
    return CONTINUE_CODE;
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
