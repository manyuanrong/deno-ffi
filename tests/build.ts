export async function buildTestLib() {
  Deno.chdir("./tests");
  const cargoCommand = Deno.run({
    cmd: ["cargo", "build", "--release", "--locked"],
    stderr: "inherit",
    stdin: "inherit",
    stdout: "inherit",
  });
  await cargoCommand.status();
}

export async function buildPlugin() {
  const cargoCommand = Deno.run({
    cmd: ["cargo", "build", "--release", "--locked", "--explain", "E0658"],
    stderr: "inherit",
    stdin: "inherit",
    stdout: "inherit",
  });
  await cargoCommand.status();
}
