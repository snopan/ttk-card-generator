
use std::thread;

use card::FontConfig;

mod config;
mod card;
mod character;
mod font;

fn main() {

    let characters = config::load_characters(String::from("./characters.json")).unwrap();
    let assets = config::load_assets(String::from("./assets.json")).unwrap();
    let styles = config::load_styles(String::from("./styles.json")).unwrap();

    let font_regular = font::load_font(assets.fonts.regular).unwrap();
    let font_bold = font::load_font(assets.fonts.bold).unwrap();
    let font_title = font::load_font(assets.fonts.title).unwrap();

    let handle = thread::spawn(move || {
        for c in characters.list {
            let name = c.name.as_str();
            let avatar_path = character::get_avatar_path(name, &assets.characters);
            let kingdom = c.kingdom.as_str();
            let monarch = c.monarch;

            println!("Creating card {}", name);
            println!("With avatar {}", avatar_path);

            let card = card::make_card(
                name,
                c.health,
                character::make_skills_text(&c.skills),
                avatar_path,
                character::get_frame_path(kingdom, monarch, &assets.frames),
                character::get_health_path(kingdom, monarch, &assets.health),
                character::get_gender_path(kingdom, monarch, c.male, &assets.genders),
                character::get_name_color(kingdom, &styles.name_outline_colors),
                character::get_box_color(kingdom, monarch, &styles.skill_box_colors),
                1024,
                FontConfig{
                    font_regular: &font_regular,
                    font_bold: &font_bold,
                    font_title: &font_title
                }
            );

            card.save(String::from(format!("./output/{}.png", name))).unwrap();
        }
    });

    handle.join().unwrap();
}
