import { FFI_OPS } from "./types.ts";
import { dispatch, encodeString, initPlugin } from "./util.ts";

const encoder = new TextEncoder();
const decoder = new TextDecoder();

interface ApiDefine {
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
            value: params[index],
          };
        }),
        return_type: define.returnType,
      }),
    ),
  );
  const json = JSON.parse(decoder.decode(buffer));
  console.log(json);
}

export async function loadLibrary<T = any>(file: string, define: ApiDefine[]) {
  await initPlugin();
  const buf = dispatch(FFI_OPS.DENO_FFI_OPEN, encodeString(file))!;
  const view = new DataView(buf.buffer);
  const id = view.getUint32(0, true);

  const apis: { [key: string]: any } = {};
  define.forEach((def) => {
    apis[def.name] = (...args: any[]) => {
      call(id, def, args);
    };
  });
  return {
    api: apis as T,
    close() {},
  };
}
