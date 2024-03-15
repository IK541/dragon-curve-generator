use image::Rgb;

#[derive(Clone,Copy,Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn mean(beg_color: Color, end_color: Color, weight: f32) -> Color {
        let ans: Color = Color {
            r:(((beg_color.r as f32)*(1.0-weight)+(end_color.r as f32)*weight) as u8),
            g:(((beg_color.g as f32)*(1.0-weight)+(end_color.g as f32)*weight) as u8),
            b:(((beg_color.b as f32)*(1.0-weight)+(end_color.b as f32)*weight) as u8),
        };
        ans
    }
    pub fn to_rgb(&self) -> Rgb<u8> {
        Rgb { 0: [self.r, self.g, self.b] }
    }
}

pub struct Colors {
    pub beg: Color,
    pub end: Color,
    pub back: Color,
}
