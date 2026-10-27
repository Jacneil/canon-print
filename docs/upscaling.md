# Image upscaling for print

## Goal
Upscale low-resolution source images before printing so they come out sharper on
Canon PIXMA PRO-100S media.

## Feasibility research

### ONNX Runtime in Rust
- The `ort` crate (ONNX Runtime bindings for Rust) supports Windows/macOS/Linux and
  can load `.onnx` models. Prebuilt runtime binaries can be downloaded by the crate
  (`download-binaries` feature) or provided by the system.
- Inference for a single image is CPU/GPU bound; expect seconds to tens of seconds
  for 2x-4x super-resolution on typical desktops.
- Cross-platform build requires the ONNX Runtime shared library to be shipped
  alongside the app or downloaded on first run.

### Model bundling / distribution
- RealPLKSR (and similar SR models) are distributed as `.onnx` weights (tens to
  hundreds of MB). Bundling them in the installer bloats download size.
- Recommended: ship a small default model or download on first use into the app data
  directory, with a progress indicator and offline fallback.
- Keep the model optional so the app works without it.

### Licensing
- Verify the license of any downloaded model before redistribution. Many SR models
  are research-only or non-commercial. Prefer permissively licensed (Apache-2.0 /
  MIT) models, or let the user supply their own weights.

## Implementation in this PR
- Built-in upscaling step in the print pipeline (`prepare_print_image` command).
- Uses high-quality Lanczos3 resampling + unsharp masking as the default
  "equivalent" super-resolution path so it works offline with no extra binaries.
- Optional ONNX/RealPLKSR path can be enabled later via a feature flag without
  changing the pipeline contract.
- Auto-upscales images whose smaller edge is below a configurable threshold
  (default 1200 px), plus a manual force toggle.
- Progress is reported through `upscale-progress` events and shown in the UI.

## Performance
- Lanczos3 + unsharp is fast (sub-second for typical photos) and memory-light.
- ONNX path should run on a background thread and report progress per tile.