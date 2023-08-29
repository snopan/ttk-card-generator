use card::FontConfig;
use image::{Pixel};
use rusttype::{Font};

mod config;
mod card;

fn main() {

    let config = config::load_config(String::from("./temp_config.json")).unwrap();


    let font_regular = Font::try_from_vec(Vec::from(include_bytes!("../asset/regular.ttf") as &[u8])).unwrap();
    let font_bold = Font::try_from_vec(Vec::from(include_bytes!("../asset/bold.ttf") as &[u8])).unwrap();
    let font_title = Font::try_from_vec(Vec::from(include_bytes!("../asset/title.ttf") as &[u8])).unwrap();

    let fonts: FontConfig = FontConfig{
        font_regular: &font_regular,
        font_bold: &font_bold,
        font_title: &font_title
    };

    let layout = card::layout::new_layout(1024);

    let card = card::make_card(
        config.characters[0].name.as_str(),
        config.characters[0].picture.as_str(),
        config.assets.frames.shu.as_str(),
        config.assets.health.shu.as_str(),
        config.characters[0].health,
        make_skills_text(&config.characters[0].skills),
        fonts,
        &layout,
    );

    card.save(String::from("./output/image.png")).unwrap();
}

fn make_skills_text(skills: &Vec<config::Skill>) -> String {
    skills.iter().map(|s| -> String {
        let name = s.name.split(' ').map(|x| format!("@bold;{}", x)).collect::<Vec<String>>().join(" ");
        format!("{}\n{}", name, s.description)
    }).collect::<Vec<String>>().join("\n\n")
}
