#![deny(clippy::all)]

mod img_match;
use napi_derive::napi;

#[napi]
pub fn crop(image: String, x: u32, y: u32, w: u32, h: u32, out: String) {
  img_match::preproc_image::crop(image, x, y, w, h, out);
}

#[napi]
pub fn find_image(image: String, template: String) -> img_match::match_image::Point {
  img_match::match_image::find_image(image, template)
}

#[napi]
pub fn match_template(image: String, template: String) -> img_match::match_image::Point {
  img_match::match_image::match_template(image, template)
}
