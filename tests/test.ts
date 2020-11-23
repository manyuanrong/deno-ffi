import { initPlugin, loadLibrary } from "../mod.ts";
import { buildPlugin, buildTestLib } from "./build.ts";
import { exists } from "./test.deps.ts";

import { setPluginUrl } from "../ts/util.ts";

await buildPlugin();
await buildTestLib();

const { test } = Deno;
setPluginUrl("file://target/release");

if (await exists(".deno_plugins")) {
  await Deno.remove(".deno_plugins", { recursive: true });
}

test("testOpen", async () => {
  const lib = await loadLibrary("../", []);
  // assertEquals(result, { success: true });
});
