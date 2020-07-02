use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use raqote::*;
use raqote::{
    DrawOptions, DrawTarget, PathBuilder, Point, SolidSource, Source, StrokeStyle, Transform,
};
use std::f32::consts::PI;
#[derive(Clone, Copy)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

// position: (start, end) for a text frame
pub fn draw_text_frame(
    dt: &mut DrawTarget,
    font: &font_kit::font::Font,
    text: String,
    text_align: TextAlign,
    position: (Point, Point),
    point_size: f32,
    src: &Source,
    options: &DrawOptions,
) {
    use TextAlign::*;
    let mut position = position;
    // 2.8 is a exp constant for making the font y axis middle stay in y axis middle of the text frame
    position.0.y += point_size / 2.8;
    position.1.y += point_size / 2.8;
    let mut start = pathfinder_geometry::vector::vec2f(position.0.x, position.0.y);

    let mut ids = Vec::new();
    let mut positions = Vec::new();
    match text_align {
        Left => {
            for c in text.chars() {
                let id = font.glyph_for_char(c).unwrap();
                ids.push(id);
                positions.push(Point::new(start.x(), start.y()));
                start += font.advance(id).unwrap() * point_size / 24. / 96.;
            }
            dt.draw_glyphs(font, point_size, &ids, &positions, src, options);
        }
        Center => {
            let mut width: f32 = 0.;
            for c in text.chars() {
                let id = font.glyph_for_char(c).unwrap();
                let moved = font.advance(id).unwrap() * point_size / 24. / 96.;
                width += moved.x();
            }

            let center_start_x = position.0.x + (((position.1 - position.0).x - width) / 2.);
            start.set_x(center_start_x);
            for c in text.chars() {
                let id = font.glyph_for_char(c).unwrap();
                ids.push(id);
                positions.push(Point::new(start.x(), start.y()));
                start += font.advance(id).unwrap() * point_size / 24. / 96.;
            }
            dt.draw_glyphs(font, point_size, &ids, &positions, src, options);
        }
        Right => {
            let mut width: f32 = 0.;
            for c in text.chars() {
                let id = font.glyph_for_char(c).unwrap();
                let moved = font.advance(id).unwrap() * point_size / 24. / 96.;
                width += moved.x();
            }
            let center_start_x = position.0.x + ((position.1 - position.0).x - width);
            start.set_x(center_start_x);
            for c in text.chars() {
                let id = font.glyph_for_char(c).unwrap();
                ids.push(id);
                positions.push(Point::new(start.x(), start.y()));
                start += font.advance(id).unwrap() * point_size / 24. / 96.;
            }
            dt.draw_glyphs(font, point_size, &ids, &positions, src, options);
        }
    }
}

pub fn draw_button(dt: &mut DrawTarget, text: &str, size: (f32, f32), pos: Point, radius: f32) {
    let left_top = pos;
    let right_top = Point::new(pos.x + size.0, pos.y);

    let middle_y = pos.y + (size.1 / 2.);

    let mut pb = PathBuilder::new();
    pb.move_to(pos.x, pos.y + radius);
    pb.arc(pos.x + radius, pos.y + radius, radius, PI, PI / 2.);
    pb.line_to(pos.x + size.0 - radius, pos.y);
    pb.arc(
        pos.x + size.0 - radius,
        pos.y + radius,
        radius,
        1.5 * PI,
        PI / 2.,
    );
    pb.line_to(pos.x + size.0, pos.y + radius);
    pb.arc(
        pos.x + size.0 - radius,
        pos.y + size.1 - radius,
        radius,
        0.,
        PI / 2.,
    );
    pb.line_to(pos.x + radius, pos.y + size.1);
    pb.arc(
        pos.x + radius,
        pos.y + size.1 - radius,
        radius,
        PI / 2.,
        PI / 2.,
    );
    pb.line_to(pos.x, pos.y + radius);
    pb.close();
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::Title("Arial".into())], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let path = pb.finish();
    dt.stroke(
        &path,
        &Source::Solid(SolidSource {
            a: 0xff,
            r: 0x66,
            g: 0xcc,
            b: 0xff,
        }),
        &StrokeStyle {
            cap: LineCap::Round,
            join: LineJoin::Round,
            width: 1.,
            miter_limit: 2.,
            dash_array: vec![5., 9.],
            dash_offset: 2.,
        },
        &DrawOptions::new(),
    );
    draw_text_frame(
        dt,
        &font,
        text.to_string(),
        TextAlign::Center,
        (
            Point::new(left_top.x, middle_y),
            Point::new(right_top.x, middle_y),
        ),
        32.,
        &Source::Solid(SolidSource::from_unpremultiplied_argb(
            0xff, 0xff, 0xcc, 0x66,
        )),
        &DrawOptions::new(),
    );
    crate::utils::ui_debug::ui_debug(&dt.get_data_u8(), (dt.width() as u32, dt.height() as u32));
}
