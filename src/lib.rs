extern crate deno_core;
extern crate dlopen;

use deno_core::plugin_api::{Interface, Op, ZeroCopyBuf};
use dlopen::raw::Library;
use std::{cell::RefCell, collections::HashMap};

thread_local! {
    static LIBS_INDEX: RefCell<u32> = RefCell::new(0);
    static LIBS_MAP: RefCell<HashMap<u32, Library>> = RefCell::new(HashMap::new());
}

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    println!("test1"); 
    interface.register_op("DENO_FFI_OPEN", op_open);
}

fn op_open(_interface: &mut dyn Interface, zero_copy: &mut [ZeroCopyBuf]) -> Op {
    println!("111");
    let lib_file = zero_copy.get(0).unwrap();
    let lib_file = String::from_utf8_lossy(lib_file);
    let lib = Library::open(lib_file.as_ref()).unwrap();

    let mut instance_id: u32 = 0;
    LIBS_INDEX.with(|cell| {
        instance_id = cell.replace_with(|&mut i| i + 1);
    });

    LIBS_MAP.with(|cell| cell.borrow_mut().insert(instance_id, lib));
    Op::Sync(Box::new([0]))
}
