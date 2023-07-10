use font_mean_value::FontInfo;
use std::fs;
mod image_example;
mod font_mean_value;
mod font_details;
mod image_handler;

fn find_closest_match(pixel: f64, font_info: &FontInfo) -> char {
    let mut best_value: f64 = 10000000.;
    let mut best_char = '!';
    for (c, v) in font_info.characters.iter().zip(font_info.coverage.iter()) {
        if (*v-pixel)*(*v-pixel) < best_value {
            best_value = (*v-pixel)*(*v-pixel);
            best_char = *c;
        }
    }
    best_char
}

enum Lighting {
    DARK,
    LIGHT
}

enum Normalization {
    NONE,
    POW(f64),
    SYMMETRICPOW(f64),
    DOUBLEPOW(f64, f64),

}


fn main() {
    println!("Hello, world!");
    let input_image_path = "images/at_wedding.jpg";
    let ascii_num_rows = 100;
    let output_file_path = "out.txt";
    let lighting = Lighting::DARK;
    let normalization = Normalization::NONE;

    let font_info = font_mean_value::get_font_info();
    let value_image = image_handler::get_value_image_from_rows(input_image_path, ascii_num_rows, font_info.hight_width_ratio);
    let mut s: String = "".to_string();
    for (i, p) in value_image.pixels().enumerate() {
        if i as u32 % value_image.width() == 0{
            s.push('\n');
        }
        let mut val = p.0[0] as f64 / 255.;
        val = match lighting {
            Lighting::DARK => val,
            Lighting::LIGHT => 1. - val,
        };
        val = match normalization {
            Normalization::NONE => val,
            Normalization::POW(p) => val.powf(p),
            Normalization::SYMMETRICPOW(p) => (val.powf(p) + val.powf(1./p))/2.,
            Normalization::DOUBLEPOW(tail, head) => (val.powf(tail) + val.powf(head))/2., 
        };
        s.push(find_closest_match(val, &font_info))
        
    }
    fs::write(output_file_path, s).expect("Unable to write file");
}
