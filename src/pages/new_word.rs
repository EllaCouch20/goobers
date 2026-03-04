use prism::layout::{Size, Offset, Padding, Stack};
use prism::display::Bin;
use prism::event::OnEvent;
use prism::drawable::Component;
use crate::components::Rectangle;
use prism::{Request, Context, drawables};
use crate::interface::general::{Page, Content, Header, Bumper};
use crate::interface::navigation::AppPage;
use crate::theme::Theme;
use crate::classes::Word;
use crate::components::language::WordCard;
use crate::interface::navigation::NavigationEvent;
use ptsd::colors;

use crate::pages::definition::WordDefinitionFlow;

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
        let bumper = Bumper::stack(theme, Some("OK"), Box::new(|_ctx: &mut Context, _theme: &Theme| {}), Some(("More".to_string(), Box::new(move |ctx: &mut Context, theme: &Theme| {
            let flow = WordDefinitionFlow::new(theme, word.clone());
            ctx.send(Request::event(NavigationEvent::push(flow)))
        }))));
        let page = Page::new(header, content, Some(bumper));
        Self(Stack::default(), page)
    }
}