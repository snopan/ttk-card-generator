use image::Rgba;

use crate::config;

pub fn make_skills_text(skills: &[config::Skill]) -> String {
    skills.iter().map(|s| -> String {
        let name = s.name.split(' ').map(|x| format!("@bold;{}", x)).collect::<Vec<String>>().join(" ");
        format!("{}\n{}", name, s.description)
    }).collect::<Vec<String>>().join("\n\n")
}

pub fn get_frame_path<'a>(kingdom: &str, monarch: bool, frames: &'a config::Frames) -> &'a str {
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

pub fn get_health_path<'a>(kingdom: &str, monarch: bool, health: &'a config::Health) -> &'a str {
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

pub fn get_name_color(kingdom: &str, monarch: bool, colors: &config::NameOutlineColors) -> Rgba<u8> {
    match kingdom {
        "kingdomless" => Rgba(colors.kingdomless),
        "shu" => Rgba(colors.shu),
        "wei" => Rgba(colors.wei),
        "wu" => Rgba(colors.wu),
        _ => panic!("Not a valid kingdom!")
    }
}

pub fn get_box_color(kingdom: &str, monarch: bool, colors: &config::SkillBoxColors) -> Rgba<u8> {
    if monarch {
        return Rgba(colors.zhu);
    }
    
    match kingdom {
        "kingdomless" => Rgba(colors.kingdomless),
        "shu" => Rgba(colors.shu),
        "wei" => Rgba(colors.wei),
        "wu" => Rgba(colors.wu),
        _ => panic!("Not a valid kingdom!")
    }
}