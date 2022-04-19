
//! Module contains program entry point and main control 
//!   loop for generating fractal frames. 
//!   @author Van Gouache

use rayon::prelude::*;
mod burning_ship_frac;
mod painter;
use image::*;
use std::{time::Instant, env};


type ImgResult = Result<(), ImageError>;
static PRINT_ROW: &str = "=============================================";

///   ### (PURE)
///    Given a vec of frame_numbers, maps to complete fractal frames.
fn map_frames_to_fractals(
    img_width : usize,
    img_height : usize,
    starting_x_range : (f64, f64),
    starting_y_range : (f64, f64), 
    zoom_rate : f64, 
    frames : Vec<u16> 
) -> Vec<burning_ship_frac::Fractal> 
{
    frames
    .par_iter()
    .map(| i |{
        burning_ship_frac::build_frame(
            img_width, 
            img_height,  
            starting_x_range, 
            starting_y_range, 
            *i, 
            zoom_rate
        )
    }).collect()
}

/// ### (I/O)
/// Given a vec of fractal frames. Generates a list of I/O results
/// correlated to frame_number.png file
fn map_fractal_to_img_io_results(
    img_width : usize,
    img_height : usize,
    first_frame : u16,
    palette : &Vec<Rgb<u8>>,
    frames: Vec<burning_ship_frac::Fractal>
) -> Vec<ImgResult>
{
    frames
    .par_iter()
    .enumerate()
    .map(|fractal_data| {
        let (i, frame) = fractal_data;
        let frame_number = i + first_frame as usize;
        painter::paint_and_save_frame(
            img_width as u32, 
            img_height as u32, 
            &frame, 
            &palette, 
            frame_number as u16
        )
    }).collect()
}


///    ### (I/O)
///    Composes map_frames_to_fractals -> map_fractal_to_img_io_results
fn gen_and_save_frames(
    img_width : usize,
    img_height : usize,
    starting_x_range : (f64, f64),
    starting_y_range : (f64, f64), 
    zoom_rate : f64, 
    first_frame : u16,
    last_frame : u16,
    palette : &Vec<Rgb<u8>>
)
{
    let frames : Vec<u16> = (first_frame..last_frame).collect();
    println!("\n\n{}\nGENERATING FRAMES {}-{}\n{}", PRINT_ROW, first_frame, last_frame-1, PRINT_ROW);
    
    let prog_timer = Instant::now();
    let frames  : Vec<burning_ship_frac::Fractal> = map_frames_to_fractals(
        img_width, 
        img_height, 
        starting_x_range, 
        starting_y_range, 
        zoom_rate, 
        frames
    );
    // ⬇    
    let build_frame_time = prog_timer.elapsed();
    let _frame_results : Vec<ImgResult> = map_fractal_to_img_io_results(
        img_width, 
        img_height, 
        first_frame, 
        palette, 
        frames
    );

    let paint_frame_time = prog_timer.elapsed() - build_frame_time;
    println!(
        "Finished generating frames in {:?}\n{}\nFinished painting frames in {:?}s\n{}\nTotal Time: {:?}\n{}", 
        build_frame_time,
        PRINT_ROW, 
        paint_frame_time, 
        PRINT_ROW,
        prog_timer.elapsed(),
        PRINT_ROW
    );
}

fn main() {
    let img_width  = 4000;
    let img_height = 2300;
    let starting_x_range = (-3.45, 0.05);
    let starting_y_range = (-0.99,0.99);
    let zoom_rate = 0.96;
    let chunk_size = 4;
    let palette = painter::generate_random_palette(
        burning_ship_frac::MAX_ITERATIONS
    );

    let args: Vec<String> = env::args().collect();
    let bursts = args
    .get(1)
    .unwrap_or_else(||{
        println!("Did not specifiy burst argument! Program terminating");
        std::process::exit(1);
    }).parse::<u16>()
    .unwrap_or_else(|_|{
        println!("Failed to parse burst argument!");
        std::process::exit(1);
    });


    //main program loop, 
    //generates and saves frames in burst of chunk_size
    let total_timer = Instant::now();
    for i in 0..bursts{
        let first_frame = i * chunk_size;
        let last_frame = first_frame + chunk_size;
        gen_and_save_frames(
            img_width, 
            img_height, 
            starting_x_range, 
            starting_y_range, 
            zoom_rate,
            first_frame,
            last_frame,
            &palette
        );
    }
    println!("{}\nTotal Runtime: {:?}\n{}", PRINT_ROW, total_timer.elapsed(), PRINT_ROW)
}
