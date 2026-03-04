use prism::layout::{Size, Offset, Padding, Stack};
use prism::display::Bin;
use prism::event::OnEvent;
use prism::drawable::{Component, Drawable};
use crate::components::Rectangle;
use prism::Context;
use crate::interface::general::{Page, Content, Header, Bumper};
use crate::interface::navigation::AppPage;
use crate::components::language::StorySentence;

use crate::classes::{Story, Level, Language};
use ptsd::colors;
use crate::theme::Theme;

#[derive(Debug, Component, Clone)]
pub struct SmallStory(Stack, Page);
impl OnEvent for SmallStory {}
impl AppPage for SmallStory {}
impl SmallStory {
    pub fn new(theme: &Theme, story: Story, level: Level, language: Language, part: usize) -> Self {
        let divider = Bin(Stack::new(Offset::Center, Offset::Center, Size::Fill, Size::Static(1.0), Padding::default()), Rectangle::new(theme.colors().get(colors::Outline::Primary), 0.0, None));

        let mut content: Vec<Box<dyn Drawable>> = Vec::new();
        story.get(language, level, part).iter().for_each(|sentence| {
            content.push(Box::new(StorySentence::new(theme, sentence.to_vec())));
            content.push(Box::new(divider.clone()));
        });

        let content = Content::new(Offset::Start, content, Box::new(|_| true));
        let header = Header::stack(theme, "Small story", None);
        let bumper = Bumper::stack(theme, Some("Continue"), Box::new(|_ctx: &mut Context, _theme: &Theme| {}), None);
        let page = Page::new(header, content, Some(bumper));
        Self(Stack::default(), page)
    }
}