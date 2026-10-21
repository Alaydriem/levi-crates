use egui::{Context, Id, RawInput};
use studio_ui::theme::Metrics;
use studio_ui::widgets::Motion;

const FRAME: f64 = 1.0 / 60.0;

fn sample(ctx: &Context, id: Id, frame: u32, on: bool) -> f32 {
    let mut value = 0.0;
    let input = RawInput { time: Some(1.0 + f64::from(frame) * FRAME), ..Default::default() };
    let mut output = ctx.run_ui(input, |ui| {
        value = Motion::flag(ui.ctx(), id, on);
    });
    output.textures_delta.clear();
    value
}

#[test]
fn motion_reaches_target_after_motion_seconds() {
    let ctx = Context::default();
    let id = Id::unique("motion-test");
    let mut frame = 0;
    let mut resting = 0.0;
    for _ in 0..5 {
        resting = sample(&ctx, id, frame, false);
        frame += 1;
    }

    let mut previous = resting;
    let mut values = Vec::new();
    let frames_to_target = (f64::from(Metrics::MOTION_SECONDS) / FRAME).ceil() as u32 + 1;
    for _ in 0..frames_to_target {
        let value = sample(&ctx, id, frame, true);
        assert!(value >= previous, "value fell from {previous} to {value}");
        previous = value;
        values.push(value);
        frame += 1;
    }

    assert_eq!(resting, 0.0);
    assert!(values.first().is_some_and(|v| *v > 0.0 && *v < 1.0));
    assert_eq!(values.last().copied(), Some(1.0));
}
