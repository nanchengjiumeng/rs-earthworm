use imageproc::{self, template_matching};
use napi_derive::napi;

#[napi]
pub struct Point {
  pub x: u32,
  pub y: u32,
  pub score: u32,
}
pub fn match_template(image: String, template: String) -> Point {
  let match_image = image::open(&image)
    .unwrap_or_else(|_| panic!("Could not load image at {:?}", &image))
    .to_luma8();
  let match_template = image::open(&template)
    .unwrap_or_else(|_| panic!("Could not load image at {:?}", &template))
    .to_luma8();

  let result = template_matching::match_template(
    &match_image,
    &match_template,
    template_matching::MatchTemplateMethod::SumOfSquaredErrors,
  );

  // 获取图像的宽度
  let width = result.width();

  // // 找到最佳匹配点
  let mut best_x = 0;
  let mut best_y = 0;
  let mut best_score = f32::MAX;

  for y in 0..result.height() {
    for x in 0..width {
      let score: f32 = result.get_pixel(x, y)[0] as f32;
      if score < best_score {
        best_score = score;
        best_x = x;
        best_y = y;
      }
    }
  }

  Point {
    x: best_x,
    y: best_y,
    score: best_score as u32,
  }
}

pub fn find_image(image: String, template: String) -> Point {
  let source = image::open(&image)
    .unwrap_or_else(|_| panic!("Could not load image at {:?}", &image))
    .to_luma8();
  let template = image::open(&template)
    .unwrap_or_else(|_| panic!("Could not load template at {:?}", &template))
    .to_luma8();

  let (template_width, template_height) = template.dimensions();
  let (source_width, source_height) = source.dimensions();
  let pixel = template.get_pixel(0, 0);
  let edge_value = pixel[0];

  for y in 0..=source_height.saturating_sub(template_height) {
    for x in 0..=source_width.saturating_sub(template_width) {
      let mut is_match = true;

      'pixel_check: for ty in 0..template_height {
        for tx in 0..template_width {
          let template_pixel = template.get_pixel(tx, ty)[0];

          // 跳过边缘色（灰度值）
          if template_pixel == edge_value {
            continue;
          }

          let source_pixel = source.get_pixel(x + tx, y + ty)[0];

          // 如果像素不完全相同，则不是匹配
          if template_pixel != source_pixel {
            is_match = false;
            break 'pixel_check;
          }
        }
      }

      // 如果找到完全匹配，立即返回位置
      if is_match {
        return Point { x, y, score: 1 };
      }
    }
  }

  // 如果没有找到匹配，返回(0, 0)
  Point {
    x: 0,
    y: 0,
    score: 0,
  }
}

#[test]
fn test_find_image() {
  let image = "./target/image/horses.png";
  let template = "./target/image/horses_section.png";

  let point = find_image(image.to_string(), template.to_string());
  println!("Found at: ({}, {}, {})", point.x, point.y, point.score);
}
