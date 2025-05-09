#![deny(clippy::all)]

mod img_match;
mod img_pre_proc;
use napi_derive::napi;

#[napi]
pub fn crop(image: String, x: u32, y: u32, w: u32, h: u32, out: String) {
  img_match::preproc_image::crop(image, x, y, w, h, out);
}

#[napi]
pub fn set_around_zero(image: String, round: u32) {
  img_match::preproc_image::set_around_zero(image, round);
}

#[napi]
pub fn find_image(image: String, template: String) -> img_match::match_image::Point {
  img_match::match_image::find_image(image, template)
}

#[napi]
pub fn match_template(image: String, template: String) -> img_match::match_image::Point {
  img_match::match_image::match_template(image, template)
}

// 预处理
#[napi]
pub fn filter_binaryzation(image: String, out: String, threshold: String) {
  // 打开原始图像
  let img = image::open(&image)
    .unwrap_or_else(|_| panic!("Could not load image at {:?}", &image))
    .to_rgb8();
  let (cols, rows) = img.dimensions();
  let data = img.into_raw();
  let data_filtered = img_pre_proc::preproc::filter_binaryzation(
    &data,
    rows.try_into().unwrap(),
    cols.try_into().unwrap(),
    &threshold,
  );
  // 保存图像
  let mut img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(cols, rows);
  for i in 0..rows {
    for j in 0..cols {
      let pixel = data_filtered.at(j as i32, i as i32);
      img.put_pixel(j, i, image::Rgb([pixel, pixel, pixel]));
    }
  }
  img
    .save_with_format(out.clone(), image::ImageFormat::Png)
    .unwrap_or_else(|_| panic!("Could not save image"));
}

#[test]
fn test_filter_binaryzation() {
  filter_binaryzation(
    "./target/image/horses.png".to_string(),
    "./target/image/horses_filter.png".to_string(),
    "128-255".to_string(),
  );
}
