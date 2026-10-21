use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use egui::{ColorImage, Context, ViewportCommand};

// waits a few frames, asks the integration for the rendered pixels, saves them as PNG and closes
pub struct ScreenshotJob {
    out: PathBuf,
    frames: u32,
    requested: bool,
    result: Arc<Mutex<Option<Arc<ColorImage>>>>,
}

impl ScreenshotJob {
    const WAIT_FRAMES: u32 = 10;

    pub fn new(out: PathBuf) -> Self {
        Self { out, frames: 0, requested: false, result: Arc::new(Mutex::new(None)) }
    }

    pub fn tick(&mut self, ctx: &Context) {
        self.frames += 1;
        ctx.request_repaint();
        if !self.requested && self.frames >= Self::WAIT_FRAMES {
            self.requested = true;
            let slot = Arc::clone(&self.result);
            ctx.request_screenshot(move |image| {
                if let Ok(mut guard) = slot.lock() {
                    *guard = Some(image);
                }
            });
            return;
        }
        let image = self.result.lock().ok().and_then(|mut g| g.take());
        if let Some(image) = image {
            self.save(&image);
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
    }

    fn save(&self, image: &ColorImage) {
        let [w, h] = image.size;
        let mut bytes = Vec::with_capacity(w * h * 4);
        for pixel in &image.pixels {
            bytes.extend_from_slice(&pixel.to_srgba_unmultiplied());
        }
        if let Some(parent) = self.out.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        match image::RgbaImage::from_raw(w as u32, h as u32, bytes) {
            Some(buffer) => match buffer.save(&self.out) {
                Ok(()) => println!("saved {} ({w}x{h})", self.out.display()),
                Err(err) => eprintln!("could not save {}: {err}", self.out.display()),
            },
            None => eprintln!("screenshot buffer size mismatch"),
        }
    }
}
