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

use ptsd::colors;
use std::collections::HashMap;
use crate::pages::small_story::SmallStory;
use std::sync::Arc;

use crate::classes::{Story, Level, Language};

mod stories;
mod classes;
mod pages;
mod layout;
mod utils;
mod components;
mod interface;
mod theme;
use theme::Button as ButtonColors;

use crate::theme::{Color, Theme, ButtonColorScheme};

ramp::run!{|ctx: &mut Context, assets: Assets| {
    let theme = Theme::dark(assets.all(), Color::from_hex("#EE3658", 255));
    // let home = NewWord::new(&assets, &theme, Word::new("naam", "Name", "name", "Mijn naam is Anna.", WordClass::Noun(NounForms { plural: "namen".into(), diminutive: None }), Language::Dutch));
    let home = SmallStory::new(&theme, Story::Cinderella, Level::A1, Language::Dutch, 0);
    let home = RootInfo::icon("explore", "My Tickets", Box::new(home));
    Interface::new(&theme, vec![home], Box::new(|page: &mut Box<dyn Drawable>, ctx: &mut Context, e: Box<dyn Event>| {
        vec![e]
    }))
}}
