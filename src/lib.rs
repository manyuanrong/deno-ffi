extern crate deno_core;
extern crate dlopen;
extern crate libffi;

use deno_core::{
    plugin_api::{Interface, Op, ZeroCopyBuf},
    serde_json::json,
};
use deno_core::{serde_json, serde_json::Value};
use dlopen::raw::Library;
use libffi::high as ffi;
use serde::Deserialize;
use std::{cell::RefCell, collections::HashMap};

thread_local! {
    static LIBS_INDEX: RefCell<u32> = RefCell::new(1);
    static LIBS_MAP: RefCell<HashMap<u32, Library>> = RefCell::new(HashMap::new());
}

#[derive(Deserialize, Debug)]
struct CallParam {
    data_type: String,
    value: Value,
}

enum ArgValue {
    I32(i32),
    I64(i64),
    F32(f32),
    Void,
}

#[derive(Deserialize, Debug)]
pub struct CallArgs {
    id: u32,
    name: String,
    return_type: String,
    params: Vec<CallParam>,
}

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("DENO_FFI_LOAD", op_open);
    interface.register_op("DENO_FFI_CALL", op_call);
    interface.register_op("DENO_FFI_UNLOAD", op_unload);
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

    let result: Result<Value, String> = LIBS_MAP.with(|cell| {
        let libs = cell.borrow();
        let lib = libs.get(&args.id).ok_or("lib is not loaded or closed")?;
        call_lib_api(lib, &args.name, &args.params, args.return_type)
    });

    let return_json: Value = match result {
        Err(msg) => json!({
            "error": msg,
            "value": null,
        }),
        Ok(return_value) => {
            json!({
                "error": null,
                "value": return_value,
            })
        }
    };

    Op::Sync(
        return_json
            .to_string()
            .as_bytes()
            .to_vec()
            .into_boxed_slice(),
    )
}

fn op_unload(_interface: &mut dyn Interface, zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let json_bytes = zero_copy.get(0).unwrap();
    let json: Value = serde_json::from_slice(json_bytes).unwrap();
    let instance_id: u32 = json.as_i64().unwrap() as u32;
    LIBS_MAP.with(|cell| cell.borrow_mut().remove(&instance_id));
    Op::Sync(Box::new([]))
}

#[allow(clippy::type_complexity)]
fn call_lib_api(
    lib: &Library,
    name: &str,
    params: &[CallParam],
    return_type: String,
) -> Result<Value, String> {
    let fn_ptr = unsafe { lib.symbol(name) }.map_err(|err| err.to_string())?;
    let fn_code_ptr = ffi::CodePtr::from_ptr(fn_ptr);
    let args: Vec<ArgValue> = params
        .iter()
        .map(|param| match param.data_type.as_str() {
            "i32" => ArgValue::I32(param.value.as_i64().unwrap() as i32),
            "i64" => ArgValue::I64(param.value.as_str().unwrap().parse::<i64>().unwrap()),
            _ => ArgValue::Void,
        })
        .collect();

    let args: Vec<ffi::Arg> = args
        .iter()
        .map(|value| match value {
            ArgValue::I32(v) => ffi::arg(v),
            ArgValue::F32(v) => ffi::arg(v),
            ArgValue::I64(v) => ffi::arg(v),
            ArgValue::Void => ffi::arg(&()),
        })
        .collect();

    let ret = match return_type.as_str() {
        "i32" => {
            json!(unsafe { ffi::call::<i32>(fn_code_ptr, args.as_slice()) })
        }
        "i64" => {
            json!(unsafe { ffi::call::<i64>(fn_code_ptr, args.as_slice()) })
        }
        _ => json!(null),
    };

    Ok(ret)
}
