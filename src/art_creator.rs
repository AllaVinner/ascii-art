use image::io::Reader as ImageReader;
use image::{GenericImageView, GrayImage, ImageBuffer, Luma};
mod font_mean_value;

fn find_closest_match(pixel: f64, font_info: &FontInfo) -> char {
    let mut best_value: f64 = 10000000;
    let mut best_char = '!';
    for (c, v) in font_info.characters.iter().zip(font_info.coverage.iter()) {
        if (*v-pixel)*(*v-pixel) < best_value {
            best_value = (*v-pixel)*(*v-pixel);
            best_char = *c;
        }
    }
    best_char
}


pub fn creat_ascii(font_info: font_mean_value::FontInfo, value_image: ImageBuffer) -> String {
    let mut s: String = "".to_string();
    for (i, p) in value_image.pixels().enumerate() {
        if i as u32 % value_image.width() == 0{
            s.push('\n');
        }
        let mut val = (p.0[0] as f64 / 255.);
        let pow: f64 = 1.;
        //val = -(1.- val.powf(pow)).powf(1./pow);
        s.push(find_closest_match(val, &font_info))
    }
    s
}
