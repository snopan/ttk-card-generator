

use image::{ImageBuffer, Rgba, RgbaImage};

use rusttype::{Font, Scale};
use text_render::{render::{render_draw_text_inputs}};


pub mod layout;
mod operations;

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
    
    operations::draw_avatar(
        &mut card,
        avatar,
        (layout.content_top_left_x() as i64, layout.content_top_left_y() as i64),
        layout.content_width() as f32,
        layout.content_height() as f32
    );

    operations::draw_frame(&mut card, frame);

    let (draw_text_inputs, skills_text_height) = operations::generate_draw_text_details(
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

    operations::draw_skills_text_box(
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

    operations::draw_outline_text_mut(
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
        6,
        Rgba([0u8, 0u8, 0u8, 255u8])
    );

    operations::draw_health(
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

