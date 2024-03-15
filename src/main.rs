use dragon::color::Colors;

fn main(){
    let iterations = dragon::io::get_iter();
    let dragon = dragon::generate_dragon(iterations);

    let direction = dragon::io::get_direction();
    let dragon_dimensions = dragon::find_dragon_dimensions(iterations, direction);
    dragon::io::write_dragon_dimensions(&dragon_dimensions);

    let picture_dimensions = dragon::io::get_picture_dimensions(&dragon_dimensions);
    let colors: Colors = dragon::io::get_colors();
    let flip_y_axis = dragon::io::get_flip_y_axis();
    let img = dragon::curve_to_img(&picture_dimensions, &dragon, direction, &colors, flip_y_axis);
    
    let file_name: String = dragon::io::get_file_name();
    let file_extension: String = dragon::io::get_file_extension();
    dragon::write_img(&img,&file_name,&file_extension);
}

