use card::FontConfig;
use image::{Pixel, Rgba};
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

    let character = &config.characters[0];

    let card = card::make_card(
        character.name.as_str(),
        character.health,
        make_skills_text(&character.skills),
        character.picture.as_str(),
        get_frame_path(
            character.kingdom.as_str(),
            character.monarch,
            &config.assets.frames
        ),
        get_health_path(
            character.kingdom.as_str(),
            character.monarch,
            &config.assets.health,
        ),
        get_name_color(
            character.kingdom.as_str(),
            character.monarch,
        ),
        get_box_color(
            character.kingdom.as_str(),
            character.monarch,
        ),
        1024,
        fonts
    );

    card.save(String::from("./output/image.png")).unwrap();
}

fn make_skills_text(skills: &Vec<config::Skill>) -> String {
    skills.iter().map(|s| -> String {
        let name = s.name.split(' ').map(|x| format!("@bold;{}", x)).collect::<Vec<String>>().join(" ");
        format!("{}\n{}", name, s.description)
    }).collect::<Vec<String>>().join("\n\n")
}

fn get_frame_path<'a>(kingdom: &str, monarch: bool, frames: &'a config::Frames) -> &'a str {
    match (kingdom, monarch) {
        ("kingdomless", false) => frames.kingdomless.as_str(),
        ("shu", false) => frames.shu.as_str(),
        ("wei", false) => frames.wei.as_str(),
        ("wu", false) => frames.wu.as_str(),
        ("kingdomless", true) => frames.kingdomless_zhu.as_str(),
        ("shu", true) => frames.shu_zhu.as_str(),
        ("wei", true) => frames.wei_zhu.as_str(),
        ("wu", true) => frames.wu_zhu.as_str(),
        _ => panic!("Not a valid kingdom!")
    }
}

fn get_health_path<'a>(kingdom: &str, monarch: bool, health: &'a config::Health) -> &'a str {
    if monarch {
        return health.zhu.as_str();
    }
    
    match kingdom {
        "kingdomless" => health.kingdomless.as_str(),
        "shu" => health.shu.as_str(),
        "wei" => health.wei.as_str(),
        "wu" => health.wu.as_str(),
        _ => panic!("Not a valid kingdom!")
    }
}

fn get_name_color(kingdom: &str, monarch: bool) -> Rgba<u8> {
    if monarch {
        return Rgba([]);
    }
    
    match kingdom {
        "kingdomless" => Rgba([]),
        "shu" => Rgba([]),
        "wei" => Rgba([]),
        "wu" => Rgba([]),
        _ => panic!("Not a valid kingdom!")
    }
}

fn get_box_color(kingdom: &str, monarch: bool) -> Rgba<u8> {
    if monarch {
        return Rgba([]);
    }
    
    match kingdom {
        "kingdomless" => Rgba([]),
        "shu" => Rgba([]),
        "wei" => Rgba([]),
        "wu" => Rgba([]),
        _ => panic!("Not a valid kingdom!")
    }
}