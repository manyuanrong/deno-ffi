import { initPlugin, loadLibrary } from "../mod.ts";
import { buildPlugin, buildTestLib } from "./build.ts";
import { exists } from "./test.deps.ts";

import { closePlugin, setPluginUrl } from "../ts/util.ts";

await buildPlugin();
await buildTestLib();

const { test } = Deno;
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

test("testOpen", async () => {
  const lib = await loadLibrary<{
    rust_fun_print_something(): void;
  }>(testLibPath, [
    {
      name: "rust_fun_print_something",
      type: "function",
    },
  ]);

  lib.api.rust_fun_print_something();

  closePlugin();
});
