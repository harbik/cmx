use cmx::profile::Profile;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {

    std::env::set_current_dir(std::path::Path::new(file!()).parent().unwrap())?;   

    let icc_rgb = Profile::from_file("sRGB.icc");

    if let Err(err) = icc_rgb {
        println!("icc_rgb read error {:?}", err.to_string());
    }

    Ok(())
}

