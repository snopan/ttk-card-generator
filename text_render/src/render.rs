use image::{Rgba, ImageBuffer, Pixel};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Scale, Font};
use super::text::types::{Segment, Format};


pub struct RenderConfig<'a> {
    font_regular: &'a Font<'a>,
    font_bold: &'a Font<'a>,
    font_scale: Scale,
    default_font_color: Rgba<u8>,
    space_width: u32,
    new_line_height: u32,
}

impl<'a> RenderConfig<'a> {
    pub fn new(
        font_regular: &'a Font,
        font_bold: &'a Font,
        font_scale: Scale,
        default_font_color: Rgba<u8>
    ) -> RenderConfig<'a> {
        RenderConfig {
            font_regular: font_regular,
            font_bold: font_bold,
            font_scale,
            default_font_color,
            space_width: font_scale.x as u32,
            new_line_height: font_scale.y as u32
        }
    }

    pub fn set_font_bold(&'a mut self, font: &'a Font) {
        self.font_bold = font;
    }

    pub fn set_space_width(&mut self, width: u32) {
        self.space_width = width;
    }

    pub fn set_new_line_height(&mut self, height: u32) {
        self.new_line_height = height;
    }
}


pub struct DrawTextInput<'a> {
    color: Rgba<u8>,
    offset: (u32, u32),
    font: &'a Font<'a>,
    font_scale: Scale,
    word: String,
}

pub fn generate_draw_text_details(text: Vec<Segment>, box_width: u32, render_config: RenderConfig) -> (Vec<DrawTextInput>, u32) {
    let mut current_offset: (u32, u32) = (0, 0);

    let mut draw_text_inputs: Vec<DrawTextInput> = vec![];
    for segment in text {

        // If it's a new line then reset the offset to a new line
        if segment == Segment::NewLine {
            current_offset.0 = 0;
            current_offset.1 += render_config.new_line_height;
            continue;
        }

        // Otherwise it should be a word segment so retrieve the relevant information
        let Segment::Word(word, formats) = segment else {
            panic!("invalid segment enum");
        };

        // Set default inputs for a normal word segment
        let mut dti = DrawTextInput {
            color: render_config.default_font_color.clone(),
            offset: current_offset.clone(),
            font: render_config.font_regular,
            font_scale: render_config.font_scale.clone(),
            word
        };

        // Apply changes to the draw input based on the formats
        for f in formats {
            match f {
                Format::Bold => {
                    dti.font = render_config.font_bold;
                },
                Format::Color(color) => {
                    dti.color = color;
                }
            }
        }

        // Find out how much space this text will take
        let (text_width, _)  = text_size(dti.font_scale.clone(), dti.font, dti.word.as_str());
        
        // When the text have overflown, set offset to new line and also update draw input offset to that
        if current_offset.0 + text_width as u32 > box_width {
            current_offset.0 = 0;
            current_offset.1 += render_config.font_scale.y as u32;
            dti.offset = current_offset.clone();
        }
        
        // Finally push this draw text input
        draw_text_inputs.push(dti);

        // Add the text width and another space to the current offset for next word
        current_offset.0 += text_width as u32 + render_config.space_width;
    }
    
    (draw_text_inputs, current_offset.1 + render_config.font_scale.y as u32)
}

pub fn render_draw_text_inputs(canvas: &mut ImageBuffer<Rgba<u8>, Vec<<Rgba<u8> as Pixel>::Subpixel>>, draw_text_inputs: Vec<DrawTextInput>, box_offset: (i32, i32)) {
    for dti in draw_text_inputs {
        draw_text_mut(
            canvas, 
            dti.color,
            dti.offset.0 as i32 + box_offset.0, 
            dti.offset.1 as i32 + box_offset.1,
            dti.font_scale, 
            dti.font,
            dti.word.as_str()
        )
    }
}

mod tests {
    #[test]
    fn test_generate_draw_text_details() {
        let font_regular = Vec::from(include_bytes!("../test/regular.ttf") as &[u8]);
        let font_regular: Font<'static> = Font::try_from_vec(font_regular).unwrap();


    }
}