export async function buildTestLib() {
<<<<<<< HEAD
=======
  Deno.chdir("./tests");
>>>>>>> wip
  const cargoCommand = Deno.run({
    cmd: ["cargo", "build", "--release", "--locked"],
    stderr: "inherit",
    stdin: "inherit",
    stdout: "inherit",
  });
  await cargoCommand.status();
}

export async function buildPlugin() {
<<<<<<< HEAD
  Deno.chdir("../");
  const cargoCommand = Deno.run({
    cmd: ["cargo", "build", "--release", "--locked"],
=======
  const cargoCommand = Deno.run({
    cmd: ["cargo", "build", "--release", "--locked", "--explain", "E0658"],
>>>>>>> wip
    stderr: "inherit",
    stdin: "inherit",
    stdout: "inherit",
  });
  await cargoCommand.status();
}
