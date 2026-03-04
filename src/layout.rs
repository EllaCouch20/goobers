use prism::layout::{Area, SizeRequest, Size, Offset, Layout, Padding, Stack, Wrap, Row};
use prism::display::Bin;
use prism::event::{OnEvent, Event};
use prism::drawable::Component;
use crate::components::button::ButtonSize;
use crate::components::text::{TextStyle, Text, TextSize};
use crate::components::{Rectangle, TextInput};
use prism::{Request, Context, drawables, canvas::{self, Align}};
use ptsd::utils::ValidationFn;
use crate::components::button::PrimaryButton;
use crate::components::button::SecondaryButton;
use crate::components::text::ExpandableText;
use crate::canvas::{RgbaImage, Image, ShapeType, Shape};
use crate::interface::general::{Page, Content, Header, Bumper, Interface};
use crate::interface::navigation::AppPage;
use crate::interface::navigation::{Flow, FlowContainer};
use crate::interface::navigation::NavigationEvent;
use crate::interface::navigation::RootInfo;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ReversedStack(pub f32, pub Offset, pub Size, pub Padding);

impl ReversedStack {
    pub fn new(spacing: f32, offset: Offset, size: Size, padding: Padding) -> Self {
        ReversedStack(spacing, offset, size, padding)
    }

    pub fn center(spacing: f32) -> Self {
        ReversedStack(spacing, Offset::Center, Size::Fit, Padding::default())
    }

    pub fn start(spacing: f32) -> Self {
        ReversedStack(spacing, Offset::Start, Size::Fit, Padding::default())
    }

    pub fn end(spacing: f32) -> Self {
        ReversedStack(spacing, Offset::End, Size::Fit, Padding::default())
    }
}

impl Layout for ReversedStack {
    fn request_size(&self, children: Vec<SizeRequest>) -> SizeRequest {
        let n = children.len() as f32;
        let (w, h): (Vec<_>, Vec<_>) = children.into_iter()
            .map(|r| ((r.min_width(), r.max_width()), (r.min_height(), r.max_height())))
            .unzip();
        let w = self.2.get(w, Size::max);
        let h = self.2.get(h, Size::max);
        let extra = (n - 1.0).max(0.0) * self.0;
        self.3.adjust_request(SizeRequest::new(w.0, h.0 + extra, w.1, h.1 + extra))
    }

    fn build(&self, stack_size: (f32, f32), children: Vec<SizeRequest>) -> Vec<Area> {
        let s = self.3.adjust_size(stack_size);
        children.into_iter().enumerate().map(|(i, r)| {
            let size = r.get(s);
            Area {
                offset: self.3.adjust_offset((self.1.get(s.0, size.0), self.1.get(s.1, size.1) + i as f32 * self.0)),
                size
            }
        }).rev().collect()
    }
}
