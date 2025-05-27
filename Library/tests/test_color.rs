use Library::color::{self, blue, bold, green, red, reset, Color, ColorString};

#[test]
pub fn test_red_coloring()
{
    let mut color_string = ColorString {
        color: Color::Red,
        string: String::from("This is red"),
        colorized: String::new(),
    };
    ColorString::paint(&mut color_string);
    assert_eq!(color_string.colorized, "\x1b[31mThis is red\x1b[0m");
}