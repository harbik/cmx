use cmx::profile::DisplayProfile;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get input path from CLI or return a usage error.
    let input = std::env::args().nth(1).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "usage: set_intent <input-icc-file>",
        )
    })?;

    // Read input profile.
    let in_profile = DisplayProfile::read(&input)?;
    let out_profile =
        in_profile.with_rendering_intent(cmx::tag::RenderingIntent::RelativeColorimetric);
    println!("{out_profile}");

    // Build output path: <stem>-absolute.<ext> in the same directory.
    let input_path = std::path::Path::new(&input);
    let stem = input_path
        .file_stem()
        .unwrap_or_else(|| std::ffi::OsStr::new("output"));
    let ext = input_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("icc");
    let out_path =
        input_path.with_file_name(format!("{}-relative.{}", stem.to_string_lossy(), ext));
    let out_path = out_path.to_string_lossy().into_owned();

    // Write output profile.
    out_profile.write(&out_path)
}
