use image::io::Reader as ImageReader;
use image::{GenericImageView, GrayImage, ImageBuffer, Luma};


pub fn get_value_image(image_path: &str, sub_image_height: u32, font_scale: f64) -> GrayImage {
    let img = ImageReader::open(image_path).unwrap().decode().unwrap().into_luma8();

    let window_height = sub_image_height as f64;
    let image_height = img.height() as f64;
    let image_width = img.width() as f64;
    let num_rows = (image_height / window_height).floor() as f64;

    let window_width = (window_height/font_scale).floor() as f64;
    let num_columns = (image_width / window_width).floor() as f64;

    println!("Number of ASCII rows: {}", num_rows as u32);
    println!("Number of ASCII columns: {}", num_columns as u32);
    
    println!("Image height: {}", image_height as u32);
    println!("Image width: {}", image_width as u32);

    println!("Window height: {}", window_height as u32);
    println!("Window width: {}", window_width as u32);

    let mut value_image: GrayImage = ImageBuffer::new(num_columns as u32, num_rows as u32);
    for (rowi, row) in (0..(img.height()-window_height as u32)).step_by(window_height as usize).enumerate() {       
        for (coli, col) in (0..(img.width()-window_width as u32)).step_by(window_width as usize).enumerate() {
            let sub_view: image::SubImage<&image::ImageBuffer<image::Luma<u8>, Vec<u8>>> = img.view(col, row, window_width as u32, window_height as u32);
            let mut window_value = 0.;
            for p in sub_view.pixels() {
                window_value += p.2.0[0] as f64;
            }           
            let p = value_image.get_pixel_mut(coli as u32, rowi as u32);
            *p = Luma([((window_value / (window_height*window_width)).floor()) as u8]);
        }

    }
    return value_image;
}


pub fn main() {
    let img = ImageReader::open("images/letters.png").unwrap().decode().unwrap().into_luma8();
    println!("Num Rows: {}", img.height());
    println!("Num columns: {}", img.width());
    let v = img.view(0,0, 3, 2);
    println!("{:?}", v.pixels().next().unwrap());
    for p in v.pixels() {
        println!("Pixel: {:?}", p.2.0[0]);
    }

    let s = 0.5;
    let num_rows: f64 = 5.;
    let image_height = img.height() as f64;
    let image_width = img.width() as f64;
    let num_columns = num_rows * s;
    let window_height = (image_height / num_rows).floor() as f64;
    let window_width = (image_width / num_columns).floor() as f64; 

    println!("Num columns: {}", num_columns as u32);
    println!("Num rows: {}", num_rows as u32);
    println!("Window height: {}", window_height as u32);
    println!("Window width: {}", window_width as u32);

    let mut value_image: GrayImage = ImageBuffer::new(num_columns as u32, num_rows as u32);
    for (rowi, row) in (0..img.height()-window_height as u32).step_by(window_height as usize).enumerate() {       
        for (coli, col) in (0..img.width()-window_width as u32).step_by(window_width as usize).enumerate() {
            let sub_view: image::SubImage<&image::ImageBuffer<image::Luma<u8>, Vec<u8>>> = img.view(col, row, window_width as u32, window_height as u32);
            let mut window_value = 0.;
            for p in sub_view.pixels() {
                window_value += p.2.0[0] as f64;
            }            
            print!("{} ", ((window_value / (window_height*window_width)).floor()) as u32);
            let mut p = value_image.get_pixel_mut(coli as u32, rowi as u32);
            *p = Luma([((window_value / (window_height*window_width)).floor()) as u8]);
        }
        println!("");
    }
    println!("{:?}", value_image);
    //let img = open_image("images/julia_coffee.jpg", Colors::Rgb).unwrap();
    //println!("img shape: {:?}", img.shape());
    println!("Done")
}




