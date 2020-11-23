import { loadLibrary } from "../mod.ts";
import { DataType } from "../ts/lib.ts";
import { setPluginUrl } from "../ts/util.ts";
import { buildPlugin, buildTestLib } from "./build.ts";
import { exists } from "./test.deps.ts";

await buildPlugin();
await buildTestLib();

setPluginUrl("file://target/release");
let testLibPath = "";

if (Deno.build.os === "windows") {
  testLibPath = "./tests/target/release/tests.dll";
} else if (Deno.build.os === "linux") {
  testLibPath = "./tests/target/release/libtests.so";
} else {
  testLibPath = "./tests/target/release/libtests.dylib";
}

if (await exists(".deno_plugins")) {
  await Deno.remove(".deno_plugins", { recursive: true });
}

interface LibApi {
  rust_fun_print_something(): void;
  rust_fun_add_one(num: number): number;
}

const lib = await loadLibrary<LibApi>(testLibPath, [
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
]);

async function test(name: string, fn: (lib: LibApi) => void) {
  Deno.test(name, async () => {
    fn(lib.api);
  });
}

test("rust_fun_print_something", (lib) => {
  lib.rust_fun_print_something();
});

test("rust_fun_add_one", (lib) => {
  lib.rust_fun_add_one(1);
});
