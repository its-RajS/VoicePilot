import { run, codex } from "@ai-hero/sandcastle/dist/index.js";
import { docker } from "@ai-hero/sandcastle/dist/sandboxes/docker.js";

function main() {
  return run({
    agent: codex("gpt-5"),
    sandbox: docker(),
    promptFile: "./.sandcastle/prompt.md",
  });
}

main().catch((error: Error) => {
  console.error(error);
  process.exitCode = 1;
});
