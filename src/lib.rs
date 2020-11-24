extern crate deno_core;
extern crate dlopen;

use deno_core::{
    plugin_api::{Interface, Op, ZeroCopyBuf},
    serde_json::json,
};
use deno_core::{serde_json, serde_json::Value};
use dlopen::raw::Library;
use serde::Deserialize;
use std::{cell::RefCell, collections::HashMap};

thread_local! {
    static LIBS_INDEX: RefCell<u32> = RefCell::new(1);
    static LIBS_MAP: RefCell<HashMap<u32, Library>> = RefCell::new(HashMap::new());
}

type RP = *mut ();

#[derive(Deserialize, Debug)]
enum DataType {
    I32,
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
    return_type: Option<DataType>,
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
    let result: Vec<u8> = LIBS_MAP.with(|cell| {
        let libs = cell.borrow();
        let lib = libs.get(&args.id).unwrap();
        let return_value;
        match &args.params {
            None => {
                let api: fn() -> *mut () = unsafe { lib.symbol(&args.name) }.unwrap();
                return_value = api();
            }
            Some(params) => match params.len() {
                1 => {
                    let api: fn(RP) -> RP = unsafe { lib.symbol(&args.name) }.unwrap();
                    return_value = api(get_param(params, 0));
                }
                _ => panic!("Not supported"),
            },
        }
        if let Some(return_type) = args.return_type {
            let value = convert_return_value(return_value, &return_type);
            value.to_string().as_bytes().to_vec()
        } else {
            vec![]
        }
    });
    Op::Sync(result.into_boxed_slice())
}

fn get_param(params: &[CallParam], index: usize) -> RP {
    let param = params.get(index);
    match param {
        None => std::ptr::null_mut(),
        Some(param) => match &param.value {
            Some(value) => convert_data_type(value, &param.data_type),
            None => std::ptr::null_mut(),
        },
    }
}

fn convert_data_type(value: &Value, data_type: &DataType) -> RP {
    match data_type {
        DataType::I32 => {
            let v: i32 = value.as_i64().unwrap() as i32;
            v as *mut ()
        }
    }
}

fn convert_return_value(raw: *mut (), data_type: &DataType) -> Value {
    match data_type {
        DataType::I32 => {
            let v: i32 = raw as i32;
            json!(v)
        }
    }
}
