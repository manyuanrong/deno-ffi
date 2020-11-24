import { setPluginUrl } from "../mod.ts";
import { exists } from "./test.deps.ts";

export async function buildTestLib() {
  const cwd = Deno.cwd();
  Deno.chdir("./tests");
  const cargoCommand = Deno.run({
    cmd: ["cargo", "build", "--release", "--locked"],
    stderr: "inherit",
    stdin: "inherit",
    stdout: "inherit",
  });
  await cargoCommand.status();
  Deno.chdir(cwd);
}

export async function buildPlugin() {
  const cargoCommand = Deno.run({
    cmd: ["cargo", "build", "--release", "--locked"],
    stderr: "inherit",
    stdin: "inherit",
    stdout: "inherit",
  });
  await cargoCommand.status();
}

export async function pluginInit() {
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

  return testLibPath;
}
