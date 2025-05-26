use opencv::{
    core::{self, Mat, CV_8UC3},
    imgcodecs::{self, IMREAD_COLOR},
    imgproc,
    prelude::*,
};
use std::error::Error;
use xcap::{image::RgbaImage, Monitor};

fn capture_screen_region(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<RgbaImage, Box<dyn Error>> {
    let monitor = Monitor::from_point(x as i32, y as i32)?;
    let frame = monitor.capture_image()?;

    if x + width > frame.width() || y + height > frame.height() {
        return Err("Crop region out of bounds".into());
    }

    let sub_img = image::imageops::crop_imm(&frame, x, y, width, height).to_image();
    Ok(sub_img)
}

fn detect_template_match(
    screenshot: &RgbaImage,
    template_mat: &Mat,
    threshold: f64,
) -> Result<bool, opencv::Error> {
    let width = screenshot.width() as i32;
    let height = screenshot.height() as i32;

    let bgr_bytes: Vec<u8> = screenshot
        .pixels()
        .flat_map(|p| vec![p[2], p[1], p[0]])
        .collect();

    let screen_mat = Mat::new_rows_cols_with_data(height, width, &bgr_bytes)?;

    if template_mat.typ() != CV_8UC3 {
        return Err(opencv::Error::new(
            opencv::core::StsBadArg,
            "Template must be a 3-channel BGR image",
        ));
    }

    let mut result = Mat::default();
    imgproc::match_template(
        &screen_mat,
        template_mat,
        &mut result,
        imgproc::TM_CCOEFF_NORMED,
        &core::no_array(),
    )?;

    let mut max_val = 0.0;
    core::min_max_loc(
        &result,
        None,
        Some(&mut max_val),
        None,
        None,
        &core::no_array(),
    )?;

    Ok(max_val >= threshold)
}

pub fn image_match(
    image_path: &str,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<bool, Box<dyn Error>> {
    let template_mat = imgcodecs::imread(image_path, IMREAD_COLOR)?;
    if template_mat.empty() {
        return Err(format!("Failed to load template image: {}", image_path).into());
    }

    let region = capture_screen_region(x, y, width, height)?;

    return detect_template_match(&region, &template_mat, 0.9).map_err(|e| e.into());
}
