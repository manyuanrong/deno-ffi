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

enum DataType {
  INT = "INT",
  C_INT = "INT",
  DOUBLE = "DOUBLE",
  C_STRING = "C_STRING",
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
  console.log(buffer);
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
