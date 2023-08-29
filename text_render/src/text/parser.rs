use image::Rgba;
use super::types::{ Segment, Format };

#[derive(Debug, PartialEq)]
pub enum NotFormattedWordError {
    ZeroLengthWord,
    NoStartCharacter,
    InvalidSeparator
}

#[derive(Debug, PartialEq)]
pub enum Error {
    NotFormattedWord(NotFormattedWordError),
    InvalidFormat(String),
}

const STARTING_CHAR: char = '@';
const SEPARATOR_CHAR: char = ';';

pub fn get_formatted_word(word: &str) -> Result<Segment, Error> {
    if word.len() == 0 {
        return Err(Error::NotFormattedWord(NotFormattedWordError::ZeroLengthWord));
    }

    if word.chars().nth(0).unwrap() != STARTING_CHAR {
        return Err(Error::NotFormattedWord(NotFormattedWordError::NoStartCharacter));
    }

    let mut split = word[1..].split(SEPARATOR_CHAR);

    let (Some(raw_formats), Some(text), None) = (split.next(), split.next(), split.next()) else {
        return Err(Error::NotFormattedWord(NotFormattedWordError::InvalidSeparator));
    };

    let mut formats: Vec<Format> = vec![];
    for rf in raw_formats.split(",") {
        let f = match rf {
            "bold" => Some(Format::Bold),
            "red" => Some(Format::Color(color_to_rgb(rf))),
            _ => None
        };

        // Invalid format detected
        let Some(f) = f else {
            return Err(Error::InvalidFormat(String::from(rf)));
        };

        formats.push(f);
    }

    Ok(Segment::Word(String::from(text), formats))
}

fn color_to_rgb(color: &str) -> Rgba<u8> {
    match color {
        "red" => Rgba([255u8, 51u8, 34u8, 1u8]),
        _ => panic!("invalid color detected, this shouldn't happen")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_formatted_word_fail() {
        assert_eq!(
            get_formatted_word(""), 
            Err(Error::NotFormattedWord(NotFormattedWordError::ZeroLengthWord))
        );

        assert_eq!(
            get_formatted_word(format!("!abc{}efg", SEPARATOR_CHAR).as_str()), 
            Err(Error::NotFormattedWord(NotFormattedWordError::NoStartCharacter)), 
        );
    
        assert_eq!(
            get_formatted_word(format!("{}abc", STARTING_CHAR).as_str()), 
            Err(Error::NotFormattedWord(NotFormattedWordError::InvalidSeparator)),
            "no separator" 
        );

        assert_eq!(
            get_formatted_word(format!("{}abc{y}efg{y}hij", STARTING_CHAR, y=SEPARATOR_CHAR).as_str()), 
            Err(Error::NotFormattedWord(NotFormattedWordError::InvalidSeparator)),
            "too many separator" 
        );

        assert_eq!(
            get_formatted_word(format!("{}abc{}hello", STARTING_CHAR, SEPARATOR_CHAR).as_str()), 
            Err(Error::InvalidFormat(String::from("abc")))
        );
    }

    #[test]
    fn test_get_formatted_word_success() {
        assert_eq!(
            get_formatted_word(format!("{}bold,red{}hello", STARTING_CHAR, SEPARATOR_CHAR).as_str()),
            Ok(Segment::Word(
                String::from("hello"),
                vec![
                    Format::Bold,
                    Format::Color(color_to_rgb("red"))
                ]
            )), 
            "correctly retrieve a FormattedWord"
        );
    }
}