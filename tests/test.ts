import { assertEquals } from "https://deno.land/std@0.74.0/testing/asserts.ts";
import { ApiDefine, DataType, loadLibrary } from "../mod.ts";
import { pluginInit } from "./util.ts";

const libPath = await pluginInit();
interface LibApi {
  rust_fun_print_something(): void;
  rust_fun_add_one(num: number): number;
}

const apiDefine: ApiDefine[] = [
  {
    name: "rust_fun_print_something",
    type: "function",
  },
  {
    name: "rust_fun_add_one",
    type: "function",
    params: [DataType.i32],
    returnType: DataType.i32,
  },
];

const lib = await loadLibrary<LibApi>(libPath, apiDefine);

async function test(name: string, fn: (lib: LibApi) => void) {
  Deno.test(name, async () => {
    fn(lib.api);
  });
}

test("rust_fun_print_something", (lib) => {
  lib.rust_fun_print_something();
});

test("rust_fun_add_one", (lib) => {
  const value = lib.rust_fun_add_one(1);
  assertEquals(value, 2);
});
