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
    let text = "This is RustType rendered into a png!";
    let scale = Scale::uniform(32.0);
    let text = " !\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~';";

    let colour = (150, 0, 0);

    let v_metrics = font.v_metrics(scale);

    // layout the glyphs in a line with 20 pixels padding
    let glyphs: Vec<_> = font
        .layout(text, scale, point(20.0, 20.0 + v_metrics.ascent))
        .collect();

    let mut acum_values: Vec<_> = glyphs.iter().map(|g| {
        let mut accum_value = 0.;
        g.draw(|x, y, v| {
            accum_value += v;
        });
        accum_value
    }).collect();
    acum_values = min_max_normalize(acum_values);
    acum_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("acum_values {:?}", acum_values);

}

