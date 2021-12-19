use {
  executable_path::executable_path,
  pretty_assertions::assert_eq,
  std::{fs, process::Command, str},
};

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn assert_output_eq(args: &[&str], expected_bitmap: &str) -> Result<()> {
  let mut command = Command::new(executable_path("degenerate"));

  command.args(args);

  let output = command.output()?;

  assert!(
    output.status.success(),
    "Command {:?} failed: {}",
    command,
    str::from_utf8(&output.stderr)?
  );

  let mut expected_bitmap = expected_bitmap.replace(" ", "");
  expected_bitmap.push('\n');

  assert_eq!(str::from_utf8(&output.stdout)?, expected_bitmap);

  Ok(())
}

#[test]
fn circle() -> Result<()> {
  assert_output_eq(
    &["resize:10:10", "circle"],
    "0001111000
     0111111110
     0111111110
     1111111111
     1111111111
     1111111111
     1111111111
     0111111110
     0111111110
     0001111000",
  )
}

#[test]
fn even() -> Result<()> {
  assert_output_eq(
    &["resize:4:4", "even"],
    "1111
     0000
     1111
     0000",
  )
}

#[test]
fn top() -> Result<()> {
  assert_output_eq(
    &["resize:2:2", "top"],
    "11
     00",
  )
}

#[test]
fn resize() -> Result<()> {
  assert_output_eq(&["resize:2:1"], "00")
}

#[test]
fn invert() -> Result<()> {
  assert_output_eq(&["resize:1:1", "all"], "1")
}

#[test]
fn square() -> Result<()> {
  assert_output_eq(
    &["resize:4:4", "square"],
    "0000
     0110
     0110
     0000",
  )
}

#[test]
fn modulus() -> Result<()> {
  assert_output_eq(
    &["resize:4:2", "mod:2:0"],
    "1111
     0000",
  )
}

#[test]
fn default_bitmap_size() -> Result<()> {
  assert_output_eq(
    &[],
    "00000000000000000000000000000000000000000000000000000000000000000000000000000000
     00000000000000000000000000000000000000000000000000000000000000000000000000000000
     00000000000000000000000000000000000000000000000000000000000000000000000000000000
     00000000000000000000000000000000000000000000000000000000000000000000000000000000
     00000000000000000000000000000000000000000000000000000000000000000000000000000000
     00000000000000000000000000000000000000000000000000000000000000000000000000000000
     00000000000000000000000000000000000000000000000000000000000000000000000000000000
     00000000000000000000000000000000000000000000000000000000000000000000000000000000
     00000000000000000000000000000000000000000000000000000000000000000000000000000000
     00000000000000000000000000000000000000000000000000000000000000000000000000000000
     00000000000000000000000000000000000000000000000000000000000000000000000000000000
     00000000000000000000000000000000000000000000000000000000000000000000000000000000
     00000000000000000000000000000000000000000000000000000000000000000000000000000000
     00000000000000000000000000000000000000000000000000000000000000000000000000000000
     00000000000000000000000000000000000000000000000000000000000000000000000000000000
     00000000000000000000000000000000000000000000000000000000000000000000000000000000
     00000000000000000000000000000000000000000000000000000000000000000000000000000000
     00000000000000000000000000000000000000000000000000000000000000000000000000000000
     00000000000000000000000000000000000000000000000000000000000000000000000000000000
     00000000000000000000000000000000000000000000000000000000000000000000000000000000",
  )
}

#[ignore]
#[test]
fn default_image_size() -> Result<()> {
  let output = Command::new(executable_path("degenerate"))
    .arg("--output=output.txt")
    .output()?;

  assert!(
    output.status.success(),
    "Command failed: {}",
    str::from_utf8(&output.stderr)?
  );

  let content = fs::read_to_string("output.txt")?;
  let lines = content.lines();

  assert_eq!(lines.clone().count(), 4096);

  for line in lines {
    assert_eq!(line.len(), 4096);
  }

  Ok(())
}
