use prism::layout::{Size, Offset, Padding, Stack};
use prism::display::Bin;
use prism::event::OnEvent;
use prism::drawable::Component;
use crate::components::Rectangle;
use prism::drawables;
use crate::interface::general::{Page, Content, Header, Bumper};
use crate::interface::navigation::AppPage;
use crate::interface::navigation::{Flow, FlowContainer};
use crate::theme::Theme;
use crate::classes::Word;
use crate::components::language::{WordCard, Form};
use ptsd::colors;

#[derive(Debug, Component, Clone)]
pub struct WordDefinitionFlow(Stack, Flow);
impl OnEvent for WordDefinitionFlow {}
impl FlowContainer for WordDefinitionFlow{fn flow(&mut self) -> &mut Flow {&mut self.1}}
impl WordDefinitionFlow {
    pub fn new(theme: &Theme, word: Word) -> Self {
        WordDefinitionFlow(Stack::default(), Flow::new(vec![Box::new(WordDefinition::new(theme, word))]))
    }
}

#[derive(Debug, Component, Clone)]
pub struct WordDefinition(Stack, Page);
impl OnEvent for WordDefinition {}
impl AppPage for WordDefinition {}
impl WordDefinition {
    pub fn new(theme: &Theme, word: Word) -> Self {
        let foreign = WordCard::foreign(theme, &word, true);
        let local = Form::new(&theme, &("Translation".to_string(), word.translation.clone()));
        let example = Form::new(&theme, &("Example".to_string(), word.example.clone()));
        let class = Form::new(&theme, &("Class".to_string(), word.class.get().to_lowercase()));
        let divider = Bin(Stack::new(Offset::Center, Offset::Center, Size::Fill, Size::Static(1.0), Padding::default()), Rectangle::new(theme.colors().get(colors::Outline::Primary), 0.0, None));

        let mut content = drawables![foreign, divider, local, example, class];
        word.forms().iter().for_each(|form| content.push(Box::new(Form::new(&theme, form))));
        let content = Content::new(Offset::Start, content, Box::new(|_| true));
        let header = Header::stack(theme, "Definition", None);
        let bumper = Bumper::stack_end(theme, Some(1));
        let page = Page::new(header, content, Some(bumper));
        Self(Stack::default(), page)
    }
}
