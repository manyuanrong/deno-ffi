import { initPlugin } from "../mod.ts";
import { buildPlugin, buildTestLib } from "./build.ts";
import { exists } from "./test.deps.ts";

const { test } = Deno;

if (await exists(".deno_plugins")) {
  await Deno.remove(".deno_plugins", { recursive: true });
}
await buildTestLib();
const ops = await initPlugin("file://./target/release");

test("testClose", async () => {
  // const lib = loadLibrary("../")
  // assertEquals(result, { success: true });
});
