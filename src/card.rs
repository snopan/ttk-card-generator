

use image::{ImageBuffer, Rgba, RgbaImage, io::Reader, Pixel, imageops::{resize, FilterType, overlay}, DynamicImage};

use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use text_render::{render::{RenderConfig, DrawTextInput, render_draw_text_inputs}, text::split_text};

type Card = ImageBuffer<Rgba<u8>, Vec<<Rgba<u8> as Pixel>::Subpixel>>;

pub mod layout;
mod text_box;
mod resize;

pub enum Error {

}

pub struct FontConfig<'a> {
    pub font_regular: &'a Font<'a>,
    pub font_bold: &'a Font<'a>,
    pub font_title: &'a Font<'a>,
}

pub fn make_card(
    name: &str,
    avatar: &str,
    frame: &str,
    health: &str,
    num_health: u32,
    skills: String,
    fonts: FontConfig,
    layout: &layout::Layout
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    
    let mut card = RgbaImage::new(layout.card_width(), layout.card_height());
    
    draw_avatar(
        &mut card,
        avatar,
        (layout.content_top_left_x() as i64, layout.content_top_left_y() as i64),
        layout.content_width() as f32,
        layout.content_height() as f32
    );

    draw_frame(&mut card, frame);

    let (draw_text_inputs, skills_text_height) = generate_draw_text_details(
        skills,
        fonts.font_regular,
        fonts.font_bold,
        Scale {
            x: layout.skills_text_scale() as f32,
            y: layout.skills_text_scale() as f32
        },
        Rgba([0u8, 0u8, 0u8, 255u8]),
        layout.skills_text_space_width(),
        layout.skills_text_width()
    );

    draw_skills_text_box(
        &mut card,
        (
            layout.skills_box_top_left_x() as i64,
            layout.skills_box_top_left_y(skills_text_height) as i64
        ),
        layout.skills_box_width(),
        layout.skills_box_height(skills_text_height),
        layout.skills_box_outline(),
        layout.skills_box_gap(),
        layout.skills_box_corner(),
        Rgba([255u8, 255u8, 255u8, 150u8])
    );

    render_draw_text_inputs(
        &mut card,
        draw_text_inputs,
        (
            layout.skills_text_top_left_x() as i32,
            layout.skills_text_top_left_y(skills_text_height) as i32,
        )
    );

    draw_outline_text_mut(
        &mut card, 
        layout.name_top_left_x() as i32,
        layout.name_top_left_y(skills_text_height) as i32,
        Scale {
            x: layout.name_text_scale() as f32,
            y: layout.name_text_scale() as f32
        }, 
        fonts.font_title,
        name,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        4,
        Rgba([0u8, 0u8, 0u8, 255u8])
    );

    draw_health(
        &mut card,
        health,
        num_health,
        layout.health_top_right_x() as i64,
        layout.health_top_right_y(skills_text_height) as i64,
        layout.health_offset() as i64,
        layout.health_length()
    );

    card
}

fn draw_avatar(
    card: &mut Card,
    avatar_path: &str,
    content_top_left: (i64, i64),
    content_width: f32,
    content_height: f32
) {
    let avatar = Reader::open(avatar_path).unwrap().decode().unwrap();

    // Get new size for avatar to fit into current card
    let (avatar_width, avatar_height) = resize::reisize_maintain_ratio(
        avatar.width() as f32,
        avatar.height() as f32,
        content_width,
        content_height
    );

    // Resize the actual avatar
    let avatar = resize::<DynamicImage>(
        &avatar,
        avatar_width as u32,
        avatar_height as u32,
        FilterType::Lanczos3
    );

    overlay(card, &avatar, content_top_left.0, content_top_left.1);
}

fn draw_frame(card: &mut Card, frame_path: &str) {
    let frame = Reader::open(frame_path).unwrap().decode().unwrap();
    let frame = resize::<DynamicImage>(
        &frame,
        card.width(),
        card.height(),
        FilterType::Lanczos3
    );
    overlay(card, &frame, 0, 0);
}

fn generate_draw_text_details<'a>(
    skills: String,
    font_regular: &'a Font,
    font_bold: &'a Font,
    font_scale: Scale,
    font_color: Rgba<u8>,
    space_width: u32,
    skills_text_width: u32
) -> (Vec<DrawTextInput<'a>>, u32) {
    let parsed_text = split_text(skills);
    let mut render_config = RenderConfig::new(
        font_regular,
        font_bold,
        font_scale,
        font_color
    );
    render_config.set_space_width(space_width);
    text_render::render::generate_draw_text_details(
        parsed_text, 
        skills_text_width, 
        render_config
    )
}

fn draw_skills_text_box(
    card: &mut Card,
    box_top_left: (i64, i64),
    box_width: u32,
    box_height: u32,
    outline: u32,
    gap: u32,
    corner_size: u32,
    box_color: Rgba<u8>
) {
    let skills_text_box = text_box::make_skills_text_box(
        box_width,
        box_height, 
        outline,
        gap,
        corner_size,
        box_color
    );
    overlay(card, &skills_text_box, box_top_left.0, box_top_left.1);
}

fn draw_outline_text_mut<'a>(
    card: &mut Card,
    x: i32,
    y: i32,
    scale: Scale,
    font: &'a Font<'a>,
    text: &'a str,
    text_color: Rgba<u8>,
    outline_size: i32,
    outline_color: Rgba<u8>
) {
    let offset: Vec<(i32, i32)> = vec![
        (0, outline_size),
        (0, -outline_size),
        (outline_size, 0),
        (-outline_size, 0),
    ];

    for (x_offset, y_offset) in offset {
        draw_text_mut(card, outline_color, x + x_offset, y + y_offset, scale, font, text);
    }
    draw_text_mut(card, text_color, x, y, scale, font, text);
}

fn draw_health(
    card: &mut Card,
    health_path: &str,
    num_hearts: u32,
    top_right_x: i64,
    top_right_y: i64,
    offset: i64,
    length: u32,
) {
    let health = Reader::open(health_path).unwrap().decode().unwrap();
    let health = resize::<DynamicImage>(
        &health,
        length,
        length,
        FilterType::Lanczos3
    );
    for i in 0..num_hearts {
        overlay(
            card,
            &health,
            top_right_x - (i as i64 * offset),
            top_right_y
        );
    }
}