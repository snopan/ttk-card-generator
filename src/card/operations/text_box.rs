use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};

pub fn make_skills_text_box(width: u32, height: u32, outline_size: u32, gap_size: u32, corner_size: u32, color: Rgba<u8>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut text_box: ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(width, height);
    let transparent = Rgba([0u8, 0u8, 0u8, 0u8]);

    draw_filled_rect_mut(&mut text_box, Rect::at(0, 0).of_size(width, height), color);
    
    let corners: Vec<(i32, i32)> = vec![
        (0, 0),
        (0, (height-corner_size) as i32),
        ((width-corner_size) as i32, 0),
        ((width-corner_size) as i32, (height-corner_size) as i32),
    ];
    for (x, y) in corners {
        draw_filled_rect_mut(&mut text_box, Rect::at(x, y).of_size(corner_size, corner_size), transparent);
    }

    let corner_gap_size = corner_size+outline_size;
    let gaps: Vec<(i32, i32, u32, u32)> = vec![
        (corner_gap_size as i32, outline_size as i32, width-2*corner_gap_size, gap_size),
        (corner_gap_size as i32, (height-outline_size-gap_size) as i32, (width-2*corner_gap_size), gap_size),
        (outline_size as i32, corner_gap_size as i32, gap_size, (height-2*corner_gap_size)),
        ((width-outline_size-gap_size) as i32, corner_gap_size as i32, gap_size, (height-2*corner_gap_size)),
    ];
    for (x, y, w, h) in gaps {
        draw_filled_rect_mut(&mut text_box, Rect::at(x, y).of_size(w, h), transparent);
    }

    let corner_gap_offset = corner_size+outline_size+gap_size;
    draw_filled_rect_mut(&mut text_box, Rect::at(outline_size as i32, corner_gap_size as i32).of_size(corner_gap_size, gap_size), transparent);
    draw_filled_rect_mut(&mut text_box, Rect::at(corner_gap_size as i32, outline_size as i32).of_size(gap_size, corner_gap_size), transparent);
    
    draw_filled_rect_mut(&mut text_box, Rect::at(outline_size as i32, (height-corner_gap_offset) as i32).of_size(corner_gap_size, gap_size), transparent);
    draw_filled_rect_mut(&mut text_box, Rect::at(corner_gap_size as i32, (height-corner_gap_offset) as i32).of_size(gap_size, corner_gap_size), transparent);

    draw_filled_rect_mut(&mut text_box, Rect::at((width-corner_gap_offset) as i32, outline_size as i32).of_size(gap_size, corner_gap_size), transparent);
    draw_filled_rect_mut(&mut text_box, Rect::at((width-corner_gap_offset) as i32, corner_gap_size as i32).of_size(corner_gap_size, gap_size), transparent);

    draw_filled_rect_mut(&mut text_box, Rect::at((width-corner_gap_offset) as i32, (height-corner_gap_offset) as i32).of_size(gap_size, corner_gap_size), transparent);
    draw_filled_rect_mut(&mut text_box, Rect::at((width-corner_gap_offset) as i32, (height-corner_gap_offset) as i32).of_size(corner_gap_size, gap_size), transparent);

    text_box
}
