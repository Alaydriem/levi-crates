mod app;
mod screenshot;

use app::GalleryApp;
use screenshot::ScreenshotJob;

fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    let job = match args.iter().position(|a| a == "--screenshot") {
        Some(index) => match args.get(index + 1) {
            Some(out) => Some(ScreenshotJob::new(out.into())),
            None => {
                eprintln!("usage: gallery [--screenshot <out.png>]");
                std::process::exit(2);
            }
        },
        None => None,
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1180.0, 1040.0])
            .with_resizable(false)
            .with_title("studio-ui gallery"),
        ..Default::default()
    };
    eframe::run_native(
        "studio-ui-gallery",
        options,
        Box::new(move |cc| Ok(Box::new(GalleryApp::new(&cc.egui_ctx, job)))),
    )
}
