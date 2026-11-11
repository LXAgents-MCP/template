import { readFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const PACKAGE_JSON = resolve(
  dirname(fileURLToPath(import.meta.url)),
  "..",
  "..",
  "package.json"
);

/** The one field of `package.json` this module reads. */
interface PackageManifest {
  version?: string;
}

let resolved = "0.0.0";
try {
  const manifest = JSON.parse(await readFile(PACKAGE_JSON, "utf8")) as PackageManifest;
  resolved = manifest.version ?? resolved;
} catch {
  // Keep the bundled server usable if package.json cannot be read.
}

export const version: string = resolved;
