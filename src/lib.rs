pub mod io;
pub mod color;
mod movement;
use std::cmp::{min,max};
use movement::Direction;
use color::{Color, Colors};

use image::{RgbImage, ImageBuffer};

/// Generates a dragon curve as a list of left/right turns (true - right, false - left)
/// 0 itertions corresponds to a single right turn
pub fn generate_dragon(iterations: u8) -> Vec<bool>{
    if iterations == 0 {return Vec::from([true])}
    else {
        let mut curve_1 = generate_dragon(iterations-1);
        let mut curve_2 = generate_dragon(iterations-1);
        curve_1.push(true);
        negate(&mut curve_2);
        curve_2.reverse();
        curve_1.append(&mut curve_2);
        curve_1
    }
}

/// Finds dimensions of dragon curve given the number of iterations and initial direction
/// dimensions are width, height and x and y coordinate of starting point
pub fn find_dragon_dimensions(mut iter: u8, direction: Direction) -> Dimensions{
    
    let (mut max_x, mut max_y): (i32, i32) = (1, 1);
    let (mut start_x, mut start_y): (i32, i32) = (0,0);
    let (mut end_x, mut end_y): (i32, i32) = (0,0);
    let (mut new_end_x, mut new_end_y): (i32, i32);
    let (mut new_min_x, mut new_min_y): (i32, i32);

    match direction {
        Direction::Up => {end_x = 1; end_y = 1; }
        Direction::Right => {start_y = 1; end_x = 1; }
        Direction::Down => {start_x = 1; start_y = 1; }
        Direction::Left => {start_x = 1; end_y = 1; }
    };

    while iter > 0 {
        new_min_x = min(0, end_x - max_y + end_y);
        new_min_y = min(0,end_y - end_x);
        max_y = max(max_y, end_y + max_x - end_x);
        max_x = max(max_x, end_x + end_y);

        new_end_x = end_x + end_y - start_y;
        new_end_y = end_y - end_x + start_x;
        end_x = new_end_x;
        end_y = new_end_y;

        start_x -= new_min_x;
        start_y -= new_min_y;
        end_x -= new_min_x;
        end_y -= new_min_y;
        max_x -= new_min_x;
        max_y -= new_min_y;
        
        iter -= 1;
    }

    Dimensions{
        x: (max_x+1) as u32,
        y: (max_y+1) as u32,
        init_x: start_x as u32,
        init_y: start_y as u32,
    }

}

/// Turns an array of left/right turns into an image of the dragon curve
pub fn curve_to_img(dimensions: &Dimensions,curve: &Vec<bool>, mut direction: Direction, colors: &Colors, flip_y_axis: bool)
-> RgbImage
{
    let mut img = ImageBuffer::new(dimensions.x, dimensions.y);
    for pixel in img.pixels_mut() {
        *pixel = colors.back.to_rgb();
    }

    let (mut x, mut y) = (dimensions.init_x, dimensions.init_y);
    let mut iter = 1;
    let y_max = dimensions.y-1;

    img[(x,if flip_y_axis {y} else {y_max-y})] = colors.beg.to_rgb();
    let length = curve.len() as f32;
    movement::move_u32(&mut direction, &mut x, &mut y);
    img[(x,if flip_y_axis {y} else {y_max-y})] = Color::mean(colors.beg, colors.end, (iter as f32)/length).to_rgb();

    for i in curve{
        iter+=1;
        if *i {direction=Direction::clockwise(direction);}
        else {direction=Direction::counterclockwise(direction);}
        movement::move_u32(&mut direction, &mut x, &mut y);
        img[(x,if flip_y_axis {y} else {y_max-y})] = Color::mean(colors.beg, colors.end, (iter as f32)/length).to_rgb();
    }

    img
}

// Writes an image to a file in the pictures folder
pub fn write_img (img: &RgbImage, file_name: &str, file_extension: &str) {
    let path = format!("pictures/{}.{}",file_name, file_extension);
    if !std::path::Path::new("pictures").is_dir() {
        std::fs::create_dir("pictures").expect("Failed to create folder");
    }
    img.save(path).expect("Failed to save image!");

}



fn negate(vector: &mut Vec<bool>){
    for elem in vector{
        *elem = !*elem;
    }
}

#[derive(Clone,PartialEq,Debug)]
pub struct Dimensions {
    pub x: u32,
    pub y: u32,
    pub init_x: u32,
    pub init_y: u32,
}

mod tests;
