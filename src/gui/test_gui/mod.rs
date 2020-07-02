use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use raqote::*;
use raqote::{
    DrawOptions, DrawTarget, PathBuilder, Point, SolidSource, Source, StrokeStyle, Transform,
};
const PI:f32 = 3.1415926535;

pub fn draw_button(dt: &mut DrawTarget, text: &str, size: (f32, f32), pos: Point, radius: f32) {
    let mut pb = PathBuilder::new();
    pb.move_to(pos.x, pos.y + radius);
    pb.arc(pos.x + radius, pos.y + radius, radius, PI, PI/2.);
    pb.line_to(pos.x + size.0 - radius, pos.y);
    pb.arc(pos.x + size.0 - radius, pos.y + radius, radius, 1.5*PI, PI/2.);
    pb.line_to(pos.x + size.0, pos.y + radius);
    pb.arc(pos.x + size.0 - radius,pos.y + size.1 - radius, radius,0.,PI/2.);
    pb.line_to(pos.x + radius, pos.y + size.1);
    pb.arc(pos.x + radius, pos.y + size.1 - radius, radius, PI/2., PI/2.);
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
    dt.draw_text(&font, 32., text, Point::new(pos.x, pos.y + (size.1 - 32.)/2. + 32.), &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xcc, 0x66)), &DrawOptions::new());
    //TODO
    crate::utils::ui_debug::ui_debug(&dt.get_data_u8(), (dt.width() as u32, dt.height() as u32));
}