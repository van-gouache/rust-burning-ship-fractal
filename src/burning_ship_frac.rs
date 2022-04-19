
//!   Module contains funcs responsible for generating 
//!   a graph representing the burning_ship fractal func.
//!   Mandlebrot's ugly step sister.
//!   (non Cauchy–Riemann equation)
//!   Z[n+1] = (|Re(Z[n])| + |Im(Z[n])|i)^2 + C
//!   @author Van Gouache



//max length of burning_ship sequence 
pub const MAX_ITERATIONS : u8 = 100;
//prints debug logs if true
const DEBUG_MODULE : bool = false;

pub type Fractal = Vec<Vec<u8>>;
type Range = (f64, f64);

#[derive(Debug)]
struct ComplexNumber{   
    a : f64,
    b : f64,
}

// sqrt shorthand
fn sqr(x : f64) -> f64{
    x * x
}


///    ### (PURE)
///    Calcualtes Z\[n+1\] in burning frac func
///    Z\[n+1\] = (|Re(Z\[n\])| + |Im(Z\[n\])|i)^2 + C
///    where Z\[0\] = 0 and C = a + bi where 
///    a = (x pixel coordinate) and b = (y pixel coordinate)
fn calculate_next_z(constant : &ComplexNumber, prev :&ComplexNumber) -> ComplexNumber{
    let sqr_a = sqr(prev.a);
    let sqr_b = sqr(prev.b);

    if sqr_a.is_infinite() || sqr_b.is_infinite(){
        return ComplexNumber{
            a : f64::INFINITY,
            b : f64::INFINITY
        };
    }

    let new_a = sqr_a - sqr_b + constant.a;
    let new_b = (2.0 * prev.a * prev.b).abs() + constant.b;

    
    ComplexNumber { 
        a: new_a, 
        b: new_b 
    }
}


///    ### (PURE)
///  Predicate to determine if burning_ship sequence is still in orbit.
fn orbit_contained(z : &ComplexNumber) -> bool{
    match z.a.is_infinite() || z.b.is_infinite(){
        true => false,
        false =>{
            (sqr(z.a) + sqr(z.b)) < 4.0
        } 
    }
}



///    ### (PURE)
///    Calculates the orbit rate for a given pixel. \[0 to MAX_ITERATIONS\]
fn get_orbit_rate(
    x : usize, 
    y: usize, 
    x_step_size: f64, 
    y_step_size : f64, 
    a_floor : f64, 
    b_floor : f64
) -> u8
{
    let starting_a = a_floor +  (x as f64 * x_step_size);
    let starting_b = b_floor + (y as f64 * y_step_size);
    let constant = ComplexNumber {
        a : starting_a,
        b : starting_b
    };
    let mut i = 0;
    let mut z = ComplexNumber {
        a : starting_a,
        b : starting_b
    };
    while i < MAX_ITERATIONS && orbit_contained(&z) {
        z = calculate_next_z(&constant, &z);
        i = i + 1;
    }
    i
}



///    ### (PURE)
///    Calculates the height and width of the current frame given zoom_rate and frame number.
///    Returns the new x and y ranges for zoom.
fn calc_zoomed_ranges(
    starting_width : f64,
    starting_height : f64,
    starting_x_range : Range,
    starting_y_range : Range,
    frame_number : u16,
    zoom_rate : f64
) -> (Range, Range)
{
    let (x_floor, x_ceil) = starting_x_range;
    let (y_floor, y_ceil) = starting_y_range;

    let scale = zoom_rate.powf(frame_number as f64);
    let curr_width = scale * starting_width;
    let curr_height = scale * starting_height;

    let focus_x = (starting_width - curr_width) / 2.0;
    let focus_y = (starting_height - curr_height) / 2.0;

    let x_range = ( 
        focus_x + x_floor,
        -focus_x + x_ceil,
    );
    let y_range = ( 
        focus_y + y_floor, 
        -focus_y + y_ceil
    );
    (x_range, y_range)
}




///    ### (PURE)
///    Takes a row of pixels and maps each entry to orbit 
///    representation 0 to MAX_ITERATIONS

fn map_row(
    curr_row_tuple : (usize, Vec<u8>), 
    x_step_size: f64, y_step_size : f64,
    x_range : Range, 
    y_range : Range
) -> Vec<u8>
{
    let (row_index, curr_row) = curr_row_tuple;
    let (x_floor, _) = x_range;
    let (y_floor, _) = y_range;
    let updated_row = curr_row
    .iter()
    .enumerate()
    .map(|curr_cell_tuple| {
        let (col_index, _) = curr_cell_tuple;
        get_orbit_rate(
            col_index,
            row_index,
             x_step_size,
             y_step_size, 
             x_floor,
              y_floor
        )
    });
    updated_row.collect()
}


///    ### (PURE)
///    Maps each row of pixels to corresponding orbit rate.
fn gen_burning_ship_fractal(
    img_width : usize,
    img_height : usize, 
    x_range : Range,
    y_range : Range,
    x_step_size : f64, 
    y_step_size : f64
) -> Fractal
{

    let grid : Fractal = vec![vec![0; img_width]; img_height];

    // println!("x_step_size: {}\ny_step_size: {}", x_step_size, y_step_size);
    grid.into_iter().enumerate().map(|curr_row_tuple|{
        map_row(curr_row_tuple, x_step_size, y_step_size, x_range, y_range)
    }).collect()

}


///    ### (PURE)
///    Calculates the size of each pixel in terms of the burning_ship fractal func.
fn calc_step_size(
    img_width : usize,
    img_height : usize, 
    x_range : (f64, f64), 
    y_range : (f64, f64)
) -> (f64, f64){
    let (x_floor, x_ceil) = x_range;
    let (y_floor, y_ceil) = y_range;
    let step_width = (x_ceil - x_floor).abs();
    let step_height = (y_ceil - y_floor).abs();
    let x_step_size = step_width/ img_width as f64;
    let y_step_size = step_height / img_height as f64;
    (x_step_size, y_step_size)
}



///    ### (PURE)
///    Calculates box width and height of view port given x_range and y_range.
pub fn calc_box_height_width(
    starting_x_range : Range,
    starting_y_range : Range,
) -> (f64, f64)
{
    let (x_floor, x_ceil) = starting_x_range;
    let (y_floor, y_ceil) = starting_y_range;
    let starting_width = (x_ceil - x_floor).abs();
    let starting_height = (y_ceil - y_floor).abs();
    (starting_width, starting_height)
}


///    ### (PURE) 
///    Composes functions:\ 
///    calc_box_height_width ->\
///    calc_zoomed_ranges ->\
///    calc_step_size ->\
///    gen_burning_ship_fractal\
///    To return a frame with each burning_ship fractal orbit calculated for some frame.
pub fn build_frame(
    img_width : usize,
    img_height : usize,
    starting_x_range : (f64, f64),
    starting_y_range : (f64, f64),
    frame_number : u16,
    zoom_rate : f64,
) -> Fractal
{
    //manual composition
    let (starting_width, starting_height) = calc_box_height_width(
        starting_x_range, 
        starting_y_range
    );
    // ⬇
    let (x_range, y_range) = calc_zoomed_ranges(
        starting_width, 
        starting_height, 
        starting_x_range, 
        starting_y_range, 
        frame_number, 
        zoom_rate
    );
    // ⬇
    let (x_step_size, y_step_size) = calc_step_size(
        img_width, 
        img_height, 
        x_range, 
        y_range
    );
    // ⬇
    let final_frame = gen_burning_ship_fractal(
        img_width,
        img_height, 
        x_range, 
        y_range,
        x_step_size,
        y_step_size
    );

    if DEBUG_MODULE{
        println!("\n~~~Finished building frame {}~~~", frame_number);
    }

    final_frame
}


