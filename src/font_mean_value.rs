use image::{DynamicImage, Rgba};
use rusttype::{point, Font, Scale};

fn min_max_normalize(v: Vec<f32>) -> Vec<f32> {
    let max_val = v.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
    let min_val = v.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
    v.iter().map(|val| (val-min_val)/(max_val-min_val)).collect()
}

pub fn main()
{
    // Load the font
    let font_data = include_bytes!("../fonts/Courier_Prime/CourierPrime-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");
    let scale = Scale::uniform(10.0);
    let origin = point(0.,0.);
    let characters = " !\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~';";
    let character_block_height = font.v_metrics(scale).ascent - font.v_metrics(scale).descent;
    let character_block_width = font.glyph(characters.chars().next().unwrap()).scaled(scale).h_metrics().advance_width;
    let character_bloc_area = character_block_height * character_block_width;
    println!("Block widht {} height {}", character_block_width, character_block_height);
    for c in characters.chars() {
        let mut accum_value = 0.;
        font.glyph(c).scaled(scale).positioned(origin).draw(|x, y, v|
            {
                accum_value += if v > 0.5 {
                    1.
                } else {
                    0.
                };
            } );
        println!("Character {} has a coverage of {}", c, accum_value/character_bloc_area);
    }
}

