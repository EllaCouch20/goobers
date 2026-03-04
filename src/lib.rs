use prism::event::Event;
use prism::{Context, canvas::{self}};
use crate::interface::general::Interface;
use crate::interface::navigation::RootInfo;

use crate::pages::small_story::SmallStory;
use crate::pages::new_word::NewWord;

use crate::classes::{Story, Level, Language, Word, WordClass, NounForms};

mod stories;
mod classes;
mod pages;
mod layout;
mod utils;
mod components;
mod interface;
mod theme;

use crate::theme::{Color, Theme};

ramp::run!{|_ctx: &mut Context, assets: Assets| {
    let theme = Theme::dark(assets.all(), Color::from_hex("#EE3658", 255));
    let home = NewWord::new(&theme, Word::new("naam", "Name", "name", "Mijn naam is Anna.", WordClass::Noun(NounForms { plural: "namen".into(), diminutive: None }), Language::Dutch));
    // let home = SmallStory::new(&theme, Story::Cinderella, Level::A1, Language::Dutch, 0);
    let home = RootInfo::icon("explore", "My Tickets", Box::new(home));
    Interface::new(&theme, vec![home], Box::new(|_page: &mut Box<dyn Drawable>, _ctx: &mut Context, e: Box<dyn Event>| {
        vec![e]
    }))
}}
