
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

    let c = &characters.list[0];
    let name = c.name.as_str();
    let kingdom = c.kingdom.as_str();
    let monarch = c.monarch;

    let card = card::make_card(
        name,
        c.health,
        character::make_skills_text(&c.skills),
        character::get_avatar_path(name, &assets.characters),
        character::get_frame_path(kingdom, monarch, &assets.frames),
        character::get_health_path(kingdom, monarch, &assets.health),
        character::get_gender_path(kingdom, monarch, c.male, &assets.genders),
        character::get_name_color(kingdom, &styles.name_outline_colors),
        character::get_box_color(kingdom, monarch, &styles.skill_box_colors),
        1024,
        fonts
    );

    card.save(String::from("./output/image.png")).unwrap();
}
