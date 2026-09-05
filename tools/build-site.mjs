import { copyFile, lstat, mkdir, readFile, readdir, realpath, rm } from "node:fs/promises";
import { dirname, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";

const root = await realpath(fileURLToPath(new URL("../", import.meta.url)));
const output = resolve(root, "dist");

// Explicit publication list: tools, temporary files and repository
// metadata never enter the upload, even if they are present or untracked locally.
const files = [
  "index.html", "about.html", "404.html",
  "style.css", "resume.css", "theme.js", "resume.js",
  "favicon.ico", "robots.txt", "_headers", "_redirects",
  "assets/logo.png",
  "assets/aalhendi_cv.pdf",
  "assets/aalhendi_highlights.pdf",
  "fonts/Roboto-Regular.ttf", "fonts/Roboto-Bold.ttf", "fonts/Roboto-Italic.ttf",
  "fonts/LICENSE.txt",
];

// Verify all sources before touching the previous output.
for (const file of files) {
  const source = resolve(root, file);
  const actual = await realpath(source);
  if (!actual.startsWith(root + sep) || !(await lstat(source)).isFile()) {
    throw new Error(`Public asset must be a regular file inside the project: ${file}`);
  }
}

// Refuse redirects/junctions before removing this fixed, project-local directory.
if (output !== root + sep + "dist") throw new Error("Unsafe output directory");
try {
  const info = await lstat(output);
  if (info.isSymbolicLink() || !info.isDirectory() || await realpath(output) !== output) {
    throw new Error("dist must be a real directory inside this project");
  }
} catch (error) {
  if (error.code !== "ENOENT") throw error;
}
await mkdir(output, { recursive: true });
// Keep the directory itself: Wrangler watches it during local preview on Windows.
for (const entry of await readdir(output)) {
  const stale = resolve(output, entry);
  if (dirname(stale) !== output) throw new Error("Unsafe output entry");
  await rm(stale, { recursive: true, force: true });
}
for (const file of files) {
  const destination = resolve(output, file);
  await mkdir(dirname(destination), { recursive: true });
  await copyFile(resolve(root, file), destination);
}

// Catch missing local page, stylesheet, script, font and PDF references.
const published = new Set(files);
for (const file of files.filter(name => /\.(html|css|js)$/.test(name))) {
  const content = await readFile(resolve(output, file), "utf8");
  const references = [
    ...content.matchAll(/(?:href|src)=["']([^"']+)["']/g),
    ...content.matchAll(/url\(["']?([^\s"')]+)["']?\)/g),
    ...content.matchAll(/["'](assets\/[^"']+\.pdf)["']/g),
  ];
  for (const [, reference] of references) {
    const url = new URL(reference, `https://site.invalid/${file}`);
    if (url.origin !== "https://site.invalid") continue;
    const target = decodeURIComponent(url.pathname).slice(1) || "index.html";
    if (!published.has(target)) throw new Error(`${file} links to unpublished file: ${reference}`);
  }
}
console.log(`Prepared and checked ${files.length} public files in dist/`);
