
#[derive(Clone, Debug)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8) -> Colour {
        Colour {
            r, g, b
        }
    }
}

pub fn ansi_text_colour_string(r: u8, g: u8, b: u8) -> String {
    let mut ansi = String::new();
    ansi.push_str("\x1b[38;2;");
    ansi.push_str(r.to_string().as_str());
    ansi.push_str(";");
    ansi.push_str(g.to_string().as_str());
    ansi.push_str(";");
    ansi.push_str(b.to_string().as_str());
    ansi.push_str("m");

    ansi
}