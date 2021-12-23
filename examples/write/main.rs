use cmx::profile::Profile;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {

    std::env::set_current_dir(std::path::Path::new(file!()).parent().unwrap())?;   

    let icc_rgb = Profile::from_file("sRGB.icc");

    if let Err(ref err) = icc_rgb {
        println!("icc_rgb read error {:?}", err.to_string());
    }

    let buf = icc_rgb.unwrap().to_buffer();

    if let Err(ref err) = buf {
        println!("icc_rgb write error {:?}", err.to_string());
    }

    let icc_rgb_2 = Profile::from_buffer(&buf.unwrap());

    if let Err(ref err) = icc_rgb_2 {
        println!("icc_rgb read error {:?}", err.to_string());
    }


    println!("{:?}", icc_rgb_2);


    Ok(())
}

