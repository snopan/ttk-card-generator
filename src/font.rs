use std::{
    fs::File,
    io::{self, BufReader, Read},
};

use rusttype::Font;

#[derive(Debug)]
pub enum Error {
    OpenFileFail(io::Error),
    ReadBufferFail(io::Error),
    ConvertFontFail,
}
pub fn load_font<'a>(path: String) -> Result<Font<'a>, Error> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(error) => return Err(Error::OpenFileFail(error)),
    };

    let mut buf_reader = BufReader::new(file);
    let mut buf: Vec<u8> = vec![];
    match buf_reader.read_to_end(&mut buf) {
        Err(error) => return Err(Error::ReadBufferFail(error)),
        _ => "",
    };

    match Font::try_from_vec(buf.to_vec()) {
        Some(f) => Ok(f),
        None => Err(Error::ConvertFontFail),
    }
}
