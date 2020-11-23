import { FFI_OPS } from "./types.ts";
import { dispatch, encodeString, initPlugin } from "./util.ts";

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

export async function loadLibrary<T = any>(file: string, define: ApiDefine[]) {
  await initPlugin();
  // const test = dispatch(FFI_OPS.DENO_FFI_OPEN, encodeString(file));
  // console.log(test);
  const apis: { [key: string]: any } = {};
  define.forEach((def) => {
    apis[def.name] = () => undefined;
  });
  return apis as T;
}
