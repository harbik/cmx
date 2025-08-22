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
        let display_p3_original = cmx::profile::RawProfile::from_bytes(
            include_bytes!("../tests/profiles/Display P3.icc")
        )?;
        use cmx::tag::tags::*;
        let display_p3_cmx = DisplayProfile::new()
            .with_version(4, 0)?
            .with_creation_date(None)
        //    .with_cmm(cmx::signatures::Cmm::Apple)?
            .with_data_color_space(cmx::signatures::ColorSpace::RGB)
            .with_tag(ProfileDescriptionTag)
                .as_text_description(|text| {
                    text.set_ascii("Display P3");
                })
            .with_tag(CopyrightTag)
                .as_text(|text| {
                    text.set_text("Copyright (c) 2025 Harbers Bik LLC. No copyright claimed.");
                })
            .with_tag(RedMatrixColumnTag)
                .as_xyz_array(|xyz| {
                    xyz.set([0.515121, 0.241196, -0.001053]);
                })
            .with_tag(GreenMatrixColumnTag)
                .as_xyz_array(|xyz| {
                    xyz.set([0.291977, 0.692245, 0.041885]);
                })
            .with_tag(BlueMatrixColumnTag)
                .as_xyz_array(|xyz| {
                    xyz.set([0.157104, 0.066574, 0.784073]);
                })
            .with_tag(MediaWhitePointTag)
                .as_xyz_array(|xyz| {
                    xyz.set([0.950455, 1.00000, 1.08905]);
                })
            .with_tag(RedTRCTag)
                .as_parametric_curve(|para| {
                    para.set_parameters([2.4, 0.9479, 0.0521, 0.0774, 0.0405]);
                })
            .with_tag(GreenTRCTag)
                .as_parametric_curve(|para| {
                    para.set_parameters([2.4, 0.9479, 0.0521, 0.0774, 0.0405]);
                })
            .with_tag(BlueTRCTag)
                .as_parametric_curve(|para| {
                    para.set_parameters([2.4, 0.9479, 0.0521, 0.0774, 0.0405]);
                })
            .with_tag(ChromaticAdaptationTag)
                .as_sf15_fixed_16_array(|array| {
                    array.set([
                         1.047882, 0.022919, -0.050201,
                         0.029587, 0.990479, -0.017059,
                        -0.009232, 0.015076,  0.751678
                    ]);
                })
            ;

        // Print as TOML using Profile's Display impl
     //   println!("{display_p3_cmx}");
        dbg!(&display_p3_original);
        display_p3_cmx.to_file("tests/profiles/Display P3_cmx.icc")?; 
        Ok(())
    }
}
