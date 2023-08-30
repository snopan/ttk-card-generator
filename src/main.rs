use card::FontConfig;
use image::{Pixel, Rgba};
use rusttype::{Font};

mod config;
mod card;

fn main() {

    let characters = config::load_characters(String::from("./characters.json")).unwrap();
    let assets = config::load_assets(String::from("./assets.json")).unwrap();
    let styles = config::load_styles(String::from("./styles.json")).unwrap();


    let font_regular = Font::try_from_vec(Vec::from(include_bytes!("../asset/regular.ttf") as &[u8])).unwrap();
    let font_bold = Font::try_from_vec(Vec::from(include_bytes!("../asset/bold.ttf") as &[u8])).unwrap();
    let font_title = Font::try_from_vec(Vec::from(include_bytes!("../asset/title.ttf") as &[u8])).unwrap();

    let fonts: FontConfig = FontConfig{
        font_regular: &font_regular,
        font_bold: &font_bold,
        font_title: &font_title
    };

    let character = &characters.list[0];
    let name = character.name.as_str();
    let kingdom = character.kingdom.as_str();
    let monarch = character.monarch;

    let card = card::make_card(
        name,
        character.health,
        make_skills_text(&character.skills),
        assets.characters[name].as_str(),
        get_frame_path(kingdom, monarch, &assets.frames),
        get_health_path(kingdom, monarch, &assets.health),
        get_name_color(kingdom, monarch, &styles.name_outline_colors),
        get_box_color(kingdom, monarch, &styles.skill_box_colors),
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

fn get_name_color<'a>(kingdom: &str, monarch: bool, colors: &'a config::NameOutlineColors) -> Rgba<u8> {
    if monarch {
        return Rgba(colors.zhu.clone());
    }
    
    match kingdom {
        "kingdomless" => Rgba(colors.kingdomless.clone()),
        "shu" => Rgba(colors.shu.clone()),
        "wei" => Rgba(colors.wei.clone()),
        "wu" => Rgba(colors.wu.clone()),
        _ => panic!("Not a valid kingdom!")
    }
}

fn get_box_color<'a>(kingdom: &str, monarch: bool, colors: &'a config::SkillBoxColors) -> Rgba<u8> {
    if monarch {
        return Rgba(colors.zhu.clone());
    }
    
    match kingdom {
        "kingdomless" => Rgba(colors.kingdomless.clone()),
        "shu" => Rgba(colors.shu.clone()),
        "wei" => Rgba(colors.wei.clone()),
        "wu" => Rgba(colors.wu.clone()),
        _ => panic!("Not a valid kingdom!")
    }
}