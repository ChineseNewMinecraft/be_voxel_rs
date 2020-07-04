use super::style::Style;
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use raqote::*;
use raqote::{
    DrawOptions, DrawTarget, PathBuilder, Point, SolidSource, Source, StrokeStyle, Transform,
};

pub trait Node {
    fn draw(&self, targer: &DrawTarget, style: Style);
    fn is_single_line(&self) -> bool;
    fn width(&self) -> f32;
    fn height(&self) -> f32;
}

pub trait ContainerNode {
    // span -> Node, div -> Container
    type InnerType;
    // fail in span like elements
    fn add_inner(&mut self, inner: Self::InnerType) -> Result<(), String>;
    // for example span inner is a div is not allowed
    fn set_inner_index(&mut self, index: usize, inner: Self::InnerType) -> Result<(), String>;
}

struct Span {
    inner: Vec<Box<dyn Node>>,
}

impl Node for Span {
    fn draw(&self, targer: &DrawTarget, style: Style) {
        // TODO: 老子不是写这的
    }
    fn is_single_line(&self) -> bool {
        true
    }
    fn width(&self) -> f32 {
        self.inner.iter().fold(0., |a, b| a + b.width())
    }
    fn height(&self) -> f32 {
        self.inner.iter().fold(0., |a, b| {
            let h = b.height();
            if h > a {
                h
            } else {
                a
            }
        })
    }
}

impl ContainerNode for Span {
    type InnerType = Box<dyn Node>;
    fn add_inner(&mut self, inner: Self::InnerType) -> Result<(), String> {
        if inner.is_single_line() {
            self.inner.push(inner);
            Ok(())
        } else {
            Err("Error! Span only allow single line element".to_string())
        }
    }
    fn set_inner_index(&mut self, index: usize, inner: Self::InnerType) -> Result<(), String> {
        self.inner[index] = inner;
        Ok(())
    }
}

// TODO: 补一下DIV之类的
