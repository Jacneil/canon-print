use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
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
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PrintJob {
    pub id: String,
    pub file: String,
    pub preset: String,
    pub status: String,
    pub submitted_at: String,
    pub notes: String,
}

#[derive(Default)]
pub struct AppState {
    pub presets: Mutex<Vec<Preset>>,
    pub queue: Mutex<VecDeque<PrintJob>>,
    pub history: Mutex<Vec<PrintJob>>,
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
        },
    ]
}

#[tauri::command]
fn list_presets(state: State<AppState>) -> Vec<Preset> {
    let p = state.presets.lock().unwrap();
    if p.is_empty() {
        default_presets()
    } else {
        p.clone()
    }
}

#[tauri::command]
fn save_preset(state: State<AppState>, preset: Preset) -> Result<(), String> {
    let mut p = state.presets.lock().map_err(|e| e.to_string())?;
    if let Some(i) = p.iter().position(|x| x.name == preset.name) {
        p[i] = preset;
    } else {
        p.push(preset);
    }
    Ok(())
}

#[tauri::command]
fn submit_print(state: State<AppState>, file: String, preset: String, notes: String) -> PrintJob {
    let job = PrintJob {
        id: uuid_simple(),
        file: file.clone(),
        preset,
        status: "Queued".into(),
        submitted_at: now(),
        notes,
    };
    let mut q = state.queue.lock().unwrap();
    q.push_back(job.clone());
    job
}

#[tauri::command]
fn queue_status(state: State<AppState>) -> Vec<PrintJob> {
    state.queue.lock().unwrap().iter().cloned().collect()
}

#[tauri::command]
fn cancel_job(state: State<AppState>, id: String) -> Result<(), String> {
    let mut q = state.queue.lock().map_err(|e| e.to_string())?;
    if let Some(j) = q.iter_mut().find(|j| j.id == id) {
        j.status = "Cancelled".into();
        Ok(())
    } else {
        Err("job not found".into())
    }
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
fn transform_image(path: String, out_path: String, width: u32, height: u32, rotate: i32) -> Result<String, String> {
    // Basic resize/rotate using image crate. Color transform via lcms2 is applied in a fuller implementation.
    let img = image::open(&path).map_err(|e| e.to_string())?;
    let mut img = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
    if rotate != 0 {
        img = img.rotate90();
    }
    img.save(&out_path).map_err(|e| e.to_string())?;
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
            list_presets,
            save_preset,
            submit_print,
            queue_status,
            cancel_job,
            retry_job,
            mark_done,
            job_history,
            transform_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}