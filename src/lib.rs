extern crate deno_core;
extern crate dlopen;

use deno_core::plugin_api::{Interface, Op, ZeroCopyBuf};
use deno_core::{
    serde_json,
    serde_json::{json, Value},
};
use dlopen::raw::Library;
use serde::Deserialize;
use std::{cell::RefCell, collections::HashMap};

thread_local! {
    static LIBS_INDEX: RefCell<u32> = RefCell::new(1);
    static LIBS_MAP: RefCell<HashMap<u32, Library>> = RefCell::new(HashMap::new());
}

#[derive(Deserialize, Debug)]
enum DataType {
    C_INT,
}

#[derive(Deserialize, Debug)]
struct CallParam {
    data_type: DataType,
    value: Option<Value>,
}

#[derive(Deserialize, Debug)]
pub struct CallArgs {
    id: u32,
    name: String,
    return_type: Option<String>,
    params: Option<Vec<CallParam>>,
}

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("DENO_FFI_OPEN", op_open);
    interface.register_op("DENO_FFI_CALL", op_call);
}

fn op_open(_interface: &mut dyn Interface, zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let lib_file = zero_copy.get(0).unwrap();
    let lib_file = String::from_utf8_lossy(lib_file);
    let lib = Library::open(lib_file.as_ref()).unwrap();

    let mut instance_id: u32 = 0;
    LIBS_INDEX.with(|cell| {
        instance_id = cell.replace_with(|&mut i| i + 1);
    });

    LIBS_MAP.with(|cell| cell.borrow_mut().insert(instance_id, lib));
    Op::Sync(Box::new(instance_id.to_le_bytes()))
}

fn op_call(_interface: &mut dyn Interface, zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let json_bytes = zero_copy.get(0).unwrap();
    let args: CallArgs = serde_json::from_slice(json_bytes).unwrap();
    LIBS_MAP.with(|cell| {
        let libs = cell.borrow();
        let lib = libs.get(&args.id).unwrap();
        let api: fn() = unsafe { lib.symbol(&args.name) }.unwrap();
        api();
    });
    println!("{:?}", args);
    Op::Sync(Box::new([0]))
}
