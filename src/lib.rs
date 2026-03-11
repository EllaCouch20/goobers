use prism::event::Event;
use prism::{Context, canvas::{self}};
use crate::interface::general::Interface;
use crate::interface::navigation::RootInfo;

use crate::pages::small_story::SmallStory;
use crate::pages::new_word::NewWord;

use crate::classes::{Story, Level, Language, Word, WordClass, NounForms, VerbForms};
use crate::pages::word_order::WordOrder;

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
    let home = WordOrder::new(&theme, vec![
        Word::new("er", "Introductory word meaning 'there'", "there", "Er staat een huis in het bos.", WordClass::Adverb, Language::Dutch),
        Word::new("was", "Past tense of zijn", "was", "Het was een koude dag.", WordClass::Verb(VerbForms::new("zijn","ben","bent","is","zijn","was","was","was","waren","geweest")), Language::Dutch),
        Word::new("eens", "Once (storytelling)", "once", "Lang eens geleden leefde een koning.", WordClass::Adverb, Language::Dutch),
        Word::new("een", "Indefinite article", "a", "Ik zie een vogel.", WordClass::Article, Language::Dutch),
        Word::new("meisje", "Young girl", "girl", "Het meisje speelt buiten.", WordClass::Noun(NounForms { plural: "meisjes".into(), diminutive: Some("meisje".into()) }), Language::Dutch),
    ]);
    // let home = SmallStory::new(&theme, Story::Cinderella, Level::A1, Language::Dutch, 0);
    let home = RootInfo::icon("explore", "My Tickets", Box::new(home));
    Interface::new(&theme, vec![home], Box::new(|_page: &mut Box<dyn Drawable>, _ctx: &mut Context, e: Box<dyn Event>| {
        vec![e]
    }))
}}
