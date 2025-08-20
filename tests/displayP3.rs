

pub fn read_display_p3_profile() -> Result<cmx::profile::Profile, Box<dyn std::error::Error>> {
    use std::fs;
    use std::path::Path;

    let profile_path = Path::new("tests/profiles/Display P3.icc");
    let profile_bytes = fs::read(profile_path).expect("Failed to read Display P3 profile");
    let raw_profile = cmx::profile::RawProfile::from_bytes(&profile_bytes)?;
    
    Ok(raw_profile.into_class_profile())
}

#[cfg(test)]
mod make_display_p3 {
    use cmx::profile::{DisplayProfile, RawProfile};
    use std::ffi::os_str::Display;
    use std::fs;
    use std::path::Path;

    #[test]
    fn print_display_p3() -> Result<(), Box<dyn std::error::Error>> {
        let mut display_p3 = DisplayProfile::new()
            .with_version(4, 4)?
            .with_creation_date(None);

        let t = display_p3
            .with_tag(cmx::tag::tags::RedMatrixColumnTag)
            .as_xyz_array(|xyz| {
                xyz.set_xyz([0.680, 0.320, 0.000]);
            });
        Ok(())
    }
}

