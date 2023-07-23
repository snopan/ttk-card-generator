use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut};
use imageproc::rect::Rect;
use render::{RenderConfig, generate_draw_text_details, render_draw_text_inputs};
use rusttype::{Font, Scale};
use text::split_text;
use std::{env, cmp};
use std::path::Path;

mod render;
mod text;

fn main() {
    let arg = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a target file path")
    };

    let path = Path::new(&arg);

    let canvas_size = (550, 880);
    let mut image = RgbImage::new(canvas_size.0, canvas_size.1);

    let font_regular = Vec::from(include_bytes!("regular.ttf") as &[u8]);
    let font_regular: Font<'static> = Font::try_from_vec(font_regular).unwrap();
    let font_bold = Vec::from(include_bytes!("bold.ttf") as &[u8]);
    let font_bold: Font<'static> = Font::try_from_vec(font_bold).unwrap();
    
    let font_scale = Scale{ x: 30.0, y: 30.0 };

    let box_width = 510;
    let box_padding = 10;

    let text = String::from
    ("@bold;Horsemanship
    @bold;Enforced @bold;Ability: You will always have -1 distance in any range calculations.
    
    @bold;Ironsteed
    When you use @bold;[Strike] on any target player, you can choose to make a Judgement. If the Judgement Result is a @red;♥ or @red;♦, the @bold;[Strike] cannot be evaded");
    let parsed_text = split_text(text);

    let mut render_config = RenderConfig::new(&font_regular, &font_bold, font_scale, Rgb([0u8, 0u8, 0u8]));
    render_config.set_space_width(15);
    let (draw_text_inputs, box_height_without_padding) = generate_draw_text_details(
        parsed_text, 
        box_width - box_padding as u32 * 2, 
        render_config
    );
    let box_height = cmp::max(box_height_without_padding + box_padding as u32 * 2, 300);

    let space_below_box = 20;
    let box_offset = ((canvas_size.0 - box_width) / 2, canvas_size.1 - space_below_box - box_height);
    draw_filled_rect_mut(
        &mut image, 
        Rect::at(box_offset.0 as i32, box_offset.1 as i32)
            .of_size(box_width, box_height),
        Rgb([100u8, 100u8, 50u8])
    );

    render_draw_text_inputs(&mut image, draw_text_inputs, (box_offset.0 as i32 + box_padding, box_offset.1 as i32 + box_padding));

    let _ = image.save(path).unwrap();
}

