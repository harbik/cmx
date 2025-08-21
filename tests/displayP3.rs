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

    use cmx::profile::DisplayProfile;

    #[test]
    #[rustfmt::skip]
    fn print_display_p3() -> Result<(), Box<dyn std::error::Error>> {
        use cmx::tag::tags::*;
        let binding = DisplayProfile::new()
            .with_version(4, 4)?
            .with_creation_date(None);
        let display_p3 = binding
            .with_tag(ProfileDescriptionTag)
                .as_text_description(|text| {
                    text.set_ascii("Display P3");
                })
            .with_tag(RedMatrixColumnTag)
                .as_xyz_array(|xyz| {
                    xyz.set([0.515121, 0.241196, -0.001053]);
                })
            .with_tag(GreenMatrixColumnTag)
                .as_xyz_array(|xyz| {
                    xyz.set([0.241196, 0.675814, -0.001053]);
                })
            .with_tag(BlueMatrixColumnTag)
                .as_xyz_array(|xyz| {
                    xyz.set([0.157104, 0.066574, 0.784073]);
                });

        println!("{:#?}", display_p3);
        Ok(())
    }
}
