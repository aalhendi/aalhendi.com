# aalhendi.com

Personal website built with plain HTML, CSS and JavaScript.

## Local development

Use the Node.js version in `.node-version`.

```sh
npm ci
npm run preview
```

`npm run check` validates local asset references and runs a deployment dry-run.

## Content

- `index.html`: homepage introduction.
- `about.html`: experience, selected work and resume downloads.
- `style.css`, `resume.css`, `theme.js`, `resume.js`: presentation and behaviour.
- `assets/` and `fonts/`: published images, PDFs and fonts.
- `tools/`: site packaging and PDF generation.

## Resume PDFs

`about.html` is the shared source for the website and both PDF editions.
Edit it, then regenerate the PDFs using Rust and an installed Chromium-based browser:

```sh
cargo run --quiet --manifest-path tools/resume-export/Cargo.toml -- all
```

The exporter produces a two-page resume and one-page highlights document in
`assets/`. Check the page counts, selectable text and visual layout before publishing.
Set `RESUME_BROWSER` to select a browser executable when needed.

## Deployment

The site uses Cloudflare Workers Static Assets.

```sh
npm run cf:login
npm run check
npm run deploy
```

`tools/build-site.mjs` copies an explicit public asset list into `dist/`.
Deployment publishes the current local files; it does not require a Git commit.
Build output, temporary files and local credentials are excluded from Git.
