use image::{DynamicImage, Rgba};
use rusttype::{point, Font, Scale};


pub fn main()
{
    // Load the font
    let font_data = include_bytes!("../fonts/Courier_Prime/CourierPrime-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");
    let scale = Scale::uniform(100.);
    let origo = point(0., 0.);
    println!("Details of font Courier");
    println!("Number of glyphs: {}", font.glyph_count());
    println!("Vmetric (unscaled): {:?}", font.v_metrics_unscaled());
    println!("Vmetric (scaled): {:?}", font.v_metrics(scale));
    for i in "@M".chars() {
        for j in "@MV".chars() {
            println!("Kerning {}, {}: {}",i,j, font.pair_kerning(Scale::uniform(1.), i, j));
        }
    }
    for c in "!@.'".chars() {
        let g = font.glyph(c);
        let sg = g.scaled(scale);
        println!("Dealing with char: {}", c);
        println!("\tExact bounding box: {:?}", sg.exact_bounding_box());
        println!("\tH metric: {:?}", sg.h_metrics());
        println!("\n Position metrics");
        let pg = sg.positioned(origo);
        println!("\tpixel_bounding_box: {:?}", pg.pixel_bounding_box()); 
        


    }

}




















