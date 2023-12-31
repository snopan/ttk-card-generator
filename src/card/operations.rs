use image::{io::Reader, ImageBuffer};

use image::{
    imageops::{overlay, resize, FilterType},
    DynamicImage, Pixel, Rgba,
};

use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use text_render::{
    render::{DrawTextInput, RenderConfig},
    text::split_text,
};

mod resize;
mod text_box;

type Card = ImageBuffer<Rgba<u8>, Vec<<Rgba<u8> as Pixel>::Subpixel>>;

pub fn draw_avatar(
    card: &mut Card,
    avatar_path: &str,
    content_top_left: (i64, i64),
    content_width: f32,
    content_height: f32,
) {
    let avatar = Reader::open(avatar_path).unwrap().decode().unwrap();

    // Get new size for avatar to fit into current card
    let (avatar_width, avatar_height) = resize::reisize_maintain_ratio(
        avatar.width() as f32,
        avatar.height() as f32,
        content_width,
        content_height,
    );

    // Resize the actual avatar
    let avatar = resize::<DynamicImage>(
        &avatar,
        avatar_width as u32,
        avatar_height as u32,
        FilterType::Lanczos3,
    );

    overlay(card, &avatar, content_top_left.0, content_top_left.1);
}

pub fn draw_frame(card: &mut Card, frame_path: &str) {
    let frame = Reader::open(frame_path).unwrap().decode().unwrap();
    let frame = resize::<DynamicImage>(&frame, card.width(), card.height(), FilterType::Lanczos3);
    overlay(card, &frame, 0, 0);
}

pub fn generate_draw_text_details<'a>(
    skills: String,
    font_regular: &'a Font,
    font_bold: &'a Font,
    font_scale: Scale,
    font_color: Rgba<u8>,
    space_width: u32,
    skills_text_width: u32,
) -> (Vec<DrawTextInput<'a>>, u32) {
    let parsed_text = split_text(skills);
    let mut render_config = RenderConfig::new(font_regular, font_bold, font_scale, font_color);
    render_config.set_space_width(space_width);
    text_render::render::generate_draw_text_details(parsed_text, skills_text_width, render_config)
}

pub fn draw_skills_text_box(
    card: &mut Card,
    box_top_left: (i64, i64),
    box_width: u32,
    box_height: u32,
    outline: u32,
    gap: u32,
    corner_size: u32,
    box_color: Rgba<u8>,
) {
    let skills_text_box =
        text_box::make_skills_text_box(box_width, box_height, outline, gap, corner_size, box_color);
    overlay(card, &skills_text_box, box_top_left.0, box_top_left.1);
}

pub fn draw_name_text<'a>(
    card: &mut Card,
    x: i32,
    y: i32,
    scale: Scale,
    font: &'a Font<'a>,
    text: &'a str,
    inner_outline_color: Rgba<u8>,
    inner_outline_size: i32,
    outer_outline_size: i32,
) {
    let white = Rgba([255u8, 255u8, 255u8, 255u8]);
    let black = Rgba([0u8, 0u8, 0u8, 255u8]);
    let offset: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    for (xo, yo) in offset {
        draw_text_mut(
            card,
            black,
            x + xo * outer_outline_size,
            y + yo * outer_outline_size,
            scale,
            font,
            text,
        );
        draw_text_mut(
            card,
            inner_outline_color,
            x + xo * inner_outline_size,
            y + yo * inner_outline_size,
            scale,
            font,
            text,
        );
    }
    draw_text_mut(card, white, x, y, scale, font, text);
}

pub fn draw_health(
    card: &mut Card,
    health_path: &str,
    num_hearts: u32,
    top_right_x: i64,
    top_right_y: i64,
    offset: i64,
    length: u32,
) {
    let health = Reader::open(health_path).unwrap().decode().unwrap();
    let health = resize::<DynamicImage>(&health, length, length, FilterType::Lanczos3);
    for i in 0..num_hearts {
        overlay(
            card,
            &health,
            top_right_x - (i as i64 * offset),
            top_right_y,
        );
    }
}

pub fn draw_gender(
    card: &mut Card,
    gender_path: &str,
    top_left_x: i64,
    top_left_y: i64,
    length: u32,
) {
    let gender = Reader::open(gender_path).unwrap().decode().unwrap();
    let gender = resize::<DynamicImage>(&gender, length, length, FilterType::Lanczos3);
    overlay(card, &gender, top_left_x, top_left_y);
}
