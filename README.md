# ChibishotsPrint

Windows (later macOS) native print companion for the Canon PIXMA PRO-100S, replacing Canon Print Studio Pro.

Features (v2):
- Templates: A5 folded card 2-up with fold, A4 print, 6x4, canvas wrap margins, transfers with mirroring.
- Nesting: auto-pack multiple images onto a sheet with margins/gutters, rotate for fit.
- Batch queue with presets and drag-drop reordering.
- Soft-proof preview: simulate printer profile on screen using display profile.
- Built-in image upscaling (Lanczos3 + unsharp, auto for small images) for sharper prints, with progress UI.
- ColorMunki/X-Rite ICC profile import and validation.
- Profile picker by paper.
- Application-managed color path (lcms2) and driver settings per media.
- Tauri + Rust desktop app (Windows first, macOS-compatible structure).
- Accept image and PDF files (TIFF/PNG/JPEG/PDF) via file picker and watched-folder handoff from Photoshop; PDF pages are rendered to images for the print pipeline.
- Per-media presets: media type + Canon driver settings + ICC printer profile + rendering intent + margins.
- Print queue: submit to spooler, status, cancel, retry.
- Job history: date, media, profile, notes (no cost tracking).
- Photoshop UXP panel: export flattened image + metadata to watched folder / invoke app.

Cost tracking remains out of scope.

## Build instructions

Prerequisites:
- Rust (rustup) + cargo
- Node.js + npm
- Tauri v2 system dependencies (see https://tauri.app/start/prerequisites/)

Development:
```sh
npm install
cd src-tauri
cargo build
cd ..
npm run dev
```

Production build:
```sh
npm run build
```

UXP panel: load `uxp-panel/manifest.json` in Photoshop (UXP Developer Tool) for handoff.

## CI / Builds

GitHub Actions builds the Tauri app on every push/PR to `master` for Windows and macOS,
producing installable artifacts (`.msi`/`.exe` on Windows, `.dmg` on macOS) uploaded as
workflow artifacts. Tagged releases (`v*`) also create a GitHub Release with the installers.