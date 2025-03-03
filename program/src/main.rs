use degenerate::*;

fn _fade_in(frame: Frame) {
  Filter::new()
    .x()
    .alpha((frame.time / 5000.0).min(1.0))
    .render();
}

fn _stretch(frame: Frame) {
  Filter::new()
    .circle()
    .position(Scale2::new(1.0 / (frame.time / 10000.0), 2.0))
    .times(8)
    .render();
}

fn _target() {
  Filter::new()
    .circle()
    .position(Similarity2::from_scaling(2.0))
    .times(8)
    .render();
}

fn kaleidoscope(frame: Frame) {
  let s = 1.0 / 0.75;
  Filter::new()
    .circle()
    .color(Rotation3::from_axis_angle(&Vector3::y_axis(), 0.05 * TAU))
    .position(Similarity2::from_scaling(s))
    .wrap(true)
    .times(8)
    .render()
    .color(Rotation3::from_axis_angle(&Vector3::z_axis(), 0.05 * TAU))
    .position(Similarity2::from_parts(
      Translation2::identity(),
      Rotation2::new(5.0 / 6.0 * TAU + frame.time / 30000.0 * TAU).into(),
      s,
    ))
    .render();
}

fn _orbs() {
  Filter::new()
    .circle()
    .color(Rotation3::from_axis_angle(&Vector3::y_axis(), 0.05 * TAU))
    .position(Similarity2::from_scaling(1.0 / 0.75))
    .wrap(true)
    .times(8)
    .render()
    .color(Rotation3::from_axis_angle(&Vector3::z_axis(), 0.05 * TAU))
    .render();
}

fn _x() {
  for i in 0..8 {
    Filter::new()
      .x()
      .wrap(i % 2 == 1)
      .position(Similarity2::from_scaling(2.0))
      .render();
  }
}

fn main() {
  kaleidoscope.execute();
}
