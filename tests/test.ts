import { initPlugin } from "../mod.ts";
import { buildPlugin, buildTestLib } from "./build.ts";
import { exists } from "./test.deps.ts";

import { setPluginUrl } from "../ts/util.ts";

await buildPlugin();
await buildTestLib();

const { test } = Deno;
setPluginUrl("file://../target/release");
await initPlugin();

if (await exists(".deno_plugins")) {
  await Deno.remove(".deno_plugins", { recursive: true });
}
await buildTestLib();
const ops = await initPlugin("file://./target/release");

test("testClose", async () => {
  // const lib = loadLibrary("../")
  // assertEquals(result, { success: true });
});
