use cmx::profile::Profile;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {

    std::env::set_current_dir(std::path::Path::new(file!()).parent().unwrap())?;   

    let icc_rgb = Profile::from_file("sRGB.icc")?;

    //if let Err(err) = icc_rgb {
    //    println!("icc_rgb read error {:?}", err.to_string());
   // }

    let icc_rgb_json = serde_json::to_string_pretty(&icc_rgb).or_else(|err| Err(Box::new(err) as Box<dyn std::error::Error>))?;
    println!("{}", icc_rgb_json);

    Ok(())
}

