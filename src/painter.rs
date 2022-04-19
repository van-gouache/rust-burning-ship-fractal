
//!   Module contains funcs responsible for assigning
//!   colors to escape-time values in a fractal 
//!   matrix and storing image on the hard disk.
//!   @author Van Gouache
 
use rand::prelude::*;
use image::*;

/// ### (PURE)
/// Builds a random RGB color making use of rand crate.
fn generate_random_color() -> image::Rgb<u8>{
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();
    let z: f64 = rng.gen();

    let r = (256.0 * x) as u8;
    let g = (256.0 * y) as u8;
    let b = (256.0 * z) as u8;
    image::Rgb([r,g,b])
}


///    ### (PURE)
///    Generates a palette of random colors.
pub fn generate_random_palette(
    number_of_colors : u8
) -> Vec<image::Rgb<u8>>
{
    let mut color_vec : Vec<image::Rgb<u8>>=  Vec::new();
    for i in (0..number_of_colors + 1).into_iter(){
        let color = generate_random_color();
        let _ = color_vec.insert(i as usize, color);
    }
    color_vec
}

 
///    ### (PURE)
///    Given a frame of orbits [0 to MAX_ITERATIONS], maps integer to 
///    color in palette such that each orbit rate is represented as a
///    unique color.
pub fn paint_frame(
    width: u32, 
    height: u32, 
    frame : &Vec<Vec<u8>>,
    palette : &Vec<image::Rgb<u8>>
) -> ImageBuffer<Rgb<u8>, Vec<u8>>{
    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (i, row) in frame.into_iter().enumerate(){
        for(j , cell) in row.into_iter().enumerate(){
            let color = palette.get(*cell as usize).unwrap();
            imgbuf.put_pixel(j as u32, i as u32, *color)
        }
    }
    imgbuf
}


///    ### (I/0)
///    Saves image buffer to file at "imgs/{frame_number}.png
pub fn save_img_buff(
    buffer : ImageBuffer<Rgb<u8>, Vec<u8>>,
    frame_number : u16
)-> ImageResult<()>
{
    let path = format!("frames/{:08}.png", frame_number);
    buffer.save(path)
}


///    ### (I/O)
///    Composes paint_frame and save_img_buff
pub fn paint_and_save_frame(
    width: u32, 
    height: u32, 
    frame : &Vec<Vec<u8>>,
    palette : &Vec<image::Rgb<u8>>,
    frame_number : u16
)  -> ImageResult<()>
{
    let buffer = paint_frame(width, height, frame, palette);
    // ⬇
    save_img_buff(buffer, frame_number)
    
}