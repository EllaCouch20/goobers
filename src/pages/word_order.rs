use prism::layout::{Size, Offset, Padding, Stack, Area, Wrap};
use prism::display::Bin;
use prism::event::{Event, OnEvent};
use prism::drawable::{Component, Drawable, SizedTree};
use crate::components::Rectangle;
use prism::{Request, Context, drawables};
use crate::interface::general::{Page, Content, Header, Bumper};
use crate::interface::navigation::AppPage;
// use crate::components::language::EditableWords;
use crate::components::button::SecondaryButton;

use crate::classes::{Story, Level, Language, Word};
use ptsd::colors;
use crate::theme::Theme;

#[derive(Debug, Component, Clone)]
pub struct WordOrder(Stack, Page);
impl OnEvent for WordOrder {}
impl AppPage for WordOrder {}
impl WordOrder {
    pub fn new(theme: &Theme, sentence: Vec<Word>) -> Self {
        let divider = Bin(Stack::new(Offset::Center, Offset::Center, Size::Fill, Size::Static(1.0), Padding::default()), Rectangle::new(theme.colors().get(colors::Outline::Primary), 0.0, None));

        let content = drawables![
            Bin(Stack::new(Offset::Start, Offset::Start, Size::Fill, Size::Fill, Padding::default()), EditableWords::new(theme, vec![])), 
            divider.clone(),
            StoredWords::new(theme, sentence.to_vec()), 
        ];

        let content = Content::new(Offset::Start, content, Box::new(|_| true));
        let header = Header::stack(theme, "Word order", None);
        let bumper = Bumper::stack(theme, Some("Done"), Box::new(|_ctx: &mut Context, _theme: &Theme| {}), None);
        let page = Page::new(header, content, Some(bumper));
        Self(Stack::default(), page)
    }
}

#[derive(Debug, Component, Clone)]
pub struct EditableWords(Wrap, Vec<WordPill>, #[skip] Theme);
impl OnEvent for EditableWords {
    fn on_event(&mut self, ctx: &mut Context, sized: &SizedTree, event: Box<dyn Event>) -> Vec<Box<dyn Event>> {
        if let Some(event) = event.downcast_ref::<EditableWordEvent>() {
            match event {
                EditableWordEvent::RemoveWord(i) => if let Some(pos) = self.1.iter().position(|word| word.2 == *i) {
                    let removed = self.1.remove(pos);
                    ctx.send(Request::event(EditableWordEvent::Store(removed.3)))
                },
                EditableWordEvent::Relist(word) => {
                    let i = self.1.len();
                    self.1.push(WordPill::new(
                        SecondaryButton::medium(&self.2, &word.word.clone(), None, None, move |ctx: &mut Context, theme: &Theme| {
                            ctx.send(Request::event(EditableWordEvent::RemoveWord(i)))
                        }), i, word.clone()
                    ))
                },
                _ => {}
            }
        }
        vec![event]
    }
}

impl EditableWords {
    pub fn new(theme: &Theme, words: Vec<Word>) -> Self {
        EditableWords(Wrap::start(8.0, 8.0), words.into_iter().enumerate().map(|(i, word)| 
            WordPill::new(SecondaryButton::medium(theme, &word.word.clone(), None, None, move |ctx: &mut Context, theme: &Theme| {
                ctx.send(Request::event(EditableWordEvent::RemoveWord(i)))
            }), i, word.clone())).collect::<Vec<_>>(), theme.clone()
        )
    }
}

#[derive(Debug, Component, Clone)]
pub struct StoredWords(Wrap, Vec<WordPill>, #[skip] Theme);
impl OnEvent for StoredWords {
    fn on_event(&mut self, ctx: &mut Context, sized: &SizedTree, event: Box<dyn Event>) -> Vec<Box<dyn Event>> {
        if let Some(event) = event.downcast_ref::<EditableWordEvent>() {
            match event {
                EditableWordEvent::UseWord(i) => if let Some(pos) = self.1.iter().position(|word| word.2 == *i) {
                    let removed = self.1.remove(pos);
                    ctx.send(Request::event(EditableWordEvent::Relist(removed.3)))
                },
                EditableWordEvent::Store(word) => {
                    let i = self.1.len();
                    self.1.push(WordPill::new(
                        SecondaryButton::medium(&self.2, &word.word.clone(), None, None, move |ctx: &mut Context, theme: &Theme| {
                            ctx.send(Request::event(EditableWordEvent::UseWord(i)))
                        }), i, word.clone()
                    ))
                },
                _ => {}
            }
        }
        vec![event]
    }
}

impl StoredWords {
    pub fn new(theme: &Theme, words: Vec<Word>) -> Self {
        StoredWords(Wrap::start(8.0, 8.0), words.into_iter().enumerate().map(|(i, word)| 
            WordPill::new(SecondaryButton::medium(theme, &word.word.clone(), None, None, move |ctx: &mut Context, theme: &Theme| {
                ctx.send(Request::event(EditableWordEvent::UseWord(i)))
            }), i, word.clone())).collect::<Vec<_>>(), theme.clone()
        )
    }
}

#[derive(Debug, Component, Clone)]
pub struct WordPill(Stack, SecondaryButton, #[skip] usize, #[skip] Word);
impl OnEvent for WordPill {}
impl WordPill {
    pub fn new(button: SecondaryButton, i: usize, word: Word) -> Self {
        WordPill(Stack::default(), button, i, word)
    }
}

#[derive(Clone, Debug)]
pub enum EditableWordEvent {
    RemoveWord(usize),
    UseWord(usize),
    Relist(Word),
    Store(Word),
}

impl Event for EditableWordEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: &[Area]) -> Vec<Option<Box<dyn Event>>> {
        children.iter().map(|_| Some(self.clone() as Box<dyn Event>)).collect()
    }
}
