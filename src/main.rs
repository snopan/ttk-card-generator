
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
        character::make_skills_text(&character.skills),
        assets.characters[name].as_str(),
        character::get_frame_path(kingdom, monarch, &assets.frames),
        character::get_health_path(kingdom, monarch, &assets.health),
        character::get_name_color(kingdom, monarch, &styles.name_outline_colors),
        character::get_box_color(kingdom, monarch, &styles.skill_box_colors),
        1024,
        fonts
    );

    card.save(String::from("./output/image.png")).unwrap();
}
