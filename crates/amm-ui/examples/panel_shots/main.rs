// Writes the manager panel screenshots into a folder, or with --view <n> opens shot n for a look
mod game_backdrop;
mod scene;
mod shot;
mod shot_runner;
mod shots_app;

use scene::Scene;
use shot_runner::ShotRunner;
use shots_app::ShotsApp;

fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    let (runner, first) = match (args.get(1).map(String::as_str), args.get(2)) {
        (Some("--view"), Some(number)) => (None, number.parse::<usize>().unwrap_or(1).saturating_sub(1)),
        (Some(out_dir), _) if out_dir != "--view" => (Some(ShotRunner::new(out_dir.into())), 0),
        _ => {
            eprintln!("usage: panel_shots <out dir> | --view <shot number>");
            std::process::exit(2);
        }
    };
    let scene = match Scene::new() {
        Ok(scene) => scene,
        Err(error) => {
            eprintln!("could not build the scene: {error}");
            std::process::exit(1);
        }
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_resizable(false)
            .with_title("Alaydriem Mod Manager panel shots"),
        ..Default::default()
    };
    eframe::run_native(
        "amm-ui-panel-shots",
        options,
        Box::new(move |cc| Ok(Box::new(ShotsApp::new(&cc.egui_ctx, scene, runner, first)))),
    )
}
