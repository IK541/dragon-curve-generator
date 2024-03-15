use std::io;
use crate::movement::Direction;
use crate::color::{Color,Colors};
use crate::Dimensions;

pub fn get_iter() -> u8 {
    let iter: u8;
    println!("enter the number of iterations (resource consumption rises exponentially)");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");

        let num;
        match input.trim().parse::<u8>() {
            Ok(n) => num = n,
            Err(_) => {println!("please enter a natural number from range [0;32]"); continue;}
        }; 
        if num > 32 {
            println!("please enter a natural number from range [0;32]");
            continue;
        }

        iter = num;
        break;
    } iter
}

pub fn get_direction() -> Direction {
    let direction: Direction;
    println!("enter initial direction for generating fractal [u/r/d/l]");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        direction = match input.trim() {
            "u" => Direction::Up,
            "r" => Direction::Right,
            "d" => Direction::Down,
            "l" => Direction::Left,
            _ => {println!("please enter 'u', 'r', 'd' or 'l'"); continue;}
        }; break;
    } direction
}

pub fn write_dragon_dimensions(dimensions: &Dimensions) {
    println!("The width of the dragon is {}", dimensions.x);
    println!("The height of the dragon is {}", dimensions.y);
}

pub fn get_picture_dimensions(dragon_dimensions: &Dimensions) -> Dimensions {
    let mut picture_dimensions = dragon_dimensions.clone();
    println!("enter the width and height for the picture (two space separated numbers each greater than corresponding dimension of the dragon),
can be substituted with '-' to take dragon's dimension as picture dimension");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let input: Vec<&str> = input.trim().split_whitespace().collect();
        if input.len() != 2 {
            println!("incorrect number of arguments");
            continue;
        }
        let mut picture_width = dragon_dimensions.x;
        let mut picture_height = dragon_dimensions.y;

        match input[0] {
            "-" => (),
            width => match width.parse::<u32>() {
                Ok(n) => picture_width = n,
                Err(_) => {println!("please enter a natural number or '-' for width"); continue;},
            },
        }
        match input[1] {
            "-" => (),
            height => match height.parse::<u32>() {
                Ok(n) => picture_height = n,
                Err(_) => {println!("please enter a natural number or '-' for height"); continue;},
            },
        }
        
        if picture_width < dragon_dimensions.x {
            println!("picture width cannot be less than dragon width"); continue;
        }
        if picture_height < dragon_dimensions.y {
            println!("picture height cannot be less than dragon height"); continue;
        }
        
        picture_dimensions.x = picture_width;
        picture_dimensions.y = picture_height;
        picture_dimensions.init_x += (picture_width - dragon_dimensions.x)/2;
        picture_dimensions.init_y += (picture_height - dragon_dimensions.y)/2;
        break;
    } picture_dimensions
}

pub fn get_colors() -> Colors {
    let mut colors: Vec<u8> = Vec::new();
    println!("enter the colors for the dragon (three space separated numbers from range [0;255])");
    for i in 0..3 {
        'main: loop {
            while colors.len() != 3*i {
                colors.pop();
            }

            match i {
                0 => println!("beginning color:"),
                1 => println!("end color:"),
                2 => println!("background color:"),
                _ => panic!("this state should not be reached"),
            }

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("failed to read line");
            for val in input.trim().split_whitespace() {
                match val.parse::<u8>() {
                    Ok(n) => colors.push(n),
                    Err(_) => {println!("please enter a natural number from range [0;255] for each of channels R, G and B"); continue 'main;}
                };
            }
            
            if colors.len() != 3*i+3 {
                println!("incorrect number of arguments");
                while colors.len() != 3*i {
                    colors.pop();
                }
            } else {break;}
        }
    }
    Colors {
        beg: Color{r: colors[0], g: colors[1], b: colors[2]},
        end: Color{r: colors[3], g: colors[4], b: colors[5]},
        back: Color{r: colors[6], g: colors[7], b: colors[8]},
    }
}

pub fn get_flip_y_axis() -> bool {
    println!("Do you want to flip y-axis? [y/n]");
    let flip_y_axis;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let input = input.trim();
        match input {
            "y" => flip_y_axis = true,
            "n" => flip_y_axis = false,
            _ => {println!("please enter 'y' or 'n'"); continue;}
        } break;
    } flip_y_axis
}

pub fn get_file_name() -> String {
    println!("enter file name for the dragon (no more than 256 characters, only alphanumberic characters or following: ( ) - _ , .
the picture will be saved in 'pictures' folder");
    let file_name;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        input = input.trim().to_string();
        if input.len() > 256 {println!("too long file name"); continue;}
        if input.chars().all(|c| c.is_ascii_alphanumeric() || "()-_,.".contains(c)) {file_name = input.clone(); break;}
        println!("invalid characters");
    } file_name
}

pub fn get_file_extension() -> String {
    println!("enter file extension [png/jpg/gif/bmp/tif/ppm]");
    let extension;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let input = input.trim();
        if !["png","jpg","gif","bmp","tif","ppm"].contains(&input) {
            println!("enter one of supported formats");
            continue;
        }
        extension = input.to_string();
        break;
    } extension
}