use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;
use tauri::State;

#[derive(Clone, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub media_type: String,
    pub quality: String,
    pub icc_profile: String,
    pub rendering_intent: String,
    pub bpc: bool,
    pub margin_top: f32,
    pub margin_bottom: f32,
    pub margin_left: f32,
    pub margin_right: f32,
    #[serde(default)]
    pub driver_settings: HashMap<String, String>,
    #[serde(default)]
    pub paper: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PrintJob {
    pub id: String,
    pub file: String,
    pub preset: String,
    pub status: String,
    pub submitted_at: String,
    pub notes: String,
    #[serde(default)]
    pub batch_id: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub page_width_mm: f32,
    pub page_height_mm: f32,
    pub margin_mm: f32,
    pub gutter_mm: f32,
    pub fold: bool,
    pub mirror: bool,
    pub wrap_mm: f32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PlacedImage {
    pub file: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub rotated: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Layout {
    pub template: String,
    pub page_width_mm: f32,
    pub page_height_mm: f32,
    pub placements: Vec<PlacedImage>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct IccProfile {
    pub name: String,
    pub path: String,
    pub valid: bool,
    pub color_space: String,
    pub device_class: String,
    pub version: u32,
}

#[derive(Default)]
pub struct AppState {
    pub presets: Mutex<Vec<Preset>>,
    pub queue: Mutex<VecDeque<PrintJob>>,
    pub history: Mutex<Vec<PrintJob>>,
    pub profiles: Mutex<Vec<IccProfile>>,
    pub paper_profile: Mutex<HashMap<String, String>>,
    pub templates: Mutex<Vec<Template>>,
}

#[tauri::command]
fn greet() -> &'static str {
    "ChibishotsPrint ready"
}

#[tauri::command]
fn default_presets() -> Vec<Preset> {
    vec![
        Preset {
            name: "Luster 13x19".into(),
            media_type: "Canon Luster".into(),
            quality: "Highest".into(),
            icc_profile: "Canon_PRO100S_Luster.icc".into(),
            rendering_intent: "Perceptual".into(),
            bpc: true,
            margin_top: 5.0,
            margin_bottom: 5.0,
            margin_left: 5.0,
            margin_right: 5.0,
            driver_settings: HashMap::new(),
            paper: "13x19".into(),
        },
        Preset {
            name: "Glossy 8x10".into(),
            media_type: "Canon Glossy II".into(),
            quality: "High".into(),
            icc_profile: "Canon_PRO100S_Glossy.icc".into(),
            rendering_intent: "Relative Colorimetric".into(),
            bpc: true,
            margin_top: 3.0,
            margin_bottom: 3.0,
            margin_left: 3.0,
            margin_right: 3.0,
            driver_settings: HashMap::new(),
            paper: "8x10".into(),
        },
    ]
}

#[tauri::command]
fn default_templates() -> Vec<Template> {
    vec![
        Template { id:"a5_folded_2up".into(), name:"A5 folded card 2-up".into(), kind:"a5_folded_2up".into(), page_width_mm:210.0, page_height_mm:297.0, margin_mm:5.0, gutter_mm:3.0, fold:true, mirror:false, wrap_mm:0.0 },
        Template { id:"a4_print".into(), name:"A4 print".into(), kind:"a4".into(), page_width_mm:210.0, page_height_mm:297.0, margin_mm:10.0, gutter_mm:0.0, fold:false, mirror:false, wrap_mm:0.0 },
        Template { id:"6x4".into(), name:"6x4 photo".into(), kind:"6x4".into(), page_width_mm:152.4, page_height_mm:101.6, margin_mm:3.0, gutter_mm:0.0, fold:false, mirror:false, wrap_mm:0.0 },
        Template { id:"canvas_wrap".into(), name:"Canvas wrap".into(), kind:"canvas_wrap".into(), page_width_mm:400.0, page_height_mm:300.0, margin_mm:0.0, gutter_mm:0.0, fold:false, mirror:false, wrap_mm:25.0 },
        Template { id:"transfer".into(), name:"Transfer (mirrored)".into(), kind:"transfer".into(), page_width_mm:210.0, page_height_mm:297.0, margin_mm:10.0, gutter_mm:0.0, fold:false, mirror:true, wrap_mm:0.0 },
    ]
}

#[tauri::command]
fn list_presets(state: State<AppState>) -> Vec<Preset> {
    let p = state.presets.lock().unwrap();
    if p.is_empty() { default_presets() } else { p.clone() }
}

#[tauri::command]
fn save_preset(state: State<AppState>, preset: Preset) -> Result<(), String> {
    let mut p = state.presets.lock().map_err(|e| e.to_string())?;
    if let Some(i) = p.iter().position(|x| x.name == preset.name) { p[i] = preset; }
    else { p.push(preset); }
    Ok(())
}

#[tauri::command]
fn list_templates(state: State<AppState>) -> Vec<Template> {
    let t = state.templates.lock().unwrap();
    if t.is_empty() { default_templates() } else { t.clone() }
}

#[tauri::command]
fn save_template(state: State<AppState>, template: Template) -> Result<(), String> {
    let mut t = state.templates.lock().map_err(|e| e.to_string())?;
    if let Some(i) = t.iter().position(|x| x.id == template.id) { t[i] = template; }
    else { t.push(template); }
    Ok(())
}

#[tauri::command]
fn submit_print(state: State<AppState>, file: String, preset: String, notes: String) -> PrintJob {
    let job = PrintJob { id: uuid_simple(), file, preset, status: "Queued".into(), submitted_at: now(), notes, batch_id: None };
    let mut q = state.queue.lock().unwrap();
    q.push_back(job.clone());
    job
}

#[tauri::command]
fn submit_batch(state: State<AppState>, files: Vec<String>, preset: String, notes: String) -> Vec<PrintJob> {
    let batch_id = uuid_simple();
    let mut out = Vec::new();
    let mut q = state.queue.lock().unwrap();
    for f in files {
        let job = PrintJob { id: uuid_simple(), file: f, preset: preset.clone(), status: "Queued".into(), submitted_at: now(), notes: notes.clone(), batch_id: Some(batch_id.clone()) };
        q.push_back(job.clone());
        out.push(job);
    }
    out
}

#[tauri::command]
fn queue_status(state: State<AppState>) -> Vec<PrintJob> {
    state.queue.lock().unwrap().iter().cloned().collect()
}

#[tauri::command]
fn cancel_job(state: State<AppState>, id: String) -> Result<(), String> {
    let mut q = state.queue.lock().map_err(|e| e.to_string())?;
    if let Some(j) = q.iter_mut().find(|j| j.id == id) { j.status = "Cancelled".into(); Ok(()) }
    else { Err("job not found".into()) }
}

#[tauri::command]
fn retry_job(state: State<AppState>, id: String) -> Result<PrintJob, String> {
    let mut q = state.queue.lock().map_err(|e| e.to_string())?;
    let job = q.iter().find(|j| j.id == id).cloned().ok_or("job not found")?;
    let mut newjob = job.clone();
    newjob.id = uuid_simple();
    newjob.status = "Queued".into();
    newjob.submitted_at = now();
    q.push_back(newjob.clone());
    Ok(newjob)
}

#[tauri::command]
fn reorder_queue(state: State<AppState>, order: Vec<String>) -> Result<(), String> {
    let mut q = state.queue.lock().map_err(|e| e.to_string())?;
    let mut newq = VecDeque::new();
    for id in &order {
        if let Some(j) = q.iter().find(|j| j.id == *id).cloned() { newq.push_back(j); }
    }
    // append any missing
    for j in q.iter() {
        if !order.contains(&j.id) { newq.push_back(j.clone()); }
    }
    *q = newq;
    Ok(())
}

#[tauri::command]
fn mark_done(state: State<AppState>, id: String) -> Result<(), String> {
    let mut q = state.queue.lock().map_err(|e| e.to_string())?;
    if let Some(pos) = q.iter().position(|j| j.id == id) {
        let mut j = q.remove(pos).unwrap();
        j.status = "Done".into();
        let mut h = state.history.lock().map_err(|e| e.to_string())?;
        h.push(j);
    }
    Ok(())
}

#[tauri::command]
fn job_history(state: State<AppState>) -> Vec<PrintJob> {
    state.history.lock().unwrap().clone()
}

#[tauri::command]
fn nest_images(files: Vec<String>, sheet_w: f32, sheet_h: f32, margin: f32, gutter: f32, allow_rotate: bool) -> Layout {
    // simple shelf packing
    let mut placements = Vec::new();
    let mut x = margin;
    let mut y = margin;
    let mut row_h: f32 = 0.0;
    for f in files {
        // assume image 100x100 mm placeholder? We don't read dims; use 50x50 default
        let (mut w, mut h) = (50.0_f32, 50.0_f32);
        let rotated = if allow_rotate && w > h && (x + h + gutter <= sheet_w - margin) { true } else { false };
        if rotated { std::mem::swap(&mut w, &mut h); }
        if x + w > sheet_w - margin {
            x = margin;
            y += row_h + gutter;
            row_h = 0.0;
        }
        if y + h > sheet_h - margin { break; }
        placements.push(PlacedImage { file: f, x, y, w, h, rotated });
        x += w + gutter;
        row_h = row_h.max(h);
    }
    Layout { template: "nested".into(), page_width_mm: sheet_w, page_height_mm: sheet_h, placements }
}

#[tauri::command]
fn transform_image(path: String, out_path: String, width: u32, height: u32, rotate: i32) -> Result<String, String> {
    let img = image::open(&path).map_err(|e| e.to_string())?;
    let mut img = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
    if rotate != 0 { img = img.rotate90(); }
    img.save(&out_path).map_err(|e| e.to_string())?;
    Ok(out_path)
}

#[tauri::command]
fn apply_template(template_id: String, files: Vec<String>) -> Layout {
    let tpls = default_templates();
    let t = tpls.iter().find(|t| t.id == template_id).cloned().unwrap_or_else(|| tpls[0].clone());
    // For 2-up folded card, place up to 2 images side by side
    let mut placements = Vec::new();
    let usable_w = t.page_width_mm - 2.0*t.margin_mm;
    let usable_h = t.page_height_mm - 2.0*t.margin_mm;
    let n = files.len().max(1) as f32;
    let cols = if t.kind == "a5_folded_2up" { 2.0 } else { 1.0 };
    let cell_w = usable_w / cols;
    for (i, f) in files.iter().enumerate() {
        let col = (i % cols as usize) as f32;
        let row = (i / cols as usize) as f32;
        let x = t.margin_mm + col*(cell_w + t.gutter_mm);
        let y = t.margin_mm + row*(usable_h/((n/cols).ceil().max(1.0)) + t.gutter_mm);
        placements.push(PlacedImage { file: f.clone(), x, y, w: cell_w - t.gutter_mm, h: usable_h/((n/cols).ceil().max(1.0)) - t.gutter_mm, rotated: false });
    }
    Layout { template: template_id, page_width_mm: t.page_width_mm, page_height_mm: t.page_height_mm, placements }
}

#[tauri::command]
fn import_profile(path: String) -> Result<IccProfile, String> {
    let p = lcms2::Profile::new_file(&path).map_err(|e| e.to_string())?;
    let prof = IccProfile {
        name: std::path::Path::new(&path).file_name().unwrap_or_default().to_string_lossy().to_string(),
        path: path.clone(),
        valid: true,
        color_space: format!("{:?}", p.color_space()),
        device_class: format!("{:?}", p.device_class()),
        version: p.encoded_icc_version(),
    };
    Ok(prof)
}

#[tauri::command]
fn validate_profile(path: String) -> bool {
    lcms2::Profile::new_file(&path).is_ok()
}

#[tauri::command]
fn list_profiles(state: State<AppState>) -> Vec<IccProfile> {
    state.profiles.lock().unwrap().clone()
}

#[tauri::command]
fn save_profile(state: State<AppState>, profile: IccProfile) -> Result<(), String> {
    let mut ps = state.profiles.lock().map_err(|e| e.to_string())?;
    if let Some(i) = ps.iter().position(|x| x.path == profile.path) { ps[i] = profile; } else { ps.push(profile); }
    Ok(())
}

#[tauri::command]
fn profile_for_paper(state: State<AppState>, paper: String) -> Option<String> {
    let m = state.paper_profile.lock().unwrap();
    m.get(&paper).cloned()
}

#[tauri::command]
fn set_profile_for_paper(state: State<AppState>, paper: String, profile_path: String) -> Result<(), String> {
    let mut m = state.paper_profile.lock().map_err(|e| e.to_string())?;
    m.insert(paper, profile_path);
    Ok(())
}

fn parse_intent(s: &str) -> lcms2::Intent {
    match s.to_lowercase().as_str() {
        "relative colorimetric" | "relative" => lcms2::Intent::RelativeColorimetric,
        "saturation" => lcms2::Intent::Saturation,
        "absolute colorimetric" | "absolute" => lcms2::Intent::AbsoluteColorimetric,
        _ => lcms2::Intent::Perceptual,
    }
}

#[tauri::command]
fn color_transform(input_path: String, output_path: String, src_profile_path: Option<String>, dst_profile_path: String, intent: String) -> Result<String, String> {
    let img = image::open(&input_path).map_err(|e| e.to_string())?.to_rgb8();
    let src = if let Some(p) = src_profile_path { lcms2::Profile::new_file(p).map_err(|e| e.to_string())? } else { lcms2::Profile::new_srgb() };
    let dst = lcms2::Profile::new_file(&dst_profile_path).map_err(|e| e.to_string())?;
    let t = lcms2::Transform::new(&src, lcms2::PixelFormat::RGB_8, &dst, lcms2::PixelFormat::RGB_8, parse_intent(&intent)).map_err(|e| e.to_string())?;
    let mut out = img.clone();
    let src_pixels: Vec<[u8;3]> = img.pixels().map(|p| [p[0],p[1],p[2]]).collect();
    let mut dst_pixels = vec![[0u8;3]; src_pixels.len()];
    t.transform_pixels(&src_pixels, &mut dst_pixels);
    for (i, px) in out.pixels_mut().enumerate() {
        px[0] = dst_pixels[i][0];
        px[1] = dst_pixels[i][1];
        px[2] = dst_pixels[i][2];
    }
    out.save(&output_path).map_err(|e| e.to_string())?;
    Ok(output_path)
}

#[tauri::command]
fn soft_proof(input_path: String, output_path: String, printer_profile: String, display_profile: String, intent: String) -> Result<String, String> {
    let img = image::open(&input_path).map_err(|e| e.to_string())?.to_rgb8();
    let src = lcms2::Profile::new_srgb();
    let dst = lcms2::Profile::new_file(&display_profile).map_err(|e| e.to_string())?;
    let proof = lcms2::Profile::new_file(&printer_profile).map_err(|e| e.to_string())?;
    let t = lcms2::Transform::new_proofing(&src, lcms2::PixelFormat::RGB_8, &dst, lcms2::PixelFormat::RGB_8, &proof, parse_intent(&intent), parse_intent(&intent), lcms2::Flags::SOFT_PROOFING).map_err(|e| e.to_string())?;
    let mut out = img.clone();
    let src_pixels: Vec<[u8;3]> = img.pixels().map(|p| [p[0],p[1],p[2]]).collect();
    let mut dst_pixels = vec![[0u8;3]; src_pixels.len()];
    t.transform_pixels(&src_pixels, &mut dst_pixels);
    for (i, px) in out.pixels_mut().enumerate() {
        px[0] = dst_pixels[i][0];
        px[1] = dst_pixels[i][1];
        px[2] = dst_pixels[i][2];
    }
    out.save(&output_path).map_err(|e| e.to_string())?;
    Ok(output_path)
}

const UPSCALE_MODEL_URL: &str = "https://huggingface.co/notaneimu/onnx-image-models/resolve/main/4xPurePhoto-RealPLSKR.onnx";
const UPSCALE_MODEL_FILE: &str = "4xPurePhoto-RealPLSKR.onnx";

#[derive(Clone, Serialize, Deserialize)]
pub struct UpscaleModelStatus {
    pub available: bool,
    pub downloaded: bool,
    pub path: Option<String>,
    pub url: String,
}

fn upscale_model_dir(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    use tauri::Manager;
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?.join("models");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

fn upscale_model_path(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    Ok(upscale_model_dir(app)?.join(UPSCALE_MODEL_FILE))
}

#[tauri::command]
fn upscale_model_status(app: tauri::AppHandle) -> UpscaleModelStatus {
    let path = upscale_model_path(&app).ok();
    let exists = path.as_ref().map(|p| p.exists()).unwrap_or(false);
    UpscaleModelStatus {
        available: exists,
        downloaded: exists,
        path: path.map(|p| p.to_string_lossy().to_string()),
        url: UPSCALE_MODEL_URL.into(),
    }
}

#[tauri::command]
async fn download_upscale_model(app: tauri::AppHandle) -> Result<String, String> {
    use futures_util::StreamExt;
    use tauri::Emitter;
    let path = upscale_model_path(&app)?;
    if path.exists() {
        return Ok(path.to_string_lossy().to_string());
    }
    app.emit("upscale-model-progress", serde_json::json!({"stage":"connecting","progress":0.0})).ok();
    let resp = reqwest::get(UPSCALE_MODEL_URL).await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("model download failed: {}", resp.status()));
    }
    let total = resp.content_length().unwrap_or(0);
    let mut file = std::fs::File::create(&path).map_err(|e| e.to_string())?;
    let mut downloaded: u64 = 0;
    let mut stream = resp.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        use std::io::Write;
        file.write_all(&chunk).map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;
        let progress = if total > 0 { downloaded as f64 / total as f64 } else { 0.0 };
        app.emit("upscale-model-progress", serde_json::json!({"stage":"downloading","progress":progress,"downloaded":downloaded,"total":total})).ok();
    }
    use std::io::Write;
    file.flush().map_err(|e| e.to_string())?;
    app.emit("upscale-model-progress", serde_json::json!({"stage":"done","progress":1.0,"path":path})).ok();
    Ok(path.to_string_lossy().to_string())
}

fn run_onnx_upscale(app: tauri::AppHandle, input_path: &str, out_path: &str, scale: u32) -> Result<String, String> {
    use ort::session::Session;
    use ort::value::Tensor;
    use std::collections::HashMap;

    let model_path = upscale_model_path(&app)?;
    if !model_path.exists() {
        return Err("model missing".into());
    }

    let img = image::open(input_path).map_err(|e| e.to_string())?.to_rgb8();
    let (w, h) = (img.width() as usize, img.height() as usize);
    let mut input = vec![0.0f32; 3 * w * h];
    for y in 0..h {
        for x in 0..w {
            let p = img.get_pixel(x as u32, y as u32);
            input[0 * w * h + y * w + x] = p[0] as f32 / 255.0;
            input[1 * w * h + y * w + x] = p[1] as f32 / 255.0;
            input[2 * w * h + y * w + x] = p[2] as f32 / 255.0;
        }
    }

    let mut session = Session::builder().map_err(|e| e.to_string())?
        .with_optimization_level(ort::session::builder::GraphOptimizationLevel::Level3).map_err(|e| e.to_string())?
        .commit_from_file(&model_path).map_err(|e| e.to_string())?;

    let input_tensor = Tensor::from_array(([1i64, 3, h as i64, w as i64], input)).map_err(|e| e.to_string())?;

    let input_name = session.inputs().first().map(|i| i.name.clone()).unwrap_or_else(|| "input".into());
    let outputs = session.run(ort::inputs![input_name.as_str() => input_tensor]).map_err(|e| e.to_string())?;

    let out_val = outputs.iter().next().map(|(_, v)| v).unwrap();
    let (shape, data) = out_val.try_extract_tensor::<f32>().map_err(|e| e.to_string())?;
    let out_h = shape[2] as usize;
    let out_w = shape[3] as usize;

    let mut out_img = image::RgbImage::new(out_w as u32, out_h as u32);
    for y in 0..out_h {
        for x in 0..out_w {
            let r = (data[0 * out_h * out_w + y * out_w + x] * 255.0).clamp(0.0, 255.0) as u8;
            let g = (data[1 * out_h * out_w + y * out_w + x] * 255.0).clamp(0.0, 255.0) as u8;
            let b = (data[2 * out_h * out_w + y * out_w + x] * 255.0).clamp(0.0, 255.0) as u8;
            out_img.put_pixel(x as u32, y as u32, image::Rgb([r, g, b]));
        }
    }
    // optional scale not enforced by model (model is 4x); resize if requested differs
    let final_img = if scale != 4 {
        image::DynamicImage::ImageRgb8(out_img).resize_exact(
            (w as u32).saturating_mul(scale),
            (h as u32).saturating_mul(scale),
            image::imageops::FilterType::Lanczos3,
        )
    } else {
        image::DynamicImage::ImageRgb8(out_img)
    };
    final_img.save(out_path).map_err(|e| e.to_string())?;
    Ok(out_path.to_string())
}

#[tauri::command]
fn image_dimensions(path: String) -> Result<(u32, u32), String> {
    let img = image::open(&path).map_err(|e| e.to_string())?;
    Ok((img.width(), img.height()))
}

#[tauri::command]
fn should_upscale(path: String, min_edge_px: u32) -> Result<bool, String> {
    let (w, h) = image_dimensions(path)?;
    Ok(w.min(h) < min_edge_px)
}

#[tauri::command]
async fn upscale_image(
    app: tauri::AppHandle,
    path: String,
    out_path: String,
    scale: u32,
    sharpen: bool,
) -> Result<String, String> {
    use tauri::Emitter;
    let scale = scale.clamp(1, 4);
    app.emit("upscale-progress", serde_json::json!({"stage":"loading","progress":0.0})).ok();

    // ensure model is present (download on first use)
    let model_path = upscale_model_path(&app).ok();
    if model_path.as_ref().map(|p| !p.exists()).unwrap_or(true) {
        app.emit("upscale-progress", serde_json::json!({"stage":"downloading-model","progress":0.05})).ok();
        let _ = download_upscale_model(app.clone()).await;
    }

    let model_available = upscale_model_path(&app).map(|p| p.exists()).unwrap_or(false);
    let mut out = if model_available {
        app.emit("upscale-progress", serde_json::json!({"stage":"inferencing","progress":0.25})).ok();
        let app2 = app.clone();
        let path2 = path.clone();
        let out2 = out_path.clone();
        match tauri::async_runtime::spawn_blocking(move || run_onnx_upscale(app2, &path2, &out2, scale)).await {
            Ok(Ok(p)) => {
                app.emit("upscale-progress", serde_json::json!({"stage":"saving","progress":0.9})).ok();
                image::open(&p).map_err(|e| e.to_string())?
            }
            _ => {
                // fallback
                app.emit("upscale-progress", serde_json::json!({"stage":"fallback","progress":0.3})).ok();
                let img = image::open(&path).map_err(|e| e.to_string())?;
                img.resize_exact(img.width().saturating_mul(scale), img.height().saturating_mul(scale), image::imageops::FilterType::Lanczos3)
            }
        }
    } else {
        app.emit("upscale-progress", serde_json::json!({"stage":"resizing","progress":0.25})).ok();
        let img = image::open(&path).map_err(|e| e.to_string())?;
        img.resize_exact(img.width().saturating_mul(scale), img.height().saturating_mul(scale), image::imageops::FilterType::Lanczos3)
    };

    if sharpen {
        app.emit("upscale-progress", serde_json::json!({"stage":"sharpening","progress":0.7})).ok();
        out = out.unsharpen(1.0, 1);
    }

    app.emit("upscale-progress", serde_json::json!({"stage":"saving","progress":0.9})).ok();
    out.save(&out_path).map_err(|e| e.to_string())?;

    app.emit("upscale-progress", serde_json::json!({"stage":"done","progress":1.0,"path":out_path})).ok();
    Ok(out_path)
}

#[tauri::command]
async fn prepare_print_image(
    app: tauri::AppHandle,
    path: String,
    upscale_enabled: bool,
    auto_small: bool,
    min_edge_px: u32,
    scale: u32,
    sharpen: bool,
) -> Result<String, String> {
    let need = if upscale_enabled {
        true
    } else if auto_small {
        should_upscale(path.clone(), min_edge_px).unwrap_or(false)
    } else {
        false
    };
    if !need {
        return Ok(path);
    }
    let p = std::path::Path::new(&path);
    let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("image");
    let dir = p.parent().unwrap_or_else(|| std::path::Path::new("."));
    let out = dir.join(format!("{}_upscaled.png", stem));
    let out_path = out.to_string_lossy().to_string();
    upscale_image(app, path, out_path.clone(), scale, sharpen).await?;
    Ok(out_path)
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("{:x}", t)
}

fn now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    format!("{}", t)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            greet,
            default_presets,
            default_templates,
            list_presets,
            save_preset,
            list_templates,
            save_template,
            submit_print,
            submit_batch,
            queue_status,
            cancel_job,
            retry_job,
            reorder_queue,
            mark_done,
            job_history,
            transform_image,
            apply_template,
            nest_images,
            import_profile,
            validate_profile,
            list_profiles,
            save_profile,
            profile_for_paper,
            set_profile_for_paper,
            color_transform,
            soft_proof,
            image_dimensions,
            should_upscale,
            upscale_image,
            prepare_print_image,
            upscale_model_status,
            download_upscale_model
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
