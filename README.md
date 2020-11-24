# deno-ffi
Deno Foreign Function Interface

[![tag](https://img.shields.io/github/tag/manyuanrong/deno-ffi.svg)](https://github.com/manyuanrong/deno-ffi/releases)
[![Build Status](https://github.com/manyuanrong/deno-ffi/workflows/ci/badge.svg?branch=main)](https://github.com/manyuanrong/deno-ffi/actions)
[![license](https://img.shields.io/github/license/manyuanrong/deno-ffi.svg)](https://github.com/manyuanrong/deno-ffi)
[![downloads](https://img.shields.io/github/downloads/manyuanrong/deno-ffi/total)](https://github.com/manyuanrong/deno-ffi)
[![tag](https://img.shields.io/badge/deno-v1.5.2-green.svg)](https://github.com/denoland/deno)

deno-ffi is a Deno plugin for loading and calling dynamic libraries using pure TypeScript. It can be used to create bindings to native libraries without writing any C++ code.

It also simplifies the augmentation of Deno with C code as it takes care of handling the translation of types across TypeScript and C, which can add reams of boilerplate code to your otherwise simple C. See the `./tests` for an example of this use case.

## Example

```ts
import { ApiDefine, DataType, loadLibrary } from "https://deno.land/x/ffi@v0.1.0/mod.ts";

const libPath = "test.dylib";

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

lib.rust_fun_print_something();

const value = lib.rust_fun_add_one(1);
console.log(value);
```