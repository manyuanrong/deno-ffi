import { convertValue, parseValue } from "./convert.ts";
import { FFI_OPS } from "./types.ts";
import { dispatch, initPlugin } from "./util.ts";

const encoder = new TextEncoder();
const decoder = new TextDecoder();

export interface ApiDefine {
  name: string;
  type: "function" | "variable";
  params?: DataType[];
  returnType?: DataType;
}

export enum DataType {
  i32 = "I32",
  i64 = "I64",
}

function call(id: number, define: ApiDefine, params: any[]) {
  const buffer = dispatch(
    FFI_OPS.DENO_FFI_CALL,
    encoder.encode(
      JSON.stringify({
        id,
        name: define.name,
        params: define.params?.map((type, index) => {
          return {
            data_type: type,
            value: convertValue(params[index], type),
          };
        }) ?? [],
        return_type: define.returnType,
      }),
    ),
  )!;
  const { error, value } = JSON.parse(decoder.decode(buffer));
  if (error) {
    throw new Error(error);
  }
  return parseValue(value, define.returnType);
}

function unload(id: number) {
  dispatch(
    FFI_OPS.DENO_FFI_UNLOAD,
    encoder.encode(JSON.stringify(id)),
  );
}

export async function loadLibrary<T = any>(file: string, define: ApiDefine[]) {
  await initPlugin();
  const buf = dispatch(FFI_OPS.DENO_FFI_OPEN, encoder.encode(file))!;
  const view = new DataView(buf.buffer);
  const id = view.getUint32(0, true);

  const apis: { [key: string]: any } = {};
  define.forEach((def) => {
    apis[def.name] = (...args: any[]) => {
      return call(id, def, args);
    };
  });
  return {
    api: apis as T,
    unload: () => unload(id),
  };
}
