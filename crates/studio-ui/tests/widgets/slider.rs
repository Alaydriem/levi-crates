use studio_ui::widgets::Slider;

#[test]
fn snap_rounds_to_step_and_clamps() {
    assert_eq!(Slider::snap(2.26, &(0.5..=8.0), 0.25), 2.25);
    assert_eq!(Slider::snap(9.0, &(0.5..=8.0), 0.25), 8.0);
    assert_eq!(Slider::snap(-1.0, &(0.5..=8.0), 0.25), 0.5);
}

#[test]
fn snap_with_a_zero_step_only_clamps() {
    assert_eq!(Slider::snap(3.3, &(0.0..=8.0), 0.0), 3.3);
    assert_eq!(Slider::snap(30.0, &(0.0..=8.0), 0.0), 8.0);
}
