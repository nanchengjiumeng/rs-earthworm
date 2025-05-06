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
