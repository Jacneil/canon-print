# UI Mockups — Canon Print MVP

Static HTML/CSS wireframes for the MVP screens. Open `docs/mockups/index.html` in a browser to navigate.

## Screens

1. **Main print view** (`01-main-print.html`) — single image preview, paper size, orientation, margins, scale, rendering intent, notes, print action.
2. **Per-media preset panel** (`02-presets.html`) — saved presets list + editor: media type, Canon driver settings, ICC profile, source profile handling, rendering intent, BPC, margin defaults.
3. **Image crop / resize / rotate** (`03-crop-resize.html`) — crop handles, rotation, flip, dimensions, resample method, output DPI.
4. **Print queue & history** (`04-queue-history.html`) — active spooler jobs with status/cancel/retry; job history (date, media, profile, notes; no cost).
5. **Photoshop UXP handoff** (`05-photoshop-handoff.html`) — watched folder, incoming jobs, sidecar metadata, create print job.

## Notes

- Application-managed color: driver color correction forced to "None"; ICC transform via lcms2 in Rust core.
- Cost tracking explicitly out of scope.
- These are wireframes only; awaiting captain review before full implementation.