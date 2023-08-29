use image::imageops::{overlay, resize, FilterType};
use image::io::Reader;
use image::{Rgba, Rgb, RgbImage, RgbaImage, DynamicImage, Pixel, ImageBuffer};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut, Canvas};
use imageproc::rect::Rect;
use text_render::render::{RenderConfig, generate_draw_text_details, render_draw_text_inputs};
use rusttype::{Font, Scale};
use text_render::text::split_text;
use std::{env, cmp, fmt};

mod config;

const CARD_WIDTH: u32 = 1024;
const CARD_HEIGHT: u32 = 1438;

// These are percentage size given that card width is 1024 and heigh is 1438
const CONTENT_TOP_LEFT_X: u32 = ((110.0 / 1024.0) * CARD_WIDTH as f32) as u32;
const CONTENT_TOP_LEFT_Y: u32 = ((141.0 / 1438.0) * CARD_HEIGHT as f32) as u32;
const CONTENT_BOTTOM_RIGHT_X: u32 = (((1024.0 - 87.0) / 1024.0) * CARD_WIDTH as f32) as u32;
const CONTENT_BOTTOM_RIGHT_Y: u32 = (((1438.0 - 91.0) as f32 / 1438.0) * CARD_HEIGHT as f32) as u32;
const CONTENT_WIDTH: u32 = CONTENT_BOTTOM_RIGHT_X - CONTENT_TOP_LEFT_X;
const CONTENT_HEIGHT: u32 = CONTENT_BOTTOM_RIGHT_Y - CONTENT_TOP_LEFT_X;

const SKILLS_MARGIN: u32 = ((41.0 / 1438.0) * CARD_HEIGHT as f32) as u32;
const SKILLS_PADDING: u32 = ((20.0 / 1438.0) * CARD_HEIGHT as f32) as u32;
const SKILLS_WIDTH: u32 = CONTENT_WIDTH - 2 * SKILLS_MARGIN;
const SKILLS_TEXT_WIDTH: u32 = SKILLS_WIDTH - 2 * SKILLS_PADDING;

fn main() {

    let config = config::load_config(String::from("./temp_config.json")).unwrap();

    // println!("{:?}", config);

    let mut image = RgbaImage::new(CARD_WIDTH, CARD_HEIGHT);

    // let character = config.characters[0];

    let avatar = Reader::open(&config.characters[0].picture).unwrap().decode().unwrap();
    let (avatar_width, avatar_height) = get_avatar_size(avatar.width(), avatar.height());
    let avatar = resize::<DynamicImage>(&avatar, avatar_width, avatar_height, FilterType::Lanczos3);
    overlay(&mut image, &avatar, CONTENT_TOP_LEFT_X as i64, CONTENT_TOP_LEFT_Y as i64);

    let frame = Reader::open(config.frames.shu_zhu).unwrap().decode().unwrap();
    let frame = resize::<DynamicImage>(&frame, CARD_WIDTH, CARD_HEIGHT, FilterType::Lanczos3);
    overlay(&mut image, &frame, 0, 0);


    let font_regular = Font::try_from_vec(Vec::from(include_bytes!("../asset/regular.ttf") as &[u8])).unwrap();
    let font_bold = Font::try_from_vec(Vec::from(include_bytes!("../asset/bold.ttf") as &[u8])).unwrap();

    let font_scale = Scale{ x: 30.0, y: 30.0 };
    let skills_text = make_skills_text(&config.characters[0].skills);
    let parsed_text = split_text(skills_text);
    
    let mut render_config = RenderConfig::new(
        &font_regular,
        &font_bold,
        font_scale,
        Rgba([0u8, 0u8, 0u8, 255u8])
    );
    render_config.set_space_width(13);

    let (draw_text_inputs, skills_text_height) = generate_draw_text_details(
        parsed_text, 
        SKILLS_TEXT_WIDTH, 
        render_config
    );

    let skills_height = skills_text_height + 2 * SKILLS_PADDING;
    let skills_text_box = make_skills_text_box(
        SKILLS_WIDTH,
        skills_height, 
        3,
        5,
        10,
        Rgba([255u8, 255u8, 255u8, 150u8])
    );
    overlay(
        &mut image,
        &skills_text_box,
        (CONTENT_TOP_LEFT_X + SKILLS_MARGIN) as i64,
        (CONTENT_BOTTOM_RIGHT_Y - SKILLS_MARGIN - skills_height) as i64
    );

    render_draw_text_inputs(
        &mut image,
        draw_text_inputs,
        (
            (CONTENT_TOP_LEFT_X + SKILLS_MARGIN + SKILLS_PADDING) as i32,
            (CONTENT_BOTTOM_RIGHT_Y - SKILLS_MARGIN - SKILLS_PADDING - skills_text_height) as i32,
        )
    );

    let font_title = Font::try_from_vec(Vec::from(include_bytes!("../asset/title.ttf") as &[u8])).unwrap();
    draw_outline_text_mut(
        &mut image, 
        Rgba([255u8, 255u8, 255u8, 255u8]),
        (CONTENT_TOP_LEFT_X + SKILLS_MARGIN + SKILLS_PADDING) as i32,
        (CONTENT_BOTTOM_RIGHT_Y - SKILLS_MARGIN - skills_height - 100) as i32,
        Scale { x: 90.0, y: 90.0 }, 
        &font_title,
        &config.characters[0].name,
        4,
        Rgba([0u8, 0u8, 0u8, 255u8])
    );

    let _ = image.save(String::from("./output/image.png")).unwrap();

    // let canvas_size = (550, 880);
    // let mut image = RgbImage::new(canvas_size.0, canvas_size.1);





    // let box_height = cmp::max(text_height + box_padding as u32 * 2, 300);

    // let space_below_box = 20;
    // let box_offset = ((canvas_size.0 - box_width) / 2, canvas_size.1 - space_below_box - box_height);



    // let _ = image.save(path).unwrap();
}

fn get_avatar_size(width: u32, height: u32) -> (u32, u32) {
    let current_ratio = width as f32 / height as f32;
    let required_ratio = CONTENT_WIDTH as f32 / CONTENT_HEIGHT as f32;

    // It means that the width is longer than the required width
    // so we keep height the same and modify width
    if current_ratio > required_ratio {
        return ((CONTENT_HEIGHT as f32 * current_ratio) as u32, CONTENT_HEIGHT)
    }

    // Height is longer than the required height
    else {
        return (CONTENT_WIDTH, (CONTENT_WIDTH as f32 / current_ratio) as u32)
    }
}

fn make_skills_text(skills: &Vec<config::Skill>) -> String {
    skills.iter().map(|s| -> String {
        let name = s.name.split(" ").map(|x| format!("@bold;{}", x)).collect::<Vec<String>>().join(" ");
        format!("{}\n{}", name, s.description)
    }).collect::<Vec<String>>().join("\n\n")
}

fn make_skills_text_box(width: u32, height: u32, outline_size: u32, gap_size: u32, corner_size: u32, color: Rgba<u8>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut image: ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(width, height);
    let transparent = Rgba([0u8, 0u8, 0u8, 0u8]);

    draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(width, height), color);
    
    let corners: Vec<(i32, i32)> = vec![
        (0, 0),
        (0, (height-corner_size) as i32),
        ((width-corner_size) as i32, 0),
        ((width-corner_size) as i32, (height-corner_size) as i32),
    ];
    for (x, y) in corners {
        draw_filled_rect_mut(&mut image, Rect::at(x, y).of_size(corner_size, corner_size), transparent);
    }

    let gaps: Vec<(i32, i32, u32, u32)> = vec![
        ((corner_size+outline_size) as i32, outline_size as i32, width-2*(outline_size+corner_size), gap_size),
        ((corner_size+outline_size) as i32, (height-outline_size-gap_size) as i32, (width-2*(outline_size+corner_size)), gap_size),
        (outline_size as i32, (corner_size+outline_size) as i32, gap_size, (height-2*(outline_size+corner_size))),
        ((width-outline_size-gap_size) as i32, (corner_size+outline_size) as i32, gap_size, (height-2*(outline_size+corner_size))),
    ];
    for (x, y, w, h) in gaps {
        draw_filled_rect_mut(&mut image, Rect::at(x, y).of_size(w, h), transparent);
    }

    let corner_gap_size = corner_size+outline_size;
    let corner_gap_offset = corner_size+outline_size+gap_size;
    draw_filled_rect_mut(&mut image, Rect::at(outline_size as i32, (corner_size+outline_size) as i32).of_size(corner_gap_size, gap_size), transparent);
    draw_filled_rect_mut(&mut image, Rect::at((corner_size+outline_size) as i32, outline_size as i32).of_size(gap_size, corner_gap_size), transparent);
    
    draw_filled_rect_mut(&mut image, Rect::at(outline_size as i32, (height-corner_gap_offset) as i32).of_size(corner_gap_size, gap_size), transparent);
    draw_filled_rect_mut(&mut image, Rect::at((corner_size+outline_size) as i32, (height-corner_gap_offset) as i32).of_size(gap_size, corner_gap_size), transparent);

    draw_filled_rect_mut(&mut image, Rect::at((width-corner_gap_offset) as i32, outline_size as i32).of_size(gap_size, corner_gap_size), transparent);
    draw_filled_rect_mut(&mut image, Rect::at((width-corner_gap_offset) as i32, (corner_size+outline_size) as i32).of_size(corner_gap_size, gap_size), transparent);

    draw_filled_rect_mut(&mut image, Rect::at((width-corner_gap_offset) as i32, (height-corner_gap_offset) as i32).of_size(gap_size, corner_gap_size), transparent);
    draw_filled_rect_mut(&mut image, Rect::at((width-corner_gap_offset) as i32, (height-corner_gap_offset) as i32).of_size(corner_gap_size, gap_size), transparent);

    image
}

fn draw_outline_text_mut<'a>(canvas: &mut ImageBuffer<Rgba<u8>, Vec<<Rgba<u8> as Pixel>::Subpixel>>, color: Rgba<u8>, x: i32, y: i32, scale: Scale, font: &'a Font<'a>, text: &'a str, outline_size: i32, outline_color: Rgba<u8>) {
    let offset: Vec<(i32, i32)> = vec![
        (0, outline_size),
        (0, -outline_size),
        (outline_size, 0),
        (-outline_size, 0),
    ];

    for (x_offset, y_offset) in offset {
        draw_text_mut(canvas, outline_color, x + x_offset, y + y_offset, scale, font, text);
    }
    draw_text_mut(canvas, color, x, y, scale, font, text);
}