use cmx::profile::Profile;

/**
 * Parses all "*.icc" files into "*.json" files
 * 
 */

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {

    std::env::set_current_dir(std::path::Path::new(file!()).parent().unwrap())?;   

    for icc_file in glob::glob("../test_profiles/*.icc")?.filter_map(Result::ok) {
        let stem = icc_file.file_stem().ok_or("no file stem")?;
        let icc_rgb = Profile::from_file(icc_file.to_str().unwrap())?;
        let icc_rgb_json = serde_json::to_string_pretty(&icc_rgb).or_else(|err| Err(Box::new(err) as Box<dyn std::error::Error>))?;
        std::fs::write(format!("{}.json", stem.to_str().unwrap()), icc_rgb_json)?;
    }

    Ok(())
}

