# ChibishotsPrint

Windows (later macOS) native print companion for the Canon PIXMA PRO-100S, replacing Canon Print Studio Pro.

Features (MVP):
- Tauri + Rust desktop app skeleton (Windows first, macOS-compatible structure).
- Accept image files (TIFF/PNG/JPEG) via file picker and watched-folder handoff from Photoshop.
- Per-media presets: media type + Canon driver settings + ICC printer profile + rendering intent + margins.
- Application-managed color: transform source to printer ICC via lcms2, submit to spooler with driver color correction None.
- Basic layout: single image, crop/resize/rotate, paper size, margins, orientation.
- Print queue: submit to spooler, status, cancel, retry.
- Job history: date, media, profile, notes (no cost tracking).
- Photoshop UXP panel: export flattened image + metadata to watched folder / invoke app.

Out of scope for MVP (v2): nesting, templates, soft proof, batch queue, ColorMunki in-app profiling.

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