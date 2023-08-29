
const CARD_WIDTH: u32 = 1024;
const CARD_HEIGHT: u32 = 1438;

const CONTENT_TOP_LEFT_X: u32 = 110;
const CONTENT_TOP_LEFT_Y: u32 = 141;
const CONTENT_BOT_RIGHT_X: u32 = CARD_WIDTH - 87;
const CONTENT_BOT_RIGHT_Y: u32 = CARD_HEIGHT - 91;

const SKILLS_BOX_MARGIN: u32 = 30;
const SKILLS_BOX_OUTLINE: u32 = 3;
const SKILLS_BOX_GAP: u32 = 5;
const SKILLS_BOX_CORNER: u32 = 10;
const SKILLS_BOX_PADDING: u32 = 25;

const SKILLS_TEXT_SCALE: u32 = 30;
const SKILLS_TEXT_SPACE_WIDTH: u32 = 13;

const NAME_TEXT_SCALE: u32 = 90;
const NAME_INNER_OUTLINE: u32 = 4;
const NAME_OUTER_OUTLINE: u32 = 6;
const HEALTH_LENGTH: u32 = 100;
const HEALTH_OFFSET: u32 = 50;

pub struct Layout {
    card_width: u32,
}

pub fn new_layout(card_width: u32) -> Layout {
    Layout { card_width }
}

impl Layout {
    pub fn card_width(&self) -> u32 {
        self.card_width
    }

    pub fn card_height(&self) -> u32 {
        self.scale_value(CARD_HEIGHT)
    }

    // Coordinates
    pub fn content_top_left_x(&self) -> u32 {
        self.scale_value(CONTENT_TOP_LEFT_X)
    }

    pub fn content_top_left_y(&self) -> u32 {
        self.scale_value(CONTENT_TOP_LEFT_Y)
    }

    pub fn content_bot_right_x(&self) -> u32 {
        self.scale_value(CONTENT_BOT_RIGHT_X)
    }
    
    pub fn content_bot_right_y(&self) -> u32 {
        self.scale_value(CONTENT_BOT_RIGHT_Y)
    }

    pub fn skills_box_top_left_x(&self) -> u32 {
        self.content_top_left_x() + self.skills_box_margin()
    }

    pub fn skills_box_top_left_y(&self, skills_text_height: u32) -> u32 {
        self.content_bot_right_y() - (
            self.skills_box_height(skills_text_height) +
            self.skills_box_margin()
        )
    }

    pub fn skills_text_top_left_x(&self) -> u32 {
        self.skills_box_top_left_x() + 
            self.skills_box_outline() +
            self.skills_box_gap() +
            self.skills_box_padding()
    }

    pub fn skills_text_top_left_y(&self, skills_text_height: u32) -> u32 {
        self.skills_box_top_left_y(skills_text_height) +
            self.skills_box_outline() +
            self.skills_box_gap() +
            self.skills_box_padding()
    }

    pub fn name_top_left_x(&self) -> u32 {
        self.skills_text_top_left_x()
    }

    pub fn name_top_left_y(&self, skills_text_height: u32) -> u32 {
        self.skills_box_top_left_y(skills_text_height) - (
            self.skills_box_margin()/2 +
            self.name_text_scale()
        )
    }

    pub fn health_top_right_x(&self) -> u32 {
        self.content_bot_right_x() - (
            self.skills_box_padding() +
            self.skills_box_outline() +
            self.skills_box_gap() +
            self.skills_box_padding() +
            self.health_length() * 2/3
        )
    }

    pub fn health_top_right_y(&self, skills_text_height: u32) -> u32 {
        self.name_top_left_y(skills_text_height)
    }

    // Sizing
    pub fn skills_box_margin(&self) -> u32 {
        self.scale_value(SKILLS_BOX_MARGIN)
    }

    pub fn skills_box_outline(&self) -> u32 {
        self.scale_value(SKILLS_BOX_OUTLINE)
    }

    pub fn skills_box_gap(&self) -> u32 {
        self.scale_value(SKILLS_BOX_GAP)
    }

    pub fn skills_box_corner(&self) -> u32 {
        self.scale_value(SKILLS_BOX_CORNER)
    }

    pub fn skills_box_padding(&self) -> u32 {
        self.scale_value(SKILLS_BOX_PADDING)
    }

    pub fn skills_text_scale(&self) -> u32 {
        self.scale_value(SKILLS_TEXT_SCALE)
    }
 
    pub fn skills_text_space_width(&self) -> u32 {
        self.scale_value(SKILLS_TEXT_SPACE_WIDTH)
    }

    pub fn name_text_scale(&self) -> u32 {
        self.scale_value(NAME_TEXT_SCALE)
    }

    pub fn name_inner_outline(&self) -> u32 {
        self.scale_value(NAME_INNER_OUTLINE)
    }

    pub fn name_outer_outline(&self) -> u32 {
        self.scale_value(NAME_OUTER_OUTLINE)
    }

    pub fn health_length(&self) -> u32 {
        self.scale_value(HEALTH_LENGTH)
    }

    pub fn health_offset(&self) -> u32 {
        self.scale_value(HEALTH_OFFSET)
    }

    // Derived sizing
    pub fn content_width(&self) -> u32 {
        self.content_bot_right_x() - self.content_top_left_x()
    }

    pub fn content_height(&self) -> u32 {
        self.content_bot_right_y() - self.content_top_left_y()
    }
    
    pub fn skills_box_width(&self) -> u32 {
        self.content_width() - 2 * self.skills_box_margin() 
    }

    pub fn skills_box_height(&self, skills_text_height: u32) -> u32 {
        skills_text_height + 2 * (
            self.skills_box_padding() +
            self.skills_box_outline() +
            self.skills_box_gap()
        )
    }

    pub fn skills_text_width(&self) -> u32 {
        self.skills_box_width() - 2 * (
            self.skills_box_outline() +
            self.skills_box_gap() +
            self.skills_box_padding()
        )
    }

    // Scaling helper
    fn scale_value(&self, value: u32) -> u32 {
        (value as f32 * self.card_scale()) as u32
    }
 
    fn card_scale(&self) -> f32 {
        self.card_width as f32 / CARD_WIDTH as f32
    }
}