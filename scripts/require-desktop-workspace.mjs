import { existsSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const scriptDir = dirname(fileURLToPath(import.meta.url));
const desktopPackagePath = join(scriptDir, "..", "apps", "desktop", "package.json");

if (!existsSync(desktopPackagePath)) {
  console.error(
    "Desktop workspace has not been created yet. Complete Foundation MVP Task 2 to create apps/desktop/package.json before running root dev, build, or tauri scripts.",
  );
  process.exit(1);
}
