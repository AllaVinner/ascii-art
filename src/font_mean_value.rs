use image::{DynamicImage, Rgba};
use rusttype::{point, Font, Scale};

pub struct FontInfo {
    pub hight_width_ratio: f64,
    pub characters: Vec<char>,
    pub coverage: Vec<f64>
}

fn min_max_normalize(v: Vec<f64>) -> Vec<f64> {
    let max_val = v.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
    let min_val = v.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
    v.iter().map(|val| (val-min_val)/(max_val-min_val)).collect()
}



pub fn get_font_info() -> FontInfo {
    let font_data = include_bytes!("../fonts/Courier_Prime/CourierPrime-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");
    let scale = Scale::uniform(1000.0);
    let origin = point(0.,0.);
    let text = " !\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~';";
    let character_block_height = font.v_metrics(scale).ascent - font.v_metrics(scale).descent;
    let character_block_width = font.glyph(text.chars().next().unwrap()).scaled(scale).h_metrics().advance_width;
    let character_bloc_area = character_block_height * character_block_width;
    
    let mut coverage = Vec::<f64>::new();
    let mut characters = Vec::<char>::new();

    for c in text.chars() {
        let mut accum_value = 0.;
        font.glyph(c).scaled(scale).positioned(origin).draw(|x, y, v|
            {
                accum_value += v as f64;
                let aa = if v > 0.5 {
                    1.
                } else {
                    0.
                };
            } );
        coverage.push(accum_value/character_bloc_area as f64);
        characters.push(c);
    }

    FontInfo { hight_width_ratio: (character_block_height/character_block_width) as f64,
               characters: characters, coverage: min_max_normalize(coverage)}

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

