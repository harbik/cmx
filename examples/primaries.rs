//! Creates three small png images, with size three horizontally stacked blocks,
//! each 100 by 100 pixels wide, with repectively a fully satured red, green,
//! and blue color.
//! 
//! * The first image, written to "tmp/srgb_test.png" contains no color profile, and uses the sRGB color space,
//! * The second image, written to "tmp/srgb_profile_test.png" contains an sRGB based input profile, set with 
//!   relative colorimetric intent.
//! * The third contains a display_p3 Colorimetry input profile, also with relative colorimetric intent,
//!   and is written to "tmp/display_p3_test.png".
//! 
//! The purpose of these images is to check if the application, rendering the images, interprets the 
//! color profiles correctly. This is a visual test, and you can view the images in different applications and on
//! different screens, and try to make sense of what you see, and the pixel values you read with an 
//! applications such as Apple's Digital Color Meter.

use cmx::{profile::DisplayProfile, tag::RenderingIntent};
use image::{codecs::png::PngEncoder, ExtendedColorType, ImageEncoder, Rgba, RgbaImage};
use colorimetry::rgb::RgbSpace;


pub fn main() -> Result<(), Box<dyn std::error::Error>> {

    // create the input color profiles
    // create the three blocks

    println!("Creating test images in tmp/");

    let width = 300;
    let height = 100;
    let mut image = RgbaImage::new(width, height);
    for (x, _y, pixel) in image.enumerate_pixels_mut() {
        if x < 100 {
            *pixel = Rgba([255, 0, 0, 255]); // Red block
        } else if x < 200 {
            *pixel = Rgba([0, 255, 0, 255]); // Green block
        } else {
            *pixel = Rgba([0, 0, 255, 255]); // Blue block
        }
    }
    // Save the image without a color profile
    image.save("examples/srgb_test.png")?;
    println!("Saved examples/srgb_test.png without color profile");

    // Save the image with an sRGB color profile
    let srgb_profile = DisplayProfile::from_rgb_space(RgbSpace::SRGB, RenderingIntent::RelativeColorimetric);
    let mut srgb_png_data = Vec::new();
    let mut encoder = PngEncoder::new(&mut srgb_png_data);
    encoder.set_icc_profile(srgb_profile.to_bytes()?)?;
    encoder.write_image(&image.as_raw(), width, height, ExtendedColorType::Rgba8)?;
    std::fs::write("examples/srgb_profile_test.png", &srgb_png_data)?;
    println!("Saved examples/srgb_profile_test.png with sRGB color profile");

    // Save the image with a Display P3 color profile
    let display_p3_profile = DisplayProfile::from_rgb_space(RgbSpace::DisplayP3, RenderingIntent::RelativeColorimetric);
    display_p3_profile.clone().write("examples/display_p3.icc")?;
    let mut display_p3_png_data = Vec::new();
    let mut encoder = PngEncoder::new(&mut display_p3_png_data);
    encoder.set_icc_profile(display_p3_profile.to_bytes()?)?;
    encoder.write_image(&image.as_raw(), width, height, ExtendedColorType::Rgba8)?;
    std::fs::write("examples/display_p3_test.png", &display_p3_png_data)?;
    println!("Saved examples/display_p3_test.png with Display P3 color profile");

    Ok(())
}