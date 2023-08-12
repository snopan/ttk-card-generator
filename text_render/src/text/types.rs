use image::Rgb;

#[derive(Debug, PartialEq)]
pub enum Format {
    Bold,
    Color(Rgb<u8>)
}

#[derive(Debug, PartialEq)]
pub enum Segment {
    Word(String, Vec<Format>),
    NewLine
}