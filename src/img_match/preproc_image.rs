use image;

pub fn crop(image: String, x: u32, y: u32, w: u32, h: u32, out: String) {
  // 打开原始图像
  let img = image::open(&image)
    .unwrap_or_else(|_| panic!("Could not load image at {:?}", &image))
    .to_rgb8();

  // 裁剪图像
  let mut img = img; // 重新绑定为可变
  let cropped = image::imageops::crop(&mut img, x, y, w, h);

  // 使用与输入相同的格式保存图像
  cropped
    .to_image()
    .save_with_format(out.clone(), image::ImageFormat::Png)
    .unwrap_or_else(|_| panic!("Could not save image"));
}

// 将四周round像素的像素值设置为0
pub fn set_around_zero(image: String, round: u32) {
  // 打开原始图像
  let mut img = image::open(&image)
    .unwrap_or_else(|_| panic!("Could not load image at {:?}", &image))
    .to_rgb8();
  let (w, h) = img.dimensions();

  for i in 0..w {
    for j in 0..h {
      if i < round || i > w - round || j < round || j > h - round {
        // 设置四周round像素的像素值为0
        img.put_pixel(i, j, image::Rgb([0, 0, 0]));
      }
    }
  }

  // 保存图像
  img
    .save_with_format(image.clone(), image::ImageFormat::Png)
    .unwrap_or_else(|_| panic!("Could not save image"));
}
