
use prism::layout::{Size, Offset, Padding, Stack, Wrap, Row};
use prism::display::Bin;
use prism::event::OnEvent;
use prism::drawable::Component;
use crate::components::text::{TextStyle, Text, TextSize};
use prism::{Request, Context, canvas::Align};
use crate::components::button::SecondaryButton;
use crate::canvas::{RgbaImage, Image, ShapeType, Shape};
use crate::interface::navigation::NavigationEvent;
use crate::classes::{Language, Word};
use crate::pages::definition::WordDefinitionFlow;
use crate::theme::{Theme, Color};
use std::sync::Arc;

#[derive(Debug, Component, Clone)]
pub struct WordCard(Stack, LanguageCoin, Bin<Stack, Text>);
impl OnEvent for WordCard {}
impl WordCard {
    pub fn foreign(theme: &Theme, word: &Word, hug: bool) -> Self {
        let text = Text::new(&theme, &word.word, TextSize::H4, TextStyle::Heading, Align::Center, None);
        let layout = Stack::new(Offset::Start, Offset::Start, Size::Fill, if hug {Size::Fit} else {Size::custom(|h| (h[0].0, 250.0))}, Padding::default());
        let stack = if hug {Stack::new(Offset::Center, Offset::Center, Size::Fill, Size::Fit, Padding::default())} else {Stack::fill()};
        WordCard(layout, LanguageCoin::new(theme, word.language), Bin(stack, text))
    }

    pub fn local(theme: &Theme, word: &Word) -> Self {
        let text = Text::new(&theme, &word.translation, TextSize::H4, TextStyle::Heading, Align::Center, None);
        let layout = Stack::new(Offset::Start, Offset::Start, Size::Fill, Size::custom(|h| (h[0].0, 250.0)), Padding::default());
        WordCard(layout, LanguageCoin::new(theme, Language::English), Bin(Stack::fill(), text))
    }
}

#[derive(Debug, Component, Clone)]
pub struct LanguageCoin(Stack, Shape, Image);
impl OnEvent for LanguageCoin {}
impl LanguageCoin {
    pub fn new(theme: &Theme, language: Language) -> Self {
        let image: Arc<RgbaImage> = theme.brand().language(language);
        let img = Image{shape: ShapeType::Ellipse(0.0, (25.0, 25.0), 0.0), image: image.clone(), color: None};
        let shape = Shape{shape: ShapeType::Ellipse(0.0, (28.0, 28.0), 0.0), color: Color::WHITE.into()};
        LanguageCoin(Stack::center(), shape, img)
    }
}

#[derive(Debug, Component, Clone)]
pub struct Form(Row, Text, Text);
impl OnEvent for Form {}
impl Form {
    pub fn new(theme: &Theme, form: &(String, String)) -> Self {
        let title = Text::new(&theme, &form.0, TextSize::H5, TextStyle::Heading, Align::Center, None);
        let subtitle = Text::new(&theme, &form.1, TextSize::Sm, TextStyle::Primary, Align::Center, None);
        Form(Row::center(12.0), title, subtitle)
    }
}

#[derive(Debug, Component, Clone)]
pub struct StorySentence(Wrap, Vec<SecondaryButton>);
impl OnEvent for StorySentence {}
impl StorySentence {
    pub fn new(theme: &Theme, words: Vec<Word>) -> Self {
        StorySentence(Wrap::start(8.0, 8.0), words.into_iter().map(|word| SecondaryButton::medium(theme, &word.word.clone(), None, None, move |ctx: &mut Context, theme: &Theme| {
            let flow = WordDefinitionFlow::new(theme, word.clone());
            ctx.send(Request::event(NavigationEvent::push(flow)))
        })).collect::<Vec<_>>())
    }
}