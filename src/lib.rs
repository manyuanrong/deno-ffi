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
    I64,
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
    params: Vec<CallParam>,
}

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("DENO_FFI_OPEN", op_open);
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

    let result: Result<RP, String> = LIBS_MAP.with(|cell| {
        let libs = cell.borrow();
        let lib = libs.get(&args.id).ok_or("lib is not loaded or closed")?;
        call_lib_api(lib, &args.name, &args.params)
    });

    let return_json: Value = match result {
        Err(msg) => json!({
            "error": msg,
            "value": null,
        }),
        Ok(return_value) => {
            let value = match args.return_type {
                Some(return_type) => convert_return_value(return_value, &return_type),
                None => json!(null),
            };
            json!({
                "error": null,
                "value": value,
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

fn call_lib_api(lib: &Library, name: &str, params: &[CallParam]) -> Result<RP, String> {
    // TODO Use macro_rules to simplify
    match params.len() {
        0 => {
            let api: fn() -> RP = unsafe { lib.symbol(name) }.map_err(|err| err.to_string())?;
            Ok(api())
        }
        1 => {
            let api: fn(RP) -> RP = unsafe { lib.symbol(name) }.map_err(|err| err.to_string())?;
            Ok(api(get_param(params, 0)))
        }
        2 => {
            let api: fn(RP, RP) -> RP =
                unsafe { lib.symbol(name) }.map_err(|err| err.to_string())?;
            Ok(api(get_param(params, 0), get_param(params, 1)))
        }
        3 => {
            let api: fn(RP, RP, RP) -> RP =
                unsafe { lib.symbol(name) }.map_err(|err| err.to_string())?;
            Ok(api(
                get_param(params, 0),
                get_param(params, 1),
                get_param(params, 2),
            ))
        }
        4 => {
            let api: fn(RP, RP, RP, RP) -> RP =
                unsafe { lib.symbol(name) }.map_err(|err| err.to_string())?;
            Ok(api(
                get_param(params, 0),
                get_param(params, 1),
                get_param(params, 2),
                get_param(params, 3),
            ))
        }
        5 => {
            let api: fn(RP, RP, RP, RP, RP) -> RP =
                unsafe { lib.symbol(name) }.map_err(|err| err.to_string())?;
            Ok(api(
                get_param(params, 0),
                get_param(params, 1),
                get_param(params, 2),
                get_param(params, 3),
                get_param(params, 4),
            ))
        }
        6 => {
            let api: fn(RP, RP, RP, RP, RP, RP) -> RP =
                unsafe { lib.symbol(name) }.map_err(|err| err.to_string())?;
            Ok(api(
                get_param(params, 0),
                get_param(params, 1),
                get_param(params, 2),
                get_param(params, 3),
                get_param(params, 4),
                get_param(params, 5),
            ))
        }
        7 => {
            let api: fn(RP, RP, RP, RP, RP, RP, RP) -> RP =
                unsafe { lib.symbol(name) }.map_err(|err| err.to_string())?;
            Ok(api(
                get_param(params, 0),
                get_param(params, 1),
                get_param(params, 2),
                get_param(params, 3),
                get_param(params, 4),
                get_param(params, 5),
                get_param(params, 6),
            ))
        }
        8 => {
            let api: fn(RP, RP, RP, RP, RP, RP, RP, RP) -> RP =
                unsafe { lib.symbol(name) }.map_err(|err| err.to_string())?;
            Ok(api(
                get_param(params, 0),
                get_param(params, 1),
                get_param(params, 2),
                get_param(params, 3),
                get_param(params, 4),
                get_param(params, 5),
                get_param(params, 6),
                get_param(params, 7),
            ))
        }
        9 => {
            let api: fn(RP, RP, RP, RP, RP, RP, RP, RP, RP) -> RP =
                unsafe { lib.symbol(name) }.map_err(|err| err.to_string())?;
            Ok(api(
                get_param(params, 0),
                get_param(params, 1),
                get_param(params, 2),
                get_param(params, 3),
                get_param(params, 4),
                get_param(params, 5),
                get_param(params, 6),
                get_param(params, 7),
                get_param(params, 8),
            ))
        }
        10 => {
            let api: fn(RP, RP, RP, RP, RP, RP, RP, RP, RP, RP) -> RP =
                unsafe { lib.symbol(name) }.map_err(|err| err.to_string())?;
            Ok(api(
                get_param(params, 0),
                get_param(params, 1),
                get_param(params, 2),
                get_param(params, 3),
                get_param(params, 4),
                get_param(params, 5),
                get_param(params, 6),
                get_param(params, 7),
                get_param(params, 8),
                get_param(params, 9),
            ))
        }
        11 => {
            let api: fn(RP, RP, RP, RP, RP, RP, RP, RP, RP, RP, RP) -> RP =
                unsafe { lib.symbol(name) }.map_err(|err| err.to_string())?;
            Ok(api(
                get_param(params, 0),
                get_param(params, 1),
                get_param(params, 2),
                get_param(params, 3),
                get_param(params, 4),
                get_param(params, 5),
                get_param(params, 6),
                get_param(params, 7),
                get_param(params, 8),
                get_param(params, 9),
                get_param(params, 10),
            ))
        }
        12 => {
            let api: fn(RP, RP, RP, RP, RP, RP, RP, RP, RP, RP, RP, RP) -> RP =
                unsafe { lib.symbol(name) }.map_err(|err| err.to_string())?;
            Ok(api(
                get_param(params, 0),
                get_param(params, 1),
                get_param(params, 2),
                get_param(params, 3),
                get_param(params, 4),
                get_param(params, 5),
                get_param(params, 6),
                get_param(params, 7),
                get_param(params, 8),
                get_param(params, 9),
                get_param(params, 10),
                get_param(params, 11),
            ))
        }
        _ => Err("Not supported params size".to_string()),
    }
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
        DataType::I32 => value.as_str().unwrap().parse::<i32>().unwrap() as RP,
        DataType::I64 => value.as_str().unwrap().parse::<i64>().unwrap() as RP,
    }
}

fn convert_return_value(raw: RP, data_type: &DataType) -> Value {
    match data_type {
        DataType::I32 => json!(raw as i32),
        DataType::I64 => json!((raw as i64).to_string()),
    }
}
