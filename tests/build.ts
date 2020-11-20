export async function buildTestLib() {
  const cargoCommand = Deno.run({
    cmd: ["cargo", "build", "--release", "--locked"],
    stderr: "inherit",
    stdin: "inherit",
    stdout: "inherit",
  });
  await cargoCommand.status();
}

export async function buildPlugin() {
  Deno.chdir("../");
  const cargoCommand = Deno.run({
    cmd: ["cargo", "build", "--release", "--locked"],
    stderr: "inherit",
    stdin: "inherit",
    stdout: "inherit",
  });
  await cargoCommand.status();
}
