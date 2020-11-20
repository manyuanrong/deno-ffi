extern crate deno_core;
extern crate lazy_static;
extern crate dlopen;

use deno_core::plugin_api::{Interface, Op, ZeroCopyBuf};
use deno_core::futures::{Future, FutureExt};
use dlopen::raw::Library;

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("ffi_open", op_open);
}

fn op_open(_interface: &mut dyn Interface, zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let lib_file = zero_copy.get(0);
    let lib  = Library::open()
    Op::Sync(Box::new([1]))
}
