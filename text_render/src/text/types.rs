use image::Rgba;

#[derive(Debug, PartialEq)]
pub enum Format {
    Bold,
    Color(Rgba<u8>)
}

#[derive(Debug, PartialEq)]
pub enum Segment {
    Word(String, Vec<Format>),
    NewLine
}