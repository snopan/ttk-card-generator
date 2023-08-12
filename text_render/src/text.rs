use log::warn;
use types::{ Segment, Format };
use parser::{ get_formatted_word };

pub mod types;
mod parser;

pub fn split_text(text: String) -> Vec<Segment> {
    let mut accumulated_words: Vec<Segment> = vec![];
    for (i, line) in text.split("\n").enumerate() {

        // Only start adding newlines after the first element
        if i != 0 {
            accumulated_words.push(Segment::NewLine);
        }

        // Split the line into words and try to parse any format
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }

        // Loop through the line and attempt to parse &str to Word
        let mut words: Vec<Segment> = line.split(" ").map(|word| -> Segment {            
            match get_formatted_word(word) {
                Ok(w) => w,
                Err(e) => {
                    if let parser::Error::InvalidFormat(format) = e {
                        warn!("warning: tried parsing the following format {} on word {}", format, word);
                    }

                    // Failing to parse as Word::Formatted means it's probably Word::Normal
                    Segment::Word(word.to_string(), vec![])
                }
            }
        }).collect();

        // Join the this line of words with the current accumulated words
        accumulated_words.append(&mut words);
    } 

    accumulated_words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_text() {
        assert_eq!(
            split_text(String::from("")),
            vec![],
            "empty text should return empty"
        );

        assert_eq!(
            split_text(String::from("   ")),
            vec![],
            "just spaces should return empty"
        );

        assert_eq!(
            split_text(String::from("\n\n\n")),
            vec![
                Segment::NewLine,
                Segment::NewLine,
                Segment::NewLine
            ],
            "just new lines are parsed"
        );

        assert_eq!(
            split_text(String::from("hello @bold:word")),
            vec![
                Segment::Word(String::from("hello"), vec![]),
                Segment::Word(String::from("word"), vec![Format::Bold])
            ],
            "normal single words are parsed correctly"
        );

        assert_eq!(
            split_text(String::from("hello\n\n  @bold:word \n\n")),
            vec![
                Segment::Word(String::from("hello"), vec![]),
                Segment::NewLine,
                Segment::NewLine,
                Segment::Word(String::from("word"), vec![Format::Bold]),
                Segment::NewLine,
                Segment::NewLine
            ],
            "new lines are parsed correctly along with the words"
        );

        assert_eq!(
            split_text(String::from("@abc:hello")),
            vec![Segment::Word(String::from("@abc:hello"), vec![])],
            "parsing word with bad format should result in using the whole string as word"
        );
    }
}