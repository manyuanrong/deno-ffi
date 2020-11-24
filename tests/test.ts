import { assertEquals } from "https://deno.land/std@0.74.0/testing/asserts.ts";
import { ApiDefine, DataType, loadLibrary } from "../mod.ts";
import { pluginInit } from "./util.ts";

const libPath = await pluginInit();
interface LibApi {
  rust_fun_print_something(): void;
  rust_fun_add_one_i32(num: number): number;
}

const apiDefine: ApiDefine[] = [
  {
    name: "rust_fun_print_something",
    type: "function",
  },
  {
    name: "rust_fun_add_one_i32",
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

test("rust_fun_add_one_i32", (lib) => {
  // normal
  assertEquals(lib.rust_fun_add_one_i32(1), 2);
  assertEquals(lib.rust_fun_add_one_i32(2147483646), 2147483647);

  // overflow
  assertEquals(lib.rust_fun_add_one_i32(2147483647), -2147483648);
});

test("unload", () => {
  lib.unload();
});
