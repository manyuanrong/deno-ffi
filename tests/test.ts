import { assertEquals } from "https://deno.land/std@0.74.0/testing/asserts.ts";
import { ApiDefine, DataType, loadLibrary } from "../mod.ts";
import { pluginInit } from "./util.ts";

const libPath = await pluginInit();
interface LibApi {
  rust_fun_print_something(): void;
  rust_fun_add_one_i32(num: number): number;
  rust_fun_add_one_i64(num: bigint): bigint;
  rust_fun_add_all_12_i32(
    num1: number,
    num2: number,
    num3: number,
    num4: number,
    num5: number,
    num6: number,
    num7: number,
    num8: number,
    num9: number,
    num10: number,
    num11: number,
    num12: number
  ): number;
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
  {
    name: "rust_fun_add_one_i64",
    type: "function",
    params: [DataType.i64],
    returnType: DataType.i64,
  },
  {
    name: "rust_fun_add_all_12_i32",
    type: "function",
    params: [
      DataType.i32,
      DataType.i32,
      DataType.i32,
      DataType.i32,
      DataType.i32,
      DataType.i32,
      DataType.i32,
      DataType.i32,
      DataType.i32,
      DataType.i32,
      DataType.i32,
      DataType.i32
    ],
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

test("rust_fun_add_one_i64", (lib) => {
  // normal
  assertEquals(lib.rust_fun_add_one_i64(1n), 2n);
  assertEquals(lib.rust_fun_add_one_i64(2147483648n), 2147483649n);
  assertEquals(
    lib.rust_fun_add_one_i64(-9223372036854775808n),
    -9223372036854775807n,
  );

  // overflow
  assertEquals(
    lib.rust_fun_add_one_i64(9223372036854775807n),
    -9223372036854775808n,
  );
});

test("rust_fun_add_12_inputs_i32", (lib) => {
  // normal
  assertEquals(lib.rust_fun_add_all_12_i32(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), 78);
});

test("unload", () => {
  lib.unload();
});
