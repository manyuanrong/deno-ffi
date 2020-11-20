import { DenoCore, ffiOps, initPlugin } from "./util.ts";

interface ApiDefine {
  name: string;
  type: "function" | "variable";
  params?: DataType[];
  returnType?: DataType;
}

enum DataType {
  INT = "INT",
  DOUBLE = "DOUBLE",
  C_STRING = "C_STRING",
}

export async function loadLibrary<T = any>(file: string, define: ApiDefine[]) {
  const test = DenoCore.dispatch(ffiOps.DENO_FFI_OPEN, file);
  console.log(test);
  const apis: { [key: string]: any } = {};
  define.forEach((def) => {
    apis[def.name] = () => undefined;
  });
  return apis as T;
}

const lib = await loadLibrary<{
  add(num1: number): number;
}>("test", [
  {
    name: "add",
    type: "function",
    params: [DataType.INT],
  },
]);

lib.add(3);
