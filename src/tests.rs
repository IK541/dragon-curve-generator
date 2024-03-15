#[cfg(test)]

    use super::*;

    #[test]
    fn make_dragon_0() {
        let dragon = generate_dragon(0);
        let expected_dragon = vec![true];
        assert_eq!(dragon, expected_dragon);
    }

    #[test]
    fn make_dragon_3() {
        let dragon = generate_dragon(3);
        let expected_dragon =
        vec![true,true,false,true,true,false,false,true,true, true, false, false, true, false, false];
        assert_eq!(dragon, expected_dragon);
    }

    #[test]
    fn dimensions_0u() {
        // let dragon = generate_dragon(0);
        let dimensions = find_dragon_dimensions(0, Direction::Up);
        let expected_dimensions = Dimensions{
            x: 2,
            y: 2,
            init_x: 0,
            init_y: 0,
        };
        assert_eq!(dimensions, expected_dimensions);
    }

    #[test]
    fn dimensions_2u() {
        // let dragon = generate_dragon(2);
        let dimensions = find_dragon_dimensions(2, Direction::Up);
        let expected_dimensions = Dimensions{
            x: 3,
            y: 4,
            init_x: 0,
            init_y: 2,
        };
        assert_eq!(dimensions, expected_dimensions);
    }

    #[test]
    fn dimensions_3l() {
        // let dragon = generate_dragon(3);
        let dimensions = find_dragon_dimensions(3, Direction::Left);
        let expected_dimensions = Dimensions{
            x: 6,
            y: 4,
            init_x: 1,
            init_y: 1,
        };
        assert_eq!(dimensions, expected_dimensions);
    }

    #[test]
    fn draw_3l() {
        let curve = generate_dragon(3);
        let dimensions = find_dragon_dimensions(3, Direction::Left);
        let colors = Colors {
            beg: Color { r: 0, g: 0, b: 0 },
            end: Color { r: 0, g: 0, b: 0 },
            back: Color { r: 255, g: 255, b: 255 } };
        let img = curve_to_img(&dimensions, &curve, Direction::Left, &colors, false);
        let mut expected_img = RgbImage::new(dimensions.x, dimensions.y);
        let black: image::Rgb<u8> = image::Rgb{0: [0,0,0]};
        let white: image::Rgb<u8> = image::Rgb{0: [255,255,255]};
        expected_img[(0,0)] = white; expected_img[(1,0)] = black; expected_img[(2,0)] = black; expected_img[(3,0)] = black; expected_img[(4,0)] = black; expected_img[(5,0)] = white;
        expected_img[(0,1)] = black; expected_img[(1,1)] = black; expected_img[(2,1)] = black; expected_img[(3,1)] = black; expected_img[(4,1)] = black; expected_img[(5,1)] = white;
        expected_img[(0,2)] = black; expected_img[(1,2)] = black; expected_img[(2,2)] = white; expected_img[(3,2)] = black; expected_img[(4,2)] = black; expected_img[(5,2)] = black;
        expected_img[(0,3)] = white; expected_img[(1,3)] = white; expected_img[(2,3)] = white; expected_img[(3,3)] = white; expected_img[(4,3)] = black; expected_img[(5,3)] = black;
        assert_eq!(img,expected_img);
    }
