#[cfg(feature = "color")]
pub mod color {
    pub(crate) use rgb::RGB;

    pub fn draw_line(_x: i32, _y: i32, color: &RGB<u16>) {
        println!("{color}")
        // draw line with color
    }
}
