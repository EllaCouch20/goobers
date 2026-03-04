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
use crate::theme::Theme;
use crate::classes::{Story, Level, Language, Word};
use crate::components::language::WordCard;
use ptsd::colors;

#[derive(Debug, Component, Clone)]
pub struct NewWord(Stack, Page);
impl OnEvent for NewWord {}
impl AppPage for NewWord {}
impl NewWord {
    pub fn new(theme: &Theme, word: Word) -> Self {
        let foreign = WordCard::foreign(theme, &word, false);
        let local = WordCard::local(theme, &word);
        let divider = Bin(Stack::new(Offset::Center, Offset::Center, Size::Fill, Size::Static(1.0), Padding::default()), Rectangle::new(theme.colors().get(colors::Outline::Primary), 0.0, None));
        let content = Content::new(Offset::Start, drawables![foreign, divider, local], Box::new(|_| true));
        let header = Header::stack(theme, "New word", None);
        let bumper = Bumper::stack(theme, Some("OK"), Box::new(|ctx: &mut Context, theme: &Theme| {}), None);
        let page = Page::new(header, content, Some(bumper));
        Self(Stack::default(), page)
    }
}