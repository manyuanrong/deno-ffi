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
