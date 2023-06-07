use image::{DynamicImage, Rgba};
use rusttype::{point, Font, Scale};


pub fn main()
{
    // Load the font
    let font_data = include_bytes!("../fonts/Courier_Prime/CourierPrime-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");
    let text = "This is RustType rendered into a png!";
    let scale = Scale::uniform(32.0);
    let text = "T@his is RustType rendered into a png!";

    let colour = (150, 0, 0);

    let v_metrics = font.v_metrics(scale);

    // layout the glyphs in a line with 20 pixels padding
    let glyphs: Vec<_> = font
        .layout(text, scale, point(20.0, 20.0 + v_metrics.ascent))
        .collect();

    // work out the layout size
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };

    // Create a new rgba image with some padding
    let mut image = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba8();

    let areas: Vec<_> = glyphs.iter().zip(text.chars()).map(|(g, i)| {
            match g.pixel_bounding_box() {
                Some(bb) => (bb.max.x - bb.min.x+1) as u32 *(glyphs_height),
                None => {println!("Got glyph wihout bounding box {:?}\n  {:?}", i, g); 
                    0}
            }
        })
        .collect();
    println!("Areas {:?}", areas);   
    println!("Num areas {:?}", areas.len()); 

    let aa: Vec<_> = glyphs.iter().map(|g| {
        let mut running_mean = 0.;
        let mut counter = 0;
        g.draw(|x, y, v| {
            counter += 1;
            running_mean += v;
        });
        if counter == 0{
            return 0.;
        }
        running_mean/counter as f32
    }).collect();


    println!("sdf {:?}", aa);

}

