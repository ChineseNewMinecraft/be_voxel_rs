use std::default::Default;
pub struct Style {
    pub font_style: FontStyle,
    pub fill: Option<Fill>,
    pub boarder_style: Option<BoarderStyle>,
}
pub enum FontInnerStyle {
    Normal,
    Italic,
    Oblique,
}
pub struct FontStyle {
    pub size: f32,
    pub family: String,
    // italic bold etc.
    pub style: FontInnerStyle,
    // 400 normal 700 bold
    pub weight: f32,
    // 1.0 is normal, 1.1 is 110%
    pub spacing: f32,
    pub color: u32,
}

pub struct Fill {
    // path
    pub image: Option<String>,
    // rgba
    pub color: u32,
}

pub struct BoarderStyle {
    pub color: u32,
    pub radius: f32,
    pub thickness: f32,
    // up down left right
    pub padding: (f32, f32, f32, f32),
    pub margin: (f32, f32, f32, f32),
}

impl Default for FontInnerStyle {
    fn default() -> Self {
        FontInnerStyle::Normal
    }
}

impl Default for FontStyle {
    fn default() -> Self {
        FontStyle {
            size: 32.,
            family: "Arial".to_string(),
            style: Default::default(),
            weight: 400.,
            spacing: 1.,
            color: 0xFFFFFFFF,
        }
    }
}

// TODO: blending cliping advanced stuff
