import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const rootDir = dirname(dirname(fileURLToPath(import.meta.url)));

function run(command, args) {
  const result =
    process.platform === "win32"
      ? spawnSync(process.env.ComSpec || "cmd.exe", [
          "/d",
          "/s",
          "/c",
          [command, ...args].join(" "),
        ], {
          cwd: rootDir,
          stdio: "inherit",
        })
      : spawnSync(command, args, {
          cwd: rootDir,
          stdio: "inherit",
        });

  if (result.error) {
    throw result.error;
  }
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

function runPowerShellScript(scriptPath) {
  const args = [
    "-NoProfile",
    "-ExecutionPolicy",
    "Bypass",
    "-File",
    scriptPath,
  ];

  const pwsh = spawnSync("pwsh", args, {
    cwd: rootDir,
    stdio: "inherit",
  });

  if (!pwsh.error && pwsh.status === 0) {
    return;
  }
  if (!pwsh.error) {
    process.exit(pwsh.status ?? 1);
  }
  if (pwsh.error.code !== "ENOENT") {
    throw pwsh.error;
  }

  const windowsPowerShell = spawnSync("powershell", args, {
    cwd: rootDir,
    stdio: "inherit",
  });

  if (windowsPowerShell.error) {
    throw windowsPowerShell.error;
  }
  if (windowsPowerShell.status !== 0) {
    process.exit(windowsPowerShell.status ?? 1);
  }
}

run("pnpm", ["run", "build"]);

if (process.platform === "win32") {
  const setupScript = join(rootDir, "scripts", "setup-parakeet-cpp.ps1");
  if (!existsSync(setupScript)) {
    throw new Error(`Missing setup script: ${setupScript}`);
  }
  runPowerShellScript(setupScript);
} else {
  console.log(
    "Parakeet sidecar setup for this platform is handled by bootstrap or CI."
  );
}
