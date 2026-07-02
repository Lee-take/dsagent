import { existsSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const scriptDir = dirname(fileURLToPath(import.meta.url));
const desktopPackagePath = join(scriptDir, "..", "apps", "desktop", "package.json");

if (!existsSync(desktopPackagePath)) {
  console.error(
    "Desktop workspace is missing from this source checkout. Restore apps/desktop/package.json before running root dev, build, or tauri scripts.",
  );
  process.exit(1);
}
